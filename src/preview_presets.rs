use crate::texlab_settings::*;
use zed_extension_api as zed;

#[allow(dead_code)]
pub enum Preview {
    Zathura,
    Skim,
    Sioyek,
    QPDFView,
    Okular,
    SumatraPDF,
}

impl Preview {
    pub fn create_preset(&self) -> TexlabForwardSearchSettings {
        match self {
            Preview::Zathura => TexlabForwardSearchSettings {
                executable: Some("zathura".to_string()),
                args: Some(vec![
                    "--synctex-forward".to_string(),
                    "%l:1:%f".to_string(),
                    "-x".to_string(),
                    "zed %%{input}:%%{line}".to_string(),
                    "%p".to_string(),
                ]),
            },
            _ => TexlabForwardSearchSettings::default(),
        }
    }

    pub fn determine(worktree: &zed::Worktree) -> Option<Preview> {
        let (platform, _) = zed::current_platform();

        if platform == zed::Os::Mac {
            if worktree
                .which("/Applications/Skim.app/Contents/SharedSupport/displayline")
                .is_some()
            {
                return Some(Preview::Skim);
            }
        }

        if worktree.which("zathura").is_some() {
            return Some(Preview::Zathura);
        }
        if worktree.which("sioyek").is_some() {
            return Some(Preview::Sioyek);
        }
        if worktree.which("sioyek").is_some() {
            return Some(Preview::Sioyek);
        }
        if worktree.which("qpdfview").is_some() {
            return Some(Preview::QPDFView);
        }
        if worktree.which("okular").is_some() {
            return Some(Preview::Okular);
        }

        // Checking the existence of SumatraPDF will need
        // the ability to find the user name
        // if platform == zed::Os::Windows {
        //     if worktree
        //         .which("C:/Users/{User}/AppData/Local/SumatraPDF/SumatraPDF.exe")
        //         .is_some()
        //     {
        //         return Some(Preview::SumatraPDF);
        //     }
        // }

        None
    }
}
