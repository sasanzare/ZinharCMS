---
okf_document_id: "database-entity-marketplace-reviews-abuse"
title: "Marketplace Reviews and Abuse"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-018"
entity_name: "Marketplace Reviews and Abuse"
entity_domain: "Marketplace Trust and Feedback"
schema_objects: ["marketplace_product_reviews", "marketplace_abuse_reports", "marketplace_internal_notifications"]
owning_module: "Marketplace Feedback and Trust"
tenant_scope: "mixed"
implementation_status: "PARTIALLY_IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0024_v3_phase_ten_ratings_abuse.sql", "backend/migrations/0025_v3_phase_ten_internal_notifications.sql", "backend/src/services/marketplace_feedback.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/lifecycle-and-auditing.md", "backend/modules/marketplace-feedback-analytics-readiness.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["PLANNED_NOT_IMPLEMENTED PNI-04", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# Marketplace Reviews and Abuse

## Entity Identity

`DB-ENT-018` groups tenant/listing product reviews, abuse reports, and internal operations notifications. Database workflows are implemented; external critical notification delivery is `PNI-04`.

## Purpose

Verified purpose: collect ratings/comments, moderate abuse reports, and create internal notification records for operational attention.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| review org/listing/version/author/rating | UUID FKs/numeric check | Tenant product feedback |
| review body/status/time | Text/checks/time | Review content/lifecycle |
| report org/listing/reporter/category/severity | UUID FKs/text checks | Trust/safety intake |
| report status/resolution/actor | Text/UUID/time | Moderation lifecycle |
| notification type/subject/payload/read state | Text/JSONB/boolean/time | Internal operations record |

## Identifiers

Rows use UUID PKs. Product review uniqueness is organization/listing-qualified. Report/notification IDs are operational identifiers.

## Relationships

Reviews connect organizations, listings, optional versions, and author users. Reports connect organization, listing, optional version, and reporter. Notifications reference one abuse report through a unique non-null FK.

## Ownership

Marketplace feedback/trust services own writes; catalog, analytics, admin, and audit paths read or append related records.

## Tenant Isolation

Reviews and abuse reports are forced RLS. Internal notifications are global operational records and rely on privileged application authorization.

## Lifecycle

Review upsert maintains one tenant/listing review. Reports use status/resolution fields. Critical report creation writes report, audit, notification, and audit in one tenant transaction.

## Constraints and Indexes

Rating/status/severity checks, FKs, scoped review uniqueness, and tenant/listing/status/time indexes support feedback and moderation queues.

## Persistence Mapping

Marketplace services use direct SQLx transactions/local rows. Internal notification storage does not prove external alert delivery.

## Security and Privacy

Review bodies, report evidence, reporter identity, moderation reasons, and notification payloads may be sensitive or abusive content.

## Known Risks and Unknowns

External escalation, retention/redaction, reporter anonymity, notification delivery ownership, and moderation appeal behavior are unresolved.

## Related Documents

See [Marketplace Catalog and Review Pipeline](marketplace-catalog-and-review-pipeline.md), [Lifecycle and Auditing](../lifecycle-and-auditing.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Marketplace Feedback, Analytics, and Readiness module](../../backend/modules/marketplace-feedback-analytics-readiness.md).
