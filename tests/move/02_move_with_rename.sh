#!/bin/bash

# Test: Move a single file with the --rename switch.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_move_02"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

touch "source.log"
mkdir "logs_archive"

# --- Test ---
echo "Running test: ir move source.log logs_archive --rename archive.log"
"$EXECUTABLE" move "source.log" "logs_archive" --rename "archive.log"

# --- Verification ---
if [ -f "logs_archive/archive.log" ] && [ ! -f "source.log" ]; then
    echo "✅ PASS: File was moved and renamed successfully."
    RESULT=0
else
    echo "❌ FAIL: Renamed file not found or source file still exists."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
