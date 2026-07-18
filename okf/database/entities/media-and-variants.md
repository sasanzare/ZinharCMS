---
okf_document_id: "database-entity-media-variants"
title: "Media and Variants"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-006"
entity_name: "Media and Variants"
entity_domain: "Media"
schema_objects: ["media", "media_variants"]
owning_module: "Media"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0001_initial_schema.sql", "backend/migrations/0003_phase_one_core.sql", "backend/migrations/0008_v2_phase_one_organizations.sql", "backend/src/routes/media.rs", "backend/src/services/media_processing.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/lifecycle-and-auditing.md", "backend/modules/media.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["TRANSACTION_BOUNDARY_UNCLEAR TBU-01", "MIGRATION_MODEL_CONFLICT MMC-02", "MODULE_OWNERSHIP_UNCLEAR MOU-04"]
---

# Media and Variants

## Entity Identity

`DB-ENT-006` represents database metadata for tenant media files and derived variants. Binary content resides in the configured filesystem, not in these rows.

## Purpose

Verified purpose: track uploaded file identity, metadata, storage path, dimensions/type, and processed variants. Production object-storage/CDN behavior is unknown.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| IDs and `organization_id` | UUID/FKs | Tenant media identity |
| filename/path/MIME/size | Text/numeric constraints | Filesystem locator and content metadata |
| dimensions/metadata | Numeric/JSONB nullable | Media-specific details |
| variant media/org/kind/path | UUID FKs/text | Derived artifact identity |
| lifecycle timestamps | Time | Creation/update context |

## Identifiers

Media and variants use UUID PKs. Variant uniqueness combines parent media and transformation/name fields as migration-defined; paths are storage locators, not stable public IDs.

## Relationships

Organizations own media; media has variants. A trigger derives variant organization context. Content/page JSON may reference media logically without conventional FKs.

## Ownership

Media routes/services own metadata and file operations. Public serving and processing also participate (`MOU-04`).

## Tenant Isolation

Both tables are forced RLS. Explicit tenant predicates and variant propagation supplement policy checks; same-tenant parent coherence still requires tests.

## Lifecycle

Database row and filesystem deletion/creation are separate resources. No uniform trash, retention, or orphan cleanup contract is established.

## Constraints and Indexes

File metadata checks, parent FKs, variant uniqueness, organization/media lookup indexes, and RLS enforce selected invariants.

## Persistence Mapping

Media code uses direct SQLx and filesystem APIs. Shared media/variant models omit tenant columns (`MMC-02`).

## Security and Privacy

Paths, original filenames, MIME declarations, and uploaded bytes are security-sensitive. Database metadata does not prove file safety or access control.

## Known Risks and Unknowns

Filesystem/DB atomicity is `TBU-01`; orphan cleanup, production storage topology, retention, and same-tenant variant enforcement are unresolved.

## Related Documents

See [Transactions and Consistency](../transactions-and-consistency.md), [Multi-Tenancy](../multi-tenancy.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Media module](../../backend/modules/media.md).
