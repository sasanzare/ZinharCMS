---
okf_document_id: "workflow-content-entry-save"
title: "Content Entry Save Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-CONTENT-SAVE"
workflow_name: "Content Entry Save"
workflow_domain: "DOM-CONTENT"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/services/entry_validation.rs"
  - "backend/src/services/security.rs"
  - "backend/src/plugins/mod.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../content-lifecycle.md"
  - "../validation-rules.md"
related_diagrams:
  - "../diagrams/content-lifecycle.mmd"
---

# Content Entry Save Workflow

## Workflow Identity

- ID/name/domain: `WF-CONTENT-SAVE`, Content Entry Save, `DOM-CONTENT`.
- Trigger/actor: organization entry writer creates or updates entry data.
- Status/confidence: `IMPLEMENTED`; High.

## Preconditions

Authentication, active tenant/member context, entry-writer role, existing tenant content type, valid JSON body, and content capacity for create.

## Main Flow

1. Load content type by tenant and slug.
2. Parse and validate its current field schema.
3. Run enabled built-in `entry.before_save` hooks synchronously.
4. Sanitize rich-text fields according to the schema.
5. Validate required fields, types, lengths, numbers, references shapes, and slugs.
6. Insert a new draft/version-1 row or replace existing data and increment version.
7. If an updated entry is already published, best-effort invalidate its public content cache.
8. Return current row.

## Alternative Flows

SEO built-in plugin preserves an existing slug; when missing, it derives one from string title. A content type with no fields accepts an object without field-specific requirements.

## Failure Flows

Plugin, sanitation-related serialization, schema, entry validation, quota, tenant lookup, or DB failure aborts the write. Update has no optimistic version check; concurrent last writer wins.

## State Changes

Create initializes `draft`. Update preserves current status and publication timestamp but increments numeric version.

## Data Changes

One content entry insert/update. Plugin changes are incorporated into the same persisted JSON.

## Transaction Boundaries

One SQL statement provides row atomicity. Plugin/validation occur before it. Cache invalidation occurs afterward and is not transactional.

## Side Effects

Synchronous compiled plugin execution; possible Redis invalidation for published update. No snapshot, webhook, audit, or search event on ordinary save.

## Completion Criteria

Validated transformed data is persisted and returned. Cache invalidation is not required for handler success.

## Tests

Entry validator coverage is incomplete; sanitizer and SEO plugin have unit tests. No route, tenant, quota, concurrency, or published-update cache integration test.

## Unknowns and Risks

Schema evolution, reference target validity, global plugin scope, no revision history, last-write-wins concurrency, and published content changing without re-review.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

