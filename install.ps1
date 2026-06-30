# install.ps1
# Windows installer for ir-cli-utility

param (
    [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"

$repo = "indoctrinatedrecluse/ir-cli-utility"
if ($Version -eq "latest") {
    $downloadUrl = "https://github.com/$repo/releases/latest/download/ir-windows.zip"
} else {
    $downloadUrl = "https://github.com/$repo/releases/download/$Version/ir-windows.zip"
}

$installDir = Join-Path $env:APPDATA "ir"
Write-Host "Installing ir-cli-utility to: $installDir"

# Create install directory if it doesn't exist
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Download archive
$tempZip = Join-Path $env:TEMP "ir-windows.zip"
Write-Host "Downloading from $downloadUrl..."
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing
} catch {
    Write-Error "Failed to download release from $downloadUrl. Please verify the version exists."
    exit 1
}

# Extract archive
Write-Host "Extracting archive..."
Expand-Archive -Path $tempZip -DestinationPath $installDir -Force
Remove-Item $tempZip

# Add to User PATH if not present
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -split ';' -notcontains $installDir) {
    Write-Host "Adding $installDir to User PATH environment variable..."
    [Environment]::SetEnvironmentVariable("Path", $userPath + ";" + $installDir, "User")
    # Update current session Path
    $env:Path += ";$installDir"
}

Write-Host "`n🎉 ir-cli-utility has been successfully installed!" -ForegroundColor Green
Write-Host "Please restart your terminal or run '$env:Path += ';$installDir'' to start using 'ir'." -ForegroundColor Yellow
