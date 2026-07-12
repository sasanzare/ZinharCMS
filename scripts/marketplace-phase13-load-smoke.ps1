param(
  [string]$ApiBaseUrl = $env:API_BASE_URL,
  [string]$AccessToken = $env:ACCESS_TOKEN,
  [string]$OrganizationId = $env:ORGANIZATION_ID,
  [int]$Iterations = 30,
  [int]$Warmup = 3,
  [string]$CatalogSearch = "demo",
  [string]$ListingSlug = $env:MARKETPLACE_LISTING_SLUG,
  [string]$InstallBodyJson = $env:MARKETPLACE_INSTALL_BODY_JSON,
  [switch]$AllowInstallMutation,
  [int]$CatalogBudgetMs = 300,
  [int]$ListingBudgetMs = 250,
  [int]$InstallBudgetMs = 750
)

Add-Type -AssemblyName System.Net.Http

if ([string]::IsNullOrWhiteSpace($ApiBaseUrl)) {
  $ApiBaseUrl = "http://localhost:8080"
}

if ([string]::IsNullOrWhiteSpace($AccessToken) -or [string]::IsNullOrWhiteSpace($OrganizationId)) {
  throw "ACCESS_TOKEN and ORGANIZATION_ID are required for Marketplace load smoke checks."
}

$handler = [System.Net.Http.HttpClientHandler]::new()
$handler.UseProxy = $false
$client = [System.Net.Http.HttpClient]::new($handler)
$client.BaseAddress = [uri]$ApiBaseUrl
$client.DefaultRequestHeaders.Authorization = [System.Net.Http.Headers.AuthenticationHeaderValue]::new("Bearer", $AccessToken)
$client.DefaultRequestHeaders.Add("X-Organization-Id", $OrganizationId)

$endpoints = New-Object System.Collections.Generic.List[object]
$endpoints.Add([pscustomobject]@{
  Name = "catalog"
  Method = "GET"
  Path = "/api/marketplace/catalog"
  Body = $null
  BudgetMs = $CatalogBudgetMs
})
$endpoints.Add([pscustomobject]@{
  Name = "catalog-search"
  Method = "GET"
  Path = "/api/marketplace/catalog?search=$([uri]::EscapeDataString($CatalogSearch))"
  Body = $null
  BudgetMs = $CatalogBudgetMs
})

if (-not [string]::IsNullOrWhiteSpace($ListingSlug)) {
  $endpoints.Add([pscustomobject]@{
    Name = "listing-detail"
    Method = "GET"
    Path = "/api/marketplace/catalog/$([uri]::EscapeDataString($ListingSlug))"
    Body = $null
    BudgetMs = $ListingBudgetMs
  })
}

if ($AllowInstallMutation -and -not [string]::IsNullOrWhiteSpace($InstallBodyJson)) {
  $endpoints.Add([pscustomobject]@{
    Name = "install"
    Method = "POST"
    Path = "/api/marketplace/installations"
    Body = $InstallBodyJson
    BudgetMs = $InstallBudgetMs
  })
}

function Invoke-EndpointSample {
  param(
    [object]$Endpoint
  )

  $watch = [System.Diagnostics.Stopwatch]::StartNew()
  $response = $null
  $request = [System.Net.Http.HttpRequestMessage]::new(
    [System.Net.Http.HttpMethod]::new($Endpoint.Method),
    $Endpoint.Path
  )

  try {
    if ($Endpoint.Body) {
      $request.Content = [System.Net.Http.StringContent]::new(
        $Endpoint.Body,
        [System.Text.Encoding]::UTF8,
        "application/json"
      )
    }

    $response = $client.SendAsync($request).GetAwaiter().GetResult()
    [void]$response.Content.ReadAsByteArrayAsync().GetAwaiter().GetResult()
    $status = [int]$response.StatusCode
  } catch {
    $status = if ($_.Exception.Response) { [int]$_.Exception.Response.StatusCode } else { 0 }
  } finally {
    if ($response) {
      $response.Dispose()
    }
    $request.Dispose()
    $watch.Stop()
  }

  [pscustomobject]@{
    Name = $Endpoint.Name
    Path = $Endpoint.Path
    Status = $status
    Ms = $watch.ElapsedMilliseconds
    BudgetMs = $Endpoint.BudgetMs
  }
}

$results = New-Object System.Collections.Generic.List[object]

foreach ($endpoint in $endpoints) {
  for ($i = 0; $i -lt $Warmup; $i++) {
    [void](Invoke-EndpointSample -Endpoint $endpoint)
  }

  for ($i = 0; $i -lt $Iterations; $i++) {
    $results.Add((Invoke-EndpointSample -Endpoint $endpoint))
  }
}

$summary = $results |
  Group-Object Name |
  ForEach-Object {
    $samples = $_.Group | Sort-Object Ms
    $p95Index = [Math]::Max(0, [Math]::Ceiling($samples.Count * 0.95) - 1)
    $p95 = $samples[$p95Index].Ms
    $budget = $samples[0].BudgetMs
    [pscustomobject]@{
      Name = $_.Name
      Requests = $samples.Count
      Statuses = (($samples | Group-Object Status | ForEach-Object { "$($_.Name):$($_.Count)" }) -join ", ")
      AvgMs = [math]::Round(($samples | Measure-Object Ms -Average).Average, 2)
      P95Ms = $p95
      MaxMs = ($samples | Measure-Object Ms -Maximum).Maximum
      BudgetMs = $budget
      Passed = ($p95 -le $budget) -and (($samples | Where-Object { $_.Status -lt 200 -or $_.Status -ge 400 }).Count -eq 0)
    }
  }

$summary | Format-Table -AutoSize

$client.Dispose()

if (($summary | Where-Object { -not $_.Passed }).Count -gt 0) {
  exit 1
}
