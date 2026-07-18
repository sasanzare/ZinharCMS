---
okf_document_id: "database-overview"
title: "Database Overview"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/db/mod.rs", "backend/src/main.rs", "backend/src/routes", "backend/src/services"]
related_documents: ["database/schema-catalog.md", "database/persistence-mapping.md", "database/database-risks.md"]
related_diagrams: ["database/diagrams/database-domain-map.mmd"]
uncertainty_markers: ["SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01", "PERSISTENCE_BOUNDARY_UNCLEAR PBU-01", "TEST_ISOLATION_UNCLEAR TIU-01"]
---

# Database Overview

## 1. Database Identity

| Concern | Verified implementation |
| --- | --- |
| Engine | PostgreSQL; version 16 is used by Compose and CI |
| Driver and client | SQLx 0.8 with the PostgreSQL, UUID, Chrono, JSON, migration, and Tokio Rustls features |
| ORM/query builder | No full ORM; compile-time/runtime SQLx calls, raw SQL, and `QueryBuilder<Postgres>` |
| Migration tool | Embedded SQLx migrator over `backend/migrations` |
| Pool | SQLx `PgPool`, created lazily and capped at 10 connections |
| Configuration | `DATABASE_URL`, backend configuration, Compose, and CI workflow files |
| Status | Repository implementation is `VERIFIED`; deployed object state is `SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01` |

## 2. Database Responsibilities

Verified implemented domains are identity, global roles, organizations and membership, CMS content, pages and revisions, components, media metadata, editorial comments, plugins, delivery webhooks, public settings, navigation, plans and subscriptions, usage, billing events, SaaS operations, beta records, and Marketplace catalog, installation, runtime policy, finance, reviews, and abuse operations.

Partially implemented domains include custom-domain operation, automatic SaaS alert execution, external critical-abuse notification delivery, automated payout settlement, and production-grade media/artifact delivery. Their records exist, but end-to-end behavior is incomplete or unverified. No additional planned database domain is represented as implemented merely because a product document mentions it.

## 3. Schema Organization

The migration set creates one shared PostgreSQL database and relies on the default namespace; no explicit application namespace or separate tenant database was found. Tables are domain-named rather than prefixed uniformly. Global identity/catalog tables and tenant-scoped tables coexist in the shared schema. Tenant isolation combines `organization_id`, explicit query filters, membership checks, and forced RLS on 32 tables. This is a shared-database, shared-schema design, not a database-per-tenant design.

## 4. Persistence Architecture

Startup parses `DATABASE_URL`, creates a lazy pool, runs embedded migrations, and seeds a bootstrap administrator only when the user table is empty. Migration failure prevents HTTP startup. Handlers and services use the shared pool directly; there is no mandatory repository/DAO boundary. SQL results map to shared structs in some CMS areas and route- or service-local row types elsewhere. Transactions are explicit around multi-write flows, with tenant or bypass context established through helper functions. No dedicated database fixture/reset harness or separate backend integration suite was found.

## 5. Major Data Domains

| Domain | Representative objects | Important behavior |
| --- | --- | --- |
| Identity and access | `users`, `roles`, `user_roles`, tokens, login attempts | Global identities and role grants |
| Tenancy | organizations, memberships, invitations | Tenant root and request membership context |
| CMS | content types/entries, pages/versions, components, media, comments | Publishing, version history, asset metadata |
| Delivery | settings, navigation, webhooks/deliveries | Public configuration and outbound event records |
| SaaS operations | plans, subscriptions, usage, billing, domains, audit, alerts, beta | Commercial and operational control data |
| Marketplace | creators, catalog, reviews, installations, adapters, finance, abuse | Catalog-to-runtime and commercial lifecycle |

## 6. Main Database Risks

- Runtime migration state and drift are unknown (`UNKNOWN U-02`).
- Shared model coverage conflicts with migration-defined tenant columns, and `PageStatus` lacks `pending_review` (`MMC-01`, `MMC-02`).
- Tenant-child rows often have both a tenant FK and a parent FK without a composite constraint proving tenant coherence (`TIV-01`, `CCU-01`).
- Direct SQL is distributed across routes and services (`PBU-01`).
- Media filesystem writes, provider calls, post-commit audit writes, and spawned webhooks cross transaction boundaries (`TBU-01` through `TBU-04`).
- Retention, backup/restore, and mixed deletion behavior are not governed (`DLU-01`, `DLU-02`, `UNKNOWN U-04`, `UNKNOWN U-07`).
- Database integration and RLS regression evidence is limited (`TIU-01`, `DCC-12`).

## 7. Known Unknowns

- Which migrations are applied in each deployed environment?
- What backup, restore, RPO, RTO, and restore-test procedures exist?
- What retention/privacy requirements apply to audit, login, delivery, billing, beta, and abuse data?
- Which module owns schema-change review and production migration execution?
- Are isolation levels or pool settings overridden through deployment-only connection options?
- Is the public tenant selected by host, domain record, or another production gateway contract?
