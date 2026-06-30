#!/bin/sh
# install.sh
# Linux/macOS installer for ir-cli-utility

set -e

VERSION=${VERSION:-latest}
REPO="indoctrinatedrecluse/ir-cli-utility"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

install_man() {
    MAN_SRC="$1"
    # Resolve man directory
    MAN_DIR="/usr/local/share/man/man1"
    if [ ! -w "$MAN_DIR" ]; then
        MAN_DIR="$HOME/.local/share/man/man1"
    fi

    echo "Installing man page..."
    mkdir -p "$MAN_DIR"

    if [ -f "$MAN_SRC" ]; then
        cp "$MAN_SRC" "$MAN_DIR/ir.1"
    else
        # Download raw ir.1 from github
        RAW_MAN_URL="https://raw.githubusercontent.com/$REPO/main/docs/ir.1"
        echo "Downloading man page from $RAW_MAN_URL..."
        if command -v curl >/dev/null 2>&1; then
            curl -fsSL "$RAW_MAN_URL" -o "$MAN_DIR/ir.1"
        elif command -v wget >/dev/null 2>&1; then
            wget -qO "$MAN_DIR/ir.1" "$RAW_MAN_URL"
        else
            echo "Warning: curl or wget not found. Skipping man page installation."
            return
        fi
    fi

    if command -v gzip >/dev/null 2>&1; then
        gzip -f "$MAN_DIR/ir.1"
        echo "Man page installed to $MAN_DIR/ir.1.gz"
    else
        echo "Man page installed to $MAN_DIR/ir.1 (gzip not found)"
    fi
}

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

# Create temporary file path and extraction directory
TEMP_TAR="/tmp/ir-linux.tar.gz"
EXTRACT_DIR="/tmp/ir-extract"

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
mkdir -p "$EXTRACT_DIR"
tar -xzf "$TEMP_TAR" -C "$EXTRACT_DIR"
rm "$TEMP_TAR"

# Move the executable
mv "$EXTRACT_DIR/ir" "$INSTALL_DIR/ir"
chmod +x "$INSTALL_DIR/ir"

# Install the man page
install_man "$EXTRACT_DIR/ir.1"

# Clean up extraction directory
rm -rf "$EXTRACT_DIR"

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
