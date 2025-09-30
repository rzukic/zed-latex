//! This module's main responsibility is providing the command to start the `texlab` language server,
//! as well as the appropriate environment.
//! If necessary, it will download the latest release of `texlab` from GitHub.
//!
//! [`texlab`]: https://github.com/latex-lsp/texlab
use super::LatexExtension;
use zed_extension_api as zed;

/// Constructs the command to start the `texlab` language server.
///
/// `texlab` is searched for, or downloaded, following this order of priority:
/// 1. Use a user-provided path from settings
/// 2. Use a binary available on PATH
/// 3. Use a previously downloaded binary (from number 4 in a previous run)
/// 4. Download the latest release from GitHub
///   (using previously downloaded release if still current, or as a fallback to any network errors)
///
/// In all cases apart from the user-provided case, provide no CLI arguments to `texlab`.
///
/// This also adjusts the `TEXINPUTS` environment variable if
/// "lsp.texlab.initialization_options.extra_tex_inputs" zed setting is provided.
pub fn command(
    latex_extension: &mut LatexExtension,
    language_server_id: &zed_extension_api::LanguageServerId,
    worktree: &zed_extension_api::Worktree,
) -> Result<zed_extension_api::Command, String> {
    use zed::settings::CommandSettings;
    let lsp_settings =
        zed::settings::LspSettings::for_worktree("texlab", worktree).unwrap_or_default();

    // No CLI args are provided to `texlab` by default, but they can be provided in the settings.
    let args = match lsp_settings.binary {
        Some(CommandSettings {
            arguments: Some(ref args),
            ..
        }) => args.clone(),
        _ => vec![],
    };

    let env = Default::default();

    // First priority for texlab executable: user-provided path.
    if let Some(CommandSettings {
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
