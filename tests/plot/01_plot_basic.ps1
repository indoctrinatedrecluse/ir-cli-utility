# Test: plot utility functionality

# --- Setup ---
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot

$Executable = ".\target\debug\ir.exe"
$TestDir = Join-Path $ScriptDir "temp_test_plot"
if (Test-Path $TestDir) { Remove-Item -Recurse -Force $TestDir }
New-Item -ItemType Directory -Path $TestDir | Out-Null

$TxtFile = Join-Path $TestDir "data.txt"
$CsvFile = Join-Path $TestDir "data.csv"
$JsonFile = Join-Path $TestDir "data.json"

[System.IO.File]::WriteAllText($TxtFile, "10 20 15 30 25")
[System.IO.File]::WriteAllText($CsvFile, "month,sales`nJan,10`nFeb,20`nMar,15`nApr,30`nMay,25")
[System.IO.File]::WriteAllText($JsonFile, "[10, 20, 15, 30, 25]")

$Passed = $true
function Assert-Contains($Actual, $Expected, $Msg) {
    if ($Actual.Contains($Expected)) {
        Write-Host "✅ PASS: $Msg"
    } else {
        Write-Host "❌ FAIL: $Msg (Expected to contain '$Expected')"
        $global:Passed = $false
    }
}

# --- 1. Line Chart from Space-Separated TXT ---
Write-Host "`n--- Test 1: Line Chart from Space-Separated TXT ---"
$OutLine = & $Executable plot $TxtFile -t line -w 10 -g 5 | Out-String
# Check that the X axis start (0) and end (4) indices are present, and the plot boundary or points are shown
Assert-Contains $OutLine "30.00" "Y-axis max value label present"
Assert-Contains $OutLine "10.00" "Y-axis min value label present"
Assert-Contains $OutLine "0" "X-axis start index present"
Assert-Contains $OutLine "4" "X-axis end index present"

# --- 2. Bar Chart from Space-Separated TXT ---
Write-Host "`n--- Test 2: Bar Chart from Space-Separated TXT ---"
$OutBar = & $Executable plot $TxtFile -t bar -w 10 -g 5 --title "Sales Chart" | Out-String
Assert-Contains $OutBar "Sales Chart" "Chart title displayed"

# Use [char]0x2588 (Full Block character) to prevent script file encoding/locale parsing issues
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

# --- Teardown ---
Remove-Item -Recurse -Force $TestDir

if ($Passed) {
    Write-Host "`n✅ ALL PLOT TESTS PASSED"
    exit 0
} else {
    Write-Host "`n❌ SOME PLOT TESTS FAILED"
    exit 1
}
