---
okf_document_id: "maintenance-staleness-detection"
title: "Staleness Detection"
project: "ZinharCMS"
category: "maintenance"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "prescriptive"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "okf/index.yaml"
  - "okf/references/source-register.md"
  - "okf/project/repository-map.md"
  - ".github/workflows"
related_documents:
  - "README.md"
  - "okf-update-policy.md"
  - "review-checklist.md"
  - "validation-report.md"
  - "cross-phase-conflicts.md"
related_diagrams: []
---

# Staleness Detection

## Automated Checks Available Now

Phase 10 used non-mutating local validation logic to parse `okf/index.yaml` and every Markdown frontmatter block, compare file/index inventories and metadata, enforce unique IDs, resolve links/relations/evidence paths, scan language/secret patterns, and perform static Mermaid structure checks. This logic is not a tracked repository script or CI job; future reviewers must reproduce or formalize it.

Git and current toolchains also provide `git status --short`, `git diff`, `git diff --check`, component CI commands, and source search. No documentation workflow runs these checks automatically.

## Checks Suitable for Future Scripting

| Signal | Future check |
| --- | --- |
| Source path removed/renamed | Resolve all `primary_sources` and index evidence paths |
| Referenced type/module/route missing | Extract source identifiers and compare catalogs |
| Migration added | Require database, domain/security, delivery, and index review |
| Permission/role changed | Compare RBAC constants/routes/migrations with security catalog |
| Frontend feature/client drift | Compare routes/pages/API client methods with frontend catalogs |
| Broken links/diagram links | Resolve Markdown and metadata paths |
| Old verification commit | Compare `last_verified_commit` ancestry/changed evidence paths |
| Planned feature implemented/removed | Compare status markers with current source/tests |
| Conflicting metadata | Compare frontmatter and index values |
| Missing index entry/duplicate ID | Exact inventory and uniqueness check |
| Mermaid syntax | Add an approved pinned parser/renderer and retain static evidence checks |
| Secret exposure | Add an approved scanner with reviewed false-positive policy |

## Manual Review Requirements

Automation cannot decide product intent, actual production topology/state, ownership, retention/legal policy, operational guarantees, risk severity, inference confidence, or whether documentation narrative remains understandable. Reviewers must inspect diffs in affected source evidence, conflicts, unknowns, owner questions, and diagrams.

## Staleness Response

Mark affected content `OKF_DOCUMENTATION_STALE` when evidence changed and a safe correction cannot be completed immediately. Record the evidence path, impact, owner question, and next review. Do not silently delete unknowns or replace them with assumptions.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

