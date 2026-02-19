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

use crate::zed_command::CommandName;
use preview_presets::Preview;
use types::{TexlabBuildSettings, TexlabHoverSettings, TexlabSettings, WorkspaceSettings};
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
    previewer: &Option<Preview>,
    zed_command: CommandName,
    lsp_texlab_settings: Value,
) -> Result<WorkspaceSettings, String> {
    let provided_texlab_settings = from_value::<Option<WorkspaceSettings>>(lsp_texlab_settings)
        .map_err(|err| err.to_string())? // Do not silently pass settings on when deserialization fails anymore
        .unwrap_or_default()
        .texlab
        .unwrap_or_default();

    let texlab_settings_with_defaults =
        add_hover_default(add_build_default(provided_texlab_settings));

    let settings_with_previewer = if let Some(ref previewer) = previewer {
        add_preview(&previewer, zed_command, texlab_settings_with_defaults)
    } else {
        texlab_settings_with_defaults
    };

    Ok(WorkspaceSettings {
        texlab: Some(settings_with_previewer),
    })
}

/// Add previewer related settings to have forward and inverse search set up (if possible),
/// but only if the user has not provided forward search settings themselves.
fn add_preview(
    previewer: &Preview,
    zed_command: CommandName,
    texlab_settings_with_defaults: TexlabSettings,
) -> TexlabSettings {
    match texlab_settings_with_defaults {
        // User has provided forward search settings, do not override.
        TexlabSettings {
            forward_search: Some(_),
            ..
        } => texlab_settings_with_defaults,
        // User has not provided forward search settings, which
        // can be filled in for detected previewer; and enable build-on-save
        // and forward search after build unless explicitly disabled.
        texlab_settings_without_forward_search => TexlabSettings {
            forward_search: Some(previewer.create_preset(zed_command)),
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

fn add_build_default(input_settings: TexlabSettings) -> TexlabSettings {
    match input_settings {
        TexlabSettings {
            build:
                Some(TexlabBuildSettings {
                    executable: Some(_),
                    ..
                }),
            ..
        } => input_settings,
        _ => {
            let mut args = input_settings
                .build
                .as_ref()
                .and_then(|b| b.args.clone())
                .unwrap_or(vec![
                    "-interaction=nonstopmode".into(),
                    "-synctex=1".into(),
                    "%f".into(),
                ]);
            args.insert(0, "-e".into());
            args.insert(1, "$pdf_mode = 1 unless $pdf_mode != 0; if ($ARGV[-1] =~ /\\.log$/ or $ARGV[-1] =~ /latexmkrc$/) { exit 0; };".into());

            TexlabSettings {
                build: Some(TexlabBuildSettings {
                    executable: Some("latexmk".to_string()),
                    args: Some(args),
                    ..input_settings.build.unwrap_or_default()
                }),
                ..input_settings
            }
        }
    }
}

/// Adds glyph preview in hover of symbol commands (used to be texlab default)
fn add_hover_default(input_settings: TexlabSettings) -> TexlabSettings {
    match input_settings {
        TexlabSettings { hover: Some(_), .. } => input_settings,
        _ => TexlabSettings {
            hover: Some(TexlabHoverSettings {
                symbols: "glyph".to_string(),
            }),
            ..input_settings
        },
    }
}
