use zed_extension_api::{self as zed, Result};

struct EstelleExtension;

impl EstelleExtension {
    const SERVER_BINARY_NAME: &'static str = "estelle-lsp";
}

impl zed::Extension for EstelleExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which(Self::SERVER_BINARY_NAME).ok_or_else(|| {
            format!(
                "Could not find `{}` binary. Install it with:\n\
                 cargo install --path estelle-lsp",
                Self::SERVER_BINARY_NAME
            )
        })?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(EstelleExtension);
