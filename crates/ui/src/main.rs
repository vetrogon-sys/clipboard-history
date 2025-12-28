use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box as GtkBox, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, SelectionMode, PolicyType,
};
use anyhow::Result;
use serde::Deserialize;
use enigo::{Enigo, Key, Keyboard, Settings};
use std::io::Write;

#[derive(Debug, Clone, Deserialize)]
struct ClipboardEntry {
    text: String,
    timestamp: u64,
}

const APP_ID: &str = "com.clipboardhistory.UI";

#[tokio::main]
async fn main() -> Result<()> {
    // Fetch clipboard entries from D-Bus service
    let entries = match fetch_clipboard_entries().await {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Failed to fetch clipboard entries: {}", err);
            eprintln!("Make sure the clipboard daemon is running.");
            std::process::exit(1);
        }
    };

    // Build and run GTK UI
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        build_ui(app, entries.clone());
    });

    app.run();
    Ok(())
}

async fn fetch_clipboard_entries() -> Result<Vec<ClipboardEntry>> {
    let connection = zbus::Connection::session().await?;

    let proxy = zbus::Proxy::new(
        &connection,
        "com.clipboardhistory.Service",
        "/com/clipboardhistory/Service",
        "com.clipboardhistory.Service",
    )
    .await?;

    let entries_json: String = proxy.call("GetEntries", &()).await?;

    let entries: Vec<ClipboardEntry> = serde_json::from_str(&entries_json)?;

    Ok(entries)
}

fn build_ui(app: &Application, entries: Vec<ClipboardEntry>) {
    // Create main window with fixed size
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Clipboard History")
        .default_width(core_lib::config::Config::POPUP_WIDTH)
        .default_height(core_lib::config::Config::POPUP_HEIGHT)
        .resizable(false)
        .build();

    // Position window at mouse cursor
    // Note: GTK4 doesn't have direct window positioning like GTK3
    // The window manager controls positioning
    // We keep the fixed size set above

    // Create main container
    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);
    
    // Add header
    let header = Label::new(Some("Clipboard History"));
    header.add_css_class("title-2");
    main_box.append(&header);

    // Create list box for entries
    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::Single);
    list_box.add_css_class("boxed-list");

    // Add entries to list
    if entries.is_empty() {
        let empty_label = Label::new(Some("No clipboard history yet"));
        empty_label.set_margin_top(24);
        empty_label.set_margin_bottom(24);
        empty_label.add_css_class("dim-label");
        let row = ListBoxRow::new();
        row.set_child(Some(&empty_label));
        row.set_selectable(false);
        row.set_activatable(false);
        list_box.append(&row);
    } else {
        for entry in entries.iter() {
            let row = create_entry_row(entry);
            list_box.append(&row);
        }
    }

    // Handle entry selection
    let window_clone = window.clone();
    list_box.connect_row_activated(move |_, row| {
        if let Some(child) = row.child() {
            if let Ok(label) = child.downcast::<Label>() {
                let text = label.text();

                // Close window first
                window_clone.close();

                // Give the window time to close and focus to return to previous app
                std::thread::sleep(std::time::Duration::from_millis(100));

                // Simulate paste using Enigo
                if let Err(e) = simulate_paste(&text) {
                    eprintln!("Failed to simulate paste: {}", e);
                }
            }
        }
    });

    // Wrap list in scrolled window
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .child(&list_box)
        .build();

    main_box.append(&scrolled_window);

    // Add instructions
    let instructions = Label::new(Some("Press Enter or click to paste, Esc to cancel"));
    instructions.add_css_class("dim-label");
    instructions.add_css_class("caption");
    main_box.append(&instructions);

    // Handle Escape key to close window
    let event_controller = gtk4::EventControllerKey::new();
    let window_clone = window.clone();
    event_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            window_clone.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    window.add_controller(event_controller);

    window.set_child(Some(&main_box));
    window.present();

    // Auto-focus the list
    list_box.grab_focus();
}

fn create_entry_row(entry: &ClipboardEntry) -> ListBoxRow {
    let row = ListBoxRow::new();

    // Truncate text for display (show first 100 chars)
    let display_text = if entry.text.len() > 100 {
        format!("{}...", &entry.text[..100])
    } else {
        entry.text.clone()
    };

    // Replace newlines with spaces for single-line display
    let display_text = display_text.replace('\n', " ").replace('\r', "");

    let label = Label::new(Some(&display_text));
    label.set_xalign(0.0);
    label.set_margin_top(8);
    label.set_margin_bottom(8);
    label.set_margin_start(12);
    label.set_margin_end(12);
    label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    // Store full text in label for retrieval on activation
    label.set_text(&entry.text);

    row.set_child(Some(&label));
    row
}

fn simulate_paste(text: &str) -> Result<()> {
    // Use xclip to set clipboard (more reliable than enigo for clipboard)
    std::process::Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()?
        .stdin
        .as_mut()
        .unwrap()
        .write_all(text.as_bytes())?;

    // Small delay to ensure clipboard is set
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Simulate Ctrl+V using enigo
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;

    enigo.key(Key::Control, enigo::Direction::Press)
        .map_err(|e| anyhow::anyhow!("Failed to press Control: {:?}", e))?;
    enigo.key(Key::Unicode('v'), enigo::Direction::Click)
        .map_err(|e| anyhow::anyhow!("Failed to click V: {:?}", e))?;
    enigo.key(Key::Control, enigo::Direction::Release)
        .map_err(|e| anyhow::anyhow!("Failed to release Control: {:?}", e))?;

    Ok(())
}