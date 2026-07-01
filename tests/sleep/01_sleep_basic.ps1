# Test: sleep action

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# Measure sleep time
Write-Host "Testing sleep 200ms..."
$Start = [System.DateTime]::Now
& $Executable sleep 200ms
$Elapsed = ([System.DateTime]::Now - $Start).TotalMilliseconds

if ($Elapsed -ge 180 -and $Elapsed -le 500) {
    Write-Host "PASS: Slept for $Elapsed ms (expected ~200ms)."
} else {
    Write-Host "FAIL: Sleep duration out of range: $Elapsed ms"
    exit 1
}

exit 0
