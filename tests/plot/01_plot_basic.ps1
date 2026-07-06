# Test: plot utility functionality

# --- Setup ---
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_plot"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$TxtFile = Join-Path $TestDir "data.txt"
$TxtNegFile = Join-Path $TestDir "data_neg.txt"
$CsvFile = Join-Path $TestDir "data.csv"
$JsonFile = Join-Path $TestDir "data.json"
$JsonObjFile = Join-Path $TestDir "data_obj.json"

[System.IO.File]::WriteAllText($TxtFile, "10 20 15 30 25")
[System.IO.File]::WriteAllText($TxtNegFile, "10 -5 15 30 25")
[System.IO.File]::WriteAllText($CsvFile, "month,sales`nJan,10`nFeb,20`nMar,15`nApr,30`nMay,25")
[System.IO.File]::WriteAllText($JsonFile, "[10, 20, 15, 30, 25]")
[System.IO.File]::WriteAllText($JsonObjFile, '[{"info":{"val":10}}, {"info":{"val":20}}, {"info":{"val":15}}, {"info":{"val":30}}, {"info":{"val":25}}]')

$Passed = $true
function Assert-Contains($Actual, $Expected, $Msg) {
    if ($Actual.Contains($Expected)) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected to contain '$Expected')"
        $global:Passed = $false
    }
}

function Assert-Matches($Actual, $Pattern, $Msg) {
    if ($Actual -match $Pattern) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected to match regex '$Pattern')"
        $global:Passed = $false
    }
}

# --- 1. Line Chart from Space-Separated TXT ---
Write-Host "`n--- Test 1: Line Chart from Space-Separated TXT ---"
$OutLine = & $Executable plot $TxtFile -t line -w 10 -g 5 | Out-String
Assert-Contains $OutLine "30.00" "Y-axis max value label present"
Assert-Contains $OutLine "10.00" "Y-axis min value label present"
Assert-Contains $OutLine "0" "X-axis start index present"
Assert-Contains $OutLine "4" "X-axis end index present"

# --- 2. Bar Chart from Space-Separated TXT ---
Write-Host "`n--- Test 2: Bar Chart from Space-Separated TXT ---"
$OutBar = & $Executable plot $TxtFile -t bar -w 10 -g 5 --title "Sales Chart" | Out-String
Assert-Contains $OutBar "Sales Chart" "Chart title displayed"
$FullBlock = [char]0x2588
Assert-Contains $OutBar $FullBlock "Contains bar blocks"

# --- 3. CSV Source column parsing ---
Write-Host "`n--- Test 3: Plot CSV file column with headers ---"
$OutCsv = & $Executable plot $CsvFile --source csv --csv-col 1 --csv-headers -w 10 -g 5 | Out-String
Assert-Contains $OutCsv "30.00" "Parsed column with header successfully"

# --- 4. JSON Source flat array parsing ---
Write-Host "`n--- Test 4: Plot JSON array ---"
$OutJson = & $Executable plot $JsonFile --source json -w 10 -g 5 | Out-String
Assert-Contains $OutJson "30.00" "Parsed JSON array successfully"

# --- 5. Piped input from stdin ---
Write-Host "`n--- Test 5: Piped input from stdin ---"
$OutPiped = Get-Content $TxtFile | & $Executable plot -w 10 -g 5 | Out-String
Assert-Contains $OutPiped "30.00" "Piped standard input successfully"

# --- 6. Smooth Braille graphics ---
Write-Host "`n--- Test 6: Smooth Braille graphics ---"
$OutSmooth = & $Executable plot $TxtFile --smooth -w 10 -g 5 | Out-String
Assert-Matches $OutSmooth "[\u2800-\u28FF]" "Contains Braille characters"

# --- 7. Logarithmic scaling ---
Write-Host "`n--- Test 7: Logarithmic scaling ---"
$OutLog = & $Executable plot $TxtFile --log -w 10 -g 5 | Out-String
Assert-Contains $OutLog "30.00" "Axis labels are printed in base numbers for log scale"

# --- 8. Logarithmic scaling error on non-positive input ---
Write-Host "`n--- Test 8: Logarithmic scaling error on non-positive input ---"
$OutLogErr = & $Executable plot $TxtNegFile --log -w 10 -g 5 2>&1 | Out-String
Assert-Contains $OutLogErr "Error: Logarithmic scale requires all data points to be positive" "Negative value in log scaling rejected"

# --- 9. JSON key nested path query extraction ---
Write-Host "`n--- Test 9: JSON key nested path query extraction ---"
$OutJsonQuery = & $Executable plot $JsonObjFile --source json --json-key ".info.val" -w 10 -g 5 | Out-String
Assert-Contains $OutJsonQuery "30.00" "Extracted values using nested json key query"

# --- 10. Horizontal bar chart ---
Write-Host "`n--- Test 10: Horizontal bar chart ---"
$OutHBar = & $Executable plot $TxtFile -H -w 10 -g 5 | Out-String
$HBarTick = [char]0x253c # '┼' tick symbol
Assert-Contains $OutHBar $HBarTick "Contains horizontal axis tick characters"
$HBarBlock = [char]0x2588 # '█'
Assert-Contains $OutHBar $HBarBlock "Contains horizontal block elements"

# --- 11. Format and syntax error handling ---
Write-Host "`n--- Test 11: Format and syntax error handling ---"
$ErrCsvOut = & $Executable plot $CsvFile --source csv --csv-col 99 -w 10 -g 5 2>&1 | Out-String
Assert-Contains $ErrCsvOut "column index 99 is out of bounds" "Out-of-bounds CSV column index handled"

$ErrJsonOut = & $Executable plot $TxtFile --source json -w 10 -g 5 2>&1 | Out-String
Assert-Contains $ErrJsonOut "Failed to parse JSON source data" "JSON parsing syntax error handled"

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

if ($Passed) {
    Write-Host "`nALL PLOT TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME PLOT TESTS FAILED"
    exit 1
}
