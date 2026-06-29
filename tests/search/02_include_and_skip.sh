#!/bin/bash

# Test: search include filters and default skipped extensions.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_search_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
printf "needle in rust\n" > "$TEST_DIR/keep.rs"
printf "needle in text\n" > "$TEST_DIR/skip.txt"
printf "needle in archive\n" > "$TEST_DIR/archive.zip"

# --- Test ---
echo "Running test: ir search needle temp_test_search_02 --include rs"
INCLUDE_OUTPUT=$("$EXECUTABLE" search needle "$TEST_DIR" --include rs)
DEFAULT_OUTPUT=$("$EXECUTABLE" search needle "$TEST_DIR")

# --- Verification ---
if [[ "$INCLUDE_OUTPUT" == *"keep.rs"* ]] && [[ "$INCLUDE_OUTPUT" != *"skip.txt"* ]] && [[ "$DEFAULT_OUTPUT" != *"archive.zip"* ]]; then
    echo "PASS: search respected include filters and skipped archive extension."
    RESULT=0
else
    echo "FAIL: search did not filter files as expected."
    echo "Include output was:"
    echo "$INCLUDE_OUTPUT"
    echo "Default output was:"
    echo "$DEFAULT_OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
