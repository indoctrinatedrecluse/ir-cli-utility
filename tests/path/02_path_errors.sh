#!/bin/bash
# Test: path action error handling on Linux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
EXECUTABLE="./target/debug/ir"

# Setup dummy environment to run profile changes in a temporary file
export HOME="$PWD/temp_home_err"
mkdir -p "$HOME"
touch "$HOME/.bashrc"

# Test 1: Conflict switches (--add and --remove specified together)
echo "Testing path with conflicting switches..."
ERR_OUT=$($EXECUTABLE path -a /bin -r /bin 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"cannot be specified together"* ]]; then
    echo "PASS: Conflicting switches rejected correctly."
else
    echo "FAIL: Expected error for conflicting switches. Output: $ERR_OUT"
    rm -rf "$HOME"
    exit 1
fi

# Test 2: Positional arguments provided
echo "Testing path with positional arguments..."
ERR_OUT=$($EXECUTABLE path extra 2>&1)
if [ $? -ne 0 ] && [[ "$ERR_OUT" == *"does not accept positional"* ]]; then
    echo "PASS: Positional arguments rejected correctly."
else
    echo "FAIL: Expected error for positional arguments. Output: $ERR_OUT"
    rm -rf "$HOME"
    exit 1
fi

# Test 3: Duplicate add
echo "Testing duplicate path addition..."
$EXECUTABLE path -a "/my/bin" >/dev/null
ADD_OUT=$($EXECUTABLE path -a "/my/bin")
if [[ "$ADD_OUT" == *"already exists in"* ]]; then
    echo "PASS: Duplicate path addition detected cleanly."
else
    echo "FAIL: Expected warning for duplicate path. Output: $ADD_OUT"
    rm -rf "$HOME"
    exit 1
fi

# Test 4: Removing non-existent path
echo "Testing removal of non-existent path..."
REMOVE_OUT=$($EXECUTABLE path -r "/non/existent/path")
if [[ "$REMOVE_OUT" == *"No PATH export found"* ]]; then
    echo "PASS: Non-existent path removal reported cleanly."
else
    echo "FAIL: Expected warning for non-existent path. Output: $REMOVE_OUT"
    rm -rf "$HOME"
    exit 1
fi

rm -rf "$HOME"
echo "ALL PATH ERROR TESTS PASSED"
exit 0
