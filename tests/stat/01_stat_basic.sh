#!/bin/bash

# Test: Basic stat functionality.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="$SCRIPT_DIR/temp_test_stat_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
FILE="$TEST_DIR/test.txt"
printf "stat check" > "$FILE"

# --- Test 1: stat file ---
echo "Running test: ir stat file"
OUTPUT1=$("$EXECUTABLE" stat "$FILE")

# --- Test 2: stat -c "%A %n" file ---
echo "Running test: ir stat -c '%A %n' file"
OUTPUT2=$("$EXECUTABLE" stat -c "%A %n" "$FILE")

# --- Test 3: Conflict check ---
echo "Running test: ir stat -c '%A' -t file (conflict check)"
ERROR_OUTPUT=$("$EXECUTABLE" stat -c "%A" -t "$FILE" 2>&1)

# --- Verification ---
RESULT=0
if [[ "$OUTPUT1" != *"File:"* ]]; then
    echo "FAIL: stat output was missing 'File:' header: '$OUTPUT1'"
    RESULT=1
elif [[ "$OUTPUT2" != *"-rw"* ]]; then
    echo "FAIL: stat custom format output was incorrect: '$OUTPUT2'"
    RESULT=1
elif [[ "$ERROR_OUTPUT" != *"cannot be used together"* ]]; then
    echo "FAIL: stat conflict check output was incorrect: '$ERROR_OUTPUT'"
    RESULT=1
else
    echo "PASS: 'stat' successfully retrieved metadata and formatted outputs."
fi

# --- Teardown ---
rm -rf "$TEST_DIR"

exit $RESULT
