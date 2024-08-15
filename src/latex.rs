use zed_extension_api as zed;

struct LatexExtension;

impl zed::Extension for LatexExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("texlab")
            .ok_or_else(|| "texlab must be installed and available in $PATH.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
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

zed::register_extension!(LatexExtension);
