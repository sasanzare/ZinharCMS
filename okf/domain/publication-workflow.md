---
okf_document_id: "domain-publication-workflow"
title: "Publication Workflow"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/workflow.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/delivery.rs"
related_documents:
  - "state-transitions.md"
  - "content-lifecycle.md"
  - "workflows/editorial-publication.md"
related_diagrams:
  - "diagrams/publication-workflow.mmd"
---

# Publication Workflow

See the [publication workflow diagram](diagrams/publication-workflow.mmd).

## Publishable CMS Entities

| Entity | Current/required state | Actor and enforcement | Database change | Verified side effects | Tests/confidence |
| --- | --- | --- | --- | --- | --- |
| Content entry | `pending_review` to `published`, or `draft` to `published` with bypass | Organization entry publisher; handler calls shared workflow service | Set enum status, set `published_at = now()`, increment version | Redis invalidation; built-in after-publish plugin; signed publication webhook task | Workflow service unit tests; route/side-effect integration tests absent; High transition confidence |
| Page | Same state policy | Organization page publisher | Set enum status and `published_at`; no snapshot | Redis invalidation; signed webhook task; in-process preview broadcast | Workflow unit tests only; High transition confidence |
| Marketplace listing/version | Separate submission/review lifecycle | Approved creator submits; global admin reviews; validation/risk rules | Several version/submission/listing statuses and review events | Catalog visibility and installation eligibility | Strong service/static tests; see Marketplace domain; not the CMS workflow |

## Preconditions

- Authentication and active tenant membership are mandatory for CMS publication.
- Organization RBAC determines writer, reviewer, and publisher capabilities.
- The entity must exist in the selected organization and current state must permit the requested transition.
- Content/page data is validated when saved. Publish handlers do not independently repeat complete entry/page validation.

## Draft, Review, and Direct Publish

Writers can submit a draft for review. Reviewers can reject pending review to draft. Publisher-capable roles can publish from pending review and are explicitly allowed to bypass review from draft. This is code-level behavior, not evidence that review bypass is always the desired business policy.

## Unpublish, Archive, Restore, and Republish

- Unpublish is `published` to `draft`, clears the publication timestamp, invalidates cache, and emits an unpublish webhook.
- Archive is `published` to `archived`, clears the timestamp, and invalidates cache. No archive webhook is defined.
- Restore is `archived` to `draft`; it does not restore historical entry data or a page snapshot.
- A restored/unpublished draft can be published again through the same workflow. `published_at` becomes the new publication time.

## Immediate and Scheduled Publication

Publication is immediate inside the HTTP request. No schedule field, scheduler, publication job, retry job, search indexing, or durable event bus was found. Scheduling is `PLANNED_NOT_IMPLEMENTED` in the observed implementation.

## Preview and Public Delivery

Page save/transition broadcasts current JSON to process-local preview subscribers. Public delivery queries only published rows and uses Redis cache. The frontend local Page Builder preview is not public delivery and is not an authorization boundary.

## Transaction and Failure Boundaries

The status update is a database operation. Cache invalidation, plugin after-publish, preview broadcast, and webhook dispatch happen afterward rather than in a shared transaction/outbox.

- Cache invalidation and webhook trigger failures are best effort/non-fatal in the inspected calls.
- Content after-publish plugin failure is returned after the entry is already published, so the API can report failure while state has changed.
- `CROSS_MODULE_ORCHESTRATION_UNCLEAR`: success semantics across state write and external side effects are not uniform.

## Audit Behavior

Dedicated audit records are present for destructive and administrative operations, but content/page publish/unpublish handlers do not call the shared audit service. Webhook delivery history is not an audit substitute. This is `AUDIT_COVERAGE_UNCLEAR`, not proof that publication auditing is intentionally absent.

