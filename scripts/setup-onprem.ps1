param(
    [string]$EnvFile = ".env"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (-not (Test-Path $EnvFile)) {
    Copy-Item ".env.onprem.example" $EnvFile
    Write-Host "Created $EnvFile from .env.onprem.example"
}

$content = Get-Content $EnvFile -Raw
$required = @("MODE=onprem", "BOT_TOKEN=", "ADMIN_IDS=", "DATABASE_URL=", "LICENSE_KEY=")
foreach ($entry in $required) {
    if ($content -notmatch [regex]::Escape($entry)) {
        throw "Missing required setting marker: $entry"
    }
}

docker compose --env-file $EnvFile up -d --build

Write-Host ""
Write-Host "On-prem setup complete."
Write-Host "Verification:"
Write-Host "1) docker compose ps"
Write-Host "2) docker compose logs -f api bot"
Write-Host "3) Send /start to the bot"
