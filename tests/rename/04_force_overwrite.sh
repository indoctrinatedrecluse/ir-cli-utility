#!/bin/bash

# Test: Force rename overwrites an existing destination file.

# --- Setup ---
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_rename_04"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
touch "source_file"
echo "destination content" > "destination_file"

# --- Test ---
echo "Running test: ir rename -f source_file destination_file"
"$EXECUTABLE" rename -f "source_file" "destination_file"

# --- Verification ---
# The destination should now exist, but be empty (like the source was).
if [ -f "destination_file" ] && [ ! -f "source_file" ] && [ ! -s "destination_file" ]; then
    echo "✅ PASS: Destination was overwritten successfully."
    RESULT=0
else
    echo "❌ FAIL: Destination was not overwritten correctly."
    ls -l
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
