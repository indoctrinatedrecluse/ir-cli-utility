#!/bin/bash
# Test: dns action error handling on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
EXECUTABLE="./target/debug/ir"

# Test 1: Invalid hostname (labels > 63 chars)
echo "Testing dns with extremely long label..."
LONG_LABEL=$(printf 'a%.0s' {1..70})".com"
ERR_OUT=$($EXECUTABLE dns "$LONG_LABEL" 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"too long"* ]]; then
    echo "PASS: Too long label rejected correctly."
else
    echo "FAIL: Expected error for too long label. Output: $ERR_OUT"
    exit 1
fi

# Test 2: Non-existent domain
echo "Testing dns with non-existent domain..."
ERR_OUT=$($EXECUTABLE dns thisdomaindoesnotexistatall12345.xyz 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"Failed to resolve records"* ]]; then
    echo "PASS: Non-existent domain lookup failed correctly."
else
    echo "FAIL: Expected failure for non-existent domain. Output: $ERR_OUT"
    exit 1
fi

# Test 3: Multiple arguments
echo "Testing dns with multiple arguments..."
ERR_OUT=$($EXECUTABLE dns google.com extra 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"requires exactly one"* ]]; then
    echo "PASS: Multiple positional arguments rejected correctly."
else
    echo "FAIL: Expected error for multiple arguments. Output: $ERR_OUT"
    exit 1
fi

echo "ALL DNS ERROR TESTS PASSED"
exit 0
