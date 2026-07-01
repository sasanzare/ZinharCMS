# ZinharCMS V2 General Availability Release Notes

ZinharCMS V2 is the first General Availability release of the multi-tenant SaaS architecture. It keeps the V1 headless CMS and page-builder foundation while adding organization isolation, billing, quota enforcement, SaaS operations, beta feedback workflows, and production release documentation.

## Highlights

- Multi-tenant organization model with memberships, tenant-aware request context, and PostgreSQL row-level security.
- Organization-scoped content types, entries, pages, media, components, workflow comments, webhooks, settings, and navigation.
- Billing plans, subscription state, Stripe webhook ingestion, usage counters, and quota enforcement.
- Organization operations for invitations, workspace URLs, custom domains, rate limits, audit logs, email delivery, and SaaS alerts.
- Security hardening for tenant isolation, webhook ordering, upload validation, security headers, and production readiness.
- Beta release dashboard for selected organizations, in-product feedback, and GA blocker tracking.
- Internationalized admin shell with English and Persian locale support.
- GA documentation for migration, administration, billing, operations, support, and rollback.

## Breaking Changes From V1

- V2 APIs are organization-aware. Authenticated tenant APIs require an active organization context.
- Existing V1 global content must be migrated into a target organization.
- Users manage content through organization memberships and roles instead of only global roles.
- Usage is counted per organization and can be limited by the active billing plan.
- Billing and quota behavior depends on organization subscription state.
- Admin workflows should use the V2 organization, billing, audit, and rate-limit screens rather than direct database edits.

## Upgrade Notes

1. Read `docs/V2_MIGRATION_GUIDE.md`.
2. Back up the V1 database and uploaded media storage.
3. Create the target V2 organization and owner membership.
4. Run the migration on staging first.
5. Rebuild usage counters.
6. Validate tenant isolation and RLS checks.
7. Verify billing plans and Stripe webhook delivery.
8. Run `.\scripts\v2-ga-check.ps1` before production rollout.

## Operational Notes

- Use `docs/V2_OPERATIONS_RUNBOOK.md` for release, monitoring, incident response, and rollback.
- Use `docs/V2_BILLING_GUIDE.md` for plan changes, quota behavior, Stripe webhook checks, and billing support.
- Use `docs/V2_ADMIN_GUIDE.md` for organization management, memberships, domains, audit logs, and operational admin tasks.

## Known Limitations

- Enterprise pricing remains configurable per deployment and may be shown as a contact-sales/custom plan in production.
- Large frontend bundles may produce a Vite chunk-size warning. This does not block GA unless build output fails or runtime performance is unacceptable for target users.
- V1-to-V2 migration should be treated as a staged operation. Do not run it directly on production data before a staging dry run and rollback drill.

## Support Policy

During the first GA monitoring window, prioritize:

- tenant isolation or RLS issues
- account access and invitation failures
- billing webhook or subscription state mismatches
- quota counters that block valid usage
- migration failures or data integrity issues