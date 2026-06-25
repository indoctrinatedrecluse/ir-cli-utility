#!/bin/bash

# Test: Basic list functionality.

# --- Setup ---
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_list_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
touch "file1.txt"
mkdir "folder1"

# --- Test ---
echo "Running test: ir list"
OUTPUT=$("$EXECUTABLE" list)

# --- Verification ---
if [[ "$OUTPUT" == *"file1.txt"* ]] && [[ "$OUTPUT" == *"folder1"* ]]; then
    echo "✅ PASS: 'list' command output contains the created file and folder."
    RESULT=0
else
    echo "❌ FAIL: 'list' command output is missing expected items."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
