#!/bin/bash
# Test: Tree metadata switches, hidden files, and report suppression

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
TEST_DIR="temp_test_tree_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create test files
mkdir -p "subdir"
touch "file1.txt"
touch ".hidden" # Starts with dot

# Write some content to make size non-zero
echo "Hello, world!" > "file1.txt"

RESULT=0

# Helper to check if string contains substring
contains() {
    [[ "$1" == *"$2"* ]]
}

# --- Test 1: Hidden Files (default vs -a) ---
echo "Running test 1: Hidden files"
DEFAULT_OUTPUT=$("$EXECUTABLE" tree)
ALL_OUTPUT=$("$EXECUTABLE" tree -a)

if contains "$DEFAULT_OUTPUT" "file1.txt" && ! contains "$DEFAULT_OUTPUT" ".hidden"; then
    echo "✅ Sub-test 1.1 PASS: Default tree hides dotfiles."
else
    echo "❌ Sub-test 1.1 FAIL: Default tree did not hide dotfiles."
    RESULT=1
fi

if contains "$ALL_OUTPUT" "file1.txt" && contains "$ALL_OUTPUT" ".hidden"; then
    echo "✅ Sub-test 1.2 PASS: Tree with -a shows dotfiles."
else
    echo "❌ Sub-test 1.2 FAIL: Tree with -a did not show dotfiles."
    RESULT=1
fi

# --- Test 2: Permissions (-p) ---
echo -e "\nRunning test 2: Permissions (-p)"
OUTPUT=$("$EXECUTABLE" tree -p)
echo "Output:"
echo "$OUTPUT"

# Verify permissions formatting like `[d...]` or `[-...]`
if [[ "$OUTPUT" =~ \[[dl-][r-][w-][x-][r-][w-][x-][r-][w-][x-]\] ]]; then
    echo "✅ Test 2 PASS: Tree with -p contains permission brackets."
else
    echo "❌ Test 2 FAIL: Permissions format did not match expected pattern."
    RESULT=1
fi

# --- Test 3: Sizes (-s and -h) ---
echo -e "\nRunning test 3: Sizes (-s and -h)"
OUTPUT_S=$("$EXECUTABLE" tree -s)
OUTPUT_H=$("$EXECUTABLE" tree -h)

if [[ "$OUTPUT_S" =~ \[[0-9]+\] ]]; then
    echo "✅ Sub-test 3.1 PASS: Tree with -s contains raw size brackets."
else
    echo "❌ Sub-test 3.1 FAIL: Tree with -s does not contain size formatting."
    RESULT=1
fi

if [[ "$OUTPUT_H" =~ \[[0-9]+(\.[0-9]+)?\ *(B|KB|MB|GB)\] ]]; then
    echo "✅ Sub-test 3.2 PASS: Tree with -h contains human-readable size brackets."
else
    echo "❌ Sub-test 3.2 FAIL: Tree with -h does not contain human-readable size formatting."
    RESULT=1
fi

# --- Test 4: Full Path (-f) ---
echo -e "\nRunning test 4: Full path (-f)"
OUTPUT=$("$EXECUTABLE" tree -f)
echo "Output:"
echo "$OUTPUT"

if [[ "$OUTPUT" =~ (\./|temp_test_tree_02)file1\.txt ]]; then
    echo "✅ Test 4 PASS: Tree with -f outputs path prefixes."
else
    echo "❌ Test 4 FAIL: Tree with -f did not output path prefix."
    RESULT=1
fi

# --- Test 5: No Report (--noreport) ---
echo -e "\nRunning test 5: No report (--noreport)"
OUTPUT=$("$EXECUTABLE" tree --noreport)

if ! [[ "$OUTPUT" =~ directories,\ +[0-9]+\ files ]]; then
    echo "✅ Test 5 PASS: Tree with --noreport omitted the final count summary."
else
    echo "❌ Test 5 FAIL: Tree with --noreport still contained the count summary."
    RESULT=1
fi

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
