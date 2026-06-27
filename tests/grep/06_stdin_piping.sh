#!/bin/bash

# Test: grep with stdin piping.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test ---
echo "Running test: echo content | ir grep 'test'"
OUTPUT=$(printf "this is a test\nno match\nanother test\nnope\n" | "$EXECUTABLE" grep "test")

# --- Verification ---
LINE_COUNT=$(echo "$OUTPUT" | grep -c "test")
if [ "$LINE_COUNT" -eq 2 ]; then
    echo "PASS: grep via stdin found 2 lines matching 'test'."
    RESULT=0
else
    echo "FAIL: grep via stdin should have found 2 lines, found $LINE_COUNT"
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

exit $RESULT

