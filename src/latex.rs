use zed_extension_api as zed;

struct LatexExtension;

impl zed::Extension for LatexExtension {
    fn new() -> Self {
        Self
    }

    /// Read user-provided settings for the language server path and arguments,
    /// if present, and use them.
    /// Otherwise, find `texlab` in the workspace path and call it without any arguments,
    /// returning an error if not found.
    fn language_server_command(
        &mut self,
        _config: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        use zed::settings::BinarySettings;

        let binary_settings = zed::settings::LspSettings::for_worktree("texlab", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let command = match binary_settings {
            Some(BinarySettings {
                path: Some(ref p), ..
            }) => p.clone(),
            _ => worktree.which("texlab").ok_or_else(|| {
                "texlab must be installed and available in $PATH,\
                        or location specified in lsp.texlab.binary Zed setting."
                    .to_string()
            })?,
        };
        let args = match binary_settings {
            Some(BinarySettings {
                arguments: Some(ref a),
                ..
            }) => a.clone(),
            _ => vec![],
        };
        let env = Default::default();

        Ok(zed::Command { command, args, env })
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
