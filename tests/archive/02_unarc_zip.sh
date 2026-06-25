#!/bin/bash

# Test: Extract a zip archive.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_archive_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create a dummy archive using our helper
rustc ../create_zip.rs
./create_zip test.zip

# --- Test ---
echo "Running test: ir archive --unarc test.zip"
"$EXECUTABLE" archive --unarc "test.zip"

# --- Verification ---
if [ -f "file1.txt" ] && [[ "$(cat file1.txt)" == "Hello, world!" ]]; then
    echo "✅ PASS: File was extracted successfully with correct content."
    RESULT=0
else
    echo "❌ FAIL: Extracted file not found or content is incorrect."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"
rm tests/archive/create_zip

exit $RESULT
