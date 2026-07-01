# Test: path action error handling

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
$Executable = ".\target\debug\ir.exe"

# Test 1: Conflict switches (--add and --remove specified together)
Write-Host "Testing path with conflicting switches..."
$ErrOut = & $Executable path -a C:\bin -r C:\bin 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "cannot be specified together") {
    Write-Host "PASS: Conflicting switches rejected correctly."
} else {
    Write-Host "FAIL: Expected error for conflicting switches. Output: $ErrOut"
    exit 1
}

# Test 2: Positional arguments provided
Write-Host "Testing path with positional arguments..."
$ErrOut = & $Executable path extra 2>&1 | Out-String
if ($LASTEXITCODE -ne 0 -and $ErrOut -match "does not accept positional") {
    Write-Host "PASS: Positional arguments rejected correctly."
} else {
    Write-Host "FAIL: Expected error for positional arguments. Output: $ErrOut"
    exit 1
}

# Test 3: Duplicate add (already exists in registry)
# Let's read first path from registry
$RegistryOut = & $Executable path | Out-String
$Lines = $RegistryOut -split "`r`n"
$FirstPathLine = ""
foreach ($Line in $Lines) {
    if ($Line -match "^\s{2}[A-Za-z]:\\") {
        $FirstPathLine = $Line.Trim()
        break
    }
}

if ($FirstPathLine -ne "") {
    Write-Host "Testing duplicate path addition with '$FirstPathLine'..."
    $AddOut = & $Executable path -a $FirstPathLine | Out-String
    if ($AddOut -match "already in user PATH") {
        Write-Host "PASS: Duplicate path addition detected and reported as Info cleanly."
    } else {
        Write-Host "FAIL: Expected info statement for duplicate path. Output: $AddOut"
        exit 1
    }
} else {
    Write-Host "SKIPPING: No directory registry PATH entry found to test duplicate addition."
}

# Test 4: Removing non-existent path
Write-Host "Testing removal of non-existent path..."
$NonExistentPath = "C:\non_existent_dir_1234_test"
$RemoveOut = & $Executable path -r $NonExistentPath | Out-String
if ($RemoveOut -match "was not found in user PATH") {
    Write-Host "PASS: Non-existent path removal reported as Info cleanly."
} else {
    Write-Host "FAIL: Expected info statement for non-existent path removal. Output: $RemoveOut"
    exit 1
}

Write-Host "ALL PATH ERROR TESTS PASSED"
exit 0
