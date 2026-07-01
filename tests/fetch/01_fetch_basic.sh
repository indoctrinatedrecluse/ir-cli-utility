#!/bin/bash
# Test: fetch action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: Fetch plain text IP ---
echo "Testing ir fetch api.ipify.org..."
OUTPUT=$($EXECUTABLE fetch https://api.ipify.org)
if [[ $OUTPUT =~ ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3} ]]; then
    echo "PASS: fetch successfully retrieved public IP address ($OUTPUT)."
else
    echo "FAIL: Did not get expected IP address format. Output was:"
    echo "$OUTPUT"
    exit 1
fi

# --- Test 2: Fetch headers using -i ---
echo "Testing ir fetch -i api.ipify.org..."
HEADER_OUT=$($EXECUTABLE fetch -i https://api.ipify.org)
if [[ $HEADER_OUT == *HTTP/1.1\ 200* ]]; then
    echo "PASS: fetch -i correctly included response headers."
else
    echo "FAIL: Response headers missing or mismatch. Output was:"
    echo "$HEADER_OUT"
    exit 1
fi

echo "ALL FETCH TESTS PASSED"
exit 0
