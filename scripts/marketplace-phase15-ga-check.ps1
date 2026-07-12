param(
  [switch]$SkipBackendTests,
  [switch]$SkipFrontendBuild,
  [switch]$SkipApiSmoke,
  [switch]$AdminMode,
  [switch]$ReportOnly,
  [string]$ApiBaseUrl = $env:API_BASE_URL,
  [string]$AccessToken = $env:ACCESS_TOKEN,
  [string]$OrganizationId = $env:ORGANIZATION_ID
)

$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.Net.Http

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

$results = [System.Collections.Generic.List[object]]::new()

function Add-Result {
  param(
    [string]$Name,
    [string]$Phase,
    [bool]$Passed,
    [string]$Detail
  )

  $results.Add([pscustomobject]@{
    Name = $Name
    Phase = $Phase
    Passed = $Passed
    Detail = $Detail
  }) | Out-Null
}

function Invoke-GaStep {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Name,
    [Parameter(Mandatory = $true)]
    [string]$Phase,
    [Parameter(Mandatory = $true)]
    [scriptblock]$Command
  )

  Write-Host "[marketplace-ga-check] $Name"
  $global:LASTEXITCODE = 0
  try {
    & $Command
    $exitCode = $global:LASTEXITCODE
    if ($exitCode -ne 0) {
      throw "$Name exited with code $exitCode"
    }
    Add-Result -Name $Name -Phase $Phase -Passed $true -Detail "passed"
  } catch {
    Add-Result -Name $Name -Phase $Phase -Passed $false -Detail $_.Exception.Message
    if (-not $ReportOnly) {
      throw
    }
    Write-Warning $_.Exception.Message
  }
}

function New-GaHttpClient {
  $handler = [System.Net.Http.HttpClientHandler]::new()
  $handler.UseProxy = $false
  $client = [System.Net.Http.HttpClient]::new($handler)
  $client.BaseAddress = [uri]$ApiBaseUrl
  if (-not [string]::IsNullOrWhiteSpace($AccessToken)) {
    $client.DefaultRequestHeaders.Authorization = [System.Net.Http.Headers.AuthenticationHeaderValue]::new("Bearer", $AccessToken)
  }
  if (-not [string]::IsNullOrWhiteSpace($OrganizationId)) {
    $client.DefaultRequestHeaders.Add("X-Organization-Id", $OrganizationId)
  }
  return $client
}

function Invoke-Json {
  param(
    [System.Net.Http.HttpClient]$Client,
    [string]$Path,
    [switch]$Optional
  )

  $response = $null
  $request = [System.Net.Http.HttpRequestMessage]::new([System.Net.Http.HttpMethod]::Get, $Path)
  try {
    $response = $Client.SendAsync($request).GetAwaiter().GetResult()
    $body = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    $status = [int]$response.StatusCode
    if ($status -lt 200 -or $status -ge 400) {
      if ($Optional) {
        return $null
      }
      throw "GET $Path returned HTTP $status"
    }
    if ([string]::IsNullOrWhiteSpace($body)) {
      return $null
    }
    return ($body | ConvertFrom-Json)
  } finally {
    if ($response) {
      $response.Dispose()
    }
    $request.Dispose()
  }
}

$npmCommand = Resolve-NpmCommand

if (-not $SkipBackendTests) {
  Invoke-GaStep -Name "Phase 15 backend contract tests" -Phase "15.1 Launch Readiness" -Command {
    cargo test --manifest-path backend/Cargo.toml marketplace_phase_fifteen
  }

  Invoke-GaStep -Name "Marketplace backend regression" -Phase "15.2 General Availability" -Command {
    cargo test --manifest-path backend/Cargo.toml marketplace
  }
}

Invoke-GaStep -Name "Frontend lint" -Phase "15.2 General Availability" -Command {
  & $npmCommand --prefix frontend run lint
}

if (-not $SkipFrontendBuild) {
  Invoke-GaStep -Name "Frontend build" -Phase "15.2 General Availability" -Command {
    & $npmCommand --prefix frontend run build
  }
}

if (-not $SkipApiSmoke -and -not [string]::IsNullOrWhiteSpace($ApiBaseUrl)) {
  $client = New-GaHttpClient
  try {
    Invoke-GaStep -Name "Health endpoint" -Phase "15.1 Launch Readiness" -Command {
      Invoke-Json -Client $client -Path "/health" | Out-Null
    }

    Invoke-GaStep -Name "Readiness endpoint" -Phase "15.1 Launch Readiness" -Command {
      Invoke-Json -Client $client -Path "/ready" | Out-Null
    }

    if (-not [string]::IsNullOrWhiteSpace($AccessToken) -and -not [string]::IsNullOrWhiteSpace($OrganizationId)) {
      Invoke-GaStep -Name "Approved product install surface" -Phase "15.2 General Availability" -Command {
        Invoke-Json -Client $client -Path "/api/marketplace/installations" | Out-Null
      }

      Invoke-GaStep -Name "Purchase support surface" -Phase "15.1 Launch Readiness" -Command {
        Invoke-Json -Client $client -Path "/api/marketplace/purchases" -Optional | Out-Null
      }

      Invoke-GaStep -Name "Beta blocker surface" -Phase "15.1 Launch Readiness" -Command {
        Invoke-Json -Client $client -Path "/api/beta/ga-blockers?limit=100" -Optional | Out-Null
      }
    }

    if ($AdminMode) {
      Invoke-GaStep -Name "Monitoring dashboard" -Phase "15.2 General Availability" -Command {
        Invoke-Json -Client $client -Path "/api/marketplace/analytics/admin" | Out-Null
      }

      Invoke-GaStep -Name "Marketplace reports queue" -Phase "15.1 Launch Readiness" -Command {
        Invoke-Json -Client $client -Path "/api/marketplace/reports" | Out-Null
      }
    }
  } finally {
    $client.Dispose()
  }
}

$results | Format-Table -AutoSize

$summary = [pscustomobject]@{
  phase = "V3 Marketplace Phase 15"
  launch_readiness = ($results | Where-Object { $_.Phase -eq "15.1 Launch Readiness" -and -not $_.Passed }).Count -eq 0
  general_availability = ($results | Where-Object { $_.Phase -eq "15.2 General Availability" -and -not $_.Passed }).Count -eq 0
  failed_checks = @($results | Where-Object { -not $_.Passed } | Select-Object Name, Phase, Detail)
}

$summary | ConvertTo-Json -Depth 6

if (-not $ReportOnly -and $summary.failed_checks.Count -gt 0) {
  exit 1
}
