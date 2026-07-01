#!/bin/bash
# Test: uuid action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: Default UUIDv4 ---
echo "Testing default ir uuid..."
OUTPUT=$($EXECUTABLE uuid)
if [[ $OUTPUT =~ ^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$ ]]; then
    echo "PASS: Successfully generated standard UUIDv4 ($OUTPUT)."
else
    echo "FAIL: UUIDv4 format mismatch: '$OUTPUT'"
    exit 1
fi

# --- Test 2: Generate Multiple UUIDs ---
echo "Testing ir uuid -c 5..."
MULTIPLE=$($EXECUTABLE uuid -c 5)
LINE_COUNT=$(echo "$MULTIPLE" | wc -l)
if [ "$LINE_COUNT" -eq 5 ]; then
    echo "PASS: Generated exactly 5 UUIDs."
else
    echo "FAIL: Expected 5 lines, got $LINE_COUNT."
    exit 1
fi

# --- Test 3: No hyphens and uppercase ---
echo "Testing ir uuid -n -u..."
NO_HYPHENS=$($EXECUTABLE uuid -n -u)
if [[ $NO_HYPHENS =~ ^[0-9A-F]{32}$ ]]; then
    echo "PASS: Successfully generated compact uppercase UUID ($NO_HYPHENS)."
else
    echo "FAIL: Compact uppercase UUID mismatch: '$NO_HYPHENS'"
    exit 1
fi

# --- Test 4: UUIDv7 ---
echo "Testing ir uuid -v 7..."
UUIDV7=$($EXECUTABLE uuid -v 7)
if [[ $UUIDV7 =~ ^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$ ]]; then
    echo "PASS: Successfully generated standard UUIDv7 ($UUIDV7)."
else
    echo "FAIL: UUIDv7 format mismatch: '$UUIDV7'"
    exit 1
fi

# --- Test 5: Invalid version should fail ---
echo "Testing invalid UUID version fails..."
$EXECUTABLE uuid -v 5 &>/dev/null
if [ $? -ne 0 ]; then
    echo "PASS: Specifying version 5 failed correctly."
else
    echo "FAIL: Specifying version 5 did not return error."
    exit 1
fi

echo "ALL UUID TESTS PASSED"
exit 0
