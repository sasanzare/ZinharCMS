---
okf_document_id: "workflow-editorial-publication"
title: "Editorial Publication Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-PUBLISH"
workflow_name: "Editorial Publication"
workflow_domain: "DOM-CONTENT"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/services/workflow.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/services/webhooks.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../publication-workflow.md"
  - "../state-transitions.md"
related_diagrams:
  - "../diagrams/publication-workflow.mmd"
---

# Editorial Publication Workflow

## Workflow Identity

- ID/name/domain: `WF-PUBLISH`, Editorial Publication, shared `DOM-CONTENT`/`DOM-PAGE`.
- Trigger/actors: writer submits; reviewer rejects/archives/restores; publisher publishes/unpublishes.
- Status/confidence: `IMPLEMENTED`; High state-policy, Medium complete side-effect guarantee.

## Preconditions

Authentication, active tenant membership, entity in current tenant, operation-specific organization role, and current state accepted by `WorkflowStatus`. Saved data is assumed valid; publish does not repeat full content/page validation.

## Main Flow

1. Writer changes `draft` to `pending_review`.
2. Reviewer UI loads pending entries/pages and optional comments.
3. Reviewer rejects to `draft`, or publisher moves to `published`; publisher may also bypass review from `draft`.
4. Persistence sets status and publication timestamp; content entry increments version.
5. Page transition broadcasts JSON; publication invalidates delivery cache.
6. Entry publication runs enabled after-publish hooks.
7. Entry/page publish/unpublish triggers matching webhook dispatch tasks.
8. Public delivery now includes/excludes the row according to status.

## Alternative Flows

Publisher unpublishes `published` to `draft`. Reviewer archives `published` to `archived` and restores archive to draft. Comments can be added/resolved independently and do not block publication.

## Failure Flows

Invalid transition or role is rejected before write. DB failure prevents state change. Cache and webhook failures are non-fatal; after-publish plugin failure is returned after state changed. No automated rollback to prior status exists.

## State Changes

See the complete [Editorial Transition Matrix](../state-transitions.md). Entry version increments on all transitions; page snapshots do not.

## Data Changes

Entry/page status, `published_at`, entry version, optional webhook delivery rows; comments are a separate workflow.

## Transaction Boundaries

Status update is atomic by statement. Side effects are after the write and do not share a transaction/outbox.

## Side Effects

Redis invalidation, compiled plugin hook, preview broadcast, outbound signed HTTP webhook, and delivery history. No search index or publication audit record.

## Completion Criteria

Handler returns the transitioned entity. This does not prove all asynchronous webhook deliveries succeeded.

## Tests

Three workflow unit tests and webhook helper tests. Missing full transition matrix, RBAC/tenant, DB, cache, plugin-failure, webhook, and frontend workflow tests.

## Unknowns and Risks

Review bypass product intent, no publish-time revalidation, non-atomic effects, no scheduling, missing audit, and no page snapshot at status change.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

