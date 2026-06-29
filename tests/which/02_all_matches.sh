#!/bin/bash

# Test: which -a prints all matches in PATH order.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_which_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/one" "$TEST_DIR/two"
printf "#!/bin/sh\nexit 0\n" > "$TEST_DIR/one/samplecmd"
printf "#!/bin/sh\nexit 0\n" > "$TEST_DIR/two/samplecmd"
chmod +x "$TEST_DIR/one/samplecmd" "$TEST_DIR/two/samplecmd"
OLD_PATH="$PATH"
PATH="$(pwd)/$TEST_DIR/one:$(pwd)/$TEST_DIR/two"

# --- Test ---
echo "Running test: ir which -a samplecmd"
OUTPUT=$("$EXECUTABLE" which -a samplecmd)
LINE_COUNT=$(printf "%s\n" "$OUTPUT" | grep -c "samplecmd")

# --- Verification ---
if [ "$LINE_COUNT" -eq 2 ] && [[ "$OUTPUT" == *"one/samplecmd"* ]] && [[ "$OUTPUT" == *"two/samplecmd"* ]]; then
    echo "PASS: which -a printed all matches in PATH order."
    RESULT=0
else
    echo "FAIL: which -a did not print expected matches."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
PATH="$OLD_PATH"
rm -rf "$TEST_DIR"

exit $RESULT
