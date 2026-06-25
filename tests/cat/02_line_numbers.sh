#!/bin/bash

# Test: cat line numbering.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_cat_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "alpha\nbeta\n" > "$FILE"

# --- Test ---
echo "Running test: ir cat -n sample.txt"
OUTPUT=$("$EXECUTABLE" cat -n "$FILE")

# --- Verification ---
if [[ "$OUTPUT" == *"1"$'\t'"alpha"* ]] && [[ "$OUTPUT" == *"2"$'\t'"beta"* ]]; then
    echo "PASS: 'cat -n' printed line numbers."
    RESULT=0
else
    echo "FAIL: 'cat -n' output was missing expected line numbers."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
