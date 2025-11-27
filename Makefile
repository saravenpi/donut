# Donut - Simple todolist TUI application (Rust)
BINARY_NAME=donut
VERSION?=$(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")

# Build directory
BUILD_DIR=build
DIST_DIR=dist

# Platforms for cross-compilation
TARGETS=x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin

.PHONY: all build clean test install uninstall release help build-all

# Default target
all: build

# Build for current platform
build:
	@echo "Building $(BINARY_NAME) v$(VERSION)..."
	@mkdir -p $(BUILD_DIR)
	cargo build --release
	@cp target/release/$(BINARY_NAME) $(BUILD_DIR)/$(BINARY_NAME)

# Build for all platforms
build-all: clean
	@echo "Building $(BINARY_NAME) v$(VERSION) for all platforms..."
	@mkdir -p $(DIST_DIR)
	@for target in $(TARGETS); do \
		echo "Building for $$target..."; \
		cargo build --release --target $$target || echo "Failed to build for $$target (target may not be installed)"; \
		if [ -f target/$$target/release/$(BINARY_NAME) ]; then \
			mkdir -p $(DIST_DIR)/$$target; \
			cp target/$$target/release/$(BINARY_NAME) $(DIST_DIR)/$$target/; \
			tar -czf $(DIST_DIR)/$(BINARY_NAME)_$$target.tar.gz -C $(DIST_DIR)/$$target $(BINARY_NAME); \
			rm -rf $(DIST_DIR)/$$target; \
		fi; \
	done
	@echo "All builds completed in $(DIST_DIR)/"

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Install to ~/.local/bin
install: build
	@echo "Installing $(BINARY_NAME) to ~/.local/bin..."
	@mkdir -p ~/.local/bin
	@cp $(BUILD_DIR)/$(BINARY_NAME) ~/.local/bin/
	@chmod +x ~/.local/bin/$(BINARY_NAME)
	@echo "$(BINARY_NAME) installed successfully!"
	@echo "Make sure ~/.local/bin is in your PATH"

# Uninstall from ~/.local/bin
uninstall:
	@echo "Uninstalling $(BINARY_NAME) from ~/.local/bin..."
	@rm -f ~/.local/bin/$(BINARY_NAME)
	@echo "$(BINARY_NAME) uninstalled successfully!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(BUILD_DIR) $(DIST_DIR)
	@cargo clean

# Create release (requires gh CLI)
release: build-all
	@echo "Creating release v$(VERSION)..."
	@if [ "$(VERSION)" = "dev" ]; then \
		echo "Error: Cannot create release with dev version. Please tag your commit first."; \
		exit 1; \
	fi
	@gh release create v$(VERSION) $(DIST_DIR)/*.tar.gz --title "Release v$(VERSION)" --generate-notes

# Show help
help:
	@echo "Available targets:"
	@echo "  build      - Build for current platform"
	@echo "  build-all  - Build for all supported platforms"
	@echo "  test       - Run tests"
	@echo "  install    - Install to ~/.local/bin"
	@echo "  uninstall  - Remove from ~/.local/bin"
	@echo "  clean      - Clean build artifacts"
	@echo "  release    - Create GitHub release (requires version tag)"
	@echo "  help       - Show this help message"