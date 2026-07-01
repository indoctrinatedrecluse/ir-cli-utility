#!/bin/bash
# Test: math action on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"

# --- Test 1: Basic addition and multiplication (PEMDAS) ---
echo "Testing basic math evaluation..."
OUTPUT=$($EXECUTABLE math "2 * (3.5 + 4)")
if [ "$OUTPUT" = "15" ]; then
    echo "PASS: Evaluated 2 * (3.5 + 4) correctly."
else
    echo "FAIL: Output mismatch: '$OUTPUT'"
    exit 1
fi

# --- Test 2: Modulo operator ---
echo "Testing modulo operator..."
MOD_OUT=$($EXECUTABLE math "10 % 3")
if [ "$MOD_OUT" = "1" ]; then
    echo "PASS: Evaluated 10 % 3 correctly."
else
    echo "FAIL: Modulo mismatch: '$MOD_OUT'"
    exit 1
fi

# --- Test 3: Power operator (Right-associative) ---
echo "Testing power operator..."
POW_OUT=$($EXECUTABLE math "2^3^2")
if [ "$POW_OUT" = "512" ]; then
    echo "PASS: Evaluated 2^3^2 correctly."
else
    echo "FAIL: Power mismatch: '$POW_OUT'"
    exit 1
fi

# --- Test 4: Floating point division ---
echo "Testing floating point division..."
DIV_OUT=$($EXECUTABLE math "7 / 2")
if [ "$DIV_OUT" = "3.5" ]; then
    echo "PASS: Evaluated 7 / 2 correctly."
else
    echo "FAIL: Float division mismatch: '$DIV_OUT'"
    exit 1
fi

# --- Test 5: Division by zero should fail ---
echo "Testing division by zero fails..."
$EXECUTABLE math "10 / 0" &>/dev/null
if [ $? -ne 0 ]; then
    echo "PASS: Division by zero failed correctly."
else
    echo "FAIL: Division by zero did not return error code."
    exit 1
fi

# --- Test 6: Syntax error should fail ---
echo "Testing syntax error fails..."
$EXECUTABLE math "2 + * 3" &>/dev/null
if [ $? -ne 0 ]; then
    echo "PASS: Syntax error failed correctly."
else
    echo "FAIL: Syntax error did not return error code."
    exit 1
fi

echo "ALL MATH TESTS PASSED"
exit 0
