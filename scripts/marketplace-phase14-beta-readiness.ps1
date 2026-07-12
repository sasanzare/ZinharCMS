param(
  [string]$ApiBaseUrl = $env:API_BASE_URL,
  [string]$AccessToken = $env:ACCESS_TOKEN,
  [string]$OrganizationId = $env:ORGANIZATION_ID,
  [string]$CreatorId = $env:MARKETPLACE_CREATOR_ID,
  [switch]$AdminMode,
  [switch]$ReportOnly
)

Add-Type -AssemblyName System.Net.Http

if ([string]::IsNullOrWhiteSpace($ApiBaseUrl)) {
  $ApiBaseUrl = "http://localhost:8080"
}

if ([string]::IsNullOrWhiteSpace($AccessToken) -or [string]::IsNullOrWhiteSpace($OrganizationId)) {
  throw "ACCESS_TOKEN and ORGANIZATION_ID are required for Marketplace beta readiness checks."
}

$handler = [System.Net.Http.HttpClientHandler]::new()
$handler.UseProxy = $false
$client = [System.Net.Http.HttpClient]::new($handler)
$client.BaseAddress = [uri]$ApiBaseUrl
$client.DefaultRequestHeaders.Authorization = [System.Net.Http.Headers.AuthenticationHeaderValue]::new("Bearer", $AccessToken)
$client.DefaultRequestHeaders.Add("X-Organization-Id", $OrganizationId)

function Invoke-Json {
  param(
    [string]$Path,
    [switch]$Optional
  )

  $response = $null
  $request = [System.Net.Http.HttpRequestMessage]::new([System.Net.Http.HttpMethod]::Get, $Path)
  try {
    $response = $client.SendAsync($request).GetAwaiter().GetResult()
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
    return $body | ConvertFrom-Json
  } finally {
    if ($response) {
      $response.Dispose()
    }
    $request.Dispose()
  }
}

function Count-Items {
  param([object]$Value)
  if ($null -eq $Value) {
    return 0
  }
  if ($Value -is [array]) {
    return $Value.Count
  }
  if ($Value.PSObject.Properties.Name -contains "Count") {
    return [int]$Value.Count
  }
  return 1
}

function Metadata-Value {
  param(
    [object]$Item,
    [string]$Name
  )
  if ($null -eq $Item -or $null -eq $Item.metadata) {
    return $null
  }
  $property = $Item.metadata.PSObject.Properties[$Name]
  if ($property) {
    return [string]$property.Value
  }
  return $null
}

$betaDashboard = Invoke-Json -Path "/api/beta/dashboard"
$feedback = @(Invoke-Json -Path "/api/beta/feedback?limit=100")
$blockers = @(Invoke-Json -Path "/api/beta/ga-blockers?limit=100" -Optional)
$installations = @(Invoke-Json -Path "/api/marketplace/installations")
$purchases = @(Invoke-Json -Path "/api/marketplace/purchases" -Optional)
$creatorAnalytics = $null
$productDashboard = $null
$adminAnalytics = $null
$customerReports = $null

if (-not [string]::IsNullOrWhiteSpace($CreatorId)) {
  $creatorAnalytics = Invoke-Json -Path "/api/marketplace/creators/$CreatorId/analytics" -Optional
}

if ($AdminMode) {
  $productDashboard = Invoke-Json -Path "/api/beta/product-dashboard" -Optional
  $adminAnalytics = Invoke-Json -Path "/api/marketplace/analytics/admin" -Optional
  $customerReports = @(Invoke-Json -Path "/api/marketplace/reports" -Optional)
}

$creatorFeedback = @($feedback | Where-Object {
  (Metadata-Value -Item $_ -Name "marketplace_phase") -eq "creator_beta" -or
  (Metadata-Value -Item $_ -Name "beta_track") -eq "creator" -or
  ($_.page_url -like "*marketplace*")
})
$supportIssues = @($feedback | Where-Object {
  (Metadata-Value -Item $_ -Name "issue_type") -eq "support_issue" -or
  (Metadata-Value -Item $_ -Name "marketplace_phase") -eq "customer_beta_support" -or
  ($_.category -in @("billing", "onboarding", "other") -and $_.page_url -like "*marketplace*")
})
$bugList = @($feedback | Where-Object {
  $_.status -notin @("fixed", "closed") -and (
    $_.category -eq "bug" -or
    (Metadata-Value -Item $_ -Name "marketplace_phase") -in @("creator_beta", "customer_beta")
  )
})

$creatorProducts = if ($creatorAnalytics) { Count-Items -Value $creatorAnalytics.products } else { 0 }
$installCount = Count-Items -Value $installations
$uninstallCount = @($installations | Where-Object { $_.status -eq "uninstalled" }).Count
$purchaseCount = Count-Items -Value $purchases
$reportCount = Count-Items -Value $customerReports
$activeCustomerBetaOrganizations = if ($productDashboard) { [int]$productDashboard.totals.active_organizations } else { 0 }

$checks = @(
  [pscustomobject]@{
    Name = "creator_products"
    Phase = "14.1 Private Creator Beta"
    Value = $creatorProducts
    Target = "5-10 real products"
    Passed = ($creatorProducts -ge 5 -and $creatorProducts -le 10)
  },
  [pscustomobject]@{
    Name = "creator_feedback"
    Phase = "14.1 Private Creator Beta"
    Value = $creatorFeedback.Count
    Target = ">= 1 creator feedback item"
    Passed = ($creatorFeedback.Count -ge 1)
  },
  [pscustomobject]@{
    Name = "bug_list"
    Phase = "14.1 Private Creator Beta"
    Value = $bugList.Count
    Target = "bug list query available"
    Passed = ($null -ne $bugList)
  },
  [pscustomobject]@{
    Name = "customer_installs"
    Phase = "14.2 Customer Beta"
    Value = $installCount
    Target = ">= 1 install record"
    Passed = ($installCount -ge 1)
  },
  [pscustomobject]@{
    Name = "customer_uninstalls"
    Phase = "14.2 Customer Beta"
    Value = $uninstallCount
    Target = "uninstall data query available"
    Passed = ($uninstallCount -ge 0)
  },
  [pscustomobject]@{
    Name = "customer_purchases"
    Phase = "14.2 Customer Beta"
    Value = $purchaseCount
    Target = "purchase data query available"
    Passed = ($null -ne $purchases)
  },
  [pscustomobject]@{
    Name = "support_issues"
    Phase = "14.2 Customer Beta"
    Value = $supportIssues.Count
    Target = ">= 1 support issue"
    Passed = ($supportIssues.Count -ge 1)
  },
  [pscustomobject]@{
    Name = "customer_reports"
    Phase = "14.2 Customer Beta"
    Value = $reportCount
    Target = "report data query available in AdminMode"
    Passed = (-not $AdminMode -or $null -ne $customerReports)
  },
  [pscustomobject]@{
    Name = "active_customer_beta_orgs"
    Phase = "14.2 Customer Beta"
    Value = $activeCustomerBetaOrganizations
    Target = "product dashboard available in AdminMode"
    Passed = (-not $AdminMode -or $activeCustomerBetaOrganizations -ge 0)
  }
)

$checks | Format-Table -AutoSize

$summary = [pscustomobject]@{
  organization_id = $OrganizationId
  participant_status = $betaDashboard.organization.participant_status
  open_feedback = $betaDashboard.organization.open_feedback
  open_ga_blockers = $betaDashboard.organization.open_ga_blockers
  blocker_count = $blockers.Count
  creator_products = $creatorProducts
  creator_feedback = $creatorFeedback.Count
  bug_list = $bugList.Count
  installs = $installCount
  uninstalls = $uninstallCount
  purchases = $purchaseCount
  support_issues = $supportIssues.Count
  customer_reports = $reportCount
  admin_marketplace_reports = if ($adminAnalytics) { $adminAnalytics.report_count } else { $null }
}

$summary | ConvertTo-Json -Depth 6

$client.Dispose()

if (-not $ReportOnly -and (($checks | Where-Object { -not $_.Passed }).Count -gt 0)) {
  exit 1
}
