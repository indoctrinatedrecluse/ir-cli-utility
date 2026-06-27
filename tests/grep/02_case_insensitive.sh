#!/bin/bash

# Test: Case-insensitive grep with -i flag.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "ERROR: critical failure\nWarning: low memory\nError: disk space\ninfo: normal\n" > "$FILE"

# --- Test ---
echo "Running test: ir grep -i 'error' sample.txt"
OUTPUT=$("$EXECUTABLE" grep -i "error" "$FILE")

# --- Verification ---
LINE_COUNT=$(echo "$OUTPUT" | grep -ci "error")
if [ "$LINE_COUNT" -eq 2 ]; then
    echo "PASS: grep -i found 2 lines matching 'error' (case-insensitive)."
    RESULT=0
else
    echo "FAIL: grep -i should have found 2 lines, found $LINE_COUNT"
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

