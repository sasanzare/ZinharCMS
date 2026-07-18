---
okf_document_id: "database-readme"
title: "Database Architecture"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/migrations"
  - "backend/src/db/mod.rs"
  - "backend/src/models"
  - "backend/src/routes"
  - "backend/src/services"
related_documents:
  - "database/overview.md"
  - "backend/persistence-access.md"
  - "architecture/components.md"
related_diagrams:
  - "database/diagrams/database-domain-map.mmd"
  - "database/diagrams/entity-relationship-overview.mmd"
uncertainty_markers:
  - "SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01"
  - "UNKNOWN U-02"
---

# Database Architecture

## Purpose and Scope

This section documents the PostgreSQL schema and persistence behavior evidenced by the repository at the Phase 5 source snapshot. It covers connection setup, all 26 SQLx migrations, 51 application tables, major entity aggregates, 55 significant relationship groups, constraints, indexes, transactions, tenant isolation, lifecycle behavior, seeds, tests, and persistence-to-code mapping.

This is descriptive documentation. OKF is not an executable schema, migration plan, or substitute for inspecting SQL. Migration files define intended history; models describe only selected application shapes; queries establish actual access behavior; and these documents connect those sources without overriding them. Because deployed migration state was not available, object presence in a running environment remains `SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01` and `UNKNOWN U-02`.

## Source-of-Truth Policy

Use current migrations first, followed by constraints and indexes, current queries, models, tests and fixtures, runtime configuration, and existing documentation. Record conflicts instead of reconciling them silently. Review migrations chronologically because later files alter earlier definitions.

## Recommended Reading Order

1. [Overview](overview.md)
2. [Technology and Configuration](technology-and-configuration.md)
3. [Schema Catalog](schema-catalog.md)
4. [Entity Catalog](entity-catalog.md)
5. [Relationships](relationships.md)
6. [Module Data Ownership](module-data-ownership.md)
7. [Migrations](migrations.md)
8. [Constraints and Indexes](constraints-and-indexes.md)
9. [Transactions and Consistency](transactions-and-consistency.md)
10. [Multi-Tenancy](multi-tenancy.md)
11. [Lifecycle and Auditing](lifecycle-and-auditing.md)
12. [Persistence Mapping](persistence-mapping.md)
13. [Seeds and Fixtures](seeds-and-fixtures.md)
14. [Database Testing](database-testing.md)
15. [Database Risks](database-risks.md)

## Entity Documents

- [Identity and Global RBAC](entities/identity-and-global-rbac.md)
- [Organizations and Membership](entities/organizations-and-membership.md)
- [Content Types and Entries](entities/content-types-and-entries.md)
- [Pages and Versions](entities/pages-and-versions.md)
- [Component and Plugin Registry](entities/component-and-plugin-registry.md)
- [Media and Variants](entities/media-and-variants.md)
- [Editorial Comments](entities/editorial-comments.md)
- [Public Settings and Navigation](entities/public-settings-and-navigation.md)
- [CMS Webhooks and Deliveries](entities/cms-webhooks-and-deliveries.md)
- [Plans, Subscriptions, and Usage](entities/plans-subscriptions-and-usage.md)
- [SaaS Operations and Audit](entities/saas-operations-and-audit.md)
- [Beta Release Records](entities/beta-release-records.md)
- [Marketplace Creators](entities/marketplace-creators.md)
- [Marketplace Catalog and Review Pipeline](entities/marketplace-catalog-and-review-pipeline.md)
- [Marketplace Installations and Runtime Adapters](entities/marketplace-installations-and-runtime-adapters.md)
- [Marketplace Purchases and Entitlements](entities/marketplace-purchases-and-entitlements.md)
- [Marketplace Ledger and Payouts](entities/marketplace-ledger-and-payouts.md)
- [Marketplace Reviews and Abuse](entities/marketplace-reviews-and-abuse.md)

## Diagrams

- [Database Domain Map](diagrams/database-domain-map.mmd)
- [Entity Relationship Overview](diagrams/entity-relationship-overview.mmd)
- [Module Data Ownership](diagrams/module-data-ownership.mmd)
- [Tenant Isolation](diagrams/tenant-isolation.mmd)
- [Migration Lifecycle](diagrams/migration-lifecycle.mmd)

## Related Architecture

Read [Architecture Components](../architecture/components.md), [Architecture Boundaries](../architecture/boundaries.md), [Backend Module Catalog](../backend/module-catalog.md), [Backend Module Boundaries](../backend/module-boundaries.md), and [Backend Persistence Access](../backend/persistence-access.md) for the application-side context.

## For AI Coding Agents

1. Start with `schema-catalog.md`.
2. Read the relevant entity document.
3. Read `relationships.md`.
4. Check `module-data-ownership.md`.
5. Review migrations in chronological order.
6. Verify models against migrations.
7. Verify query behavior against repository or raw SQL code.
8. Review multi-tenancy rules before changing tenant-scoped data.
9. Review constraints and indexes before introducing duplicated validation.
10. Never edit an existing applied migration unless the project explicitly allows it.
11. Prefer a new migration for schema changes.
12. Update related OKF documents when database behavior changes.
13. Never invent data-retention, deletion, publication, or security rules.

## Deferred Topics

Phase 6 owns the endpoint-level API contract. Later phases own the complete security model, business-rule governance, Marketplace extensibility, and production operations. Phase 5 does not establish deployed schema state, backup/restore guarantees, retention policy, or production capacity.
