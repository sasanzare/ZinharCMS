# V2 Phase 1: Organization Data Model And Base Migration

Date: 2026-06-22

## Goal

Phase 1 introduces the base multi-organization structure without fully enabling SaaS behavior. The goal is to place all V1 data under a default organization and prepare the database for `TenantContext`, RLS, and member management in later phases.

## Completed Deliverables

- Added the `organizations` table.
- Added the `organization_members` table.
- Added the `organization_invitations` table.
- Added enums for organization status, member role, member status, and invitation status.
- Added the database helper `app_default_organization_id()`.
- Created `Default Organization` with slug `default`.
- Backfilled existing users into the default organization.
- Mapped existing `super_admin` and `admin` users to owner/admin membership in the default organization.
- Added `organization_id` to V1 tenant-owned tables.
- Backfilled existing V1 records into the default organization.
- Scoped `content_types.slug` and `pages.slug` uniqueness by organization.
- Scoped the `public_settings` primary key by organization.
- Added organization indexes for future high-use queries.
- Added triggers that synchronize `organization_id` on child tables such as entries, page versions, media variants, comments, and webhook deliveries.
- Connected new user registration and default admin seeding to default organization membership.
- Updated `docs/sample-data.sql` to use the new conflict targets and organization-aware lookups.

## Prepared Tenant-Owned Tables

- `content_types`
- `content_entries`
- `pages`
- `page_versions`
- `component_registry` for custom components
- `media`
- `media_variants`
- `comments`
- `webhooks`
- `webhook_deliveries`
- `public_settings`
- `navigation_items`

## Intentionally Not Enabled In Phase 1

- `TenantContext` middleware and request extensions
- `X-Organization-Id` from the frontend
- Workspace routes such as `/org/:orgSlug/...`
- PostgreSQL RLS
- Organization switcher UI
- Complete invitation flow
- Billing, plans, and quotas

These items belong to later phases. Phase 1 only prepares the migration foundation and data model.

## Acceptance Criteria

- The new migration can run after all V1 migrations on a fresh database.
- V1 data is attached to the default organization.
- Current V1 insert paths continue to work without passing an explicit `organization_id`.
- Newly registered users and the seeded admin become members of the default organization.
- Sample data can still be imported repeatedly using the new unique constraints.

## Verification Notes

- `cargo fmt --manifest-path backend/Cargo.toml` passed.
- `cargo test --manifest-path backend/Cargo.toml --all-features` passed.
- `git diff --check` passed with line-ending warnings only.
- Live PostgreSQL migration verification was not run in this environment because Docker Desktop was not running and `psql` was not installed.