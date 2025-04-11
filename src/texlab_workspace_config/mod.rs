//! Provides functionality for managing and modifying the LSP settings for the Texlab language server.
//!
//! It handles:
//! - Retrieving Texlab LSP settings for a given worktree
//! - Modifying settings based on detected PDF previewers
//! - Adding forward search settings when appropriate (never overriding user-provided settings)
//! - Providing default build command if not provided
//!
//! The settings modifications are focused on enabling build-on-save and forward search
//! features when a PDF previewer is detected, while being careful not to override any
//! existing user configurations.

pub mod preview_presets;
mod types;

use crate::LatexExtension;
use types::{TexlabBuildSettings, TexlabSettings, WorkspaceSettings};
use zed_extension_api::serde_json::{from_value, Value};

/// Retrieves and potentially modifies the texlab LSP settings for a given worktree.
///
/// The output is affected by whether a previewer was detected and recorded in the LatexExtension.
/// The build command is also defined if not provided.
///
/// Returns either:
/// - The original settings if no previewer is detected
/// - Modified settings with forward search and build settings if a previewer exists
/// - Error string if settings deserialization fails (which means the settings are invalid)
pub fn get(
    latex_extension: &mut LatexExtension,
    lsp_texlab_settings: Value,
) -> Result<WorkspaceSettings, String> {
    let provided_texlab_settings = from_value::<Option<WorkspaceSettings>>(lsp_texlab_settings)
        .map_err(|err| err.to_string())? // Do not silently pass settings on when deserialization fails anymore
        .unwrap_or_default()
        .texlab
        .unwrap_or_default();

    let texlab_settings_with_build_default = match provided_texlab_settings {
        TexlabSettings {
            build:
                Some(TexlabBuildSettings {
                    executable: Some(_),
                    ..
                }),
            ..
        } => provided_texlab_settings,
        _ => TexlabSettings {
            build: Some(TexlabBuildSettings {
                executable: Some("latexmk".to_string()),
                args: Some(vec![
                    "-e".into(),
                    "$pdf_mode = 1 unless $pdf_mode != 0;".into(),
                    "-interaction=nonstopmode".into(),
                    "-synctex=1".into(),
                    "%f".into(),
                ]),
                ..provided_texlab_settings.build.unwrap_or_default()
            }),
            ..provided_texlab_settings
        },
    };

    // Determine previewer related settings (when appropriate)
    let settings_with_previewer = match latex_extension.previewer {
        None => texlab_settings_with_build_default,
        // Only adjust settings if a previewer is detected.
        Some(ref previewer) => {
            match texlab_settings_with_build_default {
                // User has provided forward search settings, do not override.
                TexlabSettings {
                    forward_search: Some(_),
                    ..
                } => texlab_settings_with_build_default,
                // User has not provided forward search settings, which
                // can be filled in for detected previewer; and enable build-on-save
                // and forward search after build unless explicitly disabled.
                texlab_settings_without_forward_search => TexlabSettings {
                    forward_search: Some(
                        previewer.create_preset(latex_extension.zed_command.unwrap_or_default()),
                    ),
                    build: Some(
                        texlab_settings_without_forward_search
                            .build
                            .unwrap_or_default()
                            .switch_on_onsave_fields_if_not_set(),
                    ),
                    ..texlab_settings_without_forward_search
                },
            }
        }
    };

    Ok(WorkspaceSettings {
        texlab: Some(settings_with_previewer),
    })
}
