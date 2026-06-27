#!/bin/bash

# Test: grep with invert match (-v flag).

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_05"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "apple\nbanana\napricot\nberry\navocado\n" > "$FILE"

# --- Test ---
echo "Running test: ir grep -v 'a' sample.txt"
OUTPUT=$("$EXECUTABLE" grep -v "a" "$FILE")

# --- Verification ---
LINE_COUNT=$(echo "$OUTPUT" | wc -l)
if [[ "$OUTPUT" == *"banana"* ]] && [[ "$OUTPUT" == *"berry"* ]] && [ "$LINE_COUNT" -eq 2 ]; then
    echo "PASS: grep -v found 2 lines without 'a'."
    RESULT=0
else
    echo "FAIL: grep -v should have found 2 lines (banana, berry)"
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

