---
okf_document_id: "backend-persistence-access"
title: "Backend Persistence Access"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/db/mod.rs"
  - "backend/src/state.rs"
  - "backend/src/routes"
  - "backend/src/services"
  - "backend/src/models"
  - "backend/migrations"
related_documents:
  - "backend/request-handling.md"
  - "backend/services-and-domain.md"
  - "backend/configuration-and-state.md"
  - "backend/backend-risks.md"
  - "architecture/components.md"
related_diagrams:
  - "backend/diagrams/backend-dependency-flow.mmd"
  - "backend/diagrams/application-state-composition.mmd"
uncertainty_markers:
  - "PERSISTENCE_BOUNDARY_UNCLEAR PBU-01"
  - "BUSINESS_RULE_LOCATION_UNCLEAR BRU-02"
  - "UNKNOWN"
---

# Backend Persistence Access

## Persistence Components

PostgreSQL is the principal persistent store. SQLx provides the connection pool, query execution, transactions, and embedded migration runner. Redis supplies cache and counter state. The configured upload directory stores media files, while media records remain in PostgreSQL. Page preview broadcast channels are process-local state and are not durable persistence.

## Connection and Migration Lifecycle

`backend/src/db/mod.rs` parses `DATABASE_URL` and creates a lazy `PgPool` capped at ten connections in code. Startup runs SQLx migrations before serving requests. The pool is cloned into `AppState` and extracted by handlers or passed into services. Migration failure prevents startup; lazy pool creation means URL preparation does not itself prove database reachability.

## Access Pattern

SQL is written directly in route modules and service modules using SQLx `query`, `query_as`, and `query_scalar` patterns. There is no repository or data-access-object layer consistently mediating persistence. This makes the SQL call site the authoritative application evidence for selection, mapping, transaction scope, and error propagation (`PBU-01`).

## Model Coverage and Mapping

`backend/src/models` contains records for user, organization, content, media, and page concepts. Many feature-specific request/response and persistence mappings, especially Marketplace structures, are defined in routes or services instead. Models are therefore partial shared structures rather than a complete persistence domain model (`BRU-02`).

SQLx performs row mapping through typed tuples/structs and JSON values. Without compile-time offline SQL metadata in the inspected source, many query/schema mismatches are discovered at runtime or by tests that execute against PostgreSQL.

## Transaction Handling

Explicit `pool.begin()` transactions are used in selected multi-statement workflows, including bootstrap and tenant/domain operations. RLS helpers may establish transaction-scoped tenant settings before queries. Other handlers execute independent statements directly against the pool. There is no backend-wide unit-of-work abstraction, so atomicity must be reviewed per operation.

## Tenant Isolation

Tenant middleware resolves an organization context, and `services/rls.rs` provides database-context helpers for operations that use RLS. Application authorization checks also appear in handlers and services. The existence of tenant context does not prove every query is RLS-scoped; detailed policy and table coverage require Phase 4 database analysis and later security validation.

## Persistence by Module Family

| Family | High-level persisted concepts | Additional state |
|---|---|---|
| Identity and organizations | Users, roles/sessions, organizations, memberships, invitations | Login/rate counters in Redis where used |
| Billing and operations | Plans/subscriptions, quota usage, beta/release records | Stripe provider state |
| Content and collaboration | Content types, entries, workflow state, comments | Delivery cache in Redis |
| Media | Metadata and ownership records | Files in upload directory |
| Pages and plugins | Pages, builder/plugin-related configuration | Redis delivery cache and process-local preview channels |
| CMS webhooks | Webhook registrations and delivery-related records | Outbound target state |
| Marketplace | Creator artifacts, catalog/installations, runtime/adapter records, finance, feedback, analytics/readiness | Package/file/provider state depending on feature |

This is intentionally not a table or schema reference; detailed storage contracts are deferred to Phase 4.

## Error Mapping

`From<sqlx::Error> for AppError` maps missing rows to `NotFound`, PostgreSQL unique-violation code `23505` to `Conflict`, and other SQLx errors to `Internal`. Some call sites intercept database errors for domain-specific mapping. The default conversion includes database-provided text in the response message, which is recorded as a risk in [Error Handling](error-handling.md).

## Tests and Verification

The backend CI config starts PostgreSQL and Redis services, runs migrations through application/test workflows, and runs Rust checks/tests. Many unit tests exercise pure/service behavior without a database; no dedicated `backend/tests` integration suite was found. Phase 3 did not execute destructive persistence tests or inspect deployed data.

## Risks and Unknowns

- Distributed SQL and partial model ownership increase change impact for schema evolution.
- Database and filesystem writes cannot participate in one native transaction.
- Redis and preview state have different durability and replication semantics from PostgreSQL.
- Connection-pool sizing is fixed in source and not environment-configurable.
- Production backup, restore, retention, replication, and migration rollback practices are `UNKNOWN`.
- Exact RLS coverage, constraints, indexes, triggers, and query performance are Phase 4 topics.

## Related Documentation

See [Configuration and State](configuration-and-state.md), [Tenant Authorization and RLS](modules/tenant-authorization.md), the [module catalog](module-catalog.md), and [Backend Risks](backend-risks.md).

