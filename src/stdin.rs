use std::io::{self, Read};
use crate::types::StdinData;

pub fn read_stdin() -> Result<StdinData, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let data: StdinData = serde_json::from_str(&buffer)?;
    Ok(data)
}

pub fn get_model_name(data: &StdinData) -> String {
    data.model
        .as_ref()
        .and_then(|m| m.display_name.as_ref().or(m.id.as_ref()))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_context_percent(data: &StdinData) -> u8 {
    if let Some(cw) = &data.context_window {
        if let Some(pct) = cw.used_percentage {
            return pct.round().min(100.0).max(0.0) as u8;
        }

        if let (Some(size), Some(usage)) = (cw.context_window_size, &cw.current_usage) {
            if size > 0 {
                let total = usage.input_tokens.unwrap_or(0)
                    + usage.cache_creation_input_tokens.unwrap_or(0)
                    + usage.cache_read_input_tokens.unwrap_or(0);
                return ((total as f64 / size as f64) * 100.0).round().min(100.0) as u8;
            }
        }
    }
    0
}
