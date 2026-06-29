#!/bin/bash

# Test: diff brief mode with ignore-case.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_diff_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
printf "Hello\n" > "$TEST_DIR/left.txt"
printf "hello\n" > "$TEST_DIR/right.txt"

# --- Test ---
echo "Running test: ir diff -q -i left.txt right.txt"
OUTPUT=$("$EXECUTABLE" diff -q -i "$TEST_DIR/left.txt" "$TEST_DIR/right.txt")

# --- Verification ---
if [ -z "$OUTPUT" ]; then
    echo "PASS: diff ignored case differences."
    RESULT=0
else
    echo "FAIL: diff should not have reported case-only differences."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
