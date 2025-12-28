/// Configuration for clipboard history.
///
/// Configuration can be loaded from a TOML file or use defaults.
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Maximum number of clipboard entries to store
    #[serde(default = "default_max_entries")]
    pub max_entries: usize,

    /// Maximum size of a single entry in bytes
    #[serde(default = "default_max_entry_size")]
    pub max_entry_size: usize,

    /// UI popup configuration
    #[serde(default)]
    pub ui: UiConfig,

    /// Hotkey configuration
    #[serde(default)]
    pub hotkey: HotkeyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Popup window width in pixels
    #[serde(default = "default_popup_width")]
    pub width: i32,

    /// Popup window height in pixels
    #[serde(default = "default_popup_height")]
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// Hotkey string (e.g., "Ctrl+Shift+V")
    #[serde(default = "default_hotkey")]
    pub popup: String,
}

// Default values
fn default_max_entries() -> usize {
    100
}

fn default_max_entry_size() -> usize {
    1048576 // 1MB
}

fn default_popup_width() -> i32 {
    600
}

fn default_popup_height() -> i32 {
    400
}

fn default_hotkey() -> String {
    "Ctrl+Shift+V".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_entries: default_max_entries(),
            max_entry_size: default_max_entry_size(),
            ui: UiConfig::default(),
            hotkey: HotkeyConfig::default(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            width: default_popup_width(),
            height: default_popup_height(),
        }
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            popup: default_hotkey(),
        }
    }
}

impl Config {
    /// Path to the config file
    /// Uses XDG Base Directory specification (~/.config/clipboard-history/config.toml)
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"));
        path.push("clipboard-history");
        path.push("config.toml");
        path
    }

    /// Path to the persistence file
    /// Uses XDG Base Directory specification (~/.local/share/clipboard-history/history.json)
    pub fn storage_path() -> PathBuf {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"));
        path.push("clipboard-history");
        path.push("history.json");
        path
    }

    /// Load configuration from file, or use defaults if file doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            let config: Config = toml::from_str(&contents)
                .context("Failed to parse config file")?;
            Ok(config)
        } else {
            // Use default config
            Ok(Self::default())
        }
    }

    /// Save current configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();

        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_path, toml_string)
            .context("Failed to write config file")?;

        Ok(())
    }

    /// Create a default config file if it doesn't exist
    pub fn ensure_default_config() -> Result<PathBuf> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            println!("Created default config at: {}", config_path.display());
        }

        Ok(config_path)
    }
}

/// Parse a hotkey string into modifier flags and key code
/// Format: "Ctrl+Shift+V", "Alt+C", etc.
pub fn parse_hotkey(hotkey_str: &str) -> Result<(bool, bool, bool, String)> {
    let parts: Vec<&str> = hotkey_str.split('+').map(|s| s.trim()).collect();

    if parts.is_empty() {
        anyhow::bail!("Invalid hotkey format: empty string");
    }

    let mut ctrl = false;
    let mut shift = false;
    let mut alt = false;
    let mut key = String::new();

    for part in parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => ctrl = true,
            "shift" => shift = true,
            "alt" => alt = true,
            k => {
                if key.is_empty() {
                    key = k.to_uppercase();
                } else {
                    anyhow::bail!("Invalid hotkey format: multiple keys specified");
                }
            }
        }
    }

    if key.is_empty() {
        anyhow::bail!("Invalid hotkey format: no key specified");
    }

    Ok((ctrl, shift, alt, key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey() {
        let (ctrl, shift, alt, key) = parse_hotkey("Ctrl+Shift+V").unwrap();
        assert!(ctrl);
        assert!(shift);
        assert!(!alt);
        assert_eq!(key, "V");

        let (ctrl, shift, alt, key) = parse_hotkey("Alt+C").unwrap();
        assert!(!ctrl);
        assert!(!shift);
        assert!(alt);
        assert_eq!(key, "C");
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.max_entries, 100);
        assert_eq!(config.ui.width, 600);
        assert_eq!(config.hotkey.popup, "Ctrl+Shift+V");
    }
}