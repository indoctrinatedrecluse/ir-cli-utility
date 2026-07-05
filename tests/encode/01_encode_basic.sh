#!/bin/bash
# Test: Unified encode/decode functionality

# --- Setup ---
ScriptDir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RepoRoot="$(cd "$ScriptDir/../.." && pwd)"
cd "$RepoRoot"

Executable="./target/debug/ir"
TestDir="$ScriptDir/temp_test_encode"
rm -rf "$TestDir"
mkdir -p "$TestDir"

PlainFile="$TestDir/plain.txt"
PlainHello="$TestDir/hello.txt"
PlainAbc="$TestDir/abc.txt"
PlainHelloQuest="$TestDir/hello_quest.txt"
OutFile="$TestDir/output.txt"

echo -n "hello world!" > "$PlainFile"
echo -n "hello" > "$PlainHello"
echo -n "abc" > "$PlainAbc"
echo -n "hello world?" > "$PlainHelloQuest"

Passed=true
assert_equal() {
    actual="$1"
    expected="$2"
    msg="$3"
    # Trim whitespace
    actual_trimmed=$(echo "$actual" | xargs)
    expected_trimmed=$(echo "$expected" | xargs)
    if [ "$actual_trimmed" = "$expected_trimmed" ]; then
        echo "✅ PASS: $msg"
    else
        echo "❌ FAIL: $msg (Expected: '$expected_trimmed', Got: '$actual_trimmed')"
        Passed=false
    fi
}

# --- 1. Base64 & Base64Url ---
echo -e "\n--- Testing Base64 & Base64Url ---"
B64Enc=$("$Executable" encode -f base64 "$PlainFile")
assert_equal "$B64Enc" "aGVsbG8gd29ybGQh" "Base64 encode standard"

B64DecInput="$TestDir/b64_dec_input.txt"
echo -n "aGVsbG8gd29ybGQh" > "$B64DecInput"
B64Dec=$("$Executable" decode -f base64 "$B64DecInput")
assert_equal "$B64Dec" "hello world!" "Base64 decode standard"

B64UrlEnc=$("$Executable" encode -f base64url -n "$PlainHelloQuest")
assert_equal "$B64UrlEnc" "aGVsbG8gd29ybGQ_" "Base64url encode unpadded"

B64UrlDecInput="$TestDir/b64url_dec_input.txt"
echo -n "aGVsbG8gd29ybGQ_" > "$B64UrlDecInput"
B64UrlDec=$("$Executable" decode -f base64url -n "$B64UrlDecInput")
assert_equal "$B64UrlDec" "hello world?" "Base64url decode unpadded"


# --- 2. Base32 ---
echo -e "\n--- Testing Base32 ---"
B32Enc=$("$Executable" encode -f base32 "$PlainAbc")
assert_equal "$B32Enc" "MFRGG===" "Base32 encode with padding"

B32DecInput="$TestDir/b32_dec_input.txt"
echo -n "MFRGG===" > "$B32DecInput"
B32Dec=$("$Executable" decode -f base32 "$B32DecInput")
assert_equal "$B32Dec" "abc" "Base32 decode standard"

B32EncNoPad=$("$Executable" encode -f base32 -n "$PlainAbc")
assert_equal "$B32EncNoPad" "MFRGG" "Base32 encode unpadded"

B32DecNoPadInput="$TestDir/b32_dec_nopad_input.txt"
echo -n "MFRGG" > "$B32DecNoPadInput"
B32DecNoPad=$("$Executable" decode -f base32 "$B32DecNoPadInput")
assert_equal "$B32DecNoPad" "abc" "Base32 decode unpadded"


# --- 3. Hex (Base16) ---
echo -e "\n--- Testing Hex (Base16) ---"
HexEnc=$("$Executable" encode -f hex "$PlainHello")
assert_equal "$HexEnc" "68656c6c6f" "Hex encode lowercase"

HexDecInput="$TestDir/hex_dec_input.txt"
echo -n "68656c6c6f" > "$HexDecInput"
HexDec=$("$Executable" decode -f hex "$HexDecInput")
assert_equal "$HexDec" "hello" "Hex decode lowercase"

HexEncUpperSep=$("$Executable" encode -f hex --upper --separator ":" "$PlainHello")
assert_equal "$HexEncUpperSep" "68:65:6C:6C:6F" "Hex encode uppercase with separator"

HexDecSepInput="$TestDir/hex_dec_sep_input.txt"
echo -n "68:65:6C:6C:6F" > "$HexDecSepInput"
HexDecSep=$("$Executable" decode -f hex --separator ":" "$HexDecSepInput")
assert_equal "$HexDecSep" "hello" "Hex decode with separator"


# --- 4. URL (Percent-Encoding) ---
echo -e "\n--- Testing URL ---"
UrlEnc=$("$Executable" encode -f url "$PlainFile")
assert_equal "$UrlEnc" "hello%20world%21" "URL encode default"

UrlDecInput="$TestDir/url_dec_input.txt"
echo -n "hello%20world%21" > "$UrlDecInput"
UrlDec=$("$Executable" decode -f url "$UrlDecInput")
assert_equal "$UrlDec" "hello world!" "URL decode default"

UrlEncAll=$("$Executable" encode -f url --all "$PlainHello")
assert_equal "$UrlEncAll" "%68%65%6C%6C%6F" "URL encode all"


# --- 5. Rot13 ---
echo -e "\n--- Testing Rot13 ---"
RotEnc=$("$Executable" encode -f rot13 "$PlainFile")
assert_equal "$RotEnc" "Uryyb Jbeyq!" "Rot13 encode"

RotDecInput="$TestDir/rot13_dec_input.txt"
echo -n "Uryyb Jbeyq!" > "$RotDecInput"
RotDec=$("$Executable" decode -f rot13 "$RotDecInput")
assert_equal "$RotDec" "hello world!" "Rot13 decode"


# --- 6. File Input / Output ---
echo -e "\n--- Testing File Input / Output ---"
"$Executable" encode -f base64 -o "$OutFile" "$PlainFile"
FileContent=$(cat "$OutFile" | xargs)
assert_equal "$FileContent" "aGVsbG8gd29ybGQh" "Encode file to output file path"

"$Executable" decode -f base64 -o "$PlainFile" "$OutFile"
PlainContent=$(cat "$PlainFile" | xargs)
assert_equal "$PlainContent" "hello world!" "Decode file to output file path"


# --- Teardown ---
rm -rf "$TestDir"

if [ "$Passed" = true ]; then
    echo -e "\n✅ ALL ENCODE/DECODE TESTS PASSED"
    exit 0
else
    echo -e "\n❌ SOME ENCODE/DECODE TESTS FAILED"
    exit 1
fi
