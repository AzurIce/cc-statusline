use std::process::Command;
use crate::types::GitStatus;

pub fn get_git_status(cwd: &str) -> Option<GitStatus> {
    let branch = get_branch(cwd)?;
    let is_dirty = check_dirty(cwd);
    Some(GitStatus { branch, is_dirty })
}

fn get_branch(cwd: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(cwd)
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    } else {
        None
    }
}

fn check_dirty(cwd: &str) -> bool {
    Command::new("git")
        .args(["--no-optional-locks", "status", "--porcelain"])
        .current_dir(cwd)
        .output()
        .ok()
        .map(|output| !output.stdout.is_empty())
        .unwrap_or(false)
}
