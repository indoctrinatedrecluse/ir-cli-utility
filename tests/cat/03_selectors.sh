#!/bin/bash

# Test: cat head, tail, and range selectors.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_cat_03"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "one\ntwo\nthree\nfour\n" > "$FILE"

# --- Test ---
echo "Running test: ir cat selectors"
HEAD_OUTPUT=$("$EXECUTABLE" cat --head 2 "$FILE")
TAIL_OUTPUT=$("$EXECUTABLE" cat --tail 2 "$FILE")
RANGE_OUTPUT=$("$EXECUTABLE" cat --range 2:3 "$FILE")

# --- Verification ---
if [[ "$HEAD_OUTPUT" == *"one"* ]] && [[ "$HEAD_OUTPUT" == *"two"* ]] && [[ "$HEAD_OUTPUT" != *"three"* ]] &&
   [[ "$TAIL_OUTPUT" == *"three"* ]] && [[ "$TAIL_OUTPUT" == *"four"* ]] && [[ "$TAIL_OUTPUT" != *"two"* ]] &&
   [[ "$RANGE_OUTPUT" == *"two"* ]] && [[ "$RANGE_OUTPUT" == *"three"* ]] && [[ "$RANGE_OUTPUT" != *"one"* ]] && [[ "$RANGE_OUTPUT" != *"four"* ]]; then
    echo "PASS: 'cat' selectors printed the expected lines."
    RESULT=0
else
    echo "FAIL: selector output was not as expected."
    echo "Head was:"
    echo "$HEAD_OUTPUT"
    echo "Tail was:"
    echo "$TAIL_OUTPUT"
    echo "Range was:"
    echo "$RANGE_OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
