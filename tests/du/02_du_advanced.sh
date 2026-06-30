#!/bin/bash
# Test: du Action Advanced Situations (Multiple paths, invalid paths, file inputs, custom depth)

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
TEST_DIR="temp_test_du_02"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create subdirectories
mkdir -p "dir1/dir1_sub"
mkdir -p "dir2"

# fileA.txt -> 512 bytes
dd if=/dev/zero of="dir1/dir1_sub/fileA.txt" bs=512 count=1 2>/dev/null
# fileB.txt -> 2048 bytes (2 KB)
dd if=/dev/zero of="dir2/fileB.txt" bs=2048 count=1 2>/dev/null

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

# --- Test 1: Multiple Roots ---
echo "Running test 1: Multiple Roots"
OUTPUT=$("$EXECUTABLE" du dir1 dir2)
echo "Output:"
echo "$OUTPUT"

# dir1 contains dir1_sub (512B -> 1KB). dir2 contains fileB (2048B -> 2KB).
if matches "$OUTPUT" "1[[:space:]]+.*dir1" && matches "$OUTPUT" "2[[:space:]]+.*dir2"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Multiple root paths printed separately with correct size" "$PASS"

# --- Test 2: Multiple Roots with Total (-c) ---
echo -e "\nRunning test 2: Multiple Roots with Total (-c)"
OUTPUT=$("$EXECUTABLE" du -c dir1 dir2)
echo "Output:"
echo "$OUTPUT"

# Total should be dir1 (1KB) + dir2 (2KB) = 3KB
if matches "$OUTPUT" "1[[:space:]]+.*dir1" && matches "$OUTPUT" "2[[:space:]]+.*dir2" && matches "$OUTPUT" "3[[:space:]]+total"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Multiple roots total matches sum of targets" "$PASS"

# --- Test 3: Invalid/Non-existent Path ---
echo -e "\nRunning test 3: Invalid/Non-existent Path"
ERROR_OUTPUT=$("$EXECUTABLE" du dir1 non_existent_folder 2>&1)
echo "Output:"
echo "$ERROR_OUTPUT"

if matches "$ERROR_OUTPUT" "non_existent_folder" && matches "$ERROR_OUTPUT" "1[[:space:]]+.*dir1"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Invalid paths report errors to stderr but valid paths are still printed" "$PASS"

# --- Test 4: Single File Path Argument ---
echo -e "\nRunning test 4: Single File Path Argument"
OUTPUT=$("$EXECUTABLE" du dir2/fileB.txt)
echo "Output:"
echo "$OUTPUT"

if matches "$OUTPUT" "2[[:space:]]+.*fileB.txt"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Direct file argument outputs its exact KB size" "$PASS"

# --- Test 5: Depth Limit -d 1 on dir1 ---
echo -e "\nRunning test 5: Depth Limit -d 1"
OUTPUT=$("$EXECUTABLE" du -d 1 dir1)
echo "Output:"
echo "$OUTPUT"

# dir1_sub is depth 1 relative to dir1.
if matches "$OUTPUT" "1[[:space:]]+.*dir1_sub" && matches "$OUTPUT" "1[[:space:]]+.*dir1"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Depth limit -d 1 lists subdir and parent" "$PASS"

# --- Test 6: Depth Limit -d 0 on dir1 ---
echo -e "\nRunning test 6: Depth Limit -d 0"
OUTPUT=$("$EXECUTABLE" du -d 0 dir1)
echo "Output:"
echo "$OUTPUT"

# Only dir1 should be printed
if matches "$OUTPUT" "1[[:space:]]+.*dir1" && not_matches "$OUTPUT" "dir1_sub"; then
    PASS=0
else
    PASS=1
fi
Check-Test "Depth limit -d 0 acts as summarize" "$PASS"

# --- Teardown ---
cd ..
rm -rf "$TEST_DIR"

exit $RESULT
