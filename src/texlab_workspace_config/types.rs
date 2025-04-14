//! Types for (de)serializing the TeXLab language server workspace settings
//!
//! Documentation: https://github.com/latex-lsp/texlab/wiki/Configuration
//!
//! The `ForwardSearchSettings` is especially relevant outside this module,
//! as it can be modified based on the detected PDF previewer.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::vec::Vec;
use zed_extension_api::serde_json::Value;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSettings {
    pub texlab: Option<TexlabSettings>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TexlabSettings {
    pub build: Option<TexlabBuildSettings>,
    pub forward_search: Option<TexlabForwardSearchSettings>,
    pub chktex: Option<Value>,
    pub diagnostics: Option<Value>,
    pub diagnostics_delay: Option<Value>,
    pub symbols: Option<Value>,
    pub formatter_line_length: Option<Value>,
    pub bibtex_formatter: Option<Value>,
    pub latex_formatter: Option<Value>,
    pub latexindent: Option<Value>,
    pub completion: Option<Value>,
    pub inlay_hints: Option<Value>,
    pub experimental: Option<Value>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TexlabBuildSettings {
    pub executable: Option<String>,
    pub args: Option<Vec<String>>,
    pub forward_search_after: Option<bool>,
    pub on_save: Option<bool>,
    pub use_file_list: Option<Value>,
    pub aux_directory: Option<Value>,
    pub log_directory: Option<Value>,
    pub pdf_directory: Option<Value>,
}

impl TexlabBuildSettings {
    /// When autoconfiguring preview settings, the `texlab.build.forwardSearchAfter`
    /// and `texlab.build.onSave` fields should be set to `true` if they are not already set.
    /// so that the user can see what is happening.
    pub fn switch_on_onsave_fields_if_not_set(mut self) -> Self {
        if self.forward_search_after.is_none() {
            self.forward_search_after = Some(true);
        }
        if self.on_save.is_none() {
            self.on_save = Some(true);
        }
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TexlabForwardSearchSettings {
    pub executable: Option<String>,
    pub args: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::serde_json::{self, json};

    #[test]
    fn test_deserialize_workspace_settings() {
        let data = json!({
            "texlab": {
                "build": {
                    "onSave": true,
                    "forwardSearchAfter": true
                },
                "forwardSearch": {
                    "executable": "zathura",
                    "args": [
                        "--synctex-forward",
                        "%l:1:%f",
                        "-x",
                        "zed %%{input}:%%{line}",
                        "%p"
                    ]
                },
                "diagnostics": {
                    "ignoredPatterns": []
                }
            }
        });

        let settings: WorkspaceSettings = serde_json::from_value(data).unwrap();

        assert!(settings.texlab.is_some());
        let texlab_settings = settings.texlab.unwrap();

        assert!(texlab_settings.build.is_some());
        let build_settings = texlab_settings.build.unwrap();
        assert_eq!(build_settings.on_save, Some(true));
        assert_eq!(build_settings.forward_search_after, Some(true));

        assert!(texlab_settings.forward_search.is_some());
        let forward_search_settings = texlab_settings.forward_search.unwrap();
        assert_eq!(
            forward_search_settings.executable,
            Some("zathura".to_string())
        );
        assert_eq!(
            forward_search_settings.args,
            Some(vec![
                "--synctex-forward".to_string(),
                "%l:1:%f".to_string(),
                "-x".to_string(),
                "zed %%{input}:%%{line}".to_string(),
                "%p".to_string()
            ])
        );

        assert!(texlab_settings.diagnostics_delay.is_none());
        assert!(texlab_settings.chktex.is_none());
        assert!(texlab_settings.symbols.is_none());
        assert!(texlab_settings.formatter_line_length.is_none());
        assert!(texlab_settings.bibtex_formatter.is_none());
        assert!(texlab_settings.latex_formatter.is_none());
        assert!(texlab_settings.latexindent.is_none());
        assert!(texlab_settings.completion.is_none());
        assert!(texlab_settings.inlay_hints.is_none());
        assert!(texlab_settings.experimental.is_none());
    }
    #[test]
    fn test_serialize_forward_search_settings() {
        let forward_search_settings = TexlabForwardSearchSettings {
            executable: Some("zathura".to_string()),
            args: Some(vec![
                "--synctex-forward".to_string(),
                "%l:1:%f".to_string(),
                "-x".to_string(),
                "zed %%{input}:%%{line}".to_string(),
                "%p".to_string(),
            ]),
        };

        let expected_json = json!({
            "executable": "zathura",
            "args": [
                "--synctex-forward",
                "%l:1:%f",
                "-x",
                "zed %%{input}:%%{line}",
                "%p"
            ]
        });

        let serialized = serde_json::to_value(&forward_search_settings).unwrap();
        assert_eq!(serialized, expected_json);
    }
}
