# V1 to V2 Migration Guide

This guide describes the safe path for moving an existing V1 ZinharCMS installation into the V2 multi-tenant model.

## Prerequisites

- A complete V1 database backup.
- A complete uploaded media backup.
- Access to the V2 deployment environment.
- A target V2 organization for the migrated data.
- A mapped owner user for the target organization.
- Staging access for a full dry run.

## Migration Order

1. Back up the V1 database and media storage.
2. Deploy the V2 schema migrations to staging.
3. Create or select the target V2 organization.
4. Create the organization owner membership.
5. Map V1 users to V2 organization memberships.
6. Import content types.
7. Import entries with `organization_id` and mapped `type_id`.
8. Import media metadata and confirm the physical files exist in configured storage.
9. Import pages and validate `page_json`.
10. Import settings, navigation items, components, webhooks, and workflow data where applicable.
11. Rebuild usage counters.
12. Run tenant isolation, billing, and smoke checks.
13. Repeat the same process in production only after staging passes.

## User And Role Mapping

Map V1 users into V2 organization memberships:

- V1 owner or platform admin -> organization `owner` or `admin`
- V1 editor -> organization `editor`
- V1 author -> organization `author`
- V1 viewer -> organization `viewer`

Keep `super_admin` limited to platform operations. Do not use it as the normal organization owner account.

## Data Validation

After import, validate:

- every tenant-owned row has the target `organization_id`
- content type slugs are unique inside the organization
- entry payloads still match their content type schemas
- page slugs are unique inside the organization
- page JSON is valid and references registered components
- media metadata points to existing files
- webhooks have safe URLs and correct secret configuration
- usage counters match imported pages, entries, media, and API counters

## Staging Dry Run

The staging dry run must prove:

- migrations run without manual schema edits
- imported content is visible only inside the target organization
- a non-member cannot read migrated data
- billing usage displays expected counters
- rollback restores the previous database snapshot
- support can follow the runbook without direct engineering help

## Production Run

For production:

1. Announce the maintenance window.
2. Stop write-heavy workflows or put the source system in read-only mode.
3. Take a fresh backup.
4. Run migrations.
5. Import data in the validated order.
6. Rebuild usage counters.
7. Run smoke checks.
8. Enable access for the target organization.
9. Monitor account access, billing, usage, and audit logs.

## Rollback

Rollback is required when:

- migrations fail after partial schema changes
- tenant isolation checks fail
- imported data is missing or attached to the wrong organization
- account access is broken for organization owners
- billing state blocks valid access and cannot be corrected quickly

Rollback steps are documented in `docs/V2_OPERATIONS_RUNBOOK.md`.