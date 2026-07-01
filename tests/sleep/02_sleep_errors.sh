#!/bin/bash
# Test: sleep action error handling on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
EXECUTABLE="./target/debug/ir"

# Test 1: Negative duration
echo "Testing sleep with negative duration..."
ERR_OUT=$($EXECUTABLE sleep -5s 2>&1)
if [ $? -ne 0 ] && ([[ "$ERR_OUT" == *"negative"* ]] || [[ "$ERR_OUT" == *"Unknown switch"* ]]); then
    echo "PASS: Negative duration rejected correctly."
else
    echo "FAIL: Expected error for negative duration. Output: $ERR_OUT"
    exit 1
fi

# Test 2: Invalid unit suffix
echo "Testing sleep with invalid suffix..."
ERR_OUT=$($EXECUTABLE sleep 5x 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"Unknown unit"* ]]; then
    echo "PASS: Invalid unit rejected correctly."
else
    echo "FAIL: Expected error for invalid unit. Output: $ERR_OUT"
    exit 1
fi

# Test 3: Non-numeric duration
echo "Testing sleep with non-numeric value..."
ERR_OUT=$($EXECUTABLE sleep abc 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"No numeric value"* ]]; then
    echo "PASS: Non-numeric duration rejected correctly."
else
    echo "FAIL: Expected error for non-numeric duration. Output: $ERR_OUT"
    exit 1
fi

# Test 4: Multiple positional arguments
echo "Testing sleep with multiple arguments..."
ERR_OUT=$($EXECUTABLE sleep 5s 10s 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"requires exactly one"* ]]; then
    echo "PASS: Multiple arguments rejected correctly."
else
    echo "FAIL: Expected error for multiple arguments. Output: $ERR_OUT"
    exit 1
fi

# Test 5: Empty arguments
echo "Testing sleep with empty arguments..."
ERR_OUT=$($EXECUTABLE sleep 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"requires exactly one"* ]]; then
    echo "PASS: Empty arguments rejected correctly."
else
    echo "FAIL: Expected error for empty arguments. Output: $ERR_OUT"
    exit 1
fi

echo "ALL SLEEP ERROR TESTS PASSED"
exit 0
