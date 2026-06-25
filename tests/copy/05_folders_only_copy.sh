#!/bin/bash

# Test: Copy with -l (folders only).

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_copy_05"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

mkdir -p "source_dir/subdir"
touch "source_dir/file1.txt"
mkdir "dest_dir"

# --- Test ---
echo "Running test: ir copy -l source_dir dest_dir"
"$EXECUTABLE" copy -l "source_dir" "dest_dir"

# --- Verification ---
if [ -d "dest_dir/source_dir/subdir" ] && [ ! -f "dest_dir/source_dir/file1.txt" ]; then
    echo "✅ PASS: Only the subdirectory was copied, not the file."
    RESULT=0
else
    echo "❌ FAIL: The file was copied or the subdirectory was not."
    find dest_dir
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
