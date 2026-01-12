use zed_extension_api as zed;

struct AxeAccessibilityLinter {}

impl zed::Extension for AxeAccessibilityLinter {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "axe-ls".to_string(),
            args: vec!["--stdio".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(AxeAccessibilityLinter);
