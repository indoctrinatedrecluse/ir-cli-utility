#!/bin/bash

# Test: Default behavior fails to remove a non-empty directory without -y or -f.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_remove_04"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

mkdir -p "dir_to_keep/subdir"

# --- Test ---
echo "Running test: ir remove dir_to_keep (with 'n' piped in)"
# Pipe 'n' to the confirmation prompt.
OUTPUT=$(echo "n" | "$EXECUTABLE" remove "dir_to_keep")

# --- Verification ---
if [ -d "dir_to_keep" ] && [[ "$OUTPUT" == *"Operation cancelled."* ]]; then
    echo "✅ PASS: Operation was cancelled as expected for non-empty directory."
    RESULT=0
else
    echo "❌ FAIL: Directory was removed or cancellation message was not shown."
    echo "Output was: $OUTPUT"
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
