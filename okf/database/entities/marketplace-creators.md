---
okf_document_id: "database-entity-marketplace-creators"
title: "Marketplace Creators"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-013"
entity_name: "Marketplace Creators"
entity_domain: "Marketplace Creator"
schema_objects: ["marketplace_creators"]
owning_module: "Marketplace Creator"
tenant_scope: "global"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0015_v3_phase_one_marketplace_foundation.sql", "backend/migrations/0016_v3_phase_two_creator_submission.sql", "backend/src/routes/marketplace.rs", "backend/src/services/marketplace_catalog.rs"]
related_documents: ["database/entity-catalog.md", "database/module-data-ownership.md", "backend/modules/marketplace-creator-review.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["MODULE_OWNERSHIP_UNCLEAR MOU-05", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# Marketplace Creators

## Entity Identity

`DB-ENT-013` is the global Marketplace creator profile and approval/provider state. Marketplace-local row types map the table.

## Purpose

Verified purpose: associate a creator identity with catalog ownership, review status, profile data, and payout/provider setup. Approval policy rationale is not inferred.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| creator ID and user | UUID PK/non-null FK | Creator identity and platform account link |
| display/profile/contact | Text/JSON-like fields | Public and operational profile |
| status/review fields | Constrained text/time/actor | Creator admission lifecycle |
| payout readiness | Constrained status | Commercial setup state |
| timestamps | Time | Lifecycle |

## Identifiers

Creator UUID is primary. User linkage is unique, and the lowercased slug has a unique index. Payout-provider identifiers live in the separate payout-account entity rather than this table.

## Relationships

A creator links to exactly one platform user through a unique non-null FK and owns many listings, payout accounts, payouts, and attributed ledger rows.

## Ownership

Marketplace creator/review services own writes; catalog and finance are major readers.

## Tenant Isolation

Creators are global Marketplace control-plane rows and are not forced-RLS tenant data. Access requires creator/admin authorization.

## Lifecycle

Status and reviewer timestamps express onboarding/approval. Deletion, anonymization, and provider unlink behavior are not governed.

## Constraints and Indexes

PK, user/profile/provider uniqueness where defined, status checks, and review/catalog lookup indexes support creator flows.

## Persistence Mapping

Marketplace routes/services use direct SQLx and local row types; there is no shared Marketplace model module.

## Security and Privacy

Contact, identity, review, and provider account data are sensitive. Public profile and private operational fields must not be conflated.

## Known Risks and Unknowns

Retention, creator-user deletion behavior, ownership transfer, and provider reconciliation are unresolved.

## Related Documents

See [Marketplace Catalog and Review Pipeline](marketplace-catalog-and-review-pipeline.md), [Marketplace Ledger and Payouts](marketplace-ledger-and-payouts.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Marketplace Creator and Review module](../../backend/modules/marketplace-creator-review.md).
