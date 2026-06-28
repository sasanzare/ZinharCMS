# V2 Phase 5: Plans, Quotas, and Usage Limits

## Scope

Phase 5 adds the SaaS plan and quota layer before Stripe integration. Billing is manual in this phase; Stripe checkout, customer portal, and subscription webhooks belong to Phase 6.

## Backend

- Added `plans`, `organization_subscriptions`, and `usage_counters` tables.
- Seeded `Free`, `Pro`, and `Enterprise` plans.
- Assigned existing and newly created organizations to the `Free` plan by default.
- Added billing API endpoints for plans, subscription state, usage, manual plan changes, and usage-counter rebuilds.
- Enforced quotas for members, pages plus entries, media storage, and tenant API requests.
- Kept billing endpoints reachable even when API request quota is exhausted so users can inspect usage and change plans.
- Added RLS policies for organization subscriptions and usage counters.

## Frontend

- Added a Billing page at `/billing`.
- Added plan cards with manual plan switching.
- Added monthly usage meters for members, pages plus entries, media storage, and API requests.
- Surfaced the same usage summary on the organization dashboard.
- Added rebuild usage action for organization owners, admins, and billing managers.
- Added English and Persian i18n messages for billing and usage.

## Notes

Usage enforcement uses live organization data as the source of truth. `usage_counters` stores monthly snapshots and API request counts, and rebuildable counters can be regenerated from the source tables.
