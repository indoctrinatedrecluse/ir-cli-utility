#!/bin/bash

# Test: Basic tee functionality.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_tee_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE1="$TEST_DIR/file1.txt"
FILE2="$TEST_DIR/file2.txt"

# --- Test ---
echo "Running test: echo 'hello world' | ir tee file1 file2"
OUTPUT=$(echo "hello world" | "$EXECUTABLE" tee "$FILE1" "$FILE2")

# --- Verification ---
RESULT=0
CONTENT1=$(cat "$FILE1")
CONTENT2=$(cat "$FILE2")

if [ "$OUTPUT" != "hello world" ]; then
    echo "FAIL: tee stdout was incorrect: '$OUTPUT'"
    RESULT=1
elif [ "$CONTENT1" != "hello world" ]; then
    echo "FAIL: file1 content was incorrect: '$CONTENT1'"
    RESULT=1
elif [ "$CONTENT2" != "hello world" ]; then
    echo "FAIL: file2 content was incorrect: '$CONTENT2'"
    RESULT=1
else
    echo "PASS: 'tee' successfully replicated stdin to stdout and files."
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
