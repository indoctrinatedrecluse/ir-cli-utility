#!/bin/bash

# Test: Basic chmod functionality, including symbolic mode parsing.

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT" || exit 1
cargo build --quiet
EXECUTABLE="./target/debug/ir"

TEST_FILE="temp_chmod_test.txt"
echo "chmod test content" > "$TEST_FILE"

# --- Test 1: Octal Chmod ---
echo "Running test: ir chmod 600"
"$EXECUTABLE" chmod 600 "$TEST_FILE"
PERMS=$(stat -c "%a" "$TEST_FILE" 2>/dev/null || stat -f "%Lp" "$TEST_FILE")
if [ "$PERMS" != "600" ]; then
    echo "FAIL: Octal chmod 600 set permissions to $PERMS instead of 600."
    rm -f "$TEST_FILE"
    exit 1
fi

# --- Test 2: Symbolic Chmod Add Execute ---
echo "Running test: ir chmod +x"
"$EXECUTABLE" chmod +x "$TEST_FILE"
PERMS=$(stat -c "%a" "$TEST_FILE" 2>/dev/null || stat -f "%Lp" "$TEST_FILE")
# Should be 711 or similar depending on umask, but definitely executable
if [[ "$PERMS" != "711" && "$PERMS" != "755" && "$PERMS" != "700" && "$PERMS" != "775" ]]; then
    # Double check executable flag via bash check
    if [ ! -x "$TEST_FILE" ]; then
        echo "FAIL: Symbolic chmod +x did not make file executable."
        rm -f "$TEST_FILE"
        exit 1
    fi
fi

# --- Test 3: Symbolic Chmod Multiple Clauses ---
echo "Running test: ir chmod u-x,go+r"
"$EXECUTABLE" chmod u-x,go+r "$TEST_FILE"
if [ -x "$TEST_FILE" ]; then
    echo "FAIL: u-x clause did not remove user execution permission."
    rm -f "$TEST_FILE"
    exit 1
fi

echo "PASS: 'chmod' successfully handled octal and symbolic modes."
rm -f "$TEST_FILE"
exit 0
