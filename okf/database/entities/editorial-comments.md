---
okf_document_id: "database-entity-editorial-comments"
title: "Editorial Comments"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-007"
entity_name: "Editorial Comments"
entity_domain: "Editorial Collaboration"
schema_objects: ["comments"]
owning_module: "Comments and Workflow"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0006_phase_six_workflow_collaboration.sql", "backend/migrations/0008_v2_phase_one_organizations.sql", "backend/src/routes/comments.rs"]
related_documents: ["database/relationships.md", "database/multi-tenancy.md", "backend/modules/comments.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["RELATIONSHIP_UNCLEAR RLU-01", "MODULE_OWNERSHIP_UNCLEAR MOU-03"]
---

# Editorial Comments

## Entity Identity

`DB-ENT-007` is the tenant-scoped editorial discussion record. It is implemented in `comments` and mapped through comment/workflow SQL.

## Purpose

Verified purpose: attach editorial comments to CMS entities and track resolution/edit state. The schema does not define comment threading; target semantics beyond supported entity types are not inferred.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| ID, organization, author | UUID/FKs | Comment, tenant, author |
| `entity_type`, `entity_id` | Text/UUID | Polymorphic target |
| body and resolution fields | Text/actor/time | Editorial content and workflow |
| timestamps | Time | Creation/update history |

## Identifiers

The comment has a UUID PK. Target identity is the logical pair `(entity_type, entity_id)`; it is not a conventional FK.

## Relationships

Comments belong to organizations, optionally reference author/resolver users, and logically target a page or content entry. Target integrity is `RELATIONSHIP_UNCLEAR RLU-01`.

## Ownership

Comments/workflow owns writes; Content and Pages provide target authorization, creating `MOU-03`.

## Tenant Isolation

The table is forced RLS and has explicit organization context. A trigger derives tenant information from target context in selected writes, but polymorphic same-tenant behavior requires testing.

## Lifecycle

Comments can be edited/resolved and form editorial history. Retention and hard deletion are not governed.

## Constraints and Indexes

FKs protect organization, author, and resolver references; entity/tenant indexes support retrieval. Polymorphic target existence is not FK-enforced.

## Persistence Mapping

Comment routes use direct SQLx and resource-specific authorization rather than a generic repository.

## Security and Privacy

Bodies may contain unpublished or personal information. RLS does not replace target-level authorization.

## Known Risks and Unknowns

Target integrity, cross-tenant target validation, deletion effects, and retention are unresolved.

## Related Documents

See [Relationships](../relationships.md), [Multi-Tenancy](../multi-tenancy.md), and [Lifecycle and Auditing](../lifecycle-and-auditing.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Comments module](../../backend/modules/comments.md).
