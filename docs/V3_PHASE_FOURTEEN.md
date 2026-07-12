# V3 Marketplace Phase 14 — Private Creator Beta and Customer Beta

Phase 14 is an operational beta gate over the Marketplace features already built
through Phase 13. It does not introduce a separate beta API or execute uploaded
package code. The beta evidence uses the existing V2 beta cohort/feedback/GA
blocker records plus existing Marketplace creator analytics, admin analytics,
installations, purchases, reviews, abuse reports, and support feedback.

## 14.1 Private Creator Beta

Goal: exercise the creator publishing cycle with a small trusted group before a
broader customer rollout.

Required evidence:

- 5 to 10 real products prepared by selected creators;
- creator feedback captured through `/api/beta/feedback`;
- a bug list tracked through `/api/beta/feedback` and `/api/beta/ga-blockers`;
- creator-owned product telemetry from
  `/api/marketplace/creators/{creator_id}/analytics`;
- Marketplace admin health/risk telemetry from `/api/marketplace/analytics/admin`
  when a global admin token is available.

Recommended feedback metadata for creator beta records:

```json
{
  "marketplace_phase": "creator_beta",
  "beta_track": "creator",
  "listing_id": "<optional listing id>",
  "package_version": "<optional package version>"
}
```

Acceptance gate:

- creator product count is between 5 and 10 for the selected beta creator or
  creator cohort;
- at least one creator feedback item is captured;
- open bugs and blockers are visible to the product/support team;
- no P0/P1 security, payment, install, review, or package-validation blocker is
  left without an owner.

## 14.2 Customer Beta

Goal: validate install and purchase behavior with real organizations before
launch readiness.

Required evidence:

- real install records from `/api/marketplace/installations`;
- uninstall data from installation lifecycle status;
- purchase and receipt data from `/api/marketplace/purchases`;
- support issue feedback captured through `/api/beta/feedback`;
- abuse/report data from `/api/marketplace/reports` when a global admin token is
  available;
- cross-organization beta cohort status from `/api/beta/product-dashboard` when
  a global admin token is available.

Recommended feedback metadata for customer beta support records:

```json
{
  "marketplace_phase": "customer_beta",
  "issue_type": "support_issue",
  "listing_id": "<optional listing id>",
  "installation_id": "<optional installation id>",
  "purchase_id": "<optional purchase id>"
}
```

Acceptance gate:

- customer beta organizations are enrolled through the existing beta participant
  workflow;
- at least one real install path is recorded before sign-off;
- purchase data is queryable for organizations that test paid products;
- support issues and Marketplace reports are visible in the beta evidence;
- unresolved P0/P1 beta blockers stop launch readiness.

## Readiness script

Use the Phase 14 script after seeding real beta data or against a staging
environment with real beta participants:

```powershell
$env:API_BASE_URL = "http://localhost:8080"
$env:ACCESS_TOKEN = "<tenant access token>"
$env:ORGANIZATION_ID = "<organization id>"
$env:MARKETPLACE_CREATOR_ID = "<creator id>"
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase14-beta-readiness.ps1 -AdminMode
```

For a non-blocking evidence report while a beta cohort is still being assembled:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase14-beta-readiness.ps1 -ReportOnly
```

The script is read-only. It calls existing beta and Marketplace APIs, summarizes
creator products, creator feedback, bug list, installs, uninstalls, purchases,
support issues, customer reports, and beta cohort status, then exits non-zero
unless readiness gates pass or `-ReportOnly` is set.

## Validation

Run the Phase 14 backend contract test:

```powershell
cargo test --manifest-path backend/Cargo.toml marketplace_phase_fourteen
```

Broader Marketplace regression:

```powershell
cargo test --manifest-path backend/Cargo.toml marketplace
```
