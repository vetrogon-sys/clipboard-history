# Clipboard Buffer for Linux

A configurable clipboard history manager for Linux — inspired by Windows Clipboard History.

The goal is to provide a fast, reliable, and secure clipboard buffer that works on modern Linux desktops, with proper support for X11 and Wayland.

---

## Features (planned)

- Store last **N** clipboard entries
- Configurable history size and limits
- Deduplication of clipboard entries
- Keyboard-driven popup UI
- Fast search and paste
- X11 and Wayland support
- Privacy-aware filtering (password managers, regex rules)
- Optional persistent storage
- Minimal memory footprint

---

## Architecture Overview

The project is split into multiple crates to keep concerns isolated:

```
clipboard-buffer
├── core # Clipboard buffer logic, policies, config
├── clipboard # OS-specific clipboard listeners (X11 / Wayland)
├── daemon # Background service
└── ui # Popup UI (GTK)
```


### Core Principles

- **Daemon-first design**
- **No UI dependency in core logic**
- **Platform abstraction**
- **Memory safety**
- **Low latency**

---

## Clipboard Model

Each clipboard entry contains:

- Text content
- Timestamp
- Optional source application
- Size metadata

Behavior:
- Most recent entries appear first
- Duplicate entries are moved to the top
- Size and content filters are applied before storing

---

## Prerequisites

Before building, ensure you have the following dependencies installed:

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install build-essential libxdo-dev libgtk-4-dev pkg-config xclip
```

### Fedora
```bash
sudo dnf install gcc libxdo-devel gtk4-devel pkg-config xclip
```

### Arch Linux
```bash
sudo pacman -S base-devel xdotool gtk4 pkg-config xclip
```

You also need Rust. Install it from [https://rustup.rs/](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Building

### Quick Build
```bash
./build.sh
```

### Manual Build
```bash
cargo build --release
```

Binaries will be available at:
- `target/release/daemon` - The clipboard daemon
- `target/release/clipboard-ui` - The popup UI

---

## Installation

### Automated Installation (Recommended)
```bash
./install.sh
```

This will:
1. Build the project in release mode
2. Install binaries to `~/.local/bin/`
3. Install systemd service to `~/.config/systemd/user/`

### Manual Installation
```bash
# Install binaries
make install-user  # Install to ~/.local/bin (no sudo required)
# OR
sudo make install  # Install to /usr/local/bin (requires sudo)

# Install systemd service (for auto-start)
mkdir -p ~/.config/systemd/user
cp clipboard-daemon.service ~/.config/systemd/user/
systemctl --user daemon-reload
```

---

## Running the Daemon

### Start Manually
```bash
clipboard-daemon
```

### Start with systemd
```bash
# Start the daemon now
systemctl --user start clipboard-daemon

# Enable auto-start on login
systemctl --user enable clipboard-daemon

# Check status
systemctl --user status clipboard-daemon

# View logs
journalctl --user -u clipboard-daemon -f
```

---

## Usage

Once the daemon is running:

1. **Copy text** - Any text you copy will be automatically stored in the clipboard history
2. **Press `Ctrl+Shift+V`** - Opens the clipboard history popup UI
3. **Select an entry** - Click or press Enter to paste the selected entry
4. **Press `Esc`** - Close the popup without pasting

The daemon will:
- Monitor clipboard changes on X11
- Store up to 200 entries (configurable)
- Persist history to `~/.local/share/clipboard-history/history.json`
- Provide a D-Bus service at `com.clipboardhistory.Service`

---

## Configuration

Configuration is automatically loaded from `~/.config/clipboard-history/config.toml`.

When you first run the daemon, a default configuration file will be created automatically.

### Configuration File Location

- **Config**: `~/.config/clipboard-history/config.toml`
- **History**: `~/.local/share/clipboard-history/history.json`

### Configuration Options

```toml
# Maximum number of clipboard entries to store in history
max_entries = 100

# Maximum size of a single clipboard entry in bytes (1MB default)
max_entry_size = 1048576

# UI Configuration
[ui]
# Popup window width in pixels
width = 600

# Popup window height in pixels
height = 400

# Hotkey Configuration
[hotkey]
# Global hotkey to open clipboard history popup
# Format: Modifiers+Key (e.g., "Ctrl+Shift+V", "Alt+C", "Ctrl+Alt+H")
# Supported modifiers: Ctrl, Shift, Alt
# Supported keys: A-Z
popup = "Ctrl+Shift+V"
```

### Customizing Your Configuration

1. **Edit the config file**:
   ```bash
   nano ~/.config/clipboard-history/config.toml
   ```

2. **Example customizations**:
   ```toml
   # Store more entries
   max_entries = 200

   # Larger popup window
   [ui]
   width = 800
   height = 600

   # Different hotkey
   [hotkey]
   popup = "Ctrl+Alt+V"
   ```

3. **Restart the daemon** for changes to take effect:
   ```bash
   systemctl --user restart clipboard-daemon
   ```

See [`config.toml.example`](config.toml.example) for a complete example with all options documented.

