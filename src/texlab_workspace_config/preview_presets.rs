use super::texlab_settings::*;
use crate::zed_command::CommandName;
use chrono::TimeZone;
use chrono::Utc;
use std::time::SystemTime;
use zed_extension_api as zed;

#[allow(dead_code)]
pub enum Preview {
    Zathura,
    Skim,
    Sioyek,
    QPDFView,
    Okular,
    SumatraPDF,
    Evince { evince_synctex_path: String },
}

impl Preview {
    pub fn create_preset(&self, zed_command: CommandName) -> TexlabForwardSearchSettings {
        match self {
            Preview::Zathura => TexlabForwardSearchSettings {
                executable: Some("zathura".to_string()),
                args: Some(vec![
                    "--synctex-forward".to_string(),
                    "%l:1:%f".to_string(),
                    "-x".to_string(),
                    format!("{} {}", zed_command.to_str(), "%%{input}:%%{line}"),
                    "%p".to_string(),
                ]),
            },
            Preview::Skim => TexlabForwardSearchSettings {
                executable: Some(
                    "/Applications/Skim.app/Contents/SharedSupport/displayline".to_string(),
                ),
                args: Some(vec![
                    "-r".to_string(),
                    "%l".to_string(),
                    "%p".to_string(),
                    "%f".to_string(),
                ]),
            },
            Preview::Sioyek => TexlabForwardSearchSettings {
                executable: Some("sioyek".to_string()),
                args: Some(vec![
                    "--reuse-window".to_string(),
                    "--inverse-search".to_string(),
                    format!("{} \"%%1\":%%2", zed_command.to_str()),
                    "--forward-search-file".to_string(),
                    "%f".to_string(),
                    "--forward-search-line".to_string(),
                    "%l".to_string(),
                    "%p".to_string(),
                ]),
            },
            Preview::Okular => TexlabForwardSearchSettings {
                // Unfortunately, there is no single okular command that can be used for the
                // forward search command in a way that sets up the inverse search command.
                // Therefore, we resort to a shell command involving two okular commands.
                //
                // This shell command attempts to open okular performing a forward search and
                // setting the inverse-search command to open the file in zed at the correct
                // location.
                // However the `--unique` flag conflicts with the `--editor-cmd` flag, but
                // only if okular is already open. At that point, the same command is run
                // again but without the `--editor-cmd` flag, which is ok because the editor
                // command (inverse search) would already be set at that point.
                executable: Some("sh".to_string()),
                args: Some(vec![
                    "-c".to_string(),
                    format!(
                        "okular --unique --noraise --editor-cmd \"{} '%%f':%%l:%%c\" \"%p#src:%l %f\" || okular --unique --noraise \"%p#src:%l %f\"",
                        zed_command.to_str()
                    ),
                ]),
            },
            Preview::QPDFView => TexlabForwardSearchSettings {
                executable: Some("qpdfview".to_string()),
                args: Some(vec!["--unique".to_string(), "%p#src:%f:%l:1".to_string()]),
            },
            Preview::Evince{ ref evince_synctex_path} => TexlabForwardSearchSettings {
                executable: Some("python".to_string()),
                args: Some(vec![
                    evince_synctex_path.clone(),
                    "-f".to_string(),
                    "%l".to_string(),
                    "-t".to_string(),
                    "%f".to_string(),
                    "%p".to_string(),
                    format!("{} %%f:%%l", zed_command.to_str())
                ]),
            },
            _ => TexlabForwardSearchSettings::default(),
        }
    }

    pub fn determine(worktree: &zed::Worktree) -> Option<Preview> {
        let (platform, _) = zed::current_platform();

        if platform == zed::Os::Mac {
            if worktree
                .which("/Applications/Skim.app/Contents/SharedSupport/displayline")
                .is_some()
            {
                return Some(Preview::Skim);
            }
        }

        if worktree.which("evince").is_some() {
            const SCRIPT_NAME: &str = "evince_synctex.py";
            const GITHUB_REPO_NAME: &str = "lnay/evince-synctex";
            const COMMIT_HASH: &str = "635f7863408a44f3aaa0dbad512f2ba6ac1ad6ff";
            // Following values refer to the estimated latest time when a
            // release of this extension updates the version of
            // evince_synctex.py is to be downloaded. (i.e. possibly the near
            // future to account for Zed extension release pipeline).
            const LAST_UPDATE_YEAR: i32 = 2025;
            const LAST_UPDATE_MONTH: u32 = 3;
            const LAST_UPDATE_DAY: u32 = 20;

            // The following would all be useless if the string path for
            // evince_synctex.py in CWD cannot be obtained:
            if let Some(evince_synctex_path) = (|| {
                Some(format!(
                    "{}/{SCRIPT_NAME}",
                    std::env::current_dir().ok()?.as_os_str().to_str()?
                ))
            })() {
                // Check if `evince_synctex.py` has already downloaded to
                // latex extension work directory since the last time this
                // extension updated the version of `evince_synctex.py`.
                if let Ok(stat) = std::fs::metadata(SCRIPT_NAME) {
                    if stat.is_file() {
                        if let Ok(last_download) = stat.modified() {
                            // SystemTime estimate for last extension update.
                            // When evince_synctex.py was updated:
                            let last_update: SystemTime = Utc
                                .with_ymd_and_hms(
                                    LAST_UPDATE_YEAR,
                                    LAST_UPDATE_MONTH,
                                    LAST_UPDATE_DAY,
                                    0,
                                    0,
                                    0,
                                )
                                .single()
                                .unwrap_or_default()
                                .into();
                            if last_download > last_update {
                                return Some(Preview::Evince {
                                    evince_synctex_path,
                                });
                            }
                        }
                    }
                }
                // Choose evince for preview, provided that evince_synctex.py
                // downloads successfully.
                if zed::download_file(
                    format!("https://raw.githubusercontent.com/{GITHUB_REPO_NAME}/{COMMIT_HASH}/{SCRIPT_NAME}").as_str(),
                    SCRIPT_NAME,
                    zed::DownloadedFileType::Uncompressed
                ).is_ok() {
                    return Some(Preview::Evince { evince_synctex_path });
                }
            }
        }
        if worktree.which("zathura").is_some() {
            return Some(Preview::Zathura);
        }
        if worktree.which("sioyek").is_some() {
            return Some(Preview::Sioyek);
        }
        if worktree.which("qpdfview").is_some() {
            return Some(Preview::QPDFView);
        }
        if worktree.which("okular").is_some() {
            return Some(Preview::Okular);
        }

        // Checking the existence of SumatraPDF will need
        // the ability to find the user name
        // if platform == zed::Os::Windows {
        //     if worktree
        //         .which("C:/Users/{User}/AppData/Local/SumatraPDF/SumatraPDF.exe")
        //         .is_some()
        //     {
        //         return Some(Preview::SumatraPDF);
        //     }
        // }

        None
    }
}
