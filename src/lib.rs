use std::{env, fs};

use zed_extension_api as zed;

struct AxeAccessibilityLinter {}

fn get_language_server_binary_path(worktree: &zed::Worktree) -> String {
    if let Ok(path) = env::var("AXE_LS_PATH") {
        if fs::metadata(&path).map_or(false, |metadata| metadata.is_file()) {
            return path;
        }
    }
    if let Some(path) = worktree.which("axe-ls") {
        return path;
    }
    return "axe-ls".to_string();
}

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
            command: get_language_server_binary_path(worktree),
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AxeAccessibilityLinter);
