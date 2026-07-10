#!/bin/bash

# Test: Basic anispeak functionality.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: anispeak basic ---
echo "Running test: ir anispeak 'hello world'"
OUTPUT1=$("$EXECUTABLE" anispeak "hello world")

# --- Test 2: anispeak animal selection ---
echo "Running test: ir anispeak -a crab 'hello world'"
OUTPUT2=$("$EXECUTABLE" anispeak -a crab "hello world")

# --- Verification ---
RESULT=0
if [[ "$OUTPUT1" != *"hello world"* ]]; then
    echo "FAIL: anispeak basic output was missing message: '$OUTPUT1'"
    RESULT=1
elif [[ "$OUTPUT1" != *"oo"* ]]; then
    echo "FAIL: anispeak basic output was missing cow art: '$OUTPUT1'"
    RESULT=1
elif [[ "$OUTPUT2" != *"o o"* ]]; then
    echo "FAIL: anispeak crab output was missing crab art: '$OUTPUT2'"
    RESULT=1
else
    echo "PASS: 'anispeak' successfully wrapped messages and output ASCII art."
fi

exit $RESULT
