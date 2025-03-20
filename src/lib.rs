mod texlab_invocation;
mod zed_command;

use texlab_workspace_config::preview_presets::Preview;
use zed_command::CommandName;
use zed_extension_api as zed;

#[derive(Default)]
struct LatexExtension {
    /// cached path to the texlab language server that was downloaded
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
        texlab_workspace_config::get(self, worktree)
    }
}

mod texlab_workspace_config;

zed::register_extension!(LatexExtension);
