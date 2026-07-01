# V2 Operations Runbook

This runbook covers the General Availability release path, monitoring, common incidents, support response, and rollback.

## Release Freeze

Freeze these areas before GA:

- database schema changes
- tenant context and RLS logic
- authentication and account access
- billing and Stripe webhooks
- quota enforcement
- migration scripts
- release-critical frontend routing

Only low-risk documentation, copy, and clearly isolated fixes should be merged during the freeze window.

## Pre-Release Checklist

1. Confirm no `p0` or `p1` GA blockers are open.
2. Confirm staging migration completed from a production-like backup.
3. Confirm staging rollback was tested.
4. Run backend tests.
5. Run frontend lint and build.
6. Run `.\scripts\v2-ga-check.ps1`.
7. Confirm paid plans and Stripe webhook settings.
8. Confirm support has access to this runbook and the billing guide.

## Deployment Order

1. Take a database backup.
2. Deploy migrations.
3. Deploy backend.
4. Run `/health` and `/ready` checks.
5. Run authenticated smoke checks for organizations, billing, content types, pages, and media.
6. Deploy frontend.
7. Enable V2 for target users without an internal feature flag.
8. Enable paid plans.
9. Monitor the first release window.

## Monitoring

Watch these signals during and after release:

- `/health` and `/ready`
- backend error rate
- failed login rate
- account invitation failures
- billing webhook failures
- subscription state mismatches
- quota exceeded events
- usage counter rebuild failures
- RLS or tenant context errors
- audit logs for sensitive operations
- email delivery failures

## Incident Response

### Billing State Is Wrong

1. Pause manual plan changes.
2. Check the active subscription row.
3. Check billing events for duplicate, failed, or out-of-order events.
4. Refresh billing from Stripe if available.
5. Apply a temporary support override only when approved.
6. Record the incident and affected organization IDs.

### Account Access Fails

1. Confirm the user can authenticate.
2. Confirm the active organization is correct.
3. Confirm membership exists and is active.
4. Confirm the role has permission for the requested action.
5. Check invitation and email delivery logs.
6. Check CORS and tenant header behavior if the browser blocks requests.

### Tenant Isolation Concern

1. Treat the issue as high severity.
2. Stop access for the affected organization if exposure is active.
3. Preserve logs and request IDs.
4. Check organization context, membership, RLS policies, and bypass usage.
5. Confirm whether data crossed organization boundaries.
6. Escalate to the release owner before re-enabling access.

### Migration Failure

1. Stop the migration process.
2. Preserve logs and the migration version.
3. Identify the last successful migration.
4. Do not manually edit production data unless approved.
5. Restore from backup if data integrity is uncertain.
6. Re-run in staging with the same input before trying production again.

### Quota Counters Look Wrong

1. Rebuild usage counters.
2. Compare counters against live organization data.
3. Check plan limits and overrides.
4. Check recent imports, media uploads, or deleted content.
5. Keep a record if temporary relief is granted.

## Rollback Plan

Rollback is required if any of these occur:

- tenant isolation fails
- migration corrupts or loses production data
- account owners cannot access their organizations
- billing blocks valid customers and cannot be repaired quickly
- backend readiness fails after deployment
- frontend release blocks core admin workflows

Rollback steps:

1. Announce rollback to the release channel.
2. Disable new plan changes and risky write operations.
3. Restore the last known good backend and frontend versions.
4. Restore the database backup if migrations or imported data are unsafe.
5. Confirm `/health` and `/ready`.
6. Confirm owner login and organization access.
7. Confirm billing state is either restored or safely paused.
8. Document the cause, affected users, and follow-up fixes.

## Post-Release Review

Within one business day:

- summarize incidents and support tickets
- review billing and account access errors
- review usage counter rebuilds
- review audit logs for unexpected operations
- close or reclassify remaining beta blockers
- update this runbook if support needed missing instructions