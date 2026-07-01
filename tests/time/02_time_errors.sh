#!/bin/bash
# Test: time action error handling on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
EXECUTABLE="./target/debug/ir"

# Test 1: Empty command argument
echo "Testing time with empty command..."
ERR_OUT=$($EXECUTABLE time 2>&1)
if [ $? -ne 0 ] && ([[ "$ERR_OUT" == *"requires a command"* ]] || [[ "$ERR_OUT" == *"ir-time"* ]]); then
    echo "PASS: Empty command rejected correctly."
else
    echo "FAIL: Expected error for empty command. Output: $ERR_OUT"
    exit 1
fi

# Test 2: Invalid command that doesn't exist
echo "Testing time with non-existent command..."
ERR_OUT=$($EXECUTABLE time non_existent_command_123 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"Failed to spawn"* ]]; then
    echo "PASS: Non-existent command failed correctly."
else
    echo "FAIL: Expected error for non-existent command. Output: $ERR_OUT"
    exit 1
fi

echo "ALL TIME ERROR TESTS PASSED"
exit 0
