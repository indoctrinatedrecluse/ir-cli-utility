# Test: Command Aliases on Windows
Write-Host "Building..."
cargo build --quiet
$Executable = ".\target\debug\ir.exe"

$TestFile = "temp_alias_touch.txt"
$TempDir = "temp_alias_dir"
$MovedFile = "temp_alias_moved.txt"

# Clean up
foreach ($file in @($TestFile, $MovedFile)) {
    if (Test-Path $file) { Remove-Item $file -Force }
}
if (Test-Path $TempDir) { Remove-Item $TempDir -Recurse -Force }

$Result = 0

Write-Host "Testing alias 'touch' -> maps to 'create'..."
& $Executable touch $TestFile

if (Test-Path $TestFile) {
    Write-Host "✅ PASS: 'ir touch' created the file successfully."
} else {
    Write-Host "❌ FAIL: 'ir touch' did not create the file."
    $Result = 1
}

Write-Host "Testing alias 'ls' -> maps to 'list'..."
$ls_out = & $Executable ls
if ($ls_out -match "temp_alias_touch") {
    Write-Host "✅ PASS: 'ir ls' executed successfully and listed files."
} else {
    Write-Host "❌ FAIL: 'ir ls' did not return expected list."
    $Result = 1
}

Write-Host "Testing alias 'tar' -> maps to 'archive'..."
# Run tar with incorrect/missing args to see if it triggers archive help
$tar_out = & $Executable tar 2>&1
if ($tar_out -match "ir-archive" -or $tar_out -match "Destination") {
    Write-Host "✅ PASS: 'ir tar' successfully routed to archive."
} else {
    Write-Host "❌ FAIL: 'ir tar' did not route to archive: $tar_out"
    $Result = 1
}

Write-Host "Testing alias 'cp' -> maps to 'copy'..."
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
& $Executable cp $TestFile $TempDir
if (Test-Path "$TempDir\$TestFile") {
    Write-Host "✅ PASS: 'ir cp' copied file successfully."
} else {
    Write-Host "❌ FAIL: 'ir cp' did not copy file."
    $Result = 1
}

Write-Host "Testing alias 'mv' -> maps to 'move'..."
& $Executable mv "$TempDir\$TestFile" $MovedFile
if ((Test-Path $MovedFile) -and (-not (Test-Path "$TempDir\$TestFile"))) {
    Write-Host "✅ PASS: 'ir mv' moved file successfully."
} else {
    Write-Host "❌ FAIL: 'ir mv' did not move file."
    $Result = 1
}

Write-Host "Testing alias 'rm' -> maps to 'remove'..."
& $Executable rm $MovedFile
if (-not (Test-Path $MovedFile)) {
    Write-Host "✅ PASS: 'ir rm' removed files successfully."
} else {
    Write-Host "❌ FAIL: 'ir rm' did not remove files."
    $Result = 1
}

Write-Host "Testing alias 'ff' -> maps to 'fastfetch'..."
$ff_out = & $Executable ff
$ff_str = [string]::Join("`n", $ff_out)
if ($ff_str -match "OS" -or $ff_str -match "Host") {
    Write-Host "✅ PASS: 'ir ff' successfully ran fastfetch."
} else {
    Write-Host "❌ FAIL: 'ir ff' did not run fastfetch properly."
    $Result = 1
}

Write-Host "Testing alias 'ncdu' -> maps to 'dua' (via help)..."
$ncdu_out = & $Executable help ncdu 2>&1 | Out-String
if ($ncdu_out -match "ir-dua") {
    Write-Host "✅ PASS: 'ir ncdu' alias correctly routes to dua."
} else {
    Write-Host "❌ FAIL: 'ir ncdu' did not route to dua: $ncdu_out"
    $Result = 1
}

Write-Host "Testing alias 'fm' -> maps to 'browse' (via help)..."
$fm_out = & $Executable help fm 2>&1 | Out-String
if ($fm_out -match "ir-browse") {
    Write-Host "✅ PASS: 'ir fm' alias correctly routes to browse."
} else {
    Write-Host "❌ FAIL: 'ir fm' did not route to browse: $fm_out"
    $Result = 1
}

Write-Host "Testing alias 'ed' -> maps to 'edit' (via help)..."
$ed_out = & $Executable help ed 2>&1 | Out-String
if ($ed_out -match "ir-edit") {
    Write-Host "✅ PASS: 'ir ed' alias correctly routes to edit."
} else {
    Write-Host "❌ FAIL: 'ir ed' did not route to edit: $ed_out"
    $Result = 1
}

Write-Host "Testing alias 'dl' -> maps to 'scrape' (via help)..."
$dl_out = & $Executable help dl 2>&1 | Out-String
if ($dl_out -match "ir-scrape") {
    Write-Host "✅ PASS: 'ir dl' alias correctly routes to scrape."
} else {
    Write-Host "❌ FAIL: 'ir dl' did not route to scrape: $dl_out"
    $Result = 1
}

Write-Host "Testing alias 'gin' -> maps to 'gitinfo' (via help)..."
$gin_out = & $Executable help gin 2>&1 | Out-String
if ($gin_out -match "ir-gitinfo") {
    Write-Host "✅ PASS: 'ir gin' alias correctly routes to gitinfo."
} else {
    Write-Host "❌ FAIL: 'ir gin' did not route to gitinfo: $gin_out"
    $Result = 1
}

Write-Host "Testing alias 'dbv' -> maps to 'dbview' (via help)..."
$dbv_out = & $Executable help dbv 2>&1 | Out-String
if ($dbv_out -match "ir-dbview") {
    Write-Host "✅ PASS: 'ir dbv' alias correctly routes to dbview."
} else {
    Write-Host "❌ FAIL: 'ir dbv' did not route to dbview: $dbv_out"
    $Result = 1
}

Write-Host "Testing alias 'req' -> maps to 'request' (via help)..."
$req_out = & $Executable help req 2>&1 | Out-String
if ($req_out -match "ir-request") {
    Write-Host "✅ PASS: 'ir req' alias correctly routes to request."
} else {
    Write-Host "❌ FAIL: 'ir req' did not route to request: $req_out"
    $Result = 1
}

Write-Host "Testing alias 'hexv' -> maps to 'hexview' (via help)..."
$hexv_out = & $Executable help hexv 2>&1 | Out-String
if ($hexv_out -match "ir-hexview") {
    Write-Host "✅ PASS: 'ir hexv' alias correctly routes to hexview."
} else {
    Write-Host "❌ FAIL: 'ir hexv' did not route to hexview: $hexv_out"
    $Result = 1
}

Write-Host "Testing alias 'sys' -> maps to 'sysinfo' (via help)..."
$sys_out = & $Executable help sys 2>&1 | Out-String
if ($sys_out -match "ir-sysinfo") {
    Write-Host "✅ PASS: 'ir sys' alias correctly routes to sysinfo."
} else {
    Write-Host "❌ FAIL: 'ir sys' did not route to sysinfo: $sys_out"
    $Result = 1
}

# Clean up
foreach ($file in @($TestFile, $MovedFile)) {
    if (Test-Path $file) { Remove-Item $file -Force }
}
if (Test-Path $TempDir) { Remove-Item $TempDir -Recurse -Force }

exit $Result

