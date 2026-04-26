param(
    [string]$EnvFile = ".env"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (Test-Path $EnvFile) {
    Get-Content $EnvFile | ForEach-Object {
        if ($_ -match "^\s*#" -or $_ -match "^\s*$") { return }
        $key, $value = $_ -split "=", 2
        if ($key) {
            [System.Environment]::SetEnvironmentVariable($key.Trim(), $value.Trim())
        }
    }
}

cargo run --bin nailbot-api
