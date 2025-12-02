mod texlab_invocation;
mod texlab_workspace_config;
mod zed_command;

use texlab_workspace_config::preview_presets::Preview;
use zed_command::CommandName;
use zed_extension_api::{self as zed, serde_json};

#[derive(Default)]
struct LatexExtension {
    /// Cached path to the texlab language server that was downloaded
    /// from GitHub releases
    cached_texlab_path: Option<String>,
    /// Detected PDF previewer
    previewer: Option<Preview>,
    /// Executable to invoke the zed editor (None if not on PATH)
    zed_command: Option<CommandName>,
}

impl zed::Extension for LatexExtension {
    fn new() -> Self {
        Self::default()
        // Although this would be a good place to check for the existence of a
        // previewer and zed executable name, there is no access to a zed
        // worktree which is needed to access to the environment and a
        // `which`-like command via the zed extension API.
        // Attempting to search for executables on PATH directly circumventing
        // the zed extension API appears not to work presumably due to some
        // sandboxing by wasmtime.
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // Check for the existence of a previewer, and zed executable name
        // (this has nothing to do with the language server but this
        // is a convenient place to minimize the number of times this
        // is done).
        self.previewer = Preview::determine(worktree);
        self.zed_command = CommandName::determine(worktree);

        texlab_invocation::command(self, language_server_id, worktree)
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

        Ok(Some(
            serde_json::to_value(texlab_workspace_config::get(
                &self.previewer,
                self.zed_command.unwrap_or_default(),
                settings,
            )?)
            .unwrap_or_default(),
        ))
    }
}

zed::register_extension!(LatexExtension);
