# V2 Phase 9 Beta Release and Early Customer Feedback

Phase 9 turns the hardened V2 build into a limited beta that can be used by a small number of real organizations while product, support, and engineering monitor feedback, tenant safety, billing behavior, quota behavior, and GA readiness.

## Delivered Scope

- Added beta release data model:
  - `beta_participants`
  - `beta_feedback`
  - `beta_ga_blockers`
- Added RLS policies for all beta-owned tables.
- Added tenant-scoped beta APIs for feedback, dashboard metrics, and GA blockers.
- Added product-admin beta APIs for cross-organization dashboarding and beta participant selection.
- Added a Beta page in the admin UI with:
  - in-product feedback form
  - current organization beta health dashboard
  - GA blocker tracking
  - product beta dashboard for platform admins
  - beta participant upsert form for selecting test organizations
- Added static hardening coverage for phase 9 migration and RLS.

## API Surface

Tenant-scoped endpoints:

- `GET /api/beta/dashboard`
- `GET /api/beta/feedback`
- `POST /api/beta/feedback`
- `PATCH /api/beta/feedback/{feedback_id}`
- `GET /api/beta/ga-blockers`
- `POST /api/beta/ga-blockers`
- `PATCH /api/beta/ga-blockers/{blocker_id}`

Product-admin endpoints:

- `GET /api/beta/product-dashboard`
- `PUT /api/beta/participants/{organization_id}`

`super_admin` and global `admin` users can use product-admin endpoints. Organization owners, admins, and editors can triage beta feedback and manage GA blockers for their own organization. Any authenticated member in the active organization can submit feedback.

## Limited Beta Plan

Start with 3 to 5 organizations:

- 1 internal test organization with seeded/staging data.
- 1 small real organization on the Free plan.
- 1 real organization on Pro or a Stripe checkout path.
- 1 organization with multiple roles and invitations.
- 1 organization with media-heavy content, if available.

Each beta organization should be added through:

```http
PUT /api/beta/participants/{organization_id}
```

Suggested participant status flow:

- `candidate`
- `invited`
- `onboarding`
- `active`
- `paused`
- `graduated`
- `rejected`

## Onboarding Flow

For each beta organization:

1. Create or select the organization.
2. Confirm owner membership exists.
3. Add the organization to beta participants.
4. Invite at least one editor and one viewer.
5. Create one content type.
6. Create and publish one entry.
7. Create and publish one page.
8. Upload one image asset.
9. Open Billing and confirm the current plan and usage meters are understandable.
10. Submit one feedback item from the Beta page.

The organization is considered onboarded when it can complete these steps without direct database edits.

## V1 Data Sample Migration

For the beta, migrate one realistic V1 data sample into one V2 organization:

1. Export V1 content types, entries, media metadata, pages, users, and roles.
2. Create the target V2 organization.
3. Map V1 global users to V2 organization members.
4. Import content types first.
5. Import entries with `organization_id` and mapped `type_id`.
6. Import media metadata and verify files exist in the configured storage path.
7. Import pages and validate `page_json`.
8. Rebuild usage counters.
9. Run tenant isolation checks from `docs/V2_PHASE_EIGHT.md`.
10. Record migration issues as beta feedback or GA blockers.

Do not run a beta migration against production data until a staging dry run has completed successfully.

## Monitoring During Beta

Check daily:

- Open feedback count.
- High or critical feedback count.
- Open GA blockers.
- Failed billing events in the last 30 days.
- Failed email deliveries in the last 30 days.
- Quota metrics where usage exceeds plan limits.
- Organization audit logs for failed or repeated sensitive operations.

The product dashboard aggregates selected beta organizations. The tenant dashboard focuses on the active organization.

## GA Blocker Rules

Create a GA blocker when any of these occur:

- Tenant data is visible across organizations.
- Billing webhook retries or out-of-order events create an incorrect subscription state.
- Upgrade or downgrade is confusing to beta users.
- Organization onboarding requires database edits.
- A common page or content workflow produces repeated 5xx errors.
- Migration of realistic V1 data requires unplanned schema edits.

Suggested priorities:

- `p0`: must fix immediately; blocks all beta/GA work.
- `p1`: must fix before GA.
- `p2`: should fix before GA if it affects common workflows.
- `p3`: can be deferred if documented.

## Acceptance Checklist

- At least 3 beta organizations are marked `active`.
- Each active beta organization completed the onboarding flow without manual database intervention.
- No critical tenant isolation issue is open.
- No `p0` or `p1` GA blocker remains open.
- Billing and quota behavior is understandable to beta users.
- V1 sample migration completed in staging.
- Product dashboard shows beta health without requiring direct SQL access.
- Feedback form is available inside the admin UI.
