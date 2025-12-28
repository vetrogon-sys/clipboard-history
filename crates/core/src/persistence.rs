use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::buffer::ClipboardEntry;

#[derive(Serialize, Deserialize)]
struct PersistedData {
    version: u32,
    entries: Vec<ClipboardEntry>,
}

const CURRENT_VERSION: u32 = 1;

/// Save clipboard entries to a JSON file
pub fn save_to_file<P: AsRef<Path>>(path: P, entries: &[ClipboardEntry]) -> Result<()> {
    let path = path.as_ref();

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create storage directory")?;
    }

    let data = PersistedData {
        version: CURRENT_VERSION,
        entries: entries.to_vec(),
    };

    let file = File::create(path)
        .context("Failed to create persistence file")?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &data)
        .context("Failed to serialize clipboard data")?;

    Ok(())
}

/// Load clipboard entries from a JSON file
pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<ClipboardEntry>> {
    let path = path.as_ref();

    if !path.exists() {
        // File doesn't exist yet, return empty vec (not an error)
        return Ok(Vec::new());
    }

    let file = File::open(path)
        .context("Failed to open persistence file")?;
    let reader = BufReader::new(file);

    let data: PersistedData = serde_json::from_reader(reader)
        .context("Failed to deserialize clipboard data")?;

    // Future: handle version migrations here
    if data.version != CURRENT_VERSION {
        // For now, just log a warning and proceed
        eprintln!("Warning: persistence file version mismatch (expected {}, got {})",
                 CURRENT_VERSION, data.version);
    }

    Ok(data.entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::ClipboardEntry;

    #[test]
    fn test_save_and_load() {
        let temp_path = "/tmp/test_clipboard_history.json";

        let entries = vec![
            ClipboardEntry::new("Hello".to_string()),
            ClipboardEntry::new("World".to_string()),
        ];

        save_to_file(temp_path, &entries).unwrap();
        let loaded = load_from_file(temp_path).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].text, "Hello");
        assert_eq!(loaded[1].text, "World");

        std::fs::remove_file(temp_path).ok();
    }
}