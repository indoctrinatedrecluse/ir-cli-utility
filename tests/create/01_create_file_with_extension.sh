#!/bin/bash

# Test: Default behavior creates a file when an extension is present.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_create_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# --- Test ---
echo "Running test: ir create new_file.txt"
"$EXECUTABLE" create "new_file.txt"

# --- Verification ---
if [ -f "new_file.txt" ]; then
    echo "✅ PASS: File 'new_file.txt' was created successfully."
    RESULT=0
else
    echo "❌ FAIL: File 'new_file.txt' was not created."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
