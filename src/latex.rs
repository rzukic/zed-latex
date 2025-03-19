mod preview_presets;
mod texlab_invocation;
mod texlab_settings;
mod zed_command;

use preview_presets::*;
use texlab_settings::{TexlabBuildSettings, TexlabSettings, WorkspaceSettings};
use zed_command::CommandName;
use zed_extension_api::{self as zed, serde_json};

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
        let settings = zed::settings::LspSettings::for_worktree("texlab", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        match self.previewer {
            None => Ok(Some(settings)),
            // Only adjust settings if a previewer is detected.
            Some(ref previewer) => {
                match serde_json::from_value::<Option<WorkspaceSettings>>(settings.clone()) {
                    // User has provided forward search settings, do not override.
                    Ok(Some(WorkspaceSettings {
                        texlab:
                            Some(TexlabSettings {
                                forward_search: Some(_),
                                ..
                            }),
                    })) => Ok(Some(settings)),
                    // No settings provided, construct settings from scratch with build-on-save
                    // and forward search with detected previewer.
                    Ok(None | Some(WorkspaceSettings { texlab: None })) => Ok(Some(
                        serde_json::to_value(WorkspaceSettings {
                            texlab: Some(TexlabSettings {
                                build: Some(TexlabBuildSettings::build_and_search_on()),
                                forward_search: Some(
                                    previewer.create_preset(self.zed_command.unwrap_or_default()),
                                ),
                                ..Default::default()
                            }),
                        })
                        .unwrap_or_default(),
                    )),
                    // User has provided some settings, but not forward search, which
                    // can be filled in for detected previewer; and enable build-on-save
                    // and forward search after build unless explicitly disabled.
                    Ok(Some(WorkspaceSettings {
                        texlab: Some(texlab_settings_without_forward_search),
                    })) => Ok(Some(
                        serde_json::to_value(WorkspaceSettings {
                            texlab: Some(TexlabSettings {
                                forward_search: Some(
                                    previewer.create_preset(self.zed_command.unwrap_or_default()),
                                ),
                                build: Some(
                                    texlab_settings_without_forward_search
                                        .build
                                        .unwrap_or_default()
                                        .switch_on_onsave_fields_if_not_set(),
                                ),
                                ..texlab_settings_without_forward_search
                            }),
                        })
                        .unwrap_or_default(),
                    )),
                    Err(e) => Err(format!("Error deserializing workspace settings: {}", e)),
                }
            }
        }
    }
}

zed::register_extension!(LatexExtension);
