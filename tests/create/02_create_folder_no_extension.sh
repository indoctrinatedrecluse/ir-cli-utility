#!/bin/bash

# Test: Default behavior creates a folder when no extension is present.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_create_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# --- Test ---
echo "Running test: ir create new_folder"
"$EXECUTABLE" create "new_folder"

# --- Verification ---
if [ -d "new_folder" ]; then
    echo "✅ PASS: Directory 'new_folder' was created successfully."
    RESULT=0
else
    echo "❌ FAIL: Directory 'new_folder' was not created."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
