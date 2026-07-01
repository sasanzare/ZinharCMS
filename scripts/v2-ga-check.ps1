param(
    [switch]$SkipBackendTests,
    [switch]$SkipFrontendBuild,
    [string]$ApiBaseUrl = $env:API_BASE_URL
)

$ErrorActionPreference = "Stop"

function Resolve-NpmCommand {
    $windowsNpm = "C:\Program Files\nodejs\npm.cmd"
    if (Test-Path -LiteralPath $windowsNpm) {
        return $windowsNpm
    }

    $npmCommand = Get-Command npm.cmd -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($npmCommand) {
        return $npmCommand.Source
    }

    $npmCommand = Get-Command npm -ErrorAction Stop | Select-Object -First 1
    return $npmCommand.Source
}

function Invoke-GaStep {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [scriptblock]$Command
    )

    Write-Host "[ga-check] $Name"
    $global:LASTEXITCODE = 0
    & $Command
    $exitCode = $global:LASTEXITCODE
    if ($exitCode -ne 0) {
        throw "GA check step failed: $Name exited with code $exitCode"
    }
}

$NpmCommand = Resolve-NpmCommand

if (-not $SkipBackendTests) {
    Invoke-GaStep "Backend tests" {
        cargo test --manifest-path backend/Cargo.toml --all-features
    }
}

Invoke-GaStep "Frontend lint" {
    & $NpmCommand --prefix frontend run lint
}

if (-not $SkipFrontendBuild) {
    Invoke-GaStep "Frontend build" {
        & $NpmCommand --prefix frontend run build
    }
}

if ($ApiBaseUrl) {
    Invoke-GaStep "Health endpoint" {
        Invoke-RestMethod -Method Get -Uri "$ApiBaseUrl/health" | Out-Null
    }

    Invoke-GaStep "Readiness endpoint" {
        Invoke-RestMethod -Method Get -Uri "$ApiBaseUrl/ready" | Out-Null
    }
}

Write-Host "[ga-check] V2 GA checks completed"