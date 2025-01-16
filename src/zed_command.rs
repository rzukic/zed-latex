use zed_extension_api as zed;

#[derive(Copy, Clone)]
pub enum CommandName {
    Zed,
    Zeditor,
    Zedit,
    ZedEditor,
    Flatpak,
}

impl CommandName {
    pub fn to_str(&self) -> &'static str {
        match self {
            CommandName::Zed => "zed",
            CommandName::Zeditor => "zeditor",
            CommandName::Zedit => "zedit",
            CommandName::ZedEditor => "zed-editor",
            CommandName::Flatpak => "flatpak run dev.zed.Zed",
        }
    }

    pub fn determine(worktree: &zed::Worktree) -> Option<Self> {
        if zed::Os::Linux == zed::current_platform().0 {
            // The existence of the ZED_FLATPAK_LIB_PATH environment variable is
            // a very strong indicator that Zed is running through flatpak.
            // Even if zed is also installed the default way and is on PATH,
            // the existence of this variable shows that, at the very least,
            // the current process is a subprocess of the flatpak sandbox
            // (and so the current zed window is from the flatpak install).
            if worktree
                .shell_env()
                .iter()
                .find(|&var| var.0 == "ZED_FLATPAK_LIB_PATH")
                .is_some()
            {
                return Some(CommandName::Flatpak);
            }
        }
        // MINOR EDGE CASE WARNING
        // Unlike the flatpak case, the rest of these tests could in principal be
        // incorrect. For example, a user could have installed zed through a package
        // manager and also the official way. In that case, the executable determined
        // might not actually be the one that is running.
        if worktree.which("zed").is_some() {
            // typical case
            return Some(CommandName::Zed);
        }

        if zed::Os::Linux == zed::current_platform().0 {
            // Known executables created by third-party package managers in linux
            if worktree.which("zeditor").is_some() {
                return Some(CommandName::Zeditor);
            }
            if worktree.which("zedit").is_some() {
                return Some(CommandName::Zedit);
            }
            if worktree.which("zed-editor").is_some() {
                return Some(CommandName::ZedEditor);
            }
        }

        None
    }
}

impl Default for CommandName {
    fn default() -> Self {
        CommandName::Zed
    }
}
