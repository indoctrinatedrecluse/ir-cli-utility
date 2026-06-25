#!/bin/bash

# Test: Force remove of a non-empty directory.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_remove_03"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

mkdir -p "dir_to_remove/subdir"
touch "dir_to_remove/file.txt"

# --- Test ---
echo "Running test: ir remove -f dir_to_remove"
"$EXECUTABLE" remove -f "dir_to_remove"

# --- Verification ---
if [ ! -d "dir_to_remove" ]; then
    echo "✅ PASS: Non-empty directory was force-removed successfully."
    RESULT=0
else
    echo "❌ FAIL: Directory still exists after force remove operation."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
