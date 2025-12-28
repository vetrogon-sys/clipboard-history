#!/bin/bash
# Installation script for clipboard-history

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="$HOME/.local/bin"
SERVICE_DIR="$HOME/.config/systemd/user"

echo "======================================"
echo "Installing Clipboard History Manager"
echo "======================================"
echo ""

# Check if binaries exist
if [ ! -f "$SCRIPT_DIR/target/release/daemon" ] || [ ! -f "$SCRIPT_DIR/target/release/clipboard-ui" ]; then
    echo "Binaries not found. Building first..."
    "$SCRIPT_DIR/build.sh"
fi

# Create directories if they don't exist
mkdir -p "$INSTALL_DIR"
mkdir -p "$SERVICE_DIR"

# Install binaries
echo "Installing binaries to $INSTALL_DIR..."
install -m755 "$SCRIPT_DIR/target/release/daemon" "$INSTALL_DIR/clipboard-daemon"
install -m755 "$SCRIPT_DIR/target/release/clipboard-ui" "$INSTALL_DIR/clipboard-ui"

echo "✓ Binaries installed"
echo ""

# Install systemd service
echo "Installing systemd service..."
cp "$SCRIPT_DIR/clipboard-daemon.service" "$SERVICE_DIR/clipboard-daemon.service"

echo "✓ Systemd service installed"
echo ""

# Reload systemd
systemctl --user daemon-reload

echo "======================================"
echo "Installation complete!"
echo "======================================"
echo ""
echo "Installed to:"
echo "  - Binaries: $INSTALL_DIR"
echo "  - Service:  $SERVICE_DIR/clipboard-daemon.service"
echo ""
echo "To start the daemon now:"
echo "  systemctl --user start clipboard-daemon"
echo ""
echo "To enable auto-start on login:"
echo "  systemctl --user enable clipboard-daemon"
echo ""
echo "To check status:"
echo "  systemctl --user status clipboard-daemon"
echo ""
echo "To view logs:"
echo "  journalctl --user -u clipboard-daemon -f"
echo ""