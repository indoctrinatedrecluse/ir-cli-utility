#!/bin/bash
# Test: env action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: List all env variables ---
echo "Testing ir env..."
OUTPUT=$($EXECUTABLE env)
if [[ $OUTPUT == *PATH=* ]]; then
    echo "PASS: env listed variables successfully."
else
    echo "FAIL: env did not list variables. Output:"
    echo "$OUTPUT"
    exit 1
fi

# --- Test 2: Format PATH variable ---
echo "Testing ir env PATH..."
PATH_OUT=$($EXECUTABLE env PATH)
# Path output should be split by lines and not contain colons
if [[ $PATH_OUT == *:* ]]; then
    echo "FAIL: PATH variable was not split line-by-line. Output:"
    echo "$PATH_OUT"
    exit 1
else
    echo "PASS: PATH variable was split line-by-line."
fi

# --- Test 3: Search env variables ---
echo "Testing ir env -s CARGO..."
export CARGO_TEST_VAR="hello_cargo"
SEARCH_OUT=$($EXECUTABLE env -s CARGO)
if [[ $SEARCH_OUT == *CARGO_TEST_VAR=* ]]; then
    echo "PASS: env search filter works."
else
    echo "FAIL: env search did not find expected key. Output:"
    echo "$SEARCH_OUT"
    exit 1
fi

# --- Test 4: Query non-existent variable fails ---
echo "Testing non-existent env variable fails..."
$EXECUTABLE env NON_EXISTENT_VAR_1234 &>/dev/null
if [ $? -eq 0 ]; then
    echo "FAIL: Querying non-existent variable did not return failure code."
    exit 1
else
    echo "PASS: Querying non-existent variable correctly returned error."
fi

echo "ALL ENV TESTS PASSED"
exit 0
