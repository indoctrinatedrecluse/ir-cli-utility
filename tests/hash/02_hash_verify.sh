#!/bin/bash
# Test: Hash verification (-v and -c) on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_hash_02_sh"
mkdir -p "$TEST_DIR"

FILE1="$TEST_DIR/file1.txt"
FILE2="$TEST_DIR/file2.txt"
echo -n "hello" > "$FILE1"
echo -n "world" > "$FILE2"

# --- Test 1: Single Verification Success ---
echo "Testing single hash verification success..."
$EXECUTABLE hash -a sha256 -v 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824 "$FILE1"
if [ $? -ne 0 ]; then
    echo "FAIL: Single verification returned error exit code $?."
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test 2: Single Verification Failure ---
echo "Testing single hash verification failure..."
$EXECUTABLE hash -a sha256 -v badhash "$FILE1" 2>/dev/null
if [ $? -eq 0 ]; then
    echo "FAIL: Single verification succeeded with a bad hash."
    rm -rf "$TEST_DIR"
    exit 1
else
    echo "PASS: Single verification correctly failed with a bad hash."
fi

# --- Test 3: Checksum File Verification Success ---
echo "Testing checksum file verification..."
CHECKSUM_FILE="$TEST_DIR/checksums.txt"
echo "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  $FILE1" > "$CHECKSUM_FILE"
echo "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7  $FILE2" >> "$CHECKSUM_FILE"

$EXECUTABLE hash -c "$CHECKSUM_FILE"
if [ $? -ne 0 ]; then
    echo "FAIL: Checksum file verification failed with exit code $?."
    rm -rf "$TEST_DIR"
    exit 1
else
    echo "PASS: Checksum file verification succeeded."
fi

# --- Test 4: Checksum File Verification Failure ---
echo "Testing checksum file verification failure..."
BAD_CHECKSUM_FILE="$TEST_DIR/bad_checksums.txt"
echo "badhash  $FILE1" > "$BAD_CHECKSUM_FILE"
echo "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7  $FILE2" >> "$BAD_CHECKSUM_FILE"

$EXECUTABLE hash -c "$BAD_CHECKSUM_FILE" 2>/dev/null
if [ $? -eq 0 ]; then
    echo "FAIL: Checksum file verification did not fail with a bad hash."
    rm -rf "$TEST_DIR"
    exit 1
else
    echo "PASS: Checksum file verification correctly failed on bad hash."
fi

# --- Teardown ---
rm -rf "$TEST_DIR"
echo "ALL HASH VERIFICATION TESTS PASSED"
exit 0
