#!/bin/bash
# Test: clip action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# Helper to get clipboard content on Linux
get_clip_text() {
    if command -v wl-paste &>/dev/null; then
        wl-paste -n
    elif command -v xclip &>/dev/null; then
        xclip -selection clipboard -o
    elif command -v xsel &>/dev/null; then
        xsel --clipboard --output
    else
        echo "NO_TOOL"
    fi
}

TOOL_STATUS=$(get_clip_text)
if [ "$TOOL_STATUS" = "NO_TOOL" ]; then
    echo "SKIPPING: No clipboard tool installed (wl-clipboard, xclip, or xsel)."
    exit 0
fi

# --- Test 1: Write to clip via redirection ---
echo "Testing echo redirection to clip..."
$EXECUTABLE echo "clip content 1" ">" clip
CLIP_TEXT=$(get_clip_text)
if [ "$CLIP_TEXT" = "clip content 1" ] || [ "$CLIP_TEXT" = "clip content 1"$'\r' ]; then
    echo "PASS: Redirection > clip wrote successfully."
else
    echo "FAIL: Clipboard content mismatch: '$CLIP_TEXT'"
    exit 1
fi

# --- Test 2: Append to clip via redirection ---
echo "Testing echo append redirection to clip..."
$EXECUTABLE echo "clip content 2" ">>" clip
CLIP_TEXT=$(get_clip_text)
if [[ "$CLIP_TEXT" == *"clip content 1"* ]] && [[ "$CLIP_TEXT" == *"clip content 2"* ]]; then
    echo "PASS: Redirection >> clip appended successfully."
else
    echo "FAIL: Clipboard append mismatch. Content: '$CLIP_TEXT'"
    exit 1
fi

# --- Test 3: Cat redirection to clip ---
echo "Testing cat redirection to clip..."
TEMP_FILE="temp_cat_clip.txt"
echo "hello cat clip" > "$TEMP_FILE"
$EXECUTABLE cat "$TEMP_FILE" ">" clip
CLIP_TEXT=$(get_clip_text)
if [[ "$CLIP_TEXT" == *"hello cat clip"* ]]; then
    echo "PASS: Cat redirection > clip wrote successfully."
else
    echo "FAIL: Cat clipboard content mismatch: '$CLIP_TEXT'"
    rm -f "$TEMP_FILE"
    exit 1
fi
rm -f "$TEMP_FILE"

# --- Test 4: Clear clip ---
echo "Testing ir clip --clear..."
$EXECUTABLE clip --clear
CLIP_TEXT=$(get_clip_text)
if [ -z "$CLIP_TEXT" ]; then
    echo "PASS: Clipboard cleared successfully."
else
    echo "FAIL: Clipboard was not cleared: '$CLIP_TEXT'"
    exit 1
fi

echo "ALL CLIP TESTS PASSED"
exit 0
