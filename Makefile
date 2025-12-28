.PHONY: build install install-user install-service uninstall-service clean run-daemon run-ui enable-service disable-service

# Build all binaries in release mode
build:
	cargo build --release

# Install system-wide (requires sudo)
install: build
	install -Dm755 target/release/daemon /usr/local/bin/clipboard-daemon
	install -Dm755 target/release/clipboard-ui /usr/local/bin/clipboard-ui

# Install to user's local bin (no sudo required)
install-user: build
	mkdir -p ~/.local/bin
	install -m755 target/release/daemon ~/.local/bin/clipboard-daemon
	install -m755 target/release/clipboard-ui ~/.local/bin/clipboard-ui
	@echo "Installed to ~/.local/bin/"
	@echo "Make sure ~/.local/bin is in your PATH"

# Install systemd service (user service)
install-service: install-user
	mkdir -p ~/.config/systemd/user
	cp clipboard-daemon.service ~/.config/systemd/user/
	systemctl --user daemon-reload
	@echo ""
	@echo "Service installed! To enable auto-start:"
	@echo "  systemctl --user enable clipboard-daemon"
	@echo ""
	@echo "To start now:"
	@echo "  systemctl --user start clipboard-daemon"

# Enable and start the service
enable-service: install-service
	systemctl --user enable --now clipboard-daemon
	@echo ""
	@echo "Service enabled and started!"
	@echo "Check status with: systemctl --user status clipboard-daemon"

# Disable and stop the service
disable-service:
	systemctl --user disable --now clipboard-daemon
	@echo "Service disabled and stopped"

# Uninstall the service
uninstall-service:
	-systemctl --user stop clipboard-daemon
	-systemctl --user disable clipboard-daemon
	rm -f ~/.config/systemd/user/clipboard-daemon.service
	systemctl --user daemon-reload
	@echo "Service uninstalled"

# Clean build artifacts
clean:
	cargo clean

# Run the daemon
run-daemon: build
	./target/release/daemon

# Run the UI (for testing)
run-ui: build
	./target/release/clipboard-ui

# Development build
dev:
	cargo build