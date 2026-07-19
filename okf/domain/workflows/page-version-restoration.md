---
okf_document_id: "workflow-page-version-restoration"
title: "Page Version Restoration Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-PAGE-RESTORE"
workflow_name: "Page Version Restoration"
workflow_domain: "DOM-PAGE"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "frontend/src/pages/PagesPage.tsx"
related_documents:
  - "../cross-module-workflows.md"
  - "../revisions-and-versioning.md"
  - "../page-builder-rules.md"
related_diagrams:
  - "../diagrams/page-builder-workflow.mmd"
---

# Page Version Restoration Workflow

## Workflow Identity

- ID/name/domain: `WF-PAGE-RESTORE`, Page Version Restoration, `DOM-PAGE`.
- Trigger/actor: page manager chooses an existing numbered snapshot.
- Status/confidence: `IMPLEMENTED`; High.

## Preconditions

Authentication, active tenant, page-manager role, positive version number, snapshot belongs to requested page/tenant, and snapshot remains valid against the current component registry.

## Main Flow

1. Load requested snapshot by page, version, and organization.
2. Load current system/tenant component keys.
3. Recursively revalidate snapshot `page_json`.
4. Begin tenant transaction.
5. Replace current page JSON with snapshot JSON.
6. Set status to `draft`, clear `published_at`, and update timestamp.
7. Append a new snapshot containing the restored JSON.
8. Commit.
9. Broadcast restored JSON to preview subscribers.
10. Frontend reloads page and version list.

## Alternative Flows

Status restore from `archived` uses a different endpoint and does not restore historical JSON.

## Failure Flows

Nonpositive/missing/wrong-tenant version or current-registry validation failure rejects before write. Transaction failure rolls back page/new snapshot. Preview or frontend reload failure occurs after commit.

## State Changes

Any current page state is overwritten to `draft` by this route; publication time becomes null. This route does not use the shared transition matrix.

## Data Changes

Updates current page JSON/status/timestamp and inserts one new snapshot. Historical snapshots remain unchanged.

## Transaction Boundaries

Page reset and new snapshot are atomic. Validation lookup precedes transaction; preview/reload follow commit.

## Side Effects

Preview broadcast. Public page cache is not explicitly invalidated in the inspected restore-version handler, even if the prior page was published: a suspected cache-consistency gap.

## Completion Criteria

Draft restored page and new snapshot commit and are returned.

## Tests

No backend or frontend workflow test for version restore was found.

## Unknowns and Risks

Potential stale public cache, no title/slug/state snapshot, no optimistic lock, current component deletions blocking old restore, and concurrent version numbering.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

