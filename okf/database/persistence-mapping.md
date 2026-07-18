---
okf_document_id: "database-persistence-mapping"
title: "Database Persistence Mapping"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/src/models", "backend/src/routes", "backend/src/services", "backend/src/db/mod.rs"]
related_documents: ["database/schema-catalog.md", "database/module-data-ownership.md", "backend/persistence-access.md"]
related_diagrams: ["database/diagrams/module-data-ownership.mmd"]
uncertainty_markers: ["PERSISTENCE_BOUNDARY_UNCLEAR PBU-01", "MIGRATION_MODEL_CONFLICT MMC-01", "MIGRATION_MODEL_CONFLICT MMC-02", "BUSINESS_RULE_LOCATION_UNCLEAR BRU-02"]
---

# Database Persistence Mapping

## Access Pattern

The backend shares a SQLx `PgPool` through application state. Route handlers and services call SQLx directly; no mandatory repository or DAO layer was found. Source occurrence inventory at the snapshot found 71 `sqlx::query(` calls, 175 `query_as` calls, and 44 `query_scalar` calls, for 290 direct SQLx query-call occurrences. Counts describe source text, not runtime query volume.

Most SQL is static. Delivery uses `QueryBuilder<Postgres>` for a dynamic query, while content/pages contain formatted dynamic SQL paths. Migrations, raw queries, triggers, and policies must all be searched before classifying a column or table as unused.

## Domain Mapping

| Domain | Persistence location | Mapping style |
| --- | --- | --- |
| Auth/identity | auth routes/services and shared models | Shared structs plus direct SQLx |
| Organizations/SaaS | organization routes, middleware, billing and operational services | Local row/request types plus direct SQLx |
| Content | content routes, validation/workflow/delivery services | Partial shared models and local projections |
| Pages/builder | page routes and page/adapter services | Partial shared models and local string-status rows |
| Media | media routes/services | Shared partial structs plus filesystem operations |
| Delivery/settings | delivery/webhook/settings routes | Direct projections and dynamic query building |
| Beta | beta routes/services | Route/service-local persistence rows |
| Marketplace | Marketplace route and service modules | Predominantly route/service-local row types |

## Entity-to-Code Mapping Matrix

No row has a mandatory repository abstraction; the repository column is therefore “none” and direct SQLx is the verified access method.

| Entity/table group | Model or database row type | Query/service/handler | Migration(s) | Test evidence |
| --- | --- | --- | --- | --- |
| Identity/RBAC | Shared user/auth types plus auth-local rows | `routes/auth.rs`; JWT/password/RBAC services | 0001, 0002, 0007 | Pure/auth-adjacent tests; no shared DB harness |
| Organizations/membership | Organization route-local/shared projections | `routes/organizations.rs`; tenant middleware | 0008, 0012 | Static/unit evidence; fixture covers selected tenant rows |
| Content types/entries | Partial shared `ContentType`/`ContentEntry`; local projections | `routes/content.rs`; entry validation/workflow | 0001, 0003, 0006, 0008, 0009 | Validation tests; no route DB suite |
| Pages/versions | Partial shared `Page`/`PageVersion`; local String-status rows | `routes/pages.rs`; Marketplace adapters | 0001, 0004, 0006, 0008, 0009, 0021 | Pure/page tests plus frontend; no DB transaction harness |
| Components/plugins | Partial `ComponentRegistryItem`; plugin-local rows | pages/plugin routes and built-in plugin code | 0001, 0004, 0006, 0008, 0009 | Static/unit evidence |
| Media/variants | Partial shared media structs | `routes/media.rs`; `media_processing.rs`; filesystem | 0001, 0003, 0008, 0009 | Processing tests; DB/file atomicity untested |
| Comments | Route-local query/response rows | `routes/comments.rs`; workflow/authorization | 0006, 0008, 0009 | No dedicated real-DB suite found |
| Settings/navigation/webhooks | Delivery/webhook local rows and projections | delivery/webhook routes and `services/webhooks.rs` | 0005, 0008, 0009 | Pure/static evidence; no durable-delivery DB suite |
| Billing/SaaS/beta | Route/service-local rows | billing, organization, beta routes; quota/rate/Stripe/audit services | 0010–0014 | Pure rules/static evidence; no provider DB suite |
| Marketplace creator/catalog/review | Marketplace-local row structs | Marketplace routes; catalog/submission/validation/review services | 0015–0018 | Numerous pure/service tests; no shared DB reset |
| Marketplace installation/runtime | Marketplace-local row structs | installation/policy/runtime/adapter services | 0019–0021 | Filesystem artifact and pure policy tests |
| Marketplace finance/trust | Marketplace-local finance/feedback rows | finance, Stripe billing, feedback/analytics services | 0022–0026 | Pure/service tests; concurrency/RLS DB coverage absent |

## Scalar and Custom-Type Mapping

| PostgreSQL representation | Rust/serialization mapping | Status |
| --- | --- | --- |
| `UUID` | `uuid::Uuid` or optional UUID in row/request structs | Verified broadly |
| `TIMESTAMPTZ` | Chrono date/time types, usually UTC-aware | Verified broadly |
| `JSONB` | `serde_json::Value` or typed request/response structures at boundaries | Manual mapping; shape constraints vary |
| `TEXT[]` | String vectors/arrays for permissions, hooks, or events | Manual mapping |
| PostgreSQL enums | Shared SQLx enum mapping in selected models, local strings elsewhere | Mixed; `MMC-01` applies |
| Constrained text statuses | Rust strings or application enums/checks | Duplicated contract risk `CCU-02` |
| Nullable columns | `Option<T>` in compatible row types or omitted selected-column projections | Must match each query projection |

No verified model exists as a complete one-to-one mapping for every table. Conversely, a table without a shared model is still actively mapped through local SQLx row types. No generated database client, ORM relationship mapping, or repository test abstraction was found.

## Migration-Model Conflicts

`backend/src/models/page.rs::PageStatus` lacks `PendingReview`, while migration 0006 adds `pending_review` to PostgreSQL `page_status` (`MIGRATION_MODEL_CONFLICT MMC-01`). Current page queries often map status through local `String` fields, so a specific runtime failure is not asserted.

Shared `ContentType`, `ContentEntry`, `Page`, `PageVersion`, `Media`, `MediaVariant`, and `ComponentRegistryItem` structs omit migration-defined `organization_id` columns (`MIGRATION_MODEL_CONFLICT MMC-02`). Selected-column queries and local response structs often avoid full-row mapping. These structs are partial application views; they are not complete schema definitions and are not classified as dead.

Marketplace persistence has no single shared model module. Its local rows are verified implementation, but ownership and business-rule location remain distributed (`BRU-02`).

## Mapping Safety

- Treat migrations and final constraints as schema authority.
- Match SQLx enum/text mappings to database values and nullability.
- Preserve column lists; avoid assuming a shared struct represents `SELECT *`.
- Review JSONB shape validation in both SQL and application code.
- Verify RLS context before interpreting an empty result as absent data.
- Review local row types when changing columns used by Marketplace or operational flows.
- Update model, query, test, and OKF evidence together after a forward migration.
