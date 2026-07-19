---
okf_document_id: "domain-content-editorial"
title: "Content and Editorial Workflow Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-CONTENT"
domain_name: "Content and Editorial Workflow"
domain_status: "IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/routes/comments.rs"
  - "backend/src/services/entry_validation.rs"
  - "backend/src/services/workflow.rs"
  - "backend/migrations/0001_initial_schema.sql"
  - "backend/migrations/0006_phase_six_workflow_collaboration.sql"
related_documents:
  - "../domain-catalog.md"
  - "../content-lifecycle.md"
  - "../publication-workflow.md"
related_diagrams:
  - "../diagrams/content-lifecycle.mmd"
  - "../diagrams/publication-workflow.mmd"
---

# Content and Editorial Workflow Domain

## Domain Identity

- Domain ID: `DOM-CONTENT`
- Terminology: content type, field schema, entry, workflow status, version, comment, review, publication.
- Implementation: `IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: define dynamic schemas, transform/sanitize/validate entries, manage entry lifecycle, and collaborate through comments.
- Inferred: content types define the current validation contract for future entry saves.
- Shared: workflow service with Pages; plugin hooks; public delivery/cache; webhooks; quota; security.
- Unclear: schema migration policy, entry revision history, localized variants, and publication audit requirements.

## Core Entities

`content_types`, `content_entries`, and polymorphic `comments` for `entry` targets.

## Core Services

Content/comments routes; entry validation, workflow, security sanitation, plugins, quota, RLS/RBAC, delivery, audit, and webhooks.

## API Surface

Content type CRUD, entry CRUD/list/workflow, and comment list/create/resolve/unresolve/delete. See [Content Endpoints](../../api/endpoints/content-entries-and-workflow.md) and [Editorial Comments](../../api/endpoints/editorial-comments.md).

## Frontend Surface

`ContentTypesPage`, `EntriesPage`, `WorkflowPage`, and `DynamicForm`.

## Actors

Organization readers, entry writers, content-type managers, reviewers/publishers, and comment readers/writers/managers.

## Business Rules

`BR-CONTENT-001` through `BR-CONTENT-006` and shared delivery/plugin rules.

## Invariants

Tenant-unique type slug, object entry data, positive mutation version, existing type relationship, and status enum.

## State and Lifecycle

Draft/review/published/archived transitions are explicit. Entry version increments on data or status mutation. Deletion is hard; archive restoration returns to draft.

## Access Rules

Organization role capabilities are authoritative. `author_id` is attribution and not a complete owner ACL. Comment targets are verified as existing tenant entry/page records before creation/read.

## Validation Rules

Dynamic schemas support string, numeric, boolean, reference, JSON, and slug shapes. Relation/media values are shape-checked but target existence is not enforced by entry validation. Rich text is sanitized before validation.

## Workflows

[Content Entry Save](../workflows/content-entry-save.md) and [Editorial Publication](../workflows/editorial-publication.md).

## Side Effects

Entry writes, version increments, plugin hooks, cache invalidation, publication webhooks, and selected deletion audit. No search indexing or durable publication event.

## Tests

Workflow, sanitizer, and SEO plugin helpers are tested. Content route CRUD, schema evolution, comments, state side effects, tenant/authorization, and transaction behavior lack focused integration tests.

## Risks and Unknowns

Schema changes can invalidate existing data; entry history is overwritten; after-publish plugin failure occurs after state change; type cascade deletion bypasses per-entry events; frontend/backend field support differs.

Return to the [Domain Catalog](../domain-catalog.md).

