#!/bin/bash

# Test: Copy with -f (files only).

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_copy_04"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

mkdir -p "source_dir/subdir"
touch "source_dir/file1.txt"
touch "source_dir/subdir/file2.txt"
mkdir "dest_dir"

# --- Test ---
echo "Running test: ir copy -f source_dir dest_dir"
"$EXECUTABLE" copy -f "source_dir" "dest_dir"

# --- Verification ---
if [ -f "dest_dir/source_dir/file1.txt" ] && [ ! -d "dest_dir/source_dir/subdir" ]; then
    echo "✅ PASS: Only the file was copied, not the subdirectory."
    RESULT=0
else
    echo "❌ FAIL: The subdirectory was copied or the file was not."
    find dest_dir
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
