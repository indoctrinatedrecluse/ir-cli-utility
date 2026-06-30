#!/bin/bash
# Test: Simple tree structure listing

# --- Setup ---
# Dynamically locate the workspace root by climbing up to find Cargo.toml
ROOT_DIR="$(pwd)"
while [ ! -f "$ROOT_DIR/Cargo.toml" ] && [ "$ROOT_DIR" != "/" ]; do
    ROOT_DIR="$(dirname "$ROOT_DIR")"
done

echo "Building..."
cargo build --quiet
EXECUTABLE="$ROOT_DIR/target/debug/ir"

# Create a temporary directory for the test
TEST_DIR="temp_test_tree_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create a temporary folder structure
mkdir -p "subdir1"
mkdir -p "subdir2"
touch "file2.log"
touch "subdir1/file1.txt"

RESULT=0

# Helper to check if string contains substring
contains() {
    [[ "$1" == *"$2"* ]]
}

# --- Test 1: Default Tree ---
echo "Running test 1: Default tree"
OUTPUT=$("$EXECUTABLE" tree)
echo "Output:"
echo "$OUTPUT"

if contains "$OUTPUT" "subdir1" && contains "$OUTPUT" "subdir2" && contains "$OUTPUT" "file1.txt" && contains "$OUTPUT" "file2.log" && (contains "$OUTPUT" "├──" || contains "$OUTPUT" "└──"); then
    echo "✅ Test 1 PASS: Default tree has correct structure."
else
    echo "❌ Test 1 FAIL: Default tree did not match expected output."
    RESULT=1
fi

# --- Test 2: Directories Only (-d) ---
echo -e "\nRunning test 2: Directories only (-d)"
OUTPUT=$("$EXECUTABLE" tree -d)
echo "Output:"
echo "$OUTPUT"

if contains "$OUTPUT" "subdir1" && contains "$OUTPUT" "subdir2" && ! contains "$OUTPUT" "file1.txt" && ! contains "$OUTPUT" "file2.log"; then
    echo "✅ Test 2 PASS: Dirs-only tree correctly filtered out files."
else
    echo "❌ Test 2 FAIL: Dirs-only tree contains files or incorrect listing."
    RESULT=1
fi

# --- Test 3: Depth Limit (-L 1) ---
echo -e "\nRunning test 3: Depth limit (-L 1)"
OUTPUT=$("$EXECUTABLE" tree -L 1)
echo "Output:"
echo "$OUTPUT"

if contains "$OUTPUT" "subdir1" && contains "$OUTPUT" "subdir2" && ! contains "$OUTPUT" "file1.txt" && contains "$OUTPUT" "file2.log"; then
    echo "✅ Test 3 PASS: Depth limit tree correctly omitted deeper files."
else
    echo "❌ Test 3 FAIL: Depth limit tree did not respect depth limit."
    RESULT=1
fi

# --- Test 4: Omit Indentation (-i) ---
echo -e "\nRunning test 4: Omit indentation (-i)"
OUTPUT=$("$EXECUTABLE" tree -i)
echo "Output:"
echo "$OUTPUT"

if ! contains "$OUTPUT" "├──" && ! contains "$OUTPUT" "└──" && contains "$OUTPUT" "subdir1" && contains "$OUTPUT" "file1.txt"; then
    echo "✅ Test 4 PASS: No-indent tree omitted drawing characters."
else
    echo "❌ Test 4 FAIL: No-indent tree still contains drawing characters or failed listing."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
