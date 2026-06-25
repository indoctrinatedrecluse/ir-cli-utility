#!/bin/bash

# Test: Force move overwrites an existing destination file.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_move_03"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "source content" > "source_file"
mkdir "dest_dir"
echo "destination content" > "dest_dir/source_file"

# --- Test ---
echo "Running test: ir move --force source_file dest_dir"
"$EXECUTABLE" move --force "source_file" "dest_dir"

# --- Verification ---
# The destination should now exist with the source's content.
if [ -f "dest_dir/source_file" ] && [ ! -f "source_file" ] && [[ "$(cat dest_dir/source_file)" == "source content" ]]; then
    echo "✅ PASS: Destination was overwritten successfully."
    RESULT=0
else
    echo "❌ FAIL: Destination was not overwritten correctly or source was not removed."
    ls -lR
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
