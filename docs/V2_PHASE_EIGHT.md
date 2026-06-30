# V2 Phase 8 Security QA and Hardening

Phase 8 prepares the V2 beta for security review, tenant isolation checks, billing resilience checks, and production readiness validation.

## Implemented Hardening

- Added additive migration `0013_v2_phase_eight_hardening.sql` to persist Stripe provider event timestamps on subscriptions and billing events.
- Hardened Stripe webhook handling against duplicate events and older out-of-order subscription events.
- Added static RLS coverage tests for tenant-owned tables introduced across V2 phases.
- Added a billing permission helper and permission matrix test for organization roles.
- Added quota downgrade tests to confirm exceeded usage is reported without negative remaining quota.
- Extracted security header application for direct test coverage.
- Added a staging fixture with two similar organizations for direct-ID tenant isolation testing.
- Added a PowerShell load smoke script for common public and tenant-scoped endpoints.

## Automated Checks

Run backend checks from the repository root:

```powershell
cargo test --manifest-path backend/Cargo.toml --all-features
```

The phase 8 test coverage includes:

- RLS migration coverage for tenant-scoped tables.
- Tenant context helper usage in RLS policies.
- Additive/idempotent migration checks for phase 8.
- Public and tenant endpoint coverage in the load smoke manifest.
- Organization permission matrix checks.
- Quota downgrade metric behavior.
- Security header assertions.
- Stripe webhook timestamp parsing and provider event ordering checks.

## Staging Fixture

Apply the fixture only to local or staging databases:

```powershell
psql "$env:DATABASE_URL" -f docs/V2_PHASE_EIGHT_FIXTURE.sql
```

The fixture creates:

- `phase8-alpha` and `phase8-beta` organizations.
- Similar content types, entries, pages, memberships, and rate-limit rows.
- Shared owner membership across both organizations to support positive tenant switching checks.
- Different editor/viewer/billing roles to support negative RBAC checks.

The fixture password hashes are intentionally not login-ready. Use normal seeded/admin accounts, then attach or impersonate the fixture users only in controlled local/staging QA workflows.

## Tenant Isolation Scenarios

1. Sign in as a user that can access `phase8-alpha`.
2. Set `X-Organization-Id` to the Alpha organization ID.
3. Request the Alpha entry and page by ID and confirm success.
4. Request the Beta entry and page IDs while still using the Alpha organization context.
5. Confirm the API returns `404` or `403`, never the Beta payload.
6. Repeat the same direct-ID access test in the opposite direction.
7. Repeat with `phase8-viewer@example.com` and confirm viewer access does not grant write access.

Important fixture IDs:

- Alpha entry: `40000000-0000-7000-8000-000000000001`
- Beta entry: `40000000-0000-7000-8000-000000000002`
- Alpha page: `50000000-0000-7000-8000-000000000001`
- Beta page: `50000000-0000-7000-8000-000000000002`

## Concurrent Membership Scenario

Validate invitation and role update behavior under concurrency:

1. In two terminals, invite or update the same email into the same organization with different roles.
2. Confirm only one active membership row exists for `(organization_id, user_id)`.
3. Confirm the final role is deterministic based on the last successful request.
4. Confirm membership audit entries were written.
5. Confirm the user cannot use permissions from the losing concurrent update.

## Billing Webhook Scenarios

Test Stripe webhook resilience with signed events in staging:

1. Send a valid `checkout.session.completed` event and confirm it creates/updates the subscription.
2. Resend the same event and confirm the response reports `already_processed`.
3. Send a newer `customer.subscription.updated` event and confirm the subscription updates.
4. Send an older `customer.subscription.updated` event for the same subscription and confirm it is marked `ignored`.
5. Confirm `billing_events.provider_event_created_at` stores the Stripe event `created` timestamp.
6. Confirm `organization_subscriptions.provider_event_created_at` is not moved backwards.

## Quota Downgrade Scenario

1. Put an organization on a higher plan and create usage above the Free plan limits.
2. Downgrade the organization to Free.
3. Rebuild usage counters.
4. Confirm exceeded metrics show `exceeded = true`, `remaining = 0`, and do not return negative values.
5. Confirm unlimited plan metrics return `remaining = null` and `percent = null`.

## Load Smoke Test

Run a low-volume smoke pass after the backend is running:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/phase8-load-smoke.ps1 `
  -ApiBaseUrl "http://localhost:8080" `
  -AccessToken "<JWT>" `
  -OrganizationId "<organization-uuid>" `
  -Iterations 20
```

Review:

- Non-zero status counts.
- Unexpected `401`, `403`, or `500` responses.
- High max latency for tenant-scoped endpoints.
- Repeated failures on `/api/billing/usage`, `/api/content-types`, `/api/pages`, or `/api/media`.

## Migration Readiness

Before beta:

- Back up the staging database.
- Apply all migrations to staging from a V1-like database snapshot.
- Confirm phase 8 migration is additive and idempotent.
- Run the backend test suite.
- Apply the phase 8 fixture to staging.
- Run direct-ID tenant isolation scenarios.
- Run Stripe webhook retry/out-of-order scenarios.
- Run the load smoke script.
- Confirm rollback strategy is documented as database restore plus redeploy of the previous application artifact.

## Security and CORS Checklist

- `Content-Security-Policy` includes `frame-ancestors 'none'`.
- `X-Content-Type-Options` is `nosniff`.
- `Referrer-Policy` is `same-origin`.
- `X-Frame-Options` is `DENY`.
- `Permissions-Policy` disables camera, microphone, and geolocation by default.
- Production CORS allows only the intended frontend origins.
- CORS allowed headers include every browser-sent custom header, especially `Authorization` and `X-Organization-Id`.
- Public endpoints do not expose tenant data.
- Tenant endpoints require authentication and organization context.
- Billing mutation endpoints require owner, admin, or billing manager membership.

## Production Readiness Checklist

- `DATABASE_URL`, `JWT_SECRET`, CORS origin, and public URLs are environment-specific.
- Stripe secrets and webhook signing secret are configured outside source control.
- Redis is available for cache/rate-limit paths.
- Backups and restore drills are verified.
- Log retention covers auth, billing, membership, and webhook audit trails.
- Health and readiness endpoints are monitored.
- Rate limits are enabled for production organizations.
- Media storage lifecycle and size limits are documented.
- Database migrations are run once per deployment and checked before app rollout.
- Admin seed credentials are rotated before public beta.
