#!/bin/bash

# Test: find with root paths supplied through stdin.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_find_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/docs"
touch "$TEST_DIR/docs/README.TXT"
touch "$TEST_DIR/notes.md"

# --- Test ---
echo "Running test: path | ir find -iname '*readme*'"
OUTPUT=$(printf "%s/docs\n" "$TEST_DIR" | "$EXECUTABLE" find -iname "*readme*")

# --- Verification ---
if [[ "$OUTPUT" == *"README.TXT"* ]] && [[ "$OUTPUT" != *"notes.md"* ]]; then
    echo "PASS: find searched piped root paths."
    RESULT=0
else
    echo "FAIL: find did not search piped root paths as expected."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
