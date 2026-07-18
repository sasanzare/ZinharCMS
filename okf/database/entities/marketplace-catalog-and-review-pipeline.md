---
okf_document_id: "database-entity-marketplace-catalog-review"
title: "Marketplace Catalog and Review Pipeline"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-014"
entity_name: "Marketplace Catalog and Review Pipeline"
entity_domain: "Marketplace Catalog"
schema_objects: ["marketplace_listings", "marketplace_versions", "marketplace_submissions", "marketplace_review_events"]
owning_module: "Marketplace Catalog and Review"
tenant_scope: "global"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0015_v3_phase_one_marketplace_foundation.sql", "backend/migrations/0016_v3_phase_two_creator_submission.sql", "backend/migrations/0017_v3_phase_three_validation_pipeline.sql", "backend/migrations/0018_v3_phase_four_review_moderation.sql", "backend/src/routes/marketplace.rs", "backend/src/services/marketplace_catalog.rs", "backend/src/services/marketplace_submission.rs", "backend/src/services/marketplace_review.rs"]
related_documents: ["database/lifecycle-and-auditing.md", "database/constraints-and-indexes.md", "backend/modules/marketplace-creator-review.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["CONSTRAINT_COVERAGE_UNCLEAR CCU-02", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# Marketplace Catalog and Review Pipeline

## Entity Identity

`DB-ENT-014` spans creator-owned listings, package versions, review submissions, and moderation events. It is global Marketplace catalog/control data.

## Purpose

Verified purpose: describe products, version artifacts/manifests, validate submissions, moderate release state, and preserve review events. Package execution is not implied.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| listing ID/creator/slug/type | UUID FKs/text/checks | Catalog identity and ownership |
| listing status/content/pricing | Constrained text/JSON/numeric | Publication and product metadata |
| version listing/semantic version/artifact | UUID/text/hash/path/JSON | Package identity and immutable payload |
| validation/risk/status fields | Checked text/JSON/time | Automated pipeline result |
| submission/review actor/status | UUID/FKs/text/time | Moderation workflow |
| review event action/reason/metadata | Text/JSON/time | Append-style review history |

## Identifiers

UUID PKs identify all records. Listing slugs and version numbers are catalog-qualified as migrations define; artifact hashes identify content but are not row PKs.

## Relationships

Creators own listings; listings have versions; versions have submissions; submissions have review events. Installations, purchases, reviews, and abuse reports reference catalog rows.

## Ownership

Creator, catalog, validation, and administrative review services share lifecycle ownership.

## Tenant Isolation

These are global catalog/control tables, not forced-RLS tenant rows. Privileged bypass reads/writes are used in selected flows.

## Lifecycle

Checked text statuses cover creator/catalog/validation/moderation transitions. A trigger prevents protected artifact mutations after submitted/validating/approved/deprecated/blocked states. Review events preserve history.

## Constraints and Indexes

Creator/listing/version FKs, catalog/version uniqueness, status/risk checks, artifact immutability, queue indexes, and catalog search indexes provide integrity.

## Persistence Mapping

Marketplace routes/services use local SQLx row types and explicit transactions; no shared schema model module exists.

## Security and Privacy

Artifacts, manifests, validation output, reviewer identity, and moderation reasons are sensitive until/publication according to application policy.

## Known Risks and Unknowns

Status mapping drift (`CCU-02`), artifact storage retention, review-event immutability scope, and catalog rollback policy are unresolved.

## Related Documents

See [Marketplace Creators](marketplace-creators.md), [Marketplace Installations and Runtime Adapters](marketplace-installations-and-runtime-adapters.md), and [Lifecycle and Auditing](../lifecycle-and-auditing.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with [Marketplace Creator and Review](../../backend/modules/marketplace-creator-review.md) and [Marketplace Catalog and Installation](../../backend/modules/marketplace-catalog-installation.md).
