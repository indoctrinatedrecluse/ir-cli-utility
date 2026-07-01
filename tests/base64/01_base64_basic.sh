#!/bin/bash
# Test: base64 action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: Simple Base64 encode ---
echo "Testing ir base64 encode..."
OUTPUT=$(echo "hello" | $EXECUTABLE base64)
# echo adds newline
if [ "$OUTPUT" = "aGVsbG8K" ] || [ "$OUTPUT" = "aGVsbG8NCg==" ]; then
    echo "PASS: Simple encode output matches."
else
    echo "FAIL: Simple encode output mismatch: '$OUTPUT'"
    exit 1
fi

# --- Test 2: Simple Base64 decode ---
echo "Testing ir base64 decode..."
DECODED=$(echo -n "aGVsbG8=" | $EXECUTABLE base64 -d)
if [ "$DECODED" = "hello" ]; then
    echo "PASS: Simple decode output matches."
else
    echo "FAIL: Simple decode output mismatch: '$DECODED'"
    exit 1
fi

# --- Test 3: Base64 encode URL-safe unpadded ---
echo "Testing ir base64 -u -n (URL-safe unpadded)..."
TEST_FILE="temp_b64_test_sh.bin"
printf "\xfb\xff" > "$TEST_FILE"

URL_SAFE_OUT=$($EXECUTABLE base64 -u -n "$TEST_FILE")
if [ "$URL_SAFE_OUT" = "-_8" ]; then
    echo "PASS: URL-safe unpadded encode matches."
else
    echo "FAIL: URL-safe unpadded mismatch: '$URL_SAFE_OUT'"
    rm -f "$TEST_FILE"
    exit 1
fi
rm -f "$TEST_FILE"

echo "ALL BASE64 TESTS PASSED"
exit 0
