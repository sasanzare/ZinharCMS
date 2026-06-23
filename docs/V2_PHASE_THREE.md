# V2 Phase Three: PostgreSQL Row-Level Security

This phase moves tenant isolation from application-only filtering to a database-enforced defense layer.

## Scope

- Added PostgreSQL RLS helper functions for the active organization and active user session context.
- Enabled and forced RLS on tenant-owned tables.
- Added `SELECT`, `INSERT`, `UPDATE`, and `DELETE` policies for organization-owned records.
- Added custom component registry policies so system components stay globally readable while custom components remain tenant-owned.
- Added backend helpers for RLS-scoped database connections and transactions.
- Updated tenant-owned admin routes and public delivery reads to run queries with an explicit database tenant context.
- Updated webhook background delivery logging to write with organization context.
- Updated sample data SQL to set RLS context inside its transaction.

## Session Variables

RLS policies use these PostgreSQL settings:

```sql
zinhar.organization_id
zinhar.user_id
zinhar.rls_bypass
```

Application code must set `zinhar.organization_id` before reading or writing tenant-owned tables. The backend does this through `rls::tenant_connection`, `rls::organization_connection`, and `rls::begin_tenant_transaction`.

## Protected Tables

RLS is enabled for:

- `content_types`
- `content_entries`
- `pages`
- `page_versions`
- `component_registry`
- `media`
- `media_variants`
- `comments`
- `webhooks`
- `webhook_deliveries`
- `public_settings`
- `navigation_items`

Organization membership tables are intentionally left outside RLS in this phase because tenant middleware must validate membership before opening a tenant-scoped data connection.

## Runtime Behavior

- Tenant admin routes use the authenticated `TenantContext`.
- Public delivery routes resolve the default public organization and then read through an organization-scoped connection.
- Webhook dispatch reads subscriptions and records deliveries through organization-scoped connections.
- RLS context on pooled connections is not returned to the pool. Scoped connections use `close_on_drop()` to avoid leaking a tenant context into later requests.
- Existing application-level `organization_id` filters remain in place. RLS is a second defensive layer, not a replacement for permission checks.

## Verification

Run:

```powershell
cargo test --manifest-path backend\Cargo.toml --all-features
```

For direct database checks, set the context before querying tenant-owned tables:

```sql
BEGIN;
SELECT set_config('zinhar.organization_id', '<organization-uuid>', true);
SELECT * FROM content_types;
ROLLBACK;
```

Without the correct `zinhar.organization_id`, tenant-owned tables should not expose records.
