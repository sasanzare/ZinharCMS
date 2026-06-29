# V2 Phase 0: V1 Assessment and Final V2 Design

Date: 2026-06-22

## Executive Summary

Version 1 is a single-organization CMS. Core records such as content types, entries, pages, media, comments, webhooks, public settings, and navigation are stored globally. There is no `organization_id`, tenant context, tenant-aware API contract, tenant-aware cache key, or workspace route in the current application.

For Version 2, the safest path is to add the organization data model and controlled migrations first, then make APIs tenant-aware, and only enable PostgreSQL RLS after the application consistently supplies a verified `TenantContext`. Enabling RLS before the V1 data backfill and tenant-aware API work would create unnecessary operational risk.

Final Phase 0 decisions:

- V2 uses a shared database, shared schema, and `organization_id` tenant model.
- The main frontend workspace route should be `/org/:orgSlug/...`.
- Authenticated tenant-owned APIs should use `X-Organization-Id` and the backend must validate user membership before handler execution.
- Public delivery APIs must become organization-aware. Initial route proposal: `/api/v1/o/{org_slug}/...`.
- Subdomains and custom domains are not required for the V2 MVP and should be moved to a later production/domain phase.
- All existing V1 data must be backfilled into a default organization.

## Phase 0 Deliverables

This document covers the required Phase 0 outputs from the V2 proposal:

- Complete tenant-owned table inventory
- Endpoints that require tenant context
- Jobs, webhooks, and background process audit
- Final tenant routing decision
- Migration strategy and rollback plan
- Organization role and permission matrix
- Tenant isolation test matrix
- Migration risk register

## V1 Database Audit

### Global Or Platform-Level Tables

| Table | Current state | V2 decision |
| --- | --- | --- |
| `users` | Global user with unique email | Keep global. One user can belong to multiple organizations. |
| `refresh_tokens` | User-owned tokens | Keep global. The active organization should not be stored in refresh tokens. |
| `login_attempts` | Rate limiting by email/IP | Keep global. Organization-aware rate limiting can be added later. |
| `roles` | V1 global roles | Keep temporarily for backward compatibility, but organization access should move to `organization_members.role`. |
| `user_roles` | Global user-to-role mapping | Keep temporarily for platform/admin legacy behavior. V2 organization access should come from memberships. |
| `cms_plugins` | Global plugin registry | Keep the system registry global. Per-organization enablement should live in a table such as `organization_plugin_settings`. |

### Tenant-Owned Tables

| Table | Why it is tenant-owned | Migration action |
| --- | --- | --- |
| `content_types` | Each organization owns its own content schema | Add `organization_id`; change uniqueness from `slug` to `(organization_id, slug)`. |
| `content_entries` | Content records belong to an organization | Add direct `organization_id` for RLS and fast queries; keep consistency with `content_types`. |
| `pages` | Pages belong to an organization | Add `organization_id`; change uniqueness from `slug` to `(organization_id, slug)`. |
| `page_versions` | Page snapshots belong to the same organization as the page | Add `organization_id` and backfill from `pages`. |
| `component_registry` | System components are global; custom components are organization-owned | Add nullable `organization_id`; system rows use `NULL`, custom rows use an organization. |
| `media` | Files and metadata belong to an organization | Add `organization_id`; later change file/object paths to tenant-aware paths. |
| `media_variants` | Variants inherit ownership from media | Add `organization_id` for RLS and backfill from `media`. |
| `comments` | Comments attach to tenant-owned entries or pages | Add `organization_id` and validate the target entity inside the same organization. |
| `webhooks` | Each organization owns its webhook subscriptions | Add `organization_id`; secrets and events are scoped to the organization. |
| `webhook_deliveries` | Delivery logs belong to an organization webhook | Add `organization_id` and backfill from `webhooks`. |
| `public_settings` | Public settings are organization-specific | Add `organization_id`; change primary key from `key` to `(organization_id, key)`. |
| `navigation_items` | Navigation belongs to an organization/site | Add `organization_id`; scope parent-child relationships to the same organization. |

### New V2 Tables

| Table | Purpose |
| --- | --- |
| `organizations` | Stores organization name, slug, status, owner, and base settings. |
| `organization_members` | Maps users to organizations with role and membership status. |
| `organization_invitations` | Stores invitation token, email, role, status, and expiry. |
| `plans` | Defines Free, Pro, Enterprise, and their feature limits. |
| `organization_subscriptions` | Stores plan/subscription state and Stripe customer/subscription references. |
| `usage_counters` | Tracks monthly usage for users, entries/pages, media, and API calls. |
| `billing_events` | Stores idempotent Stripe event processing state. |
| `audit_logs` | Records sensitive operations such as member changes, billing, publish/delete, and security events. |
| `organization_plugin_settings` | Stores plugin enablement and settings per organization. |

## V1 Endpoint Audit

### Global Endpoints

These endpoints do not require tenant context in V2:

| Endpoint | Decision |
| --- | --- |
| `GET /` | Global health/info endpoint. |
| `GET /health` | Global health endpoint. |
| `GET /ready` | Global readiness endpoint. |
| `GET /openapi.json` | Global OpenAPI document. |
| `POST /api/auth/login` | Global login. |
| `POST /api/auth/refresh` | Global token refresh. |
| `POST /api/auth/logout` | Global logout. |
| `POST /api/auth/register` | Global user creation; V2 should start organization onboarding after registration. |
| `GET /api/auth/me` | Global user profile; should also return memberships and default/active organization metadata. |

### Tenant-Owned Endpoints

All endpoint groups below must receive a valid `TenantContext` before handler execution:

| Endpoint group | Endpoints | V2 decision |
| --- | --- | --- |
| Content Types | `/api/content-types`, `/api/content-types/{id}` | Tenant-owned; all queries must filter by `organization_id`. |
| Entries | `/api/entries/{type_slug}`, `/api/entries/{type_slug}/{id}`, workflow actions | Tenant-owned; resolve `type_slug` inside the active organization. |
| Media | `/api/media`, `/api/media/upload`, `/api/media/{id}` | Tenant-owned; list/upload/update/delete and storage paths must be organization-aware. |
| Pages | `/api/pages`, `/api/pages/{id}`, `/api/pages/slug/{slug}` | Tenant-owned; slugs are unique only inside an organization. |
| Page Versions | `/api/pages/{id}/versions`, `/api/pages/{id}/versions/{version}/restore` | Tenant-owned; versions must belong to a page in the same organization. |
| Component Registry | `/api/component-registry`, `/api/component-registry/{id}` | Reads include system + organization custom components; writes only affect custom organization components. |
| Preview | `/api/preview/{page_id}` | Tenant-owned; websocket context must derive organization from token/context. |
| Comments | `/api/comments`, `/api/comments/{id}`, `/api/comments/{id}/resolve` | Tenant-owned; target entity must belong to the same organization. |
| Webhooks | `/api/webhooks`, `/api/webhooks/{id}`, `/api/webhooks/{id}/deliveries`, `/api/webhooks/{id}/test` | Tenant-owned; deliveries are visible only to the owning organization. |
| Plugins | `/api/plugins`, `/api/plugins/{plugin_key}`, enable/disable | Plugin registry is global; enable/disable state is organization-scoped. |
| Workflow UI data | Entries/pages/comments/plugins combinations | Tenant-owned; queues must show only active-organization data. |

### Public Delivery Endpoints

V1 public endpoints are global and currently use global cache keys:

| Endpoint | V2 decision |
| --- | --- |
| `GET /api/v1/content/{type_slug}` | Make organization-aware: `/api/v1/o/{org_slug}/content/{type_slug}`. |
| `GET /api/v1/content/{type_slug}/{id_or_slug}` | Include organization slug in route and cache key. |
| `GET /api/v1/pages` | Return only pages for the selected organization. |
| `GET /api/v1/pages/{slug}` | Resolve slug inside organization scope. |
| `GET /api/v1/settings/public` | Return public settings for the selected organization. |
| `GET /api/v1/navigation` | Return navigation for the selected organization. |
| `GET /api/v1/sitemap.xml` | Return sitemap for the selected organization. |
| `GET /api/v1/robots.txt` | Return robots rules for the selected organization. |

## Backend Query Audit

V1 query paths currently read and write global tables directly. Required V2 changes:

- `content.rs`: all entry list/get/create/update/delete and workflow operations must use `organization_id`.
- `pages.rs`: page CRUD, version restore, component registry, and preview must be tenant-aware.
- `media.rs`: list/upload/update/delete must use organization scope and tenant-aware storage paths.
- `comments.rs`: current validation checks only existence in `content_entries` or `pages`; V2 must also validate same-organization ownership.
- `webhooks.rs` and `services/webhooks.rs`: listing, dispatch, and delivery logs must include `organization_id`.
- `delivery.rs`: cache keys and public queries must be organization-aware.
- `plugins/mod.rs`: hook execution must receive organization context.
- `main.rs`: current seed creates only a user and global role; V2 also requires a default organization and owner membership.

## Background Processes, Jobs, And Webhooks

| Process | V1 state | V2 decision |
| --- | --- | --- |
| Webhook dispatch | Runs in `tokio::spawn` and reads all matching webhooks globally | Dispatch only webhooks for the active organization; payload should include `organization_id` and `organization_slug`. |
| Webhook deliveries | Stored by `webhook_id` | Add direct `organization_id` to simplify RLS and audit. |
| Redis delivery cache | Global keys such as `delivery:pages:*` | Prefix cache keys by organization: `delivery:{org_id}:pages:*`. |
| Cache invalidation | Publish/unpublish clears broad page/content cache | Invalidate only the active organization scope. |
| Media variants | Generated on upload under global `/uploads/...` path | Move storage to `/uploads/{organization_id}/...`. |
| Plugin hook `seo-auto` | Runs after entry publish | Hook context must include organization and read plugin settings in organization scope. |
| DB migrations | Run on startup | V2 migrations must be dry-run in staging before RLS is enabled. |
| Default admin seed | Creates only global user and role | Must also create default organization and owner membership. |

## Final Tenant Routing Decision

### Frontend Routing

V2 MVP frontend routes should use this shape:

```text
/org/:orgSlug
/org/:orgSlug/content-types
/org/:orgSlug/entries
/org/:orgSlug/media
/org/:orgSlug/pages
/org/:orgSlug/workflow
/org/:orgSlug/settings
/org/:orgSlug/members
/org/:orgSlug/billing
```

Reasons:

- Works with the current Vite local development setup.
- Does not require DNS, wildcard TLS, or subdomains.
- Makes the active organization visible in the URL.
- Reduces the chance of showing stale data from a previous organization after context switches.
- Leaves room for custom domains or subdomains later in public delivery.

### API Tenant Context

Authenticated tenant-owned endpoints should use:

```http
X-Organization-Id: <uuid>
Authorization: Bearer <token>
```

The backend must:

- Read the organization from the header.
- Verify active user membership in that organization.
- Add the organization role to `TenantContext`.
- Store `TenantContext` in request extensions.
- Reject tenant-owned operations without context using `400` or `403`.

Public delivery routes should include the organization slug:

```text
/api/v1/o/{org_slug}/pages/{slug}
/api/v1/o/{org_slug}/content/{type_slug}
```

### Organization Slug Convention

Slug pattern:

```text
^[a-z0-9]+(?:-[a-z0-9]+)*$
```

Additional rules:

- Length: 3 to 64 characters.
- Lowercase ASCII only.
- Globally unique.
- Reserved slugs are not allowed: `admin`, `api`, `www`, `app`, `auth`, `login`, `signup`, `settings`, `billing`, `support`, `static`, `assets`.
- Slug changes should be disabled in the V2 MVP or limited to owners with audit logging and redirects.

## Migration Strategy

### Migration Principles

- Do not start by enabling RLS.
- Add `organization_id` first, backfill data, then enforce constraints.
- Backfill all V1 data into the default organization.
- Add scoped unique constraints and not-null constraints only after backfill validation.
- Enable RLS only after tenant-aware APIs are implemented and tested.

### Step 1: Preparation

1. Take a full database backup.
2. Run the migration in staging with a realistic snapshot.
3. Record pre-migration row counts for all tenant-owned tables.
4. Create the default organization:

```text
name: Default Organization
slug: default
status: active
```

5. Create owner membership for the default admin.

### Step 2: Add Organization Tables

Add these tables:

- `organizations`
- `organization_members`
- `organization_invitations`

Required initial indexes:

- `organizations(slug)`
- `organization_members(user_id, organization_id)`
- `organization_members(organization_id, role)`
- `organization_invitations(organization_id, email)`

### Step 3: Add `organization_id` To Existing Data

Add `organization_id UUID NULL` to tenant-owned tables and backfill:

- `content_types`: default organization
- `content_entries`: from `content_types.organization_id`
- `pages`: default organization
- `page_versions`: from `pages.organization_id`
- `media`: default organization
- `media_variants`: from `media.organization_id`
- `comments`: from the target entry/page entity
- `webhooks`: default organization
- `webhook_deliveries`: from `webhooks.organization_id`
- `public_settings`: default organization
- `navigation_items`: default organization
- `component_registry`: system rows use `NULL`; custom rows use the default organization

### Step 4: Constraints And Indexes

After backfill:

- Make `organization_id` `NOT NULL` on tenant-owned tables, except system rows in `component_registry`.
- Replace global uniqueness:
  - `content_types.slug` -> `(organization_id, slug)`
  - `pages.slug` -> `(organization_id, slug)`
  - `public_settings.key` -> `(organization_id, key)`
  - `component_registry.component_key` -> scoped uniqueness for custom components, with special handling for system components
- Add high-use query indexes:
  - `(organization_id, slug)`
  - `(organization_id, status)`
  - `(organization_id, updated_at DESC)`
  - `(organization_id, created_at DESC)`

### Step 5: Application Compatibility

Before enabling RLS:

- Global endpoints should continue to work without an organization header.
- Tenant-owned endpoints should require `TenantContext`.
- The frontend should send active organization in the URL and API header.
- Cache keys and media paths should become organization-aware.

### Step 6: RLS

Enable RLS only after tenant-aware API tests pass:

- Define a session variable such as `app.current_organization_id`.
- Set the session variable before every tenant-owned query.
- Define select/insert/update/delete policies for tenant-owned tables.
- Keep any super-admin support path explicit, limited, and audit-logged.

## Rollback Plan

### Before Not-Null And RLS

Rollback is practical:

- Disable the tenant routing feature flag.
- Move the frontend back to global routes.
- Drop nullable `organization_id` columns and organization tables if no new V2 data has been created.
- Restore from backup if backfill validation fails.

### After Not-Null

Rollback without a backup is high risk. The safe path is:

- Restore the database from the pre-migration backup.
- Deploy V1.
- Clear Redis caches.
- Remove tenant-aware media files if necessary.

### After RLS

Emergency rollback can temporarily disable RLS:

```sql
ALTER TABLE <tenant_table> DISABLE ROW LEVEL SECURITY;
```

This is only an emergency recovery measure. A real rollback should use the backup and previous deployment.

## Organization Permission Matrix

| Capability | Owner | Admin | Editor | Author | Viewer | Billing Manager |
| --- | --- | --- | --- | --- | --- | --- |
| View organization dashboard | Yes | Yes | Yes | Yes | Yes | Yes |
| Manage organization settings | Yes | Yes | No | No | No | No |
| Manage members | Yes | Yes | No | No | No | No |
| Transfer ownership | Yes | No | No | No | No | No |
| Delete organization | Yes | No | No | No | No | No |
| Manage content types | Yes | Yes | Yes | No | Read only | No |
| Manage entries | Yes | Yes | Yes | Own/create/update/submit only | Read only | No |
| Publish/unpublish | Yes | Yes | Yes | No | No | No |
| Manage pages/page builder | Yes | Yes | Yes | Own drafts only | Read only | No |
| Manage media | Yes | Yes | Yes | Upload/update own only | Read only | No |
| Review comments/workflow | Yes | Yes | Yes | Submit/comment | Read only | No |
| Manage webhooks/API integrations | Yes | Yes | No | No | No | No |
| Manage billing/subscription | Yes | No | No | No | No | Yes |
| View usage | Yes | Yes | Yes | Yes | Yes | Yes |
| View audit log | Yes | Yes | No | No | No | Billing events only |

`super_admin` should remain platform-level and should not be treated as a normal organization member. Every super-admin action should be audit-logged.

## Frontend Design Audit

Current state:

- Routes are global: `/`, `/content-types`, `/entries`, `/media`, `/pages`, `/workflow`, `/settings`.
- `AppShell` does not include an organization switcher.
- `api.ts` sends only the auth token and no tenant header.
- Zustand/localStorage stores only user and token state.
- There is no organization context switch, so old page state can become unsafe after switching organizations.

Required V2 changes:

- Add organization onboarding after login.
- Redirect users with no organizations to `/organizations/new`.
- Redirect users with organizations to `/org/:orgSlug`.
- Add an organization switcher to the top bar.
- Store active organization using a dedicated key such as `zinhar.active_organization_id`.
- On organization switch:
  - Clear data cache/state for the previous organization.
  - Refetch current page data.
  - Change the route to the new organization slug.
  - Send the new organization header in API requests.

## Organization Dashboard Wireframe

```text
+------------------------------------------------------------+
| Organization switcher | Plan badge | Usage alert | User menu |
+----------------------+-------------------------------------+
| Sidebar              | Organization overview               |
| - Dashboard          | - Content count                     |
| - Content Types      | - Page count                        |
| - Entries            | - Media usage                       |
| - Media              | - Member count                      |
| - Pages              | - Current plan                      |
| - Workflow           |                                     |
| - Members            | Recent activity / audit             |
| - Billing            | Quota cards                         |
| - Settings           | Upgrade prompt                      |
+----------------------+-------------------------------------+
```

## Initial Plan Limits

| Plan | Target user | Initial limits |
| --- | --- | --- |
| Free | Testing and small projects | 1 organization, 3 users, 50 pages/entries, 1GB media, no custom domain |
| Pro | Small and medium teams | 10 users, 5000 pages/entries, 50GB media, custom domain |
| Enterprise | Large organizations | Contract-based limits, SLA, dedicated support, optional separate database |

## Initial Stripe Product And Price Design

| Product | Price type | Required metadata |
| --- | --- | --- |
| ZinharCMS Free | internal/free | `plan_key=free` |
| ZinharCMS Pro Monthly | recurring monthly | `plan_key=pro`, `interval=month` |
| ZinharCMS Pro Yearly | recurring yearly | `plan_key=pro`, `interval=year` |
| ZinharCMS Enterprise | manual/offline | `plan_key=enterprise` |

Stripe rules:

- `organization_subscriptions.stripe_customer_id` should be unique per organization.
- `billing_events.provider_event_id` must be unique to avoid duplicate webhook processing.
- Downgrades must not delete data; they should only restrict new data creation.
- Checkout success should update internal state only after a verified webhook is received.

## Tenant Isolation Test Matrix

| Scenario | Expected result |
| --- | --- |
| User A belongs to Org A and requests Org B page by direct ID | Return `404` or `403` without leaking record existence. |
| User A deletes media owned by Org B | Reject the request. |
| User is Editor in Org A and Viewer in Org B | Apply the role for the selected organization only. |
| Org A API key requests Org B content | Reject the request. |
| Org A webhook receives Org B publish event | Do not execute the webhook. |
| Org A and Org B use the same slug in delivery cache | Keep cache entries isolated. |
| Same page slug exists in two organizations | Allow it. |
| Same content type slug exists in two organizations | Allow it. |
| Comment targets an entity in another organization | Reject it. |
| RLS query forgets an organization filter | Do not return data from another organization. |
| Migration runs more than once in staging | Stay idempotent and avoid duplicates. |
| Rollback before RLS | Support backup restore or reverse migration. |

## Risks

| Risk | Severity | Mitigation |
| --- | --- | --- |
| A query misses its tenant filter | Critical | RLS, code audit, IDOR tests, and shared query helpers. |
| V1 data backfill is incomplete | High | Pre/post row counts, staging dry-run, mandatory backup. |
| Cache keys remain global after V2 | High | Mandatory organization prefix and cache isolation tests. |
| Webhooks remain global across organizations | High | Add `organization_id` to webhooks and deliveries; include organization in payloads. |
| Frontend keeps stale state after organization switch | Medium | Workspace routes, clear state on switch, mandatory refetch. |
| Role model becomes too complex | Medium | Start with simple organization roles and move advanced permissions to V2.x. |
| RLS is enabled too early | High | Enable RLS only after `TenantContext` and integration tests exist. |
| Unique constraint changes fail on real data | Medium | Detect duplicates before applying constraints and report manual fixes. |

## Phase 0 Acceptance Criteria

- Every important table has a tenancy decision.
- Global and tenant-owned endpoints are separated.
- Final tenant routing for the V2 MVP is defined.
- Migration strategy covers backfill, constraints, RLS, and rollback.
- Jobs, webhooks, cache, and media processing are reviewed for tenancy.
- Core tenant isolation tests are identified.

## Decision For Phase 1

Phase 1 should focus only on the organization data model and base migration:

1. Create `organizations`.
2. Create `organization_members`.
3. Create `organization_invitations`.
4. Seed the default organization.
5. Backfill V1 data into the default organization.
6. Add scoped indexes and unique constraints.
7. Prepare the backend to read organization membership without fully enabling RLS.

Tenant-owned APIs and RLS should not be activated until Phase 1 is complete.