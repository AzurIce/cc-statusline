use crate::colors::*;
use crate::types::{GitStatus, StdinData};

pub fn render_output(
    stdin_data: &Option<StdinData>,
    git_status: &Option<GitStatus>,
    cc_version: &Option<String>,
) {
    let mut line1 = Vec::new();
    let mut line2 = Vec::new();

    // Line 1: Model, Project, Git, Version
    if let Some(data) = stdin_data {
        let model = crate::stdin::get_model_name(data);
        line1.push(format!("{CYAN}[{model}]{RESET}"));

        // Project path
        if let Some(cwd) = &data.cwd {
            let project = cwd.split('/').last().unwrap_or("");
            let mut project_part = format!("{YELLOW}{project}{RESET}");

            // Git status
            if let Some(git) = git_status {
                let dirty = if git.is_dirty { "*" } else { "" };
                project_part.push_str(&format!(
                    " {MAGENTA}git:({CYAN}{}{dirty}{MAGENTA}){RESET}",
                    git.branch
                ));
            }
            line1.push(project_part);
        }

        // Claude Code version
        if let Some(version) = cc_version {
            line1.push(format!("{DIM}CC v{version}{RESET}"));
        }

        // Line 2: Context
        let percent = crate::stdin::get_context_percent(data);
        let color = context_color(percent);
        let bar = render_bar(percent);
        line2.push(format!("{DIM}Context{RESET} {bar} {color}{percent}%{RESET}"));
    }

    println!("{}", line1.join(" │ "));
    if !line2.is_empty() {
        println!("{}", line2.join(" │ "));
    }
}

fn render_bar(percent: u8) -> String {
    let filled = (percent as f32 / 10.0).round() as usize;
    let empty = 10 - filled;
    let color = context_color(percent);
    format!("{color}{}{RESET}{DIM}{}{RESET}", "█".repeat(filled), "░".repeat(empty))
}
