use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Result};
use core_lib::buffer::ClipboardBuffer;

/// D-Bus interface for clipboard history service
pub struct ClipboardHistoryService {
    buffer: Arc<Mutex<ClipboardBuffer>>,
}

impl ClipboardHistoryService {
    pub fn new(buffer: Arc<Mutex<ClipboardBuffer>>) -> Self {
        Self { buffer }
    }
}

#[interface(name = "com.clipboardhistory.Service")]
impl ClipboardHistoryService {
    /// Get all clipboard entries as a JSON string
    /// Returns: JSON array of entries with format: [{"text": "...", "timestamp": 123456789}, ...]
    fn get_entries(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        let entries = buffer.entries_vec();

        serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get the number of entries in clipboard history
    fn get_count(&self) -> u32 {
        let buffer = self.buffer.lock().unwrap();
        buffer.len() as u32
    }

    /// Get a specific entry by index (0 = most recent)
    /// Returns: JSON object or empty string if index out of bounds
    fn get_entry(&self, index: u32) -> String {
        let buffer = self.buffer.lock().unwrap();
        let entries = buffer.entries_vec();

        if let Some(entry) = entries.get(index as usize) {
            serde_json::to_string(entry).unwrap_or_default()
        } else {
            String::new()
        }
    }

    /// Clear all clipboard history
    fn clear(&self) -> bool {
        // This would require adding a clear method to ClipboardBuffer
        // For now, return false to indicate not implemented
        false
    }
}

/// Start the D-Bus service on the session bus
pub async fn start_dbus_service(buffer: Arc<Mutex<ClipboardBuffer>>) -> Result<Connection> {
    let service = ClipboardHistoryService::new(buffer);

    let connection = Connection::session().await?;

    connection
        .object_server()
        .at("/com/clipboardhistory/Service", service)
        .await?;

    connection
        .request_name("com.clipboardhistory.Service")
        .await?;

    println!("D-Bus service started: com.clipboardhistory.Service");

    Ok(connection)
}