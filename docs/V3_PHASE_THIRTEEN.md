# V3 Marketplace Phase 13 — QA Security, Load, and Performance

Phase 13 verifies the Marketplace paths built through Phase 12 without
restarting completed feature work. It adds P0 security QA coverage for the main
abuse paths and P1 performance controls for catalog search, listing detail, and
installation gates.

## 13.1 Marketplace security QA

The Phase 13 security suite covers these P0 paths:

| Attack path | Guard now verified |
| --- | --- |
| IDOR on creator listings/submissions | Creator-owned listing mutations and submission reads require `creator_id` or `creator.user_id` matches. |
| Permission bypass | Runtime authorization denies missing approved permissions, unknown operations, and kill-switched installations. |
| Malicious package | A package containing forbidden secret files or high-risk permissions is blocked before review publication. |
| Refund abuse | Stripe Marketplace refunds lock purchases, reject invalid amounts, ignore already-refunded purchases, deduplicate provider events, reverse ledger entries, and revoke full-refund entitlements. |
| Review abuse | Reviews require an installed or purchased product, are unique per organization/listing, reset to pending on edit, validate version/listing ownership, and keep abuse report evidence as a JSON object. |

The acceptance gate is the backend test target that includes
`marketplace_phase_thirteen`. No P0 security test may fail.

## 13.2 Load and performance

The baseline latency measurement focuses on:

- `GET /api/marketplace/catalog`
- `GET /api/marketplace/catalog?search=...`
- `GET /api/marketplace/catalog/{listing_slug}`
- optional `POST /api/marketplace/installations` only when explicitly enabled

Migration `0026_v3_phase_thirteen_marketplace_qa_performance.sql` adds
index tuning for catalog search, latest approved versions, active install
counts, paid entitlement checks, and existing checkout detection.

The catalog cache policy and listing detail responses now emit:

```text
Cache-Control: private, max-age=60, stale-while-revalidate=30
```

The policy is intentionally private because catalog compatibility depends on
the authenticated organization plan. Marketplace mutation paths are not cached.

Baseline script:

```powershell
$env:API_BASE_URL = "http://localhost:8080"
$env:ACCESS_TOKEN = "<tenant access token>"
$env:ORGANIZATION_ID = "<organization id>"
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase13-load-smoke.ps1 -Iterations 30
```

Run the latency budget against a release or production-like backend. Debug
builds on local Windows can add enough runtime overhead to make the absolute
budget noisy. The script disables local proxy use for `localhost` checks and
fails any sample outside the HTTP 2xx/3xx range.

Optional install latency sampling mutates state and must be explicitly enabled:

```powershell
$env:MARKETPLACE_INSTALL_BODY_JSON = '{"listing_id":"...","version_id":"...","approved_permissions":["page.read"]}'
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase13-load-smoke.ps1 -AllowInstallMutation
```

Default P95 budgets:

| Path class | P95 budget |
| --- | ---: |
| Catalog and search | 300 ms |
| Listing detail | 250 ms |
| Install mutation | 750 ms |

If local Docker/PostgreSQL/Redis hardware is slower than the target
environment, keep the produced P95 baseline as an environment-specific artifact
and investigate regressions against that baseline.

## Validation

Run the Phase 13 backend checks:

```powershell
cargo test --manifest-path backend/Cargo.toml marketplace_phase_thirteen
cargo test --manifest-path backend/Cargo.toml marketplace_performance
```

Broader backend validation:

```powershell
cargo test --manifest-path backend/Cargo.toml marketplace
```
