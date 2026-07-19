---
okf_document_id: "delivery-rollback-recovery"
title: "Rollback and Recovery"
project: "ZinharCMS"
category: "delivery"
phase: 10
status: "current"
review_status: "mixed"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
  - "docker-compose.prod.yml"
  - "backend/migrations"
related_documents:
  - "release-process.md"
  - "deployment-workflow.md"
  - "database-deployment.md"
  - "../database/migrations.md"
  - "../security/trust-boundaries.md"
  - "../operations/backup-and-restore.md"
related_diagrams:
  - "diagrams/release-flow.mmd"
---

# Rollback and Recovery

| Area | Classification | Evidence and limitation |
| --- | --- | --- |
| Application rollback | `PARTIALLY_DEFINED` | Runbooks say restore last known good backend/frontend; no artifact source or executable procedure |
| Container rollback | `ROLLBACK_BEHAVIOR_UNCLEAR` | Images can be built, but tags/registry/retention/previous digest are undefined |
| Frontend rollback | `MANUAL` intent | Replace static release in runbooks; no deployment target or retained bundle contract |
| Database rollback | `NOT_IMPLEMENTED` as downgrade | No down migrations; runbooks refer to restoring backup, but backup/restore is unimplemented |
| Configuration rollback | `ROLLBACK_BEHAVIOR_UNCLEAR` | Environment source/version history is unknown |
| Feature-flag rollback | `NOT_IMPLEMENTED` platform-wide | No general feature-flag system; Marketplace kill switches are domain controls, not release rollback |
| Failed migration recovery | `PARTIALLY_DEFINED` | Backend refuses startup; preserve logs/version; actual restore/retry authority undefined |
| Failed deployment recovery | `PARTIALLY_DEFINED` | Runbooks list stop/freeze, prior app, health/readiness, communication; no automation |
| Artifact retention | `ROLLBACK_BEHAVIOR_UNCLEAR` | CI uploads nothing; local outputs ignored; registry unknown |
| Previous-version availability | `UNKNOWN` | Git history exists, but deployable prior artifact availability is not established |

## Safe Repository-Supported Actions

Health/readiness checks, CI-equivalent validation, Marketplace installation rollback, product disable/kill switches, and Git history can help diagnose or contain specific failures. They do not establish a general production rollback guarantee. Database or volume restore, container replacement, configuration change, and traffic switching require explicit operational authority and verified target-specific procedures.

No rollback or recovery operation was executed in Phase 10. See [Database Deployment](database-deployment.md), [Backup and Restore](../operations/backup-and-restore.md), [Release Process](release-process.md), and [Release Flow](diagrams/release-flow.mmd).

