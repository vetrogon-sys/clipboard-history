use anyhow::Result;
use clipboard::x11::X11ClipboardListener;
use core::buffer::{ClipboardBuffer, ClipboardEntry};
use core::config::Config;
use std::sync::{Arc, Mutex};
use clipboard::ClipboardListener;

fn main() -> Result<()> {
    let storage_path = Config::storage_path();

    // Load existing history from persistence
    let buffer = match ClipboardBuffer::new_with_persistence(Config::MAX_ENTRIES, &storage_path) {
        Ok(buf) => {
            println!("Loaded {} entries from {}", buf.len(), storage_path.display());
            Arc::new(Mutex::new(buf))
        }
        Err(e) => {
            eprintln!("Failed to load persistence file: {}. Starting with empty buffer.", e);
            Arc::new(Mutex::new(ClipboardBuffer::new(Config::MAX_ENTRIES)))
        }
    };

    let buffer_clone = buffer.clone();
    let storage_path_clone = storage_path.clone();

    let mut listener = X11ClipboardListener::new(move |text| {
        let entry = ClipboardEntry::new(text);
        let mut buffer = buffer_clone.lock().unwrap();
        buffer.push(entry);

        // Save to persistence after each update
        if let Err(e) = buffer.save_to_file(&storage_path_clone) {
            eprintln!("Failed to save clipboard history: {}", e);
        }

        println!("Clipboard updated. Total entries: {}", buffer.len());
    })?;

    println!("Clipboard daemon started (X11)");
    println!("Storage: {}", storage_path.display());
    listener.start()
}
