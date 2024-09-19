use zed_extension_api as zed;

#[derive(Default)]
struct LatexExtension {
    cached_texlab_path: Option<String>,
}

impl zed::Extension for LatexExtension {
    fn new() -> Self {
        Self::default()
    }

    /// Read user-provided settings for the language server path and arguments,
    /// if present, and use them.
    /// Otherwise, find `texlab` in the workspace path.
    /// And if that fails, see if there is a cached path for `texlab`.
    /// Finally if above fail, download the latest release of `texlab` from GitHub and cache it.
    /// In all cases apart from the user-provided case, provide no arguments.
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        use zed::settings::BinarySettings;

        let binary_settings = zed::settings::LspSettings::for_worktree("texlab", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let env = Default::default();

        // First priority for texlab executable: user-provided path
        if let Some(BinarySettings {
            path: Some(ref path),
            arguments: ref potential_args,
        }) = binary_settings
        {
            let command = path.clone();
            let args = potential_args.clone().unwrap_or(vec![]);
            return Ok(zed::Command { command, args, env });
        }

        // Second priority for texlab: already installed and on PATH
        if let Some(command) = worktree.which("texlab") {
            return Ok(zed::Command {
                command,
                args: vec![],
                env,
            });
        }

        // Third priority for texlab: cached path (from download in final priority)
        if let Some(ref path) = self.cached_texlab_path {
            if std::fs::metadata(path).is_ok() {
                return Ok(zed::Command {
                    command: path.clone(),
                    args: vec![],
                    env,
                });
            }
        }

        // Final priority for texlab: download from GitHub releases
        let binary_path = acquire_latest_texlab(language_server_id)?;
        self.cached_texlab_path = Some(binary_path.clone());

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env,
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree("texlab", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree("texlab", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

// Download the latest release of `texlab` from GitHub and return the path to the binary.,
// updating the language server installation status along the way.
// If previously downloaded, skip download.
fn acquire_latest_texlab(
    language_server_id: &zed_extension_api::LanguageServerId,
) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();
    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );
    let release = zed::latest_github_release(
        "latex-lsp/texlab",
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;
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

zed::register_extension!(LatexExtension);
