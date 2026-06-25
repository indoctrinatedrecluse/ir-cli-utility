#!/bin/bash

# Test: Create a simple zip archive from a single file.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_archive_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "This is a test file." > "test_file.txt"

# --- Test ---
echo "Running test: ir archive test_file.txt --format zip"
"$EXECUTABLE" archive "test_file.txt" --format zip

# --- Verification ---
if [ -f "test_file.zip" ]; then
    echo "✅ PASS: Archive 'test_file.zip' was created successfully."
    # Optional: Add an unzip command here to verify contents if `unzip` is available
    RESULT=0
else
    echo "❌ FAIL: Archive 'test_file.zip' was not created."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
