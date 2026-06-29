#!/bin/bash

# Test: which locates a command in PATH.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_which_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
printf "#!/bin/sh\nexit 0\n" > "$TEST_DIR/samplecmd"
chmod +x "$TEST_DIR/samplecmd"
OLD_PATH="$PATH"
PATH="$(pwd)/$TEST_DIR:$PATH"

# --- Test ---
echo "Running test: ir which samplecmd"
OUTPUT=$("$EXECUTABLE" which samplecmd)

# --- Verification ---
if [[ "$OUTPUT" == *"samplecmd"* ]]; then
    echo "PASS: which located the command in PATH."
    RESULT=0
else
    echo "FAIL: which did not locate the expected command."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
PATH="$OLD_PATH"
rm -rf "$TEST_DIR"

exit $RESULT
