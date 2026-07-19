---
okf_document_id: "domain-page-builder-rules"
title: "Page Builder Rules"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/pages/PagesPage.test.tsx"
related_documents:
  - "../frontend/page-builder.md"
  - "domains/pages-and-page-builder.md"
  - "workflows/page-builder-save-and-version.md"
related_diagrams:
  - "diagrams/page-builder-workflow.mmd"
---

# Page Builder Rules

See [Frontend Page Builder](../frontend/page-builder.md) and the [Page Builder workflow diagram](diagrams/page-builder-workflow.mmd).

## Verified Rules

| Rule | Enforcement | Backend behavior | Frontend behavior | Database support | Tests/confidence |
| --- | --- | --- | --- | --- | --- |
| Page title is nonempty and slug is valid/tenant-unique. | Application + DB | Validator and unique/check constraints | Required inputs; errors displayed | Slug check and tenant uniqueness | High; no invalid route tests |
| `page_json` is an object with required `layout`. | Application + DB shape | Recursive validation | Draft initializes a valid document | JSON object check | High |
| Root node type is exactly `root`; descendants cannot be `root`. | Application | Recursive validator | Normalizer/default create root | Object storage only | High; no validator tests |
| Descendant type must exist in system or current tenant registry. | Application/RLS | Registry lookup before save/restore | Palette is built from system, tenant, and active Marketplace definitions | Component registry/RLS | High |
| Node ID is 1–128 characters; props/styles are optional objects; children is an array. | Application | Recursive validation | UI creates normalized objects | None beyond JSON shape | High |
| Layout depth is at most 12 and total nodes at most 500. | Application | Recursive counters reject excessive layout | UI does not expose matching limits | None | High backend; frontend-only visibility gap |
| Component property schema keys are identifier-shaped and types use finite allowlist. | Application + DB object | Component create/update validator | Property editor supports schema-driven controls | JSON object check | High |
| Page create/update/restore creates a numbered snapshot with the page write. | Transaction + DB | Transactional snapshot helper | Manual save/autosave/restore APIs | Unique positive page version | High; no integration tests |
| Restoring a version resets state to draft and creates another snapshot. | Application | Validates against current registry, clears publication time | Restore action reloads page/versions | Transaction and uniqueness | High |
| Published page edit invalidates public page cache. | Application | Best-effort invalidation after commit | No cache semantics in UI | Redis external | Medium guarantee |
| Page writes and transitions broadcast JSON to preview subscribers. | Application | In-process broadcast channel | UI copies preview WebSocket URL; local preview is separate | None | High behavior; no integration tests |

## Implemented Frontend Behavior

- Palette search and adding system/Marketplace component definitions.
- Top-level drag-and-drop reorder.
- Recursive rendering, selection, update, and removal of nested nodes.
- Schema-driven property controls for boolean/select/text/number/email/JSON/array/rich-text-like values.
- Local preview, manual save, delayed autosave for existing pages, state transitions, snapshot list/restore, template preview/import, and preview URL handoff.

## Partially Implemented or Unclear

- Nested structures render and edit, but arbitrary cross-level drag/drop placement is `IMPLEMENTATION_STATUS_UNCLEAR`.
- Component prop values are stored but backend validation does not apply `props_schema` constraints to each node: `PARTIALLY_ENFORCED_RULE`.
- Marketplace component definitions are namespaced host-owned adapters; runtime renderer parity remains unclear.
- Template import maps required assets and validates resulting page JSON, but rollback across imported assets/page is documented in the Marketplace workflow, not assumed.
- Page hierarchy between separate page records is not implemented; nesting exists inside `page_json` only.
- No client undo/redo, responsive preview modes, content localization workflow, dirty-navigation guard, or collaborative editing was found.
- No page ownership ACL beyond tenant role was found.

## Planned Not Implemented

- Arbitrary plugin-provided code execution is `PLANNED_NOT_IMPLEMENTED`.
- Publication scheduling and background rendering are not implemented.
- A separate layer/tree panel and cross-device collaborative editing are not documented as planned; they are simply not found.

## Deletion

Page deletion is a confirmed hard delete that cascades snapshots. Component deletion is a confirmed hard delete for non-system tenant components. Existing page JSON referencing a deleted component remains stored but can fail validation on the next save/restore: `INVARIANT_UNVERIFIED`.

