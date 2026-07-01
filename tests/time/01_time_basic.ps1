# Test: time action

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

Write-Host "Testing execution timing of cmd.exe /c exit 42..."
# We run command and redirect stderr to check for timing output
$ErrFile = "temp_time_err.txt"
& $Executable time cmd.exe /c exit 42 2>$ErrFile
$ExitCode = $LASTEXITCODE

$ErrText = Get-Content -Raw $ErrFile
Remove-Item -Force $ErrFile

Write-Host "Exit code returned: $ExitCode"
Write-Host "Stderr output: $ErrText"

if ($ExitCode -eq 42 -and $ErrText -match "Execution Time:") {
    Write-Host "PASS: Correct exit code and timing output printed to stderr."
} else {
    Write-Host "FAIL: Expected exit code 42 and Execution Time message in stderr."
    exit 1
}

exit 0
