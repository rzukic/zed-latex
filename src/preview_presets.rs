use crate::texlab_settings::*;
use crate::zed_command::CommandName;
use zed_extension_api as zed;

#[allow(dead_code)]
pub enum Preview {
    Zathura,
    Skim,
    Sioyek,
    QPDFView,
    Okular,
    SumatraPDF,
    Evince,
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
            Preview::Evince => TexlabForwardSearchSettings {
                executable: Some("evince-synctex".to_string()),
                args: Some(vec![
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
            return Some(Preview::Evince);
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
