#!/bin/bash
# Test: time action on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

echo "Testing execution timing..."
ERR_FILE="temp_time_err.txt"
$EXECUTABLE time sh -c "exit 42" 2>"$ERR_FILE"
EXIT_CODE=$?

ERR_TEXT=$(cat "$ERR_FILE")
rm -f "$ERR_FILE"

echo "Exit code returned: $EXIT_CODE"
echo "Stderr output: $ERR_TEXT"

if [ $EXIT_CODE -eq 42 ] && [[ "$ERR_TEXT" == *"Execution Time:"* ]]; then
    echo "PASS: Correct exit code and timing output."
else
    echo "FAIL: Expected exit code 42 and timing output."
    exit 1
fi

exit 0
