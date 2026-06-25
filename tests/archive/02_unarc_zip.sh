#!/bin/bash

# Test: Extract a zip archive.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_archive_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create a dummy archive
zip -q test.zip file1.txt

# --- Test ---
echo "Running test: ir archive --unarc test.zip"
"$EXECUTABLE" archive --unarc "test.zip"

# --- Verification ---
if [ -f "file1.txt" ]; then
    echo "✅ PASS: File was extracted successfully."
    RESULT=0
else
    echo "❌ FAIL: Extracted file not found."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
