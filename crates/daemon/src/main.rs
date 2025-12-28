mod dbus_service;

use anyhow::Result;
use clipboard::x11::X11ClipboardListener;
use core_lib::buffer::{ClipboardBuffer, ClipboardEntry};
use core_lib::config::Config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use clipboard::ClipboardListener;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, HotKeyState, hotkey::{HotKey, Modifiers, Code}};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;
    Config::ensure_default_config()?;

    let storage_path = Config::storage_path();

    println!("Configuration:");
    println!("  Max entries: {}", config.max_entries);
    println!("  Hotkey: {}", config.hotkey.popup);
    println!("  Storage: {}", storage_path.display());
    println!();

    // Load existing history from persistence
    let buffer = match ClipboardBuffer::new_with_persistence(config.max_entries, &storage_path) {
        Ok(buf) => {
            println!("Loaded {} entries from {}", buf.len(), storage_path.display());
            Arc::new(Mutex::new(buf))
        }
        Err(e) => {
            eprintln!("Failed to load persistence file: {}. Starting with empty buffer.", e);
            Arc::new(Mutex::new(ClipboardBuffer::new(config.max_entries)))
        }
    };

    // Start D-Bus service
    let buffer_dbus = buffer.clone();
    let _dbus_connection = dbus_service::start_dbus_service(buffer_dbus).await?;

    // Start clipboard listener in a separate thread
    let buffer_clipboard = buffer.clone();
    let storage_path_clone = storage_path.clone();

    thread::spawn(move || {
        let mut listener = X11ClipboardListener::new(move |text| {
            let entry = ClipboardEntry::new(text);
            let mut buffer = buffer_clipboard.lock().unwrap();
            buffer.push(entry);

            // Save to persistence after each update
            if let Err(e) = buffer.save_to_file(&storage_path_clone) {
                eprintln!("Failed to save clipboard history: {}", e);
            }

            println!("Clipboard updated. Total entries: {}", buffer.len());
        })
        .expect("Failed to create clipboard listener");

        println!("Clipboard listener started (X11)");
        listener.start().expect("Clipboard listener failed");
    });

    // Parse and register global hotkey from config
    let (ctrl, shift, alt, key_str) = core_lib::config::parse_hotkey(&config.hotkey.popup)?;

    let mut modifiers = Modifiers::empty();
    if ctrl {
        modifiers |= Modifiers::CONTROL;
    }
    if shift {
        modifiers |= Modifiers::SHIFT;
    }
    if alt {
        modifiers |= Modifiers::ALT;
    }

    // Map key string to Code
    let key_code = match key_str.as_str() {
        "A" => Code::KeyA, "B" => Code::KeyB, "C" => Code::KeyC, "D" => Code::KeyD,
        "E" => Code::KeyE, "F" => Code::KeyF, "G" => Code::KeyG, "H" => Code::KeyH,
        "I" => Code::KeyI, "J" => Code::KeyJ, "K" => Code::KeyK, "L" => Code::KeyL,
        "M" => Code::KeyM, "N" => Code::KeyN, "O" => Code::KeyO, "P" => Code::KeyP,
        "Q" => Code::KeyQ, "R" => Code::KeyR, "S" => Code::KeyS, "T" => Code::KeyT,
        "U" => Code::KeyU, "V" => Code::KeyV, "W" => Code::KeyW, "X" => Code::KeyX,
        "Y" => Code::KeyY, "Z" => Code::KeyZ,
        _ => anyhow::bail!("Unsupported key: {}", key_str),
    };

    let hotkey_manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(modifiers), key_code);
    hotkey_manager.register(hotkey)?;

    println!("Clipboard daemon started");

    // Listen for hotkey events
    let hotkey_receiver = GlobalHotKeyEvent::receiver();
    let mut last_trigger = Instant::now();
    let debounce_duration = Duration::from_millis(500);

    loop {
        if let Ok(event) = hotkey_receiver.recv() {
            // Only respond to key press events, not release
            if event.state == HotKeyState::Pressed {
                // Debounce: ignore if triggered too recently
                let now = Instant::now();
                if now.duration_since(last_trigger) >= debounce_duration {
                    last_trigger = now;
                    println!("Hotkey pressed! Launching UI...");

                    // Launch UI client
                    if let Err(e) = std::process::Command::new("clipboard-ui").spawn() {
                        eprintln!("Failed to launch UI: {}. Make sure clipboard-ui is installed.", e);
                    }
                }
            }
        }
    }
}
