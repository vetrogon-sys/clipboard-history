.PHONY: build install install-user clean run-daemon run-ui

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