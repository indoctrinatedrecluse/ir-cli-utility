# Test: gitinfo help, alias, and validation checks

# --- Setup ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$Executable = Join-Path $RepoRoot "target\debug\ir.exe"

$Passed = $true
function Assert-Contains($Actual, $Expected, $Msg) {
    if ($Actual.Contains($Expected)) {
        Write-Host "PASS: $Msg"
    } else {
        Write-Host "FAIL: $Msg (Expected to contain '$Expected')"
        $global:Passed = $false
    }
}

# 1. Test invalid source path repository check
Write-Host "Testing invalid source path check..."
$TempEmptyDir = Join-Path $env:TEMP "ir_gitinfo_empty_$(Get-Random)"
$null = New-Item -ItemType Directory -Path $TempEmptyDir -Force

try {
    $ErrOut = & $Executable gitinfo --source $TempEmptyDir 2>&1 | Out-String
    Assert-Contains $ErrOut "Error: Not a git repository" "Validation reports invalid git repo directory correctly"
} finally {
    Remove-Item -Path $TempEmptyDir -Recurse -Force
}

# 2. Test help command output details for gitinfo
Write-Host "Testing gitinfo help page..."
$HelpOut = & $Executable help gitinfo | Out-String
Assert-Contains $HelpOut "Launches a full-screen interactive Git repository TUI dashboard." "Help prints gitinfo description"
Assert-Contains $HelpOut "History & Graph" "Help prints TUI tabs info"

# 3. Test help command output details for gin alias
Write-Host "Testing gin help page..."
$AliasHelpOut = & $Executable help gin | Out-String
Assert-Contains $AliasHelpOut "Launches a full-screen interactive Git repository TUI dashboard." "Help for gin routes correctly to gitinfo help"

if ($Passed) {
    Write-Host "`nALL GITINFO TESTS PASSED"
    exit 0
} else {
    Write-Host "`nSOME GITINFO TESTS FAILED"
    exit 1
}
