use crate::i18n::Lang;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_true")]
    pub show_banner: bool,
    // 表示言語。デフォルトは英語
    #[serde(default)]
    pub language: Lang,
}

fn default_true() -> bool {
    true
}

fn config_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("pwatch");
    dir.join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config {
            show_banner: true,
            language: Lang::default(),
        }
    }
}

pub fn save(config: &Config) -> Result<(), String> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("failed to create dir: {}", e))?;
    }
    let content =
        toml::to_string_pretty(config).map_err(|e| format!("failed to serialize: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("failed to write: {}", e))?;
    Ok(())
}
