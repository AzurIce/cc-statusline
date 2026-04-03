use serde::Deserialize;

#[derive(Deserialize)]
pub struct StdinData {
    pub cwd: Option<String>,
    pub model: Option<ModelInfo>,
    pub context_window: Option<ContextWindow>,
}

#[derive(Deserialize)]
pub struct ModelInfo {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ContextWindow {
    pub context_window_size: Option<u64>,
    pub current_usage: Option<Usage>,
    pub used_percentage: Option<f64>,
}

#[derive(Deserialize)]
pub struct Usage {
    pub input_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

pub struct GitStatus {
    pub branch: String,
    pub is_dirty: bool,
}
