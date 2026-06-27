#!/bin/bash

# Test: grep with fixed strings (-F flag).

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_grep_07"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.txt"
printf "test.file\ntest*file\ntest[file]\ntest.ext\n" > "$FILE"

# --- Test ---
# With -F, 'test*' should match 'test*file' literally (not treat * as a wildcard)
echo "Running test: ir grep -F 'test*' sample.txt"
OUTPUT=$("$EXECUTABLE" grep -F "test*" "$FILE")

# --- Verification ---
if [[ "$OUTPUT" == *"test*file"* ]]; then
    echo "PASS: grep -F treated pattern as literal string and matched 'test*file'."
    RESULT=0
else
    echo "FAIL: grep -F should match literal 'test*' pattern"
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT

