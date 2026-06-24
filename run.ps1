# This script builds and runs the ir-cli-utility for testing in PowerShell.

$ExecutablePath = ".\target\debug\ir.exe"

# Remove the old executable if it exists
if (Test-Path $ExecutablePath) {
    Write-Host "Removing previous executable..."
    Remove-Item $ExecutablePath
}

Write-Host "Building the project..."
cargo build

# Check if the last command (cargo build) was successful
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed. Halting script."
    # The script will stop here if an error occurs, but we add an explicit exit
    # for clarity and to handle cases where ErrorActionPreference might be different.
    exit 1
}

Write-Host "Build successful. Running the executable..."
& $ExecutablePath help
