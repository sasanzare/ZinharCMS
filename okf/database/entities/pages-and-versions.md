---
okf_document_id: "database-entity-pages-versions"
title: "Pages and Versions"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-004"
entity_name: "Pages and Versions"
entity_domain: "CMS Pages"
schema_objects: ["pages", "page_versions"]
owning_module: "Pages and Builder"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0001_initial_schema.sql", "backend/migrations/0004_phase_two_page_builder.sql", "backend/migrations/0006_phase_six_workflow_collaboration.sql", "backend/src/routes/pages.rs", "backend/src/models/page.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/persistence-mapping.md", "backend/modules/pages-builder-preview.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["MIGRATION_MODEL_CONFLICT MMC-01", "MIGRATION_MODEL_CONFLICT MMC-02", "CONSTRAINT_COVERAGE_UNCLEAR CCU-01"]
---

# Pages and Versions

## Entity Identity

`DB-ENT-004` covers tenant pages and their persisted builder snapshots. Pages owns the aggregate; local SQL rows coexist with partial shared models.

## Purpose

Verified purpose: persist page identity, layout/content JSON, publication status, and restorable version snapshots. Preview/broadcast state itself is process-local, not stored here.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| page ID, organization, slug/title | UUID/text; tenant-qualified slug | Tenant page identity |
| builder/content/SEO fields | JSONB/text | Page document and metadata |
| `status`, publication times | `page_status`/time | Workflow/publication |
| version page/org/snapshot | UUID FKs/JSONB | Historical page snapshot |
| version number/actor/time | Numeric/UUID/time | Ordered revision context |

## Identifiers

Pages and versions use UUID PKs. Page slug is tenant-qualified. Version ordering/number is page-relative where constrained by the migration.

## Relationships

An organization has pages; a page has versions and may be referenced by navigation, comments, and Marketplace template imports. Page versions also carry organization context.

## Ownership

Pages owns writes. Builder, preview, delivery, plugins, comments, and Marketplace runtime adapters read or extend behavior.

## Tenant Isolation

Both tables are forced-RLS tenant tables. A trigger derives version organization context. Separate page and organization FKs leave same-tenant coherence requiring tests (`CCU-01`).

## Lifecycle

Create/update/restore transactions write a page and snapshot atomically. Status supports draft, pending review, published, and archived at the database layer. Broadcast/invalidation happens after commit.

## Constraints and Indexes

Tenant slug uniqueness, status enum, page-version FK/indexes, JSONB checks, and RLS constrain access and history.

## Persistence Mapping

Page routes frequently map status as `String`. Shared `PageStatus` lacks `PendingReview` (`MMC-01`), and shared page/version structs omit tenant columns (`MMC-02`).

## Security and Privacy

Builder JSON and SEO/configuration may expose unpublished content. Preview access and RLS require separate enforcement.

## Known Risks and Unknowns

Model/enum drift, tenant-parent coherence, revision retention, and content-schema compatibility remain open.

## Related Documents

See [Transactions and Consistency](../transactions-and-consistency.md), [Lifecycle and Auditing](../lifecycle-and-auditing.md), and [Component and Plugin Registry](component-and-plugin-registry.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Pages, Builder, and Preview module](../../backend/modules/pages-builder-preview.md).
