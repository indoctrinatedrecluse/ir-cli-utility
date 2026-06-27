#!/bin/bash

# Test: grep with line numbers (-n flag).

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_03"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "line one\nline two\nline three\nline four\n" > "$FILE"

# --- Test ---
echo "Running test: ir grep -n 'three' sample.txt"
OUTPUT=$("$EXECUTABLE" grep -n "three" "$FILE")

# --- Verification ---
if echo "$OUTPUT" | grep -q "3:"; then
    echo "PASS: grep -n displayed line number 3."
    RESULT=0
else
    echo "FAIL: grep -n should have displayed line number."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

