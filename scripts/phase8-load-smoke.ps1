param(
  [string]$ApiBaseUrl = $env:API_BASE_URL,
  [string]$AccessToken = $env:ACCESS_TOKEN,
  [string]$OrganizationId = $env:ORGANIZATION_ID,
  [int]$Iterations = 20
)

if ([string]::IsNullOrWhiteSpace($ApiBaseUrl)) {
  $ApiBaseUrl = "http://localhost:8080"
}

$endpoints = @(
  @{ Method = "GET"; Path = "/health"; Tenant = $false },
  @{ Method = "GET"; Path = "/ready"; Tenant = $false },
  @{ Method = "GET"; Path = "/api/billing/plans"; Tenant = $true },
  @{ Method = "GET"; Path = "/api/organizations/current"; Tenant = $true },
  @{ Method = "GET"; Path = "/api/billing/usage"; Tenant = $true },
  @{ Method = "GET"; Path = "/api/content-types"; Tenant = $true },
  @{ Method = "GET"; Path = "/api/pages"; Tenant = $true },
  @{ Method = "GET"; Path = "/api/media"; Tenant = $true }
)

$results = New-Object System.Collections.Generic.List[object]

foreach ($endpoint in $endpoints) {
  if ($endpoint.Tenant -and ([string]::IsNullOrWhiteSpace($AccessToken) -or [string]::IsNullOrWhiteSpace($OrganizationId))) {
    Write-Host "Skipping tenant endpoint $($endpoint.Path): ACCESS_TOKEN and ORGANIZATION_ID are required."
    continue
  }

  for ($i = 0; $i -lt $Iterations; $i++) {
    $headers = @{}
    if ($endpoint.Tenant) {
      $headers["Authorization"] = "Bearer $AccessToken"
      $headers["X-Organization-Id"] = $OrganizationId
    }

    $watch = [System.Diagnostics.Stopwatch]::StartNew()
    try {
      $response = Invoke-WebRequest -Method $endpoint.Method -Uri "$ApiBaseUrl$($endpoint.Path)" -Headers $headers -UseBasicParsing
      $status = [int]$response.StatusCode
    } catch {
      $status = if ($_.Exception.Response) { [int]$_.Exception.Response.StatusCode } else { 0 }
    } finally {
      $watch.Stop()
    }

    $results.Add([pscustomobject]@{
      Path = $endpoint.Path
      Status = $status
      Ms = $watch.ElapsedMilliseconds
    })
  }
}

$results |
  Group-Object Path |
  ForEach-Object {
    $samples = $_.Group
    [pscustomobject]@{
      Path = $_.Name
      Requests = $samples.Count
      Statuses = (($samples | Group-Object Status | ForEach-Object { "$($_.Name):$($_.Count)" }) -join ", ")
      AvgMs = [math]::Round(($samples | Measure-Object Ms -Average).Average, 2)
      MaxMs = ($samples | Measure-Object Ms -Maximum).Maximum
    }
  } |
  Format-Table -AutoSize
