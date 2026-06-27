#!/bin/bash

# Test: Basic grep functionality.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "error: something failed\nwarning: be careful\nerror: another issue\ninfo: all good\n" > "$FILE"

# --- Test ---
echo "Running test: ir grep 'error' sample.txt"
OUTPUT=$("$EXECUTABLE" grep "error" "$FILE")

# --- Verification ---
LINE_COUNT=$(echo "$OUTPUT" | grep -c "error")
if [ "$LINE_COUNT" -eq 2 ]; then
    echo "PASS: grep found 2 lines matching 'error'."
    RESULT=0
else
    echo "FAIL: grep should have found 2 lines, found $LINE_COUNT"
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

