use std::env;
use zed_extension_api as zed;

fn get_language_server_binary_path(worktree: &zed::Worktree) -> String {
    if let Ok(path) = env::var("AXE_LS_PATH") {
        return path;
    } else if let Some(path) = worktree.which("axe-ls.js") {
        return path;
    }
    "axe-ls.js".to_string()
}

struct AxeAccessibilityLinter {}

impl zed::Extension for AxeAccessibilityLinter {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: zed::node_binary_path().unwrap(),
            args: vec![
                get_language_server_binary_path(worktree),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AxeAccessibilityLinter);
