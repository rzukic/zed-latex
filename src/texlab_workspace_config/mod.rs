//! Provides functionality for managing and modifying the LSP settings for the Texlab language server.
//!
//! It handles:
//! - Retrieving Texlab LSP settings for a given worktree
//! - Modifying settings based on detected PDF previewers
//! - Adding forward search settings when appropriate (never overriding user-provided settings)
//!
//! The settings modifications are focused on enabling build-on-save and forward search
//! features when a PDF previewer is detected, while being careful not to override any
//! existing user configurations.

pub mod preview_presets;
mod types;

use crate::LatexExtension;
use types::{TexlabBuildSettings, TexlabSettings, WorkspaceSettings};
use zed_extension_api::{self as zed, serde_json};

/// Retrieves and potentially modifies the texlab LSP settings for a given worktree.
///
/// The output is affected by whether a previewer was detected and recorded in the LatexExtension.
///
/// Returns either:
/// - The original settings if no previewer is detected
/// - Modified settings with forward search and build settings if a previewer exists
/// - Error string if settings deserialization fails
pub fn get(
    latex_extension: &mut LatexExtension,
    worktree: &zed_extension_api::Worktree,
) -> Result<Option<serde_json::Value>, String> {
    let settings = zed::settings::LspSettings::for_worktree("texlab", worktree)
        .ok()
        .and_then(|lsp_settings| lsp_settings.settings.clone())
        .unwrap_or_default();

    match latex_extension.previewer {
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
                                previewer
                                    .create_preset(latex_extension.zed_command.unwrap_or_default()),
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
                                previewer
                                    .create_preset(latex_extension.zed_command.unwrap_or_default()),
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
