use std::env;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
struct ClaudeSettings {
    #[serde(rename = "apiUrl")]
    api_url: Option<String>,
    #[serde(rename = "baseUrl")]
    base_url: Option<String>,
    #[serde(rename = "anthropicApiUrl")]
    anthropic_api_url: Option<String>,
}

pub fn get_provider_url() -> Option<String> {
    // 优先级：环境变量 > Claude Code 配置

    // 1. 检查环境变量
    if let Ok(url) = env::var("ANTHROPIC_BASE_URL") {
        return Some(url);
    }

    if let Ok(url) = env::var("ANTHROPIC_API_URL") {
        return Some(url);
    }

    if let Ok(url) = env::var("CLAUDE_API_URL") {
        return Some(url);
    }

    // 2. 读取 Claude Code 配置
    if let Some(url) = read_claude_config() {
        return Some(url);
    }

    None
}

fn read_claude_config() -> Option<String> {
    let config_path = get_claude_config_path()?;
    let content = fs::read_to_string(config_path).ok()?;
    let settings: ClaudeSettings = serde_json::from_str(&content).ok()?;

    settings.api_url
        .or(settings.base_url)
        .or(settings.anthropic_api_url)
}

fn get_claude_config_path() -> Option<PathBuf> {
    let home = env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(".claude/settings.json"))
}

pub fn format_provider_display(url: &str) -> String {
    // 直接返回完整 URL
    url.to_string()
}

fn extract_domain(url: &str) -> Option<String> {
    // 移除协议前缀
    let without_protocol = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    // 提取域名（去掉路径）
    let domain = without_protocol.split('/').next()?;

    Some(domain.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://code.newcli.com/claude/ultra"),
            Some("code.newcli.com".to_string())
        );
        assert_eq!(
            extract_domain("http://api.example.com/v1"),
            Some("api.example.com".to_string())
        );
        assert_eq!(
            extract_domain("localhost:8080"),
            Some("localhost:8080".to_string())
        );
    }
}
