#!/bin/bash

# Test: Default recursive copy of a directory.

# --- Setup ---
cargo build --quiet
EXECUTABLE="../target/debug/ir"
TEST_DIR="temp_test_copy_03"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

mkdir -p "source_dir/subdir"
touch "source_dir/file1.txt"
touch "source_dir/subdir/file2.txt"
mkdir "dest_dir"

# --- Test ---
echo "Running test: ir copy source_dir dest_dir"
"$EXECUTABLE" copy "source_dir" "dest_dir"

# --- Verification ---
if [ -d "dest_dir/source_dir/subdir" ] && [ -f "dest_dir/source_dir/file1.txt" ] && [ -f "dest_dir/source_dir/subdir/file2.txt" ]; then
    echo "✅ PASS: Directory was copied recursively."
    RESULT=0
else
    echo "❌ FAIL: Recursive copy did not produce the expected structure."
    find dest_dir
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
