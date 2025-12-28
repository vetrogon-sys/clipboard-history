#!/bin/bash
# Build script for clipboard-history project

set -e  # Exit on error

echo "======================================"
echo "Building Clipboard History Manager"
echo "======================================"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build in release mode
echo "Building all components in release mode..."
cargo build --release

echo ""
echo "======================================"
echo "Build completed successfully!"
echo "======================================"
echo ""
echo "Binaries are located at:"
echo "  - Daemon: target/release/daemon"
echo "  - UI:     target/release/clipboard-ui"
echo ""
echo "To install the binaries, run:"
echo "  make install-user    # Install to ~/.local/bin (no sudo)"
echo "  sudo make install    # Install to /usr/local/bin (requires sudo)"
echo ""