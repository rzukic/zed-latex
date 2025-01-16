use zed_extension_api as zed;

#[derive(Copy, Clone)]
pub enum Exe {
    Zed,
    Zeditor,
    Zedit,
    ZedEditor,
}

impl Exe {
    pub fn to_str(&self) -> &'static str {
        match self {
            Exe::Zed => "zed",
            Exe::Zeditor => "zeditor",
            Exe::Zedit => "zedit",
            Exe::ZedEditor => "zed-editor",
        }
    }

    pub fn determine(worktree: &zed::Worktree) -> Option<Self> {
        if worktree.which("zed").is_some() {
            return Some(Exe::Zed);
        }
        if worktree.which("zeditor").is_some() {
            return Some(Exe::Zeditor);
        }
        if worktree.which("zedit").is_some() {
            return Some(Exe::Zedit);
        }
        if worktree.which("zed-editor").is_some() {
            return Some(Exe::Zedit);
        }
        None
    }
}

impl Default for Exe {
    fn default() -> Self {
        Exe::Zed
    }
}
