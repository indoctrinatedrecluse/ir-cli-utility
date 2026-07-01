#!/bin/bash
# Test: Basic hashing functions (md5, sha1, sha256) on Linux

# --- Setup ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
cargo build --quiet
EXECUTABLE="./target/debug/ir"
TEST_DIR="temp_test_hash_01_sh"
mkdir -p "$TEST_DIR"

TEST_FILE="$TEST_DIR/test.txt"
echo -n "Hello World" > "$TEST_FILE"

# --- Test MD5 ---
echo "Testing MD5 hashing..."
MD5_OUT=$($EXECUTABLE hash -a md5 "$TEST_FILE")
if [[ $MD5_OUT == *b10a8db164e0754105b7a99be72e3fe5* ]]; then
    echo "PASS: MD5 hash match."
else
    echo "FAIL: MD5 hash mismatch. Output: $MD5_OUT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test SHA-1 ---
echo "Testing SHA-1 hashing..."
SHA1_OUT=$($EXECUTABLE hash -a sha1 "$TEST_FILE")
if [[ $SHA1_OUT == *0a4d55a8d778e5022fab701977c5d840bbc486d0* ]]; then
    echo "PASS: SHA-1 hash match."
else
    echo "FAIL: SHA-1 hash mismatch. Output: $SHA1_OUT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Test SHA-256 (Default) ---
echo "Testing SHA-256 default hashing..."
SHA256_OUT=$($EXECUTABLE hash "$TEST_FILE")
if [[ $SHA256_OUT == *a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e* ]]; then
    echo "PASS: SHA-256 hash match."
else
    echo "FAIL: SHA-256 hash mismatch. Output: $SHA256_OUT"
    rm -rf "$TEST_DIR"
    exit 1
fi

# --- Teardown ---
rm -rf "$TEST_DIR"
echo "ALL BASIC HASH TESTS PASSED"
exit 0
