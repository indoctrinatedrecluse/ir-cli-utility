# Test: json utility functionality

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot

$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_json"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$JsonFile = Join-Path $TestDir "input.json"
$OutFile = Join-Path $TestDir "output.json"

$JsonData = '{
    "name": "Antigravity",
    "features": [
        "text",
        "graphics"
    ],
    "nested": {
        "count": 42
    }
}'
[System.IO.File]::WriteAllText($JsonFile, $JsonData)

$Passed = $true
function Assert-Equal($Actual, $Expected, $Msg) {
    if ($Actual.Trim() -eq $Expected.Trim()) {
        Write-Host "✅ PASS: $Msg"
    } else {
        Write-Host "❌ FAIL: $Msg (Expected: '$Expected', Got: '$Actual')"
        $global:Passed = $false
    }
}

# --- 1. Pretty Print (Default) ---
$PrettyOut = & $Executable json $JsonFile | Out-String
if ($PrettyOut.Contains("Antigravity") -and $PrettyOut.Contains("`n")) {
    Write-Host "✅ PASS: Default pretty print format"
} else {
    Write-Host "❌ FAIL: Default pretty print format"
    $Passed = $false
}

# --- 2. Minify ---
$MinifiedOut = & $Executable json -m $JsonFile | Out-String
Assert-Equal $MinifiedOut '{"features":["text","graphics"],"name":"Antigravity","nested":{"count":42}}' "Minified JSON matches exactly"

# --- 3. Query Selector - Object Field ---
$QueryName = & $Executable json -q .name $JsonFile | Out-String
Assert-Equal $QueryName '"Antigravity"' "Query object string field"

# --- 4. Query Selector - Array Index ---
$QueryFeature = & $Executable json -q ".features[1]" $JsonFile | Out-String
Assert-Equal $QueryFeature '"graphics"' "Query array element"

# --- 5. Query Selector - Nested Object ---
$QueryCount = & $Executable json -q ".nested.count" $JsonFile | Out-String
Assert-Equal $QueryCount "42" "Query nested object field"

# --- 6. Custom Indentation ---
$IndentOut = & $Executable json --indent 2 $JsonFile | Out-String
if ($IndentOut.Contains("  `"name`":")) {
    Write-Host "✅ PASS: Custom 2-space indentation works"
} else {
    Write-Host "❌ FAIL: Custom 2-space indentation works"
    $Passed = $false
}

# --- 7. Output to File ---
& $Executable json -m -o $OutFile $JsonFile
$OutData = [System.IO.File]::ReadAllText($OutFile).Trim()
Assert-Equal $OutData '{"features":["text","graphics"],"name":"Antigravity","nested":{"count":42}}' "Minified output written to file"

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

if ($Passed) {
    Write-Host "`n✅ ALL JSON TESTS PASSED"
    exit 0
} else {
    Write-Host "`n❌ SOME JSON TESTS FAILED"
    exit 1
}
