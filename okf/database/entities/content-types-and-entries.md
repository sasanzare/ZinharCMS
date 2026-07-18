---
okf_document_id: "database-entity-content-types-entries"
title: "Content Types and Entries"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-003"
entity_name: "Content Types and Entries"
entity_domain: "CMS Content"
schema_objects: ["content_types", "content_entries"]
owning_module: "Content"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0001_initial_schema.sql", "backend/migrations/0003_phase_one_core.sql", "backend/migrations/0006_phase_six_workflow_collaboration.sql", "backend/migrations/0008_v2_phase_one_organizations.sql", "backend/src/routes/content.rs"]
related_documents: ["database/multi-tenancy.md", "database/lifecycle-and-auditing.md", "backend/modules/content-workflow.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["MIGRATION_MODEL_CONFLICT MMC-02", "CONSTRAINT_COVERAGE_UNCLEAR CCU-01"]
---

# Content Types and Entries

## Entity Identity

`DB-ENT-003` represents tenant content schemas and their publishable records. Shared content models are partial; routes also use local projections. Implementation is verified with high repository confidence.

## Purpose

Verified purpose: define JSON-oriented content structures and store tenant-scoped content with workflow/publication state. Product rationale for arbitrary schema evolution is not established (`NOC-12`).

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| IDs and `organization_id` | UUID, non-null after tenancy migration, FKs | Tenant identity |
| type `name`, `slug`, schema | Text/JSONB with tenant slug uniqueness | Content definition |
| entry `type_id`, data | UUID FK/JSONB | Typed record payload |
| `status`, `version`, publish fields | `content_status`, numeric/time | Workflow, publication, data version |
| author/editor timestamps | UUID/time where defined | Actor/lifecycle context |

## Identifiers

Both tables use UUID PKs. Content type slugs are unique within an organization. Entry IDs are stable database identifiers; no global slug contract is inferred.

## Relationships

An organization has many content types and entries; a content type has many entries. Comments may reference entries polymorphically, and delivery/webhook behavior consumes published content.

## Ownership

Content owns writes; workflow/validation, delivery invalidation, comments, and audit paths participate.

## Tenant Isolation

Both tables have `organization_id`, explicit tenant predicates, and forced RLS. The entry tenant is also derived by a trigger in selected writes. Parent/tenant coherence still requires testing (`CCU-01`).

## Lifecycle

`content_status` provides draft, pending review, published, and archived states. Publication timestamps and numeric version fields support lifecycle, but no universal optimistic-lock or retention rule is established.

## Constraints and Indexes

Tenant-qualified slug uniqueness, status enum, FKs, JSONB validation, and organization/status/type lookup indexes support primary access paths.

## Persistence Mapping

Content routes use direct SQLx, validation/workflow services, partial shared models, and local row shapes. Shared models omit `organization_id` (`MMC-02`).

## Security and Privacy

Content payloads are arbitrary JSONB and may contain sensitive tenant data. RLS and field-level product policy are separate concerns.

## Known Risks and Unknowns

Schema evolution, retention, same-tenant parent coherence, and the semantic role of `version` require explicit ownership.

## Related Documents

See [Relationships](../relationships.md), [Multi-Tenancy](../multi-tenancy.md), and [Persistence Mapping](../persistence-mapping.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Content and Workflow module](../../backend/modules/content-workflow.md).
