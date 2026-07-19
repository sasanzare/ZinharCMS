---
okf_document_id: "marketplace-area-feedback-moderation-analytics"
title: "Marketplace Feedback, Moderation, and Analytics"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_analytics.rs"
  - "backend/src/services/marketplace_feedback.rs"
  - "backend/src/services/marketplace_analytics.rs"
  - "backend/migrations/0024_v3_phase_ten_ratings_abuse.sql"
  - "frontend/src/pages/MarketplacePage.tsx"
marketplace_area_id: "MPA-006"
marketplace_area_name: "Feedback Moderation and Analytics"
implementation_status: "implemented"
related_documents:
  - "../marketplace-architecture.md"
  - "../marketplace-workflows.md"
  - "package-validation-review.md"
related_diagrams:
  - "../diagrams/extensibility-context.mmd"
---

# Marketplace Feedback, Moderation, and Analytics

Eligible organizations can submit product ratings/reviews after install or purchase and can report abuse with structured evidence. Moderators publish/reject reviews and investigate/resolve/dismiss reports. Critical reports create internal operational signals.

Creator analytics aggregate listing, install, revenue, conversion, rating, report, and validation signals. Administrative analytics expose submission, approval-time, install, refund, abuse, and blocked-package health indicators.

These are host-owned Marketplace governance capabilities, not plugin hooks. Query correctness is supported by service tests, while live database scale, privacy/retention policy, and operational response SLAs remain UNKNOWN.

See [Marketplace Architecture](../marketplace-architecture.md).

## Purpose

Collect eligible customer feedback and abuse evidence, support moderation, and expose creator/platform health signals.

## Entities

marketplace_product_reviews, marketplace_abuse_reports, listing/version/install/purchase eligibility data, audit/operational signals, and aggregate analytics projections.

## Backend Module

Marketplace core review/report handlers, marketplace_feedback.rs, marketplace_analytics.rs, analytics routes, and readiness/performance helpers.

## APIs

Submit/list/moderate reviews, submit/list/resolve abuse reports, and creator/admin analytics endpoints.

## Frontend Feature

MarketplacePage contains rating/report forms, moderator queues, creator analytics, and administrative health views.

## Permissions

Authenticated organization members can report abuse; review eligibility requires install or purchase; global admins moderate and view platform analytics; creators view their metrics.

## Tenant Scope

Submissions carry organization ownership; publication/moderation and aggregate admin analytics cross product/global boundaries through authorized host queries.

## Workflows

MP-WF-13 and moderation/analytics portions of MP-WF-14.

## Tests

Feedback, analytics, phase-ten/eleven/thirteen helpers, and MarketplacePage tests cover validation, eligibility, moderation, and displayed metrics.

## Risks

Live RLS/query scale, privacy/retention, abuse-response SLA, aggregation freshness, and operational ownership remain UNKNOWN or NEEDS_OWNER_CONFIRMATION.

## Implementation Status

IMPLEMENTED host-owned feedback, moderation, and analytics contracts.
