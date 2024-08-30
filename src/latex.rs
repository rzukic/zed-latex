use zed_extension_api as zed;

struct LatexExtension;

impl zed::Extension for LatexExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("texlab")
            .ok_or_else(|| "texlab must be installed and available in $PATH.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(LatexExtension);
