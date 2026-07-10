#!/bin/bash

# Test: Basic tail functionality.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_tail_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/lines.txt"
printf "one\ntwo\nthree\nfour\nfive\n" > "$FILE"

# --- Test 1: tail -n 2 ---
echo "Running test: ir tail -n 2"
OUTPUT1=$("$EXECUTABLE" tail -n 2 "$FILE")
OUTPUT1=$(echo "$OUTPUT1" | tr -d '\r')

# --- Test 2: tail -n +3 ---
echo "Running test: ir tail -n +3"
OUTPUT2=$("$EXECUTABLE" tail -n +3 "$FILE")
OUTPUT2=$(echo "$OUTPUT2" | tr -d '\r')

# --- Test 3: Conflict check ---
echo "Running test: ir tail -n 2 -c 10 (conflict check)"
ERROR_OUTPUT=$("$EXECUTABLE" tail -n 2 -c 10 "$FILE" 2>&1)

# --- Verification ---
RESULT=0
if [ "$OUTPUT1" != "four
five" ]; then
    echo "FAIL: tail -n 2 output was incorrect: '$OUTPUT1'"
    RESULT=1
elif [ "$OUTPUT2" != "three
four
five" ]; then
    echo "FAIL: tail -n +3 output was incorrect: '$OUTPUT2'"
    RESULT=1
elif [[ "$ERROR_OUTPUT" != *"cannot be used together"* ]]; then
    echo "FAIL: tail conflict check output was incorrect: '$ERROR_OUTPUT'"
    RESULT=1
else
    echo "PASS: 'tail' successfully sliced file contents and handled conflicts."
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
