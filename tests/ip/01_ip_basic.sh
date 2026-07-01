#!/bin/bash
# Test: ip action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: Local adapter listing ---
echo "Testing local adapter listing..."
OUTPUT=$($EXECUTABLE ip)
if [[ $OUTPUT == *Status:* ]] || [[ $OUTPUT == *MAC\ Address:* ]]; then
    echo "PASS: Local network adapters listed successfully."
else
    echo "FAIL: Local network adapter output format mismatch. Output:"
    echo "$OUTPUT"
    exit 1
fi

# --- Test 2: Public IP lookup ---
echo "Testing public IP lookup..."
PUBLIC=$($EXECUTABLE ip -p)
if [[ $PUBLIC == *Public\ IP:* ]] && [[ $PUBLIC == *Location:* ]]; then
    echo "PASS: Public IP query details match expected format."
else
    echo "FAIL: Public IP query format mismatch. Output:"
    echo "$PUBLIC"
    exit 1
fi

# --- Test 3: Invalid switch fails ---
echo "Testing invalid switch fails..."
$EXECUTABLE ip -z &>/dev/null
if [ $? -ne 0 ]; then
    echo "PASS: Specifying invalid switch failed correctly."
else
    echo "FAIL: Specifying invalid switch did not return error."
    exit 1
fi

echo "ALL IP TESTS PASSED"
exit 0
