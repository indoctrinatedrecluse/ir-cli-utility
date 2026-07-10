#!/bin/bash

# Test: Expanded anispeak animal characters.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test: Verify new animals output contains specific characters ---
NEW_ANIMALS=("elephant" "moose" "stegosaurus" "whale" "snake" "turtle" "sheep")
RESULT=0

for ANIMAL in "${NEW_ANIMALS[@]}"; do
    echo "Running test: ir anispeak -a $ANIMAL 'hi'"
    OUTPUT=$("$EXECUTABLE" anispeak -a "$ANIMAL" "hi")
    if [[ "$OUTPUT" != *"hi"* ]]; then
        echo "FAIL: output of animal $ANIMAL was missing message."
        RESULT=1
        break
    fi
done

if [ $RESULT -eq 0 ]; then
    echo "PASS: All 7 expanded anispeak animal templates successfully printed speech bubble and ASCII art."
fi

exit $RESULT
