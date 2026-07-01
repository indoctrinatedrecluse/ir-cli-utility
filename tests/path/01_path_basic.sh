#!/bin/bash
# Test: path action on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# Setup dummy environment to run profile changes in a temporary file instead of ~/.bashrc
export HOME="$PWD/temp_home"
mkdir -p "$HOME"
touch "$HOME/.bashrc"

# 1. Test listing
echo "Testing path list..."
LIST_OUT=$($EXECUTABLE path)
if [[ "$LIST_OUT" == *"Active Process PATH"* ]]; then
    echo "PASS: Path list output valid."
else
    echo "FAIL: Path list output invalid."
    rm -rf "$HOME"
    exit 1
fi

# 2. Test adding path
echo "Testing path add..."
ADD_DIR="/temp/bin"
ADD_OUT=$($EXECUTABLE path -a "$ADD_DIR")
if [[ "$ADD_OUT" == *"Success: Appended"* ]]; then
    echo "PASS: Successfully added directory."
else
    echo "FAIL: Failed to add directory."
    rm -rf "$HOME"
    exit 1
fi

# 3. Verify it was written to profile
PROFILE_CONTENT=$(cat "$HOME/.bashrc")
if [[ "$PROFILE_CONTENT" == *"export PATH=\"/temp/bin:\$PATH\""* ]]; then
    echo "PASS: Profile contains added path export."
else
    echo "FAIL: Profile does not contain added path export."
    rm -rf "$HOME"
    exit 1
fi

# 4. Test removing path
echo "Testing path remove..."
REMOVE_OUT=$($EXECUTABLE path -r "$ADD_DIR")
if [[ "$REMOVE_OUT" == *"Success: Removed"* ]]; then
    echo "PASS: Successfully removed directory."
else
    echo "FAIL: Failed to remove directory."
    rm -rf "$HOME"
    exit 1
fi

# 5. Verify it was removed from profile
PROFILE_CONTENT_2=$(cat "$HOME/.bashrc")
if [[ "$PROFILE_CONTENT_2" != *"/temp/bin"* ]]; then
    echo "PASS: Profile no longer contains path export."
else
    echo "FAIL: Profile still contains path export."
    rm -rf "$HOME"
    exit 1
fi

rm -rf "$HOME"
exit 0
