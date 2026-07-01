# V2 Phase 10 General Availability

Phase 10 promotes V2 from limited beta to General Availability. The goal is to make the multi-tenant SaaS build stable for target users without relying on an internal feature flag, while giving engineering, product, and support enough operational documentation to handle migration, billing, account access, monitoring, and rollback incidents.

## Delivered Scope

- Added the V2 GA release notes.
- Added the V1-to-V2 migration guide.
- Added the organization administration guide.
- Added the short billing and quota guide.
- Added the operational runbook for launch, monitoring, incidents, and rollback.
- Added a support and rollback plan for common billing and account access failures.
- Added a GA verification script for release candidates.
- Added static backend tests that assert the GA documentation and release checklist stay present.

## GA Release Policy

Before publishing a GA release:

1. Freeze high-risk changes in schema, billing, authentication, tenant context, and RLS behavior.
2. Run final migration on staging from a production-like backup and confirm the same migration sequence is ready for production.
3. Confirm no `p0` or `p1` GA blocker remains open in the beta dashboard.
4. Confirm paid plans are configured and visible to target users.
5. Confirm billing and account access support is ready, including quota, invitation, and organization membership response paths.
6. Run the GA verification script from the repository root:

```powershell
.\scripts\v2-ga-check.ps1
```

## Release Sequence

1. Create a database backup.
2. Deploy backend migrations.
3. Deploy the backend application.
4. Run readiness and smoke checks.
5. Deploy the frontend application.
6. Enable V2 for target organizations without an internal feature flag.
7. Enable paid plans and verify Stripe webhooks.
8. Run post-release monitoring for health, billing events, email delivery, quota counters, and audit logs.
9. Keep the rollback owner available through the first post-release monitoring window.

## Acceptance Criteria

- V2 is enabled for target users without an internal feature flag.
- Core documentation is ready:
  - release notes
  - migration guide
  - admin guide
  - billing guide
  - operational runbook
  - support and rollback plan
- The team can manage common billing, account access, migration, quota, and tenant isolation incidents from the runbook.
- The GA verification script passes for the release candidate.
- Staging migration and rollback drills have been completed before production rollout.

## Verification

Recommended local checks:

```powershell
cargo test --manifest-path backend\Cargo.toml --all-features
npm --prefix frontend run lint
npm --prefix frontend run build
.\scripts\v2-ga-check.ps1
```

If the frontend build reports a chunk-size warning, treat it as an optimization task unless it is paired with a build failure.