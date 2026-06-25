#!/bin/bash

# Test: cat binary preview.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_cat_04"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/sample.bin"
printf "AB\000C" > "$FILE"

# --- Test ---
echo "Running test: ir cat --binary sample.bin"
OUTPUT=$("$EXECUTABLE" cat --binary "$FILE")

# --- Verification ---
if [[ "$OUTPUT" == *"00000000"* ]] && [[ "$OUTPUT" == *"41 42 00 43"* ]] && [[ "$OUTPUT" == *"AB.C"* ]]; then
    echo "PASS: 'cat --binary' printed a hexadecimal preview."
    RESULT=0
else
    echo "FAIL: binary preview output was not as expected."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
