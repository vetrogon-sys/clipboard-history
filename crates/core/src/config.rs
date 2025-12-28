/// Configuration constants for clipboard history.
///
/// These are currently hardcoded, but the architecture allows easy migration
/// to file-based configuration (TOML/JSON) by replacing this module with
/// a config file loader using `serde` deserialization.
use std::path::PathBuf;

pub struct Config;

impl Config {
    /// Maximum number of clipboard entries to store
    pub const MAX_ENTRIES: usize = 100;

    /// Path to the persistence file
    /// Uses XDG Base Directory specification (~/.local/share/clipboard-history/)
    pub fn storage_path() -> PathBuf {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"));
        path.push("clipboard-history");
        path.push("history.json");
        path
    }

    /// UI popup window width in pixels
    pub const POPUP_WIDTH: i32 = 600;

    /// UI popup window height in pixels
    pub const POPUP_HEIGHT: i32 = 400;

    /// Global hotkey for popup (currently Ctrl+Shift+V)
    /// Future: parse from string like "Ctrl+Shift+V"
    pub const HOTKEY_CTRL: bool = true;
    pub const HOTKEY_SHIFT: bool = true;
    pub const HOTKEY_KEY: char = 'V';
}