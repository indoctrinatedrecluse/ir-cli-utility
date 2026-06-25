#!/bin/bash

# Test: Basic cat functionality.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_cat_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "alpha\nbeta\ngamma\n" > "$FILE"

# --- Test ---
echo "Running test: ir cat sample.txt"
OUTPUT=$("$EXECUTABLE" cat "$FILE")

# --- Verification ---
if [[ "$OUTPUT" == *"alpha"* ]] && [[ "$OUTPUT" == *"beta"* ]] && [[ "$OUTPUT" == *"gamma"* ]]; then
    echo "PASS: 'cat' printed the file contents."
    RESULT=0
else
    echo "FAIL: 'cat' output was missing expected content."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
