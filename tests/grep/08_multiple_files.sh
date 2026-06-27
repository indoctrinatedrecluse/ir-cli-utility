#!/bin/bash

# Test: grep on multiple files.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_08"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE1="$TEST_DIR/file1.txt"
FILE2="$TEST_DIR/file2.txt"
printf "match here\nno match\n" > "$FILE1"
printf "another match\nstill no\nmatch here too\n" > "$FILE2"

# --- Test ---
echo "Running test: ir grep 'match' file1.txt file2.txt"
OUTPUT=$("$EXECUTABLE" grep "match" "$FILE1" "$FILE2")

# --- Verification ---
MATCH_COUNT=$(echo "$OUTPUT" | grep -c "match")
if [ "$MATCH_COUNT" -ge 3 ]; then
    echo "PASS: grep found matches across multiple files."
    RESULT=0
else
    echo "FAIL: grep should have found at least 3 matches, found $MATCH_COUNT"
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

