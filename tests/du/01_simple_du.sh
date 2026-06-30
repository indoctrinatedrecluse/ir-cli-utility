#!/bin/bash
# Test: du Action functionality

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
TEST_DIR="temp_test_du_01"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create subdirectories and files with precise sizes
mkdir -p "subdir1"
mkdir -p "subdir2"

# file1.txt -> 13 bytes
echo -n "Hello, world!" > "subdir1/file1.txt"
# file2.txt -> 1024 bytes
dd if=/dev/zero of="subdir1/file2.txt" bs=1024 count=1 2>/dev/null
# file3.txt -> 1,048,576 bytes (1 MB)
dd if=/dev/zero of="file3.txt" bs=1048576 count=1 2>/dev/null

RESULT=0

# Helper to check if string matches regex
matches() {
    [[ "$1" =~ $2 ]]
}

# Helper to check if string does not match regex
not_matches() {
    ! [[ "$1" =~ $2 ]]
}

Check-Test() {
    NAME="$1"
    PASS="$2"
    if [ "$PASS" = "0" ]; then
        echo "✅ PASS: $NAME"
    else
        echo "❌ FAIL: $NAME"
        RESULT=1
    fi
}

# --- Test 1: Default du (KB, directories recursively, child printed first) ---
echo "Running test 1: Default du"
OUTPUT=$("$EXECUTABLE" du)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "2[[:space:]]+.*subdir1" && matches "$OUTPUT" "0[[:space:]]+.*subdir2" && matches "$OUTPUT" "1026[[:space:]]+\."; then
    PASS=0
else
    PASS=1
fi
Check-Test "Default du outputs correct directory sizes in KB" "$PASS"

# --- Test 2: du with all files (-a) ---
echo -e "\nRunning test 2: du with all files (-a)"
OUTPUT=$("$EXECUTABLE" du -a)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "1[[:space:]]+.*file1.txt" && matches "$OUTPUT" "1[[:space:]]+.*file2.txt" && matches "$OUTPUT" "1024[[:space:]]+.*file3.txt"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -a outputs file sizes" "$PASS"

# --- Test 3: du with total (-c) ---
echo -e "\nRunning test 3: du with total (-c)"
OUTPUT=$("$EXECUTABLE" du -c)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "1026[[:space:]]+total"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -c outputs correct grand total" "$PASS"

# --- Test 4: du with megabytes (-m) ---
echo -e "\nRunning test 4: du with megabytes (-m)"
OUTPUT=$("$EXECUTABLE" du -m)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "1[[:space:]]+.*subdir1" && matches "$OUTPUT" "2[[:space:]]+\."; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -m outputs correct sizes in MB" "$PASS"

# --- Test 5: du with human-readable (-h) ---
echo -e "\nRunning test 5: du with human-readable (-h)"
OUTPUT=$("$EXECUTABLE" du -ah)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "13B[[:space:]]+.*file1.txt" && matches "$OUTPUT" "1\.0K[[:space:]]+.*file2.txt" && matches "$OUTPUT" "1\.0M[[:space:]]+.*file3.txt"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -h formats sizes human-readably" "$PASS"

# --- Test 6: du with summarize (-s) ---
echo -e "\nRunning test 6: du with summarize (-s)"
OUTPUT=$("$EXECUTABLE" du -s)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "1026[[:space:]]+\." && not_matches "$OUTPUT" "subdir1"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -s outputs only summarizing root" "$PASS"

# --- Test 7: du incompatible switches (-hk) ---
echo -e "\nRunning test 7: du incompatible switches (-hk)"
ERROR_OUTPUT=$("$EXECUTABLE" du -hk 2>&1)
echo "Output:"
echo "$ERROR_OUTPUT"

if matches "$ERROR_OUTPUT" "exclusive"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -hk errors out with exclusivity warning" "$PASS"

# --- Test 8: du incompatible switches (-s -d 1) ---
echo -e "\nRunning test 8: du incompatible switches (-s -d 1)"
ERROR_OUTPUT=$("$EXECUTABLE" du -s -d 1 2>&1)
echo "Output:"
echo "$ERROR_OUTPUT"

if matches "$ERROR_OUTPUT" "Cannot combine"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Tree with -s -d 1 errors out with conflict warning" "$PASS"

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
