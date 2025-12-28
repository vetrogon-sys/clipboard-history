use anyhow::Result;
use clipboard::x11::X11ClipboardListener;
use core::buffer::{ClipboardBuffer, ClipboardEntry};
use std::sync::{Arc, Mutex};
use clipboard::ClipboardListener;

fn main() -> Result<()> {
    let buffer = Arc::new(Mutex::new(ClipboardBuffer::new(100)));

    let buffer_clone = buffer.clone();

    let mut listener = X11ClipboardListener::new(move |text| {
        let entry = ClipboardEntry::new(text);
        let mut buffer = buffer_clone.lock().unwrap();
        buffer.push(entry);

        println!("Clipboard updated. Total entries: {}", buffer.entries().count());
    })?;

    println!("Clipboard daemon started (X11)");
    listener.start()
}
