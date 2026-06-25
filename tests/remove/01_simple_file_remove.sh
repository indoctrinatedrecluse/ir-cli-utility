#!/bin/bash

# Test: Simple, non-interactive file removal.

# --- Setup ---
echo "Building..."
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_remove_01"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

touch "file_to_remove.txt"

# --- Test ---
echo "Running test: ir remove file_to_remove.txt"
# The -y is added to skip any potential prompts for this basic test
"$EXECUTABLE" remove -y "file_to_remove.txt"

# --- Verification ---
if [ ! -f "file_to_remove.txt" ]; then
    echo "✅ PASS: File was removed successfully."
    RESULT=0
else
    echo "❌ FAIL: File still exists after remove operation."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
