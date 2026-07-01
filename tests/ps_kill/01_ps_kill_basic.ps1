# Test: ps and kill actions

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test 1: Basic process listing ---
Write-Host "Testing basic ps command..."
$PsOutput = & $Executable ps -n 5 | Out-String
if ($PsOutput -like "*PID*COMMAND*") {
    Write-Host "PASS: ps header is correct."
} else {
    Write-Host "FAIL: ps output format mismatch. Output:"
    Write-Host $PsOutput
    exit 1
}

# --- Test 2: Spawning process, filtering, and killing by name ---
Write-Host "Spawning a temporary background ping process..."
$PingProc = Start-Process -FilePath "ping" -ArgumentList "127.0.0.1", "-n", "30" -PassThru -WindowStyle Hidden
$PingPid = $PingProc.Id
Write-Host "Spawned ping process with PID: $PingPid"

# Wait a brief moment for it to initialize
Start-Sleep -Milliseconds 250

# Check process exists in ir ps
Write-Host "Searching process using ir ps -f ping..."
$FilteredPs = & $Executable ps -f ping | Out-String
if ($FilteredPs -like "*ping*") {
    Write-Host "PASS: Found spawned process in process list."
} else {
    Write-Host "FAIL: Did not find ping in process list. Output:"
    Write-Host $FilteredPs
    # Clean up just in case
    Stop-Process -Id $PingPid -Force
    exit 1
}

# Terminate process using ir kill by name
Write-Host "Terminating process using ir kill..."
& $Executable kill ping -a | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL: ir kill command failed with exit code $LASTEXITCODE."
    Stop-Process -Id $PingPid -Force
    exit 1
}

# Verify process is terminated
Start-Sleep -Milliseconds 250
$FilteredPsAfter = & $Executable ps -f ping | Out-String
if ($FilteredPsAfter -like "*ping*") {
    Write-Host "FAIL: Process still exists after kill. Output:"
    Write-Host $FilteredPsAfter
    Stop-Process -Id $PingPid -Force
    exit 1
} else {
    Write-Host "PASS: Process was successfully terminated and is gone."
}

# --- Test 3: Spawning process and killing by PID ---
Write-Host "Spawning another temporary background ping process..."
$PingProc2 = Start-Process -FilePath "ping" -ArgumentList "127.0.0.1", "-n", "30" -PassThru -WindowStyle Hidden
$PingPid2 = $PingProc2.Id
Write-Host "Spawned ping process with PID: $PingPid2"

Start-Sleep -Milliseconds 250

# Terminate process using ir kill by PID
Write-Host "Terminating process by PID using ir kill..."
& $Executable kill $PingPid2 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL: ir kill <PID> failed with exit code $LASTEXITCODE."
    Stop-Process -Id $PingPid2 -Force
    exit 1
}

# Verify process is terminated
Start-Sleep -Milliseconds 250
$FilteredPsAfter2 = & $Executable ps -f ping | Out-String
if ($FilteredPsAfter2 -like "*ping*") {
    Write-Host "FAIL: Process still exists after kill by PID. Output:"
    Write-Host $FilteredPsAfter2
    Stop-Process -Id $PingPid2 -Force
    exit 1
} else {
    Write-Host "PASS: Process was successfully terminated by PID."
}

Write-Host "ALL PS AND KILL TESTS PASSED"
exit 0
