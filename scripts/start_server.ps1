param(
    [string]$Executable = "$PSScriptRoot\mininventar-cli.exe"
)

if (-not (Test-Path -Path $Executable)) {
    Write-Error "Unable to locate mininventar-cli.exe next to this script."
    exit 1
}

Write-Host "Starting Minventar server..."
& $Executable start
