mod types;
mod stdin;
mod git;
mod version;
mod colors;
mod render;

use stdin::read_stdin;
use git::get_git_status;
use version::get_claude_version;
use render::render_output;

fn main() {
    let stdin_data = read_stdin().ok();

    let git_status = stdin_data
        .as_ref()
        .and_then(|d| d.cwd.as_ref())
        .and_then(|cwd| get_git_status(cwd));

    let cc_version = get_claude_version();

    render_output(&stdin_data, &git_status, &cc_version);
}
