---
okf_document_id: "domain-revisions-versioning"
title: "Revisions and Versioning"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
  - "backend/migrations/0001_initial_schema.sql"
related_documents:
  - "content-lifecycle.md"
  - "state-transitions.md"
  - "workflows/page-version-restoration.md"
related_diagrams:
  - "diagrams/page-builder-workflow.mmd"
---

# Revisions and Versioning

## Content Entries

| Concern | Observed behavior |
| --- | --- |
| Revision model | Integer `content_entries.version`; no entry revision/snapshot table found |
| Creation trigger | Entry update and each accepted status transition increment the counter |
| Snapshot | None; previous `data` is overwritten |
| Selection/comparison | No historical version API or comparison workflow |
| Publication relation | Published row is the current row; version increments during publication |
| Restoration/rollback | Not implemented for content data; archive restore only changes state |
| Deletion | Hard delete removes the current record; no revision preservation |
| Ownership/tenant | Author attribution and tenant scope on current row |
| Concurrency | No client-supplied version precondition or optimistic lock |
| Tests | No route-level version increment or concurrency test found |

Treat the content entry version as a mutation counter, not a revision history. Any requirement for audit-grade entry history is `REVISION_BEHAVIOR_UNCLEAR`/not implemented.

## Pages

| Concern | Observed behavior |
| --- | --- |
| Revision model | `page_versions` stores numbered `page_json` snapshots |
| Creation trigger | Page create, page update, and version restore |
| Number allocation | `MAX(version) + 1` within the page/tenant scope; unique `(page_id, version)` |
| Snapshot contents | `page_json`, snapshot time, and creator; title, slug, and workflow state are not captured |
| Selection | List versions descending; restore by positive version number |
| Restoration | Validate snapshot against current component registry, replace current JSON, reset page to draft, clear publication time, create a new snapshot |
| Publication relation | Publishing/status transitions do not create snapshots |
| Deletion | Page hard delete cascades all snapshots |
| Ownership/tenant | Tenant organization propagated to snapshots; `created_by` attribution |
| Concurrency | Transactional page+snapshot write, but no optimistic client token; concurrent next-number selection may conflict |
| Tests | Frontend test does not exercise list/restore; no route/database integration test found |

The new snapshot produced by restore preserves the restored content as the newest history point. It does not mutate or delete the selected historical snapshot.

## Marketplace Versions

Marketplace uses semantic package versions and immutable protected artifacts rather than CMS revision semantics. New releases are new `marketplace_versions`; submitted/approved artifact fields are protected by database triggers. Update and rollback choose explicit pinned versions. Detailed extensibility lifecycle belongs to Phase 9, while the observed business behavior is summarized in [Marketplace](domains/marketplace.md).

## Versioning Unknowns

- `REVISION_BEHAVIOR_UNCLEAR`: no retention limit, compaction, comparison, or snapshot deletion API for page versions.
- `REVISION_BEHAVIOR_UNCLEAR`: page status transitions have no corresponding immutable snapshot.
- `INVARIANT_UNVERIFIED`: concurrent page writes have no user-visible merge/conflict protocol.
- `DOCUMENTATION_CODE_CONFLICT`: the shared Page model is partial and should not be treated as the complete persisted lifecycle contract.

