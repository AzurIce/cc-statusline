pub const CYAN: &str = "\x1b[36m";
pub const YELLOW: &str = "\x1b[33m";
pub const MAGENTA: &str = "\x1b[35m";
pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const DIM: &str = "\x1b[2m";
pub const RESET: &str = "\x1b[0m";

pub fn context_color(percent: u8) -> &'static str {
    match percent {
        0..=70 => GREEN,
        71..=85 => YELLOW,
        _ => RED,
    }
}
