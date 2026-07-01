#!/bin/bash
# Test: echo action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_echo_sh"
mkdir -p "$TEST_DIR"
OUT_FILE="$TEST_DIR/output.txt"

# --- Test 1: Simple print ---
echo "Testing ir echo basic..."
OUTPUT=$($EXECUTABLE echo hello world)
if [ "$OUTPUT" = "hello world" ]; then
    echo "PASS: Simple print output matches."
else
    echo "FAIL: Simple print output mismatch: '$OUTPUT'"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test 2: Escape interpretation ---
echo "Testing ir echo -e..."
ESCAPE_OUT=$($EXECUTABLE echo -e 'line1\nline2\x41')
if [[ "$ESCAPE_OUT" == *"line1"* ]] && [[ "$ESCAPE_OUT" == *"line2A"* ]]; then
    echo "PASS: Escape sequences interpreted successfully."
else
    echo "FAIL: Escape sequences mismatch. Output:"
    echo "$ESCAPE_OUT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test 3: Redirection write > ---
echo "Testing ir echo > redirection..."
$EXECUTABLE echo "first line" ">" "$OUT_FILE"
if [ -f "$OUT_FILE" ]; then
    CONTENT=$(cat "$OUT_FILE")
    if [ "$CONTENT" = "first line" ]; then
        echo "PASS: Redirection > wrote successfully."
    else
        echo "FAIL: Content mismatch: '$CONTENT'"
        rm -rf "$TEST_DIR"
        exit 1
    fi
else
    echo "FAIL: Redirection file not created."
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test 4: Redirection append >> ---
echo "Testing ir echo >> redirection..."
$EXECUTABLE echo "second line" ">>" "$OUT_FILE"
CONTENT_COUNT=$(wc -l < "$OUT_FILE")
if [ "$CONTENT_COUNT" -eq 2 ]; then
    echo "PASS: Redirection >> appended successfully."
else
    echo "FAIL: Append content mismatch. Lines count: $CONTENT_COUNT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test 5: Redirection missing file argument fails ---
echo "Testing missing file argument for redirection fails..."
$EXECUTABLE echo "text" ">" &>/dev/null
if [ $? -ne 0 ]; then
    echo "PASS: Redirection without file failed correctly."
else
    echo "FAIL: Redirection without file did not return error code."
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"
echo "ALL ECHO TESTS PASSED"
exit 0
