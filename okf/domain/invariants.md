---
okf_document_id: "domain-invariants"
title: "Domain Invariants"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/migrations"
  - "backend/src/routes"
  - "backend/src/services"
related_documents:
  - "business-rule-catalog.md"
  - "state-transitions.md"
  - "../database/constraints-and-indexes.md"
related_diagrams:
  - "diagrams/domain-map.mmd"
---

# Domain Invariants

The 31 invariants below are limited to conditions with current implementation evidence. “Application” and “frontend” describe additional support, not persistence authority.

| ID | Statement | Domain | Enforcement and database support | Application/frontend support | Tests | Failure behavior | Confidence and exceptions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `INV-IDENTITY-001` | User email is case-insensitively unique. | Identity | `CITEXT UNIQUE` on `users.email`. | Auth handlers normalize/look up email; form uses email input. | No duplicate integration test found. | Insert/update constraint error. | High; database rows only. |
| `INV-IDENTITY-002` | Persisted refresh-token hashes are unique. | Identity | Unique `refresh_tokens.token_hash`. | Auth creates random token and stores hash. | Token lifecycle tests incomplete. | Constraint error. | High. |
| `INV-IDENTITY-003` | Every persisted global role assignment references an existing user and role. | Identity | Composite PK plus cascading FKs. | Bootstrap and role-loading code. | No FK integration test found. | Constraint error/cascade. | High. |
| `INV-TENANT-001` | Organization slug is globally unique and slug-shaped. | Tenant | Unique/check constraints. | Organization normalization. | No route test found. | Validation or constraint error. | High. |
| `INV-TENANT-002` | One membership row exists per organization/user pair. | Tenant | Composite primary key. | Invitation acceptance upserts. | No concurrency test. | Conflict/upsert. | High. |
| `INV-TENANT-003` | One pending invitation exists per organization/email. | Tenant | Partial unique index. | Create invitation uses matching conflict target. | No concurrency test. | Upsert or uniqueness failure. | High. |
| `INV-TENANT-004` | An active tenant request has an active organization and active membership. | Tenant | RLS session context follows middleware result. | Tenant middleware verifies both statuses. | No live cross-tenant suite. | Unauthorized/not found before handler. | High in code; applied RLS `INVARIANT_UNVERIFIED`. |
| `INV-TENANT-005` | A normal owner-changing workflow does not remove the last active owner. | Tenant | No database constraint. | `ensure_not_last_owner` on downgrade/remove/leave. | No concurrent workflow test. | Validation error. | Medium; race-prone `INVARIANT_UNVERIFIED`. |
| `INV-CONTENT-001` | Content type slug is unique within an organization. | Content | Unique `(organization_id, slug)`. | Slug validator. | No constraint test. | Validation/constraint error. | High. |
| `INV-CONTENT-002` | Entry `data` is a JSON object and `version` is positive. | Content | JSON type and positive checks. | Entry validator expects object and mutations increment. | Validator behavior indirectly exercised; no DB test. | Validation/constraint error. | High. |
| `INV-CONTENT-003` | Every content entry references an existing content type. | Content | FK with cascade delete. | Route loads type before writes. | No cascade integration test. | Not found/constraint/cascade. | High; same-tenant pairing relies on trigger/RLS rather than composite FK. |
| `INV-CONTENT-004` | Persisted content status is one of `draft`, `pending_review`, `published`, `archived`. | Content | PostgreSQL enum. | Shared workflow parser/transition policy; status UI. | Workflow unit tests. | Parse/cast error. | High; allowed-transition invariant is application-only. |
| `INV-PAGE-001` | Page slug is unique within an organization and slug-shaped. | Page | Unique `(organization_id, slug)` plus check. | Page validator and form. | No route integration test. | Validation/constraint error. | High. |
| `INV-PAGE-002` | Page and snapshot documents are JSON objects. | Page | JSON type checks. | Recursive validator on page save/restore. | No dedicated validator test. | Validation/constraint error. | High. |
| `INV-PAGE-003` | A page has at most one snapshot per positive version number. | Page | Unique `(page_id, version)` and positive check. | Snapshot helper chooses next number. | No concurrent snapshot test. | Unique/check failure. | High; concurrent writers may fail. |
| `INV-PAGE-004` | Valid Page Builder layouts have one root and only registered descendant component types. | Page | Database stores only object shape. | Recursive backend validator; frontend palette uses registry. | Frontend shell tests only. | Validation error. | High for handler paths; direct DB writes bypass. |
| `INV-PAGE-005` | Tenant component rows belong to an organization; system components do not require one. | Plugin/Page | Check `is_system = TRUE OR organization_id IS NOT NULL`; RLS write policy. | Component handlers filter system writes. | No applied RLS test. | Constraint/RLS rejection. | High in migration; runtime `INVARIANT_UNVERIFIED`. |
| `INV-MEDIA-001` | Media size is nonnegative and variant dimensions, when present, are positive. | Media | Check constraints. | Upload derives size/dimensions. | No DB tests. | Constraint error. | High. |
| `INV-MEDIA-002` | Variant names are unique per media item. | Media | Unique `(media_id, variant_name)`. | Fixed variant catalog. | No processing integration test. | Unique error. | High. |
| `INV-DELIVERY-001` | Webhook event list is nonempty and persisted delivery status is `delivered` or `failed`. | Delivery | Array/status checks. | Service validates event catalog and maps result. | Webhook validator tests. | Validation/constraint error. | High. |
| `INV-DELIVERY-002` | Public settings keys and navigation locales match stored formats. | Delivery | Regex checks. | Delivery reads only. | No management tests. | Constraint error. | High; management workflow unclear. |
| `INV-BILLING-001` | One subscription row exists per organization. | Billing | Organization primary key on subscription. | Change/default/provider paths upsert. | No DB integration test. | Upsert/constraint behavior. | High. |
| `INV-BILLING-002` | Usage values are nonnegative and unique per organization/month/metric. | Billing | Checks and composite unique constraint. | Quota service increments/rebuilds supported metrics. | Quota unit tests. | Constraint error. | High. |
| `INV-BILLING-003` | A provider billing event is processed at most once by provider/event ID. | Billing | Unique provider-event pair. | Transaction returns already-processed result. | Signature and ordering tests; no DB concurrency test. | Duplicate path is treated idempotently. | High. |
| `INV-PLUGIN-001` | CMS plugin keys are unique and slug-shaped. | Plugin | Unique/check constraints. | Route validator and built-in sync. | SEO tests; no constraint test. | Validation/constraint error. | High. |
| `INV-MARKET-001` | Marketplace listing slugs are unique and status values are constrained. | Marketplace | Unique/check constraints. | Submission/catalog routes validate. | Static migration/service tests. | Validation/constraint error. | High. |
| `INV-MARKET-002` | Protected Marketplace artifact fields do not change after protected lifecycle states. | Marketplace | Immutability trigger. | Routes create new versions for updates. | Static migration tests. | Trigger exception. | High. |
| `INV-MARKET-003` | One active installation exists per organization/listing under the active-status uniqueness rule. | Marketplace | Partial uniqueness/index rules. | Install service checks duplicates. | Service and frontend duplicate-install tests. | Conflict/validation error. | High; historical uninstalled rows remain. |
| `INV-MARKET-004` | Purchase amounts are nonnegative and total equals subtotal plus tax. | Marketplace | Finance check constraints. | Finance service validates/splits amounts. | Finance unit tests. | Validation/constraint error. | High. |
| `INV-MARKET-005` | One entitlement exists per purchase and its listing/version pair is coherent. | Marketplace | Unique purchase and composite listing/version FKs. | Stripe handler grants/revokes. | Static finance tests. | Constraint error. | High. |
| `INV-MARKET-006` | Product review rating remains between one and five and body length remains bounded. | Marketplace | Database checks. | Feedback service validates. | Service/frontend tests. | Validation/constraint error. | High. |

## Unverified Candidate Invariants

- `INVARIANT_UNVERIFIED`: `organizations.owner_id` always identifies the member whose role is `owner`. Transfer/provisioning maintain both, but no database constraint prevents drift.
- `INVARIANT_UNVERIFIED`: every tenant child with both `organization_id` and a parent ID belongs to the same tenant as the parent. Several triggers/policies help, but composite same-tenant FKs are not universal.
- `INVARIANT_UNVERIFIED`: database media rows and filesystem objects always agree. They do not share an atomic transaction.
- `INVARIANT_UNVERIFIED`: public delivery cache always reflects committed publication state. Invalidation is best effort.
