#!/bin/bash

# Test: Renaming a folder to a file with an extension should fail.

# --- Setup ---
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_rename_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
mkdir "my_folder"

# --- Test ---
echo "Running test: ir rename my_folder invalid.txt"
# We expect this command to fail, so we capture its output.
# The '|| true' prevents the script from exiting due to 'set -e' if the command fails as expected.
OUTPUT=$("$EXECUTABLE" rename "my_folder" "invalid.txt" 2>&1) || true

# --- Verification ---
if [[ "$OUTPUT" == *"Error: Cannot rename a folder"* ]]; then
    echo "✅ PASS: Command failed with the correct error message."
    RESULT=0
else
    echo "❌ FAIL: Command did not produce the expected error."
    echo "Output was: $OUTPUT"
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
