#!/bin/bash

# Test: Simple file move to a directory.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_move_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

touch "source_file.txt"
mkdir "dest_dir"

# --- Test ---
echo "Running test: ir move source_file.txt dest_dir"
"$EXECUTABLE" move "source_file.txt" "dest_dir"

# --- Verification ---
if [ -f "dest_dir/source_file.txt" ] && [ ! -f "source_file.txt" ]; then
    echo "✅ PASS: File was moved successfully."
    RESULT=0
else
    echo "❌ FAIL: File was not found in the destination or the source file still exists."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
