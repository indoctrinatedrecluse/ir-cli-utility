#!/bin/bash

# Test: grep with count (-c flag).

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_04"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "todo: fix bug\ndone: review code\ntodo: add tests\ntodo: update docs\ndone: deploy\n" > "$FILE"

# --- Test ---
echo "Running test: ir grep -c 'todo' sample.txt"
OUTPUT=$("$EXECUTABLE" grep -c "todo" "$FILE")

# --- Verification ---
if [[ "$OUTPUT" == *"3"* ]]; then
    echo "PASS: grep -c returned count of 3."
    RESULT=0
else
    echo "FAIL: grep -c should have returned 3, got: $OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

