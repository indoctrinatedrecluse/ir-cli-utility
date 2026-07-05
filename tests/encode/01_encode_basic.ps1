# Test: Unified encode/decode functionality

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot

$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_encode"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$PlainFile = Join-Path $TestDir "plain.txt"
$PlainHello = Join-Path $TestDir "hello.txt"
$PlainAbc = Join-Path $TestDir "abc.txt"
$PlainHelloQuest = Join-Path $TestDir "hello_quest.txt"
$OutFile = Join-Path $TestDir "output.txt"

[System.IO.File]::WriteAllText($PlainFile, "hello world!")
[System.IO.File]::WriteAllText($PlainHello, "hello")
[System.IO.File]::WriteAllText($PlainAbc, "abc")
[System.IO.File]::WriteAllText($PlainHelloQuest, "hello world?")

$Passed = $true
function Assert-Equal($Actual, $Expected, $Msg) {
    if ($Actual.Trim() -eq $Expected) {
        Write-Host "✅ PASS: $Msg"
    } else {
        Write-Host "❌ FAIL: $Msg (Expected: '$Expected', Got: '$Actual')"
        $global:Passed = $false
    }
}

# --- 1. Base64 & Base64Url ---
Write-Host "`n--- Testing Base64 & Base64Url ---"
# Base64 standard encode
$B64Enc = & $Executable encode -f base64 $PlainFile | Out-String
Assert-Equal $B64Enc "aGVsbG8gd29ybGQh" "Base64 encode standard"

# Base64 standard decode via file input
$B64DecInput = Join-Path $TestDir "b64_dec_input.txt"
[System.IO.File]::WriteAllText($B64DecInput, "aGVsbG8gd29ybGQh")
$B64Dec = & $Executable decode -f base64 $B64DecInput | Out-String
Assert-Equal $B64Dec "hello world!" "Base64 decode standard"

# Base64Url encode unpadded
$B64UrlEnc = & $Executable encode -f base64url -n $PlainHelloQuest | Out-String
Assert-Equal $B64UrlEnc "aGVsbG8gd29ybGQ_" "Base64url encode unpadded"

# Base64Url decode unpadded
$B64UrlDecInput = Join-Path $TestDir "b64url_dec_input.txt"
[System.IO.File]::WriteAllText($B64UrlDecInput, "aGVsbG8gd29ybGQ_")
$B64UrlDec = & $Executable decode -f base64url -n $B64UrlDecInput | Out-String
Assert-Equal $B64UrlDec "hello world?" "Base64url decode unpadded"


# --- 2. Base32 ---
Write-Host "`n--- Testing Base32 ---"
# Base32 standard encode with padding (abc -> MFRGG===)
$B32Enc = & $Executable encode -f base32 $PlainAbc | Out-String
Assert-Equal $B32Enc "MFRGG===" "Base32 encode with padding"

# Base32 standard decode
$B32DecInput = Join-Path $TestDir "b32_dec_input.txt"
[System.IO.File]::WriteAllText($B32DecInput, "MFRGG===")
$B32Dec = & $Executable decode -f base32 $B32DecInput | Out-String
Assert-Equal $B32Dec "abc" "Base32 decode standard"

# Base32 encode unpadded
$B32EncNoPad = & $Executable encode -f base32 -n $PlainAbc | Out-String
Assert-Equal $B32EncNoPad "MFRGG" "Base32 encode unpadded"

# Base32 decode unpadded
$B32DecNoPadInput = Join-Path $TestDir "b32_dec_nopad_input.txt"
[System.IO.File]::WriteAllText($B32DecNoPadInput, "MFRGG")
$B32DecNoPad = & $Executable decode -f base32 $B32DecNoPadInput | Out-String
Assert-Equal $B32DecNoPad "abc" "Base32 decode unpadded"


# --- 3. Hex (Base16) ---
Write-Host "`n--- Testing Hex (Base16) ---"
# Hex encode lowercase (default)
$HexEnc = & $Executable encode -f hex $PlainHello | Out-String
Assert-Equal $HexEnc "68656c6c6f" "Hex encode lowercase"

# Hex decode lowercase
$HexDecInput = Join-Path $TestDir "hex_dec_input.txt"
[System.IO.File]::WriteAllText($HexDecInput, "68656c6c6f")
$HexDec = & $Executable decode -f hex $HexDecInput | Out-String
Assert-Equal $HexDec "hello" "Hex decode lowercase"

# Hex encode uppercase with custom separator
$HexEncUpperSep = & $Executable encode -f hex --upper --separator ":" $PlainHello | Out-String
Assert-Equal $HexEncUpperSep "68:65:6C:6C:6F" "Hex encode uppercase with separator"

# Hex decode with separator
$HexDecSepInput = Join-Path $TestDir "hex_dec_sep_input.txt"
[System.IO.File]::WriteAllText($HexDecSepInput, "68:65:6C:6C:6F")
$HexDecSep = & $Executable decode -f hex --separator ":" $HexDecSepInput | Out-String
Assert-Equal $HexDecSep "hello" "Hex decode with separator"


# --- 4. URL (Percent-Encoding) ---
Write-Host "`n--- Testing URL ---"
# URL encode default
$UrlEnc = & $Executable encode -f url $PlainFile | Out-String
Assert-Equal $UrlEnc "hello%20world%21" "URL encode default"

# URL decode default
$UrlDecInput = Join-Path $TestDir "url_dec_input.txt"
[System.IO.File]::WriteAllText($UrlDecInput, "hello%20world%21")
$UrlDec = & $Executable decode -f url $UrlDecInput | Out-String
Assert-Equal $UrlDec "hello world!" "URL decode default"

# URL encode all
$UrlEncAll = & $Executable encode -f url --all $PlainHello | Out-String
Assert-Equal $UrlEncAll "%68%65%6C%6C%6F" "URL encode all"


# --- 5. Rot13 ---
Write-Host "`n--- Testing Rot13 ---"
# Rot13 encode
$RotEnc = & $Executable encode -f rot13 $PlainFile | Out-String
Assert-Equal $RotEnc "Uryyb Jbeyq!" "Rot13 encode"

# Rot13 decode (symmetric check)
$RotDecInput = Join-Path $TestDir "rot13_dec_input.txt"
[System.IO.File]::WriteAllText($RotDecInput, "Uryyb Jbeyq!")
$RotDec = & $Executable decode -f rot13 $RotDecInput | Out-String
Assert-Equal $RotDec "hello world!" "Rot13 decode"


# --- 6. File Input / Output ---
Write-Host "`n--- Testing File Input / Output ---"
# Encode file to output file
& $Executable encode -f base64 -o $OutFile $PlainFile
$FileContent = [System.IO.File]::ReadAllText($OutFile).Trim()
Assert-Equal $FileContent "aGVsbG8gd29ybGQh" "Encode file to output file path"

# Decode file to output file
& $Executable decode -f base64 -o $PlainFile $OutFile
$PlainContent = [System.IO.File]::ReadAllText($PlainFile).Trim()
Assert-Equal $PlainContent "hello world!" "Decode file to output file path"


# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

if ($Passed) {
    Write-Host "`n✅ ALL ENCODE/DECODE TESTS PASSED"
    exit 0
} else {
    Write-Host "`n❌ SOME ENCODE/DECODE TESTS FAILED"
    exit 1
}
