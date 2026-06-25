#!/bin/bash

# Test: Simple file copy to a directory.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_copy_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

touch "source_file.txt"
mkdir "dest_dir"

# --- Test ---
echo "Running test: ir copy source_file.txt dest_dir"
"$EXECUTABLE" copy "source_file.txt" "dest_dir"

# --- Verification ---
if [ -f "dest_dir/source_file.txt" ]; then
    echo "✅ PASS: File was copied successfully to the destination directory."
    RESULT=0
else
    echo "❌ FAIL: File was not found in the destination directory."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
