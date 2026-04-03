use std::env;
use std::process::Command;

pub fn get_claude_version() -> Option<String> {
    let path = env::var("PATH").ok()?;

    for dir in path.split(':') {
        let claude_path = format!("{}/claude", dir);
        if let Ok(output) = Command::new(&claude_path)
            .arg("--version")
            .output()
        {
            if output.status.success() {
                if let Ok(version_str) = String::from_utf8(output.stdout) {
                    return parse_version(&version_str);
                }
            }
        }
    }
    None
}

fn parse_version(output: &str) -> Option<String> {
    output
        .trim()
        .split_whitespace()
        .find(|s| s.chars().next().map(|c| c.is_numeric()).unwrap_or(false))
        .map(|s| s.to_string())
}
