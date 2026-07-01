#!/bin/bash
# Test: dns action on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

echo "Testing dns resolution of google.com..."
OUTPUT=$($EXECUTABLE dns google.com)

echo "Output: $OUTPUT"

if [[ "$OUTPUT" == *"A (IPv4)"* ]] && [[ "$OUTPUT" == *"MX (Mail Server)"* ]]; then
    echo "PASS: Successfully resolved A and MX records."
else
    echo "FAIL: Output missing A or MX records."
    exit 1
fi

exit 0
