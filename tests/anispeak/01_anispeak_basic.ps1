# Test: Basic anispeak functionality.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: anispeak basic ---
Write-Host "Running test: ir anispeak 'hello world'"
$Output1 = & $Executable anispeak "hello world" | Out-String

# --- Test 2: anispeak animal selection ---
Write-Host "Running test: ir anispeak -a crab 'hello world'"
$Output2 = & $Executable anispeak -a crab "hello world" | Out-String

# --- Verification ---
$Result = 0
if ($Output1 -notlike "*hello world*") {
    Write-Host "FAIL: anispeak basic output was missing message: '$Output1'"
    $Result = 1
} elseif ($Output1 -notlike "*oo*") {
    Write-Host "FAIL: anispeak basic output was missing cow art: '$Output1'"
    $Result = 1
} elseif ($Output2 -notlike "*o o*") {
    Write-Host "FAIL: anispeak crab output was missing crab art: '$Output2'"
    $Result = 1
} else {
    Write-Host "PASS: 'anispeak' successfully wrapped messages and output ASCII art."
}

exit $Result
