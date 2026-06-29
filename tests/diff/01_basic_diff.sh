#!/bin/bash

# Test: diff reports changed lines.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_diff_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
printf "same\nold\ndone\n" > "$TEST_DIR/left.txt"
printf "same\nnew\ndone\n" > "$TEST_DIR/right.txt"

# --- Test ---
echo "Running test: ir diff left.txt right.txt"
OUTPUT=$("$EXECUTABLE" diff "$TEST_DIR/left.txt" "$TEST_DIR/right.txt")

# --- Verification ---
if [[ "$OUTPUT" == *"2c2"* ]] && [[ "$OUTPUT" == *"< old"* ]] && [[ "$OUTPUT" == *"> new"* ]]; then
    echo "PASS: diff reported the changed line."
    RESULT=0
else
    echo "FAIL: diff output did not contain the expected change."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
