#!/bin/bash

# Test: find with name, type, and depth expressions.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_find_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/src/nested"
touch "$TEST_DIR/src/main.rs"
touch "$TEST_DIR/src/nested/lib.rs"
touch "$TEST_DIR/README.md"

# --- Test ---
echo "Running test: ir find temp_test_find_01 -name '*.rs' -type f -maxdepth 2"
OUTPUT=$("$EXECUTABLE" find "$TEST_DIR" -name "*.rs" -type f -maxdepth 2)

# --- Verification ---
if [[ "$OUTPUT" == *"main.rs"* ]] && [[ "$OUTPUT" != *"lib.rs"* ]] && [[ "$OUTPUT" != *"README.md"* ]]; then
    echo "PASS: find matched the expected Rust file within max depth."
    RESULT=0
else
    echo "FAIL: find output did not match expected name/type/depth filtering."
    echo "Output was:"
    echo "$OUTPUT"
    RESULT=1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
