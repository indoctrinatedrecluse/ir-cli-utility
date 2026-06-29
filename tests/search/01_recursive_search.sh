#!/bin/bash

# Test: recursive search finds matching file contents.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_search_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/src/nested"
printf "fn main() {}\nTODO: wire feature\n" > "$TEST_DIR/src/main.rs"
printf "nothing here\n" > "$TEST_DIR/src/nested/notes.txt"

# --- Test ---
echo "Running test: ir search TODO temp_test_search_01"
OUTPUT=$("$EXECUTABLE" search TODO "$TEST_DIR")

# --- Verification ---
if [[ "$OUTPUT" == *"main.rs:2:TODO: wire feature"* ]] && [[ "$OUTPUT" != *"notes.txt"* ]]; then
    echo "PASS: search found the recursive content match."
    RESULT=0
else
    echo "FAIL: search output did not match expected recursive search behavior."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
