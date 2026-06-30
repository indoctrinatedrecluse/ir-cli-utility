#!/bin/sh
# install.sh
# Linux/macOS installer for ir-cli-utility

set -e

VERSION=${VERSION:-latest}
REPO="indoctrinatedrecluse/ir-cli-utility"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

if [ "$OS" != "linux" ]; then
    echo "Warning: Pre-compiled binaries are currently only built for Linux/Windows."
    echo "Attempting to install from source using Cargo..."
    if command -v cargo >/dev/null 2>&1; then
        cargo install --git https://github.com/$REPO.git
        exit 0
    else
        echo "Error: Cargo is not installed. Please install Rust and Cargo from https://rustup.rs/ to compile from source."
        exit 1
    fi
fi

if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/ir-linux.tar.gz"
else
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/ir-linux.tar.gz"
fi

# Resolve installation directory
# Prefer /usr/local/bin if writable, otherwise default to ~/.local/bin
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    USE_SUDO=""
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo "Installing ir-cli-utility to $INSTALL_DIR..."

# Create temporary file path
TEMP_TAR="/tmp/ir-linux.tar.gz"

echo "Downloading from $DOWNLOAD_URL..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_TAR"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "$TEMP_TAR" "$DOWNLOAD_URL"
else
    echo "Error: Neither curl nor wget is installed."
    exit 1
fi

# Extract the archive
tar -xzf "$TEMP_TAR" -C "$INSTALL_DIR"
rm "$TEMP_TAR"

# Make the executable executable
chmod +x "$INSTALL_DIR/ir"

# If we installed to ~/.local/bin, make sure it is in PATH
if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
    case :$PATH: in
        *:$INSTALL_DIR:*) ;;
        *)
            echo "Adding $INSTALL_DIR to PATH in shell profile..."
            if [ -n "$SHELL" ]; then
                SHELL_NAME=$(basename "$SHELL")
                if [ "$SHELL_NAME" = "zsh" ] && [ -f "$HOME/.zshrc" ]; then
                    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc"
                elif [ -f "$HOME/.bashrc" ]; then
                    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
                else
                    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.profile"
                fi
            else
                echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.profile"
            fi
            echo "Please run: export PATH=\"\$PATH:$INSTALL_DIR\" or restart your shell."
            ;;
    esac
fi

echo "\n🎉 ir-cli-utility has been successfully installed!"
echo "You can now run: ir"
