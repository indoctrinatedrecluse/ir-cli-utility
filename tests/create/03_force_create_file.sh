#!/bin/bash

# Test: --create-file switch forces file creation without an extension.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_create_03"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# --- Test ---
echo "Running test: ir create --create-file my_file"
"$EXECUTABLE" create --create-file "my_file"

# --- Verification ---
if [ -f "my_file" ]; then
    echo "✅ PASS: File 'my_file' was created successfully."
    RESULT=0
else
    echo "❌ FAIL: File 'my_file' was not created."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
