#!/bin/bash

# Test: Interactive rename with 'y' confirmation.

# --- Setup ---
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_rename_03"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"
touch "interactive.txt"

# --- Test ---
echo "Running test: echo 'y' | ir rename -i interactive.txt confirmed.txt"
# Pipe 'y' and a newline into the command.
echo "y" | "$EXECUTABLE" rename -i "interactive.txt" "confirmed.txt"

# --- Verification ---
if [ -f "confirmed.txt" ] && [ ! -f "interactive.txt" ]; then
    echo "✅ PASS: File was successfully renamed after confirmation."
    RESULT=0
else
    echo "❌ FAIL: File was not renamed correctly."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
