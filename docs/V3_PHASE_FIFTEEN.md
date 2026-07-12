# V3 Marketplace Phase 15 — Launch Readiness and General Availability

Phase 15 is the controlled public-launch gate for the Marketplace already built
through Phase 14. It does not add a new schema, feature flag, payment flow, or
package execution runtime. It packages the existing Marketplace APIs, analytics,
QA gates, beta evidence, and operational controls into a launch-ready process.

Uploaded Marketplace package code remains unexecuted.

## 15.1 Launch Readiness

Goal: prepare for a controlled public launch with repeatable operational
handling.

Required outputs:

- runbook: `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md`;
- final policy: `docs/V3_MARKETPLACE_POLICY.md` with the Phase 15 launch policy;
- support workflow for creator, customer, billing, and abuse issues;
- rollback plan for broken install, unsafe update, payment defect, and frontend
  or backend release failure;
- incident checklist for malicious product, broken install, wrong payment,
  refund/dispute, payout, report abuse, and emergency block.

Acceptance gate:

- the team can manage a broken install without direct database edits;
- the team can manage a malicious product through report intake, moderation,
  emergency block, and takedown policy;
- the team can manage a wrong payment with purchase, refund, entitlement, ledger,
  and support triage steps;
- P0/P1 beta blockers and Marketplace incident items have owners before launch;
- rollback and communication owners are named before the launch window opens.

## 15.2 General Availability

Goal: enable Marketplace for target users after the launch readiness gate passes.

Required outputs:

- release notes: `docs/V3_MARKETPLACE_RELEASE_NOTES.md`;
- public docs for creators, customers, install permissions, reports, and support;
- monitoring dashboard using `/api/marketplace/analytics/admin`, `/health`, and
  `/ready`;
- support plan covering the first GA monitoring window and escalation policy.

Acceptance gate:

- Marketplace is enabled for installing approved products in production;
- catalog, listing detail, install, update, rollback, purchase, review, report,
  creator analytics, and admin analytics routes are available according to the
  existing route boundaries;
- Phase 14 beta readiness evidence has been reviewed before GA sign-off;
- `scripts/marketplace-phase15-ga-check.ps1` passes or produces an explicit
  `-ReportOnly` exception list with owners;
- public documentation points users to support and report-abuse paths.

## Operational boundaries

- No arbitrary uploaded code execution is enabled.
- No automated payout transfer execution is enabled.
- Partial refunds remain outside the supported runtime contract.
- External notification delivery remains deferred; internal notification and
  report queues are the operational source of truth.
- Marketplace GA is a controlled enablement over reviewed/approved products, not
  an approval bypass.

## Validation

Run the Phase 15 backend contract test:

```powershell
cargo test --manifest-path backend/Cargo.toml marketplace_phase_fifteen
```

Run the Marketplace regression suite:

```powershell
cargo test --manifest-path backend/Cargo.toml marketplace
```

Run the GA readiness script for a local/static report:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase15-ga-check.ps1 -ReportOnly -SkipFrontendBuild
```

Run the script against a live staging or production candidate when tokens and
organization context are available:

```powershell
$env:API_BASE_URL = "https://example.com"
$env:ACCESS_TOKEN = "<tenant access token>"
$env:ORGANIZATION_ID = "<organization id>"
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase15-ga-check.ps1 -AdminMode
```
