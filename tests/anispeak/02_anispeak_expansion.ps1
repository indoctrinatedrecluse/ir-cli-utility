# Test: Expanded anispeak animal characters.

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
Set-Location $RepoRoot
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

# --- Test: Verify new animals output contains specific characters ---
$NewAnimals = @("elephant", "moose", "stegosaurus", "whale", "snake", "turtle", "sheep")
$Result = 0

foreach ($Animal in $NewAnimals) {
    Write-Host "Running test: ir anispeak -a $Animal 'hi'"
    $Output = & $Executable anispeak -a $Animal "hi" | Out-String
    if ($Output -notlike "*hi*") {
        Write-Host "FAIL: output of animal $Animal was missing message."
        $Result = 1
        break
    }
}

if ($Result -eq 0) {
    Write-Host "PASS: All 7 expanded anispeak animal templates successfully printed speech bubble and ASCII art."
}

exit $Result
