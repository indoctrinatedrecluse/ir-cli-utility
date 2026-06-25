#!/bin/bash

# Test: Simple file rename

# --- Setup ---
# Build the executable
echo "Building..."
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# Create a temporary directory for the test
TEST_DIR="temp_test_rename_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create a file to rename
touch "original.txt"

# --- Test ---
echo "Running test: ir rename original.txt renamed.txt"
"$EXECUTABLE" rename "original.txt" "renamed.txt"

# --- Verification ---
if [ -f "renamed.txt" ] && [ ! -f "original.txt" ]; then
    echo "✅ PASS: 'renamed.txt' exists and 'original.txt' does not."
    RESULT=0
else
    echo "❌ FAIL: 'renamed.txt' was not created or 'original.txt' was not removed."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
