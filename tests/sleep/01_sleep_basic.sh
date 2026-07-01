#!/bin/bash
# Test: sleep action on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

echo "Testing sleep 200ms..."
START=$(date +%s%N)
$EXECUTABLE sleep 200ms
END=$(date +%s%N)

ELAPSED=$(( (END - START) / 1000000 ))

if [ $ELAPSED -ge 180 ] && [ $ELAPSED -le 500 ]; then
    echo "PASS: Slept for ${ELAPSED}ms."
else
    echo "FAIL: Sleep duration out of range: ${ELAPSED}ms"
    exit 1
fi

exit 0
