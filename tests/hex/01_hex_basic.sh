#!/bin/bash
# Test: hex dump action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_hex_01_sh"
mkdir -p "$TEST_DIR"

TEST_FILE="$TEST_DIR/sample.txt"
echo -n "Hello World!" > "$TEST_FILE"

# --- Test 1: Full Hex Dump ---
echo "Testing ir hex..."
OUTPUT=$($EXECUTABLE hex "$TEST_FILE")
if [[ $OUTPUT == *00000000*48\ 65\ 6c\ 6c\ 6f*\|Hello\ World!\|* ]]; then
    echo "PASS: Hex dump format matches."
else
    echo "FAIL: Hex dump format mismatch. Output:"
    echo "$OUTPUT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test 2: Limit Hex Dump ---
echo "Testing ir hex -n 5..."
LIMIT_OUT=$($EXECUTABLE hex -n 5 "$TEST_FILE")
if [[ $LIMIT_OUT == *48\ 65\ 6c\ 6c\ 6f*\|Hello\|* ]]; then
    echo "PASS: Hex dump limit switch (-n) works."
else
    echo "FAIL: Hex dump limit switch mismatch. Output:"
    echo "$LIMIT_OUT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"
echo "ALL HEX TESTS PASSED"
exit 0
