#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="saravenpi/donut"
BINARY_NAME="donut"
INSTALL_DIR="$HOME/.local/bin"
TMP_DIR="/tmp/donut-install"

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case $arch in
        x86_64) arch="amd64" ;;
        arm64|aarch64) arch="arm64" ;;
        armv7l) arch="arm" ;;
        *) log_error "Unsupported architecture: $arch"; exit 1 ;;
    esac

    case $os in
        linux) os="linux" ;;
        darwin) os="darwin" ;;
        *) log_error "Unsupported OS: $os"; exit 1 ;;
    esac

    echo "${os}_${arch}"
}

# Get latest release version
get_latest_version() {
    if command_exists curl; then
        curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command_exists wget; then
        wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Check if binary exists and get version
get_installed_version() {
    if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
        "$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null | head -n1 | awk '{print $NF}' || echo ""
    else
        echo ""
    fi
}

# Download and install from release
install_from_release() {
    local version="$1"
    local platform="$2"
    local download_url="https://github.com/$REPO/releases/download/$version/${BINARY_NAME}_${platform}.tar.gz"

    log_info "Downloading $BINARY_NAME $version for $platform..."

    # Create temporary directory
    mkdir -p "$TMP_DIR"
    cd "$TMP_DIR"

    # Download release
    if command_exists curl; then
        curl -sL "$download_url" -o "${BINARY_NAME}.tar.gz"
    elif command_exists wget; then
        wget -q "$download_url" -O "${BINARY_NAME}.tar.gz"
    fi

    # Extract and install
    tar -xzf "${BINARY_NAME}.tar.gz"
    chmod +x "$BINARY_NAME"

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Move binary to install directory
    mv "$BINARY_NAME" "$INSTALL_DIR/"

    # Cleanup
    cd - >/dev/null
    rm -rf "$TMP_DIR"

    log_success "$BINARY_NAME $version installed to $INSTALL_DIR"
}

# Build from source (fallback or for development)
build_from_source() {
    log_info "Building from source..."

    # Check if Go is installed
    if ! command_exists go; then
        log_error "Go is not installed. Please install Go or use a pre-built binary."
        exit 1
    fi

    # Create temporary directory
    mkdir -p "$TMP_DIR"
    cd "$TMP_DIR"

    # Clone repository
    if command_exists git; then
        git clone "https://github.com/$REPO.git" .
    else
        log_error "Git is not installed. Cannot build from source."
        exit 1
    fi

    # Check if binary already exists and is up to date
    local current_commit=$(git rev-parse HEAD)
    local installed_version=$(get_installed_version)

    if [ -n "$installed_version" ] && [ "$installed_version" = "$current_commit" ]; then
        log_info "$BINARY_NAME is already up to date (commit: ${current_commit:0:7})"
        cd - >/dev/null
        rm -rf "$TMP_DIR"
        return
    fi

    # Build binary
    log_info "Building $BINARY_NAME..."
    go build -ldflags "-X main.version=$current_commit" -o "$BINARY_NAME" .

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Move binary to install directory
    mv "$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"

    # Cleanup
    cd - >/dev/null
    rm -rf "$TMP_DIR"

    log_success "$BINARY_NAME built and installed to $INSTALL_DIR (commit: ${current_commit:0:7})"
}

# Main installation logic
main() {
    log_info "Installing $BINARY_NAME..."

    # Detect platform
    local platform=$(detect_platform)
    log_info "Detected platform: $platform"

    # Get latest version
    local latest_version=$(get_latest_version)
    local installed_version=$(get_installed_version)

    if [ -n "$latest_version" ]; then
        log_info "Latest version: $latest_version"

        # Check if already installed and up to date
        if [ "$installed_version" = "$latest_version" ]; then
            log_success "$BINARY_NAME $latest_version is already installed and up to date!"
            return
        fi

        # Try to install from release
        if install_from_release "$latest_version" "$platform" 2>/dev/null; then
            return
        else
            log_warning "Failed to install from release, falling back to building from source..."
        fi
    else
        log_warning "Could not determine latest version, building from source..."
    fi

    # Fallback to building from source
    build_from_source

    # Verify installation
    if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
        log_success "Installation completed successfully!"
        log_info "Make sure $INSTALL_DIR is in your PATH to use $BINARY_NAME from anywhere."

        # Check if directory is in PATH
        if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
            log_warning "$INSTALL_DIR is not in your PATH."
            log_info "Add the following line to your shell profile (.bashrc, .zshrc, etc.):"
            echo "export PATH=\"\$PATH:$INSTALL_DIR\""
        fi
    else
        log_error "Installation failed!"
        exit 1
    fi
}

# Run main function
main "$@"