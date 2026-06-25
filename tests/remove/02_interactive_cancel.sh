#!/bin/bash

# Test: Interactive remove where the user cancels.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_remove_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
touch "file_to_keep.txt"

# --- Test ---
echo "Running test: echo 'n' | ir remove -i file_to_keep.txt"
# Pipe 'n' and a newline into the command to simulate user cancellation.
OUTPUT=$(echo "n" | "$EXECUTABLE" remove -i "file_to_keep.txt")

# --- Verification ---
if [ -f "file_to_keep.txt" ] && [[ "$OUTPUT" == *"Operation cancelled."* ]]; then
    echo "✅ PASS: File was not removed and cancellation message was shown."
    RESULT=0
else
    echo "❌ FAIL: File was removed or cancellation message was not shown."
    echo "Output was: $OUTPUT"
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
