#!/bin/bash
# Test: ping action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test: Ping localhost ---
echo "Testing ir ping 127.0.0.1 -c 2..."
OUTPUT=$($EXECUTABLE ping 127.0.0.1 -c 2)
if [ $? -ne 0 ]; then
    echo "FAIL: ir ping failed with exit code $?."
    exit 1
fi

if [[ $OUTPUT == *bytes*from*127.0.0.1*time=* ]]; then
    echo "PASS: ping response matches expected format."
else
    echo "FAIL: ping response mismatch. Output:"
    echo "$OUTPUT"
    exit 1
fi

if [[ $OUTPUT == *packets\ transmitted* ]]; then
    echo "PASS: ping statistics are correct."
else
    echo "FAIL: ping statistics mismatch. Output:"
    echo "$OUTPUT"
    exit 1
fi

echo "ALL PING TESTS PASSED"
exit 0
