# Test: text formatting and case conversions

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"

$Passed = $true
function Assert-Equal($Actual, $Expected, $Msg) {
    if ($Actual.Trim() -eq $Expected.Trim()) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected '$Expected', got '$Actual')"
        $global:Passed = $false
    }
}

# --- 1. Basic Case Conversions ---
Write-Host "Testing basic case conversions..."

$CamelOut = "hello_world_text" | & $Executable text -c camel
Assert-Equal $CamelOut "helloWorldText" "Camel case conversion"

$SnakeOut = "HelloWorldText" | & $Executable text -c snake
Assert-Equal $SnakeOut "hello_world_text" "Snake case conversion"

$PascalOut = "hello-world-text" | & $Executable text -c pascal
Assert-Equal $PascalOut "HelloWorldText" "Pascal case conversion"

$KebabOut = "HelloWorldText" | & $Executable text -c kebab
Assert-Equal $KebabOut "hello-world-text" "Kebab case conversion"

$UpperOut = "helloWorldText" | & $Executable text -c upper
Assert-Equal $UpperOut "HELLO_WORLD_TEXT" "Upper case conversion"

$LowerOut = "HelloWorldText" | & $Executable text -c lower
Assert-Equal $LowerOut "helloworldtext" "Lower case conversion"

$TitleOut = "hello_world_text" | & $Executable text -c title
Assert-Equal $TitleOut "Hello World Text" "Title case conversion"

$SentenceOut = "hello_world_text" | & $Executable text -c sentence
Assert-Equal $SentenceOut "Hello world text" "Sentence case conversion"

$SlugOut = "Hello World! 2026." | & $Executable text -c slug
Assert-Equal $SlugOut "hello-world-2026" "Slug case conversion"

# --- 2. Alignment & Truncation ---
Write-Host "Testing alignment and truncation..."

$AlignLeft = "hello" | & $Executable text --align left -w 10
Assert-Equal $AlignLeft "hello     " "Align left with width"

$AlignRight = "hello" | & $Executable text --align right -w 10
Assert-Equal $AlignRight "     hello" "Align right with width"

$AlignCenter = "hello" | & $Executable text --align center -w 10
Assert-Equal $AlignCenter "  hello   " "Align center with width"

$TruncateOut = "hello world text" | & $Executable text --truncate -w 8
Assert-Equal $TruncateOut "hello..." "Truncation with default ellipsis"

$TruncateCustom = "hello world text" | & $Executable text --truncate --ellipsis "---" -w 8
Assert-Equal $TruncateCustom "hello---" "Truncation with custom ellipsis"

# --- 3. Stripping Operations ---
Write-Host "Testing stripping..."

$Esc = [char]0x1b
$StripAnsiOut = "${Esc}[31mRedText${Esc}[0m" | & $Executable text --strip-ansi
Assert-Equal $StripAnsiOut "RedText" "ANSI escape sequences stripping"

$StripNonAlpha = "Hello, World! 123." | & $Executable text --strip-non-alphanumeric
Assert-Equal $StripNonAlpha "Hello World 123" "Non-alphanumeric characters stripping"

# --- 4. Redirection ---
Write-Host "Testing output file redirection..."
$TempFile = Join-Path $ScriptDir "temp_text_out.txt"
if (Test-Path $TempFile) { Remove-Item $TempFile }

"hello_world" | & $Executable text -c title -o $TempFile
$FileContent = Get-Content $TempFile | Out-String
Assert-Equal $FileContent "Hello World" "Output file redirection match"
if (Test-Path $TempFile) { Remove-Item $TempFile }

# --- 5. Error Handling & Validation ---
Write-Host "Testing error handling..."

$ErrCase = & $Executable text -c invalid_case 2>&1 | Out-String
if ($ErrCase.Contains("Error: Invalid case format") -and $LASTEXITCODE -ne 0) {
    Write-Host "PASS: Invalid case format error detected"
} else {
    Write-Host "FAIL: Invalid case format should return error"
    $Passed = $false
}

$ErrWidth = & $Executable text --width abc 2>&1 | Out-String
if ($ErrWidth.Contains("Error: --width requires a valid positive integer") -and $LASTEXITCODE -ne 0) {
    Write-Host "PASS: Invalid width parameter error detected"
} else {
    Write-Host "FAIL: Invalid width parameter should return error"
    $Passed = $false
}

$ErrAlign = & $Executable text --align invalid 2>&1 | Out-String
if ($ErrAlign.Contains("Error: Invalid align option") -and $LASTEXITCODE -ne 0) {
    Write-Host "PASS: Invalid alignment option error detected"
} else {
    Write-Host "FAIL: Invalid alignment option should return error"
    $Passed = $false
}

if ($Passed) {
    Write-Host "`nALL TEXT INTEGRATION TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME TEXT INTEGRATION TESTS FAILED"
    exit 1
}
