use super::LatexExtension;
use zed_extension_api as zed;

pub fn command(
    latex_extension: &mut LatexExtension,
    language_server_id: &zed_extension_api::LanguageServerId,
    worktree: &zed_extension_api::Worktree,
) -> Result<zed_extension_api::Command, String> {
    use zed::settings::BinarySettings;
    let lsp_settings =
        zed::settings::LspSettings::for_worktree("texlab", worktree).unwrap_or_default();

    let env = texlab_env::get_from_init_opts(lsp_settings.initialization_options, worktree);

    // No CLI args are provided to `texlab` by default, but they can be provided in the settings.
    let args = match lsp_settings.binary {
        Some(BinarySettings {
            arguments: Some(ref args),
            ..
        }) => args.clone(),
        _ => vec![],
    };

    // First priority for texlab executable: user-provided path.
    if let Some(BinarySettings {
        path: Some(ref path),
        ..
    }) = lsp_settings.binary
    {
        let command = path.clone();
        return Ok(zed::Command { command, args, env });
    }

    // Second priority for texlab: already installed and on PATH.
    if let Some(command) = worktree.which("texlab") {
        return Ok(zed::Command { command, args, env });
    }

    // Third priority for texlab: cached path (from download in final priority).
    if let Some(ref path) = latex_extension.cached_texlab_path {
        if std::fs::metadata(path).is_ok() {
            let command = path.clone();
            return Ok(zed::Command { command, args, env });
        }
    }

    // Final priority for texlab: download from GitHub releases.
    let binary_path = acquire_latest_texlab(language_server_id)?;
    latex_extension.cached_texlab_path = Some(binary_path.clone());

    Ok(zed::Command {
        command: binary_path,
        args,
        env,
    })
}

// Download the latest release of `texlab` from GitHub and return the path to the binary,
// updating the language server installation status along the way.
// Cache the location if downloaded to be used the next time if available.
// If previously downloaded, skip download.
// If no network, search if previously downloaded.
fn acquire_latest_texlab(
    language_server_id: &zed_extension_api::LanguageServerId,
) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();
    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );
    let release = match zed::latest_github_release(
        "latex-lsp/texlab",
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    ) {
        Ok(release) => release,
        Err(e) => {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Failed(format!(
                    "Error finding latest GitHub release for texlab: {e}"
                )),
            );
            // Fallback: check if we can find any previously downloaded releases.
            // Do not cache in case network connection recovered later.
            return find_previously_downloaded_texlab_release(platform);
        }
    };
    let arch: &str = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X86 => "i686",
        zed::Architecture::X8664 => "x86_64",
    };
    let os: &str = match platform {
        zed::Os::Mac => "macos",
        zed::Os::Linux => "linux",
        zed::Os::Windows => "windows",
    };
    let extension: &str = match platform {
        zed::Os::Mac | zed::Os::Linux => "tar.gz",
        zed::Os::Windows => "zip",
    };
    let asset_name: String = format!("texlab-{arch}-{os}.{extension}");
    let download_url = format!(
        "https://github.com/latex-lsp/texlab/releases/download/{}/{asset_name}",
        release.version
    );
    let version_dir = format!("texlab-{}", release.version);
    let binary_path = match platform {
        zed::Os::Mac | zed::Os::Linux => format!("{version_dir}/texlab"),
        zed::Os::Windows => format!("{version_dir}/texlab.exe"),
    };
    if !std::fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(
            &download_url,
            &version_dir,
            match platform {
                zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                zed::Os::Windows => zed::DownloadedFileType::Zip,
            },
        )
        .map_err(|e| format!("failed to download file: {e}"))?;

        zed::make_file_executable(&binary_path)?;

        // Remove older GitHub releases
        let entries =
            std::fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
            if entry.file_name().to_str() != Some(&version_dir) {
                std::fs::remove_dir_all(entry.path()).ok();
            }
        }
    }
    Ok(binary_path)
}

/// Check if there are any previously downloaded GitHub releases.
/// These will be downloaded as `texlab(.exe)` in a directory `texlab-VERSION`.
/// Return the latest (largest version number) if any is found.
fn find_previously_downloaded_texlab_release(platform: zed::Os) -> Result<String, String> {
    let entries =
        std::fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
    let downloaded_releases = entries.filter_map(|dir| {
        let dir_name: String = dir.ok()?.file_name().to_str()?.to_owned();
        if !dir_name.starts_with("texlab-") {
            return None;
        }
        let binary_path = match platform {
            zed::Os::Mac | zed::Os::Linux => format!("{}/texlab", dir_name),
            zed::Os::Windows => format!("{}/texlab.exe", dir_name),
        };
        if std::fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            return Some(binary_path);
        }
        None
    });
    downloaded_releases
        .max()
        // Lexicographic ordering will coincide with numeric ordering if version numbers have same
        // number of digits.
        // Proper numeric comparison probably overkill for now since this method is a fallback for
        // an edge-case, and older downloaded GitHub releases should be deleted along the way anyway.
        .ok_or("Failed to acquire latest texlab release and no cached version found".into())
}

mod texlab_env {
    use serde::{Deserialize, Serialize};
    use zed_extension_api::{
        self as zed,
        serde_json::{from_value, Value},
        Os, Worktree,
    };

    #[derive(Debug, Serialize, Deserialize, Default)]
    struct InitOpts {
        extra_tex_inputs: Option<Vec<String>>,
    }

    /// Deserialize the input and extract the `extra_tex_inputs` entry, if any.
    /// Join them into a single string with colons separating them.
    /// If TEXINPUTS is already set in the environment, include its values.
    /// Return a vector containing a single tuple ("TEXINPUTS", joined string).
    pub fn get_from_init_opts(
        init_opts: Option<Value>,
        worktree: &Worktree,
    ) -> Vec<(String, String)> {
        // Attempt to extract extra_tex_inputs from init_opts:
        if let Some(InitOpts {
            extra_tex_inputs: Some(texinputs),
        }) = init_opts.and_then(|json| from_value::<InitOpts>(json).ok())
        {
            // Directory separator (: on Mac/Linux, ; on Windows):
            let sep = match zed::current_platform() {
                (Os::Windows, _) => ";",
                _ => ":",
            };

            let joined_extra_tex_inputs = texinputs.join(sep);

            // To keep lifetime of env vars sufficiently long:
            let shell_env = worktree.shell_env();
            // Value of TEXINPUTS in environment var, if set and non-empty:
            let current_tex_inputs = shell_env
                .iter()
                .filter_map(|(var, val)| match var.as_str() {
                    "TEXINPUTS" => Some(val),
                    _ => None,
                })
                .next()
                .and_then(|val| if val.is_empty() { None } else { Some(val) });

            let tex_inputs = match current_tex_inputs {
                // Starting . to check project first,
                // and trailing directory separator (: or ;) to check system paths
                Some(current_texinputs) => {
                    format!(".{sep}{joined_extra_tex_inputs}{sep}{current_texinputs}{sep}")
                }
                None => format!(".{sep}{joined_extra_tex_inputs}{sep}"),
            };
            //
            return vec![("TEXINPUTS".to_string(), tex_inputs)];
        }

        // In all other cases, do not explicitly set any environment variables.
        vec![]
    }
}
