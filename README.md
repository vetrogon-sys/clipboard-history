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

## Configuration

Configuration is loaded from a TOML file.

Example:

```toml
max_entries = 200
max_entry_size = 1048576 # 1MB
deduplicate = true
store_primary_selection = false

[hotkeys]
popup = "Ctrl+Shift+V"

[filters]
ignore_regex = ["password", "token"]
ignore_apps = ["keepassxc", "bitwarden"]

