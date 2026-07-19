---
okf_document_id: "delivery-database-deployment"
title: "Database Deployment"
project: "ZinharCMS"
category: "delivery"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/migrations"
  - "backend/src/db/mod.rs"
  - "backend/src/main.rs"
  - "backend/Dockerfile.prod"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
related_documents:
  - "deployment-workflow.md"
  - "rollback-and-recovery.md"
  - "../development/database-development.md"
  - "../database/migrations.md"
  - "../database/database-risks.md"
  - "../operations/backup-and-restore.md"
related_diagrams:
  - "diagrams/deployment-flow.mmd"
---

# Database Deployment

| Concern | Verified behavior or status |
| --- | --- |
| Packaging | SQLx `migrate!` embeds ordered SQL migrations; Docker images also copy the migration directory |
| Execution | Backend startup runs all pending migrations before seed, Redis state construction, bind, or readiness |
| Ordering/history | Filename order and SQLx `_sqlx_migrations` history/checksums |
| Ownership | Backend process owns execution; operational owner is unclear |
| Pre/post deployment phase | No distinct job; migration is part of application startup |
| Backward compatibility | No formal expand/contract policy; each migration must be reviewed individually |
| Locking/concurrency | SQLx migration behavior applies, but multi-replica startup coordination is not documented/tested here |
| Failure handling | Startup returns context error and does not serve if migration fails |
| Rollback | No down migrations or automated downgrade command; `ROLLBACK_BEHAVIOR_UNCLEAR` |
| Data migrations | SQL files may include data changes; no separate runner/classification |
| Backup requirement | Runbooks require backup, but no backup implementation exists (`DOCUMENTATION_CODE_CONFLICT`) |
| Test behavior | CI provides PostgreSQL but does not declare an explicit migration-deployment smoke job; backend tests/static contracts vary |

The deployment configuration cannot support a verified claim of zero-downtime schema changes, online migration safety, previous-version compatibility, or automatic recovery. A failed migration requires preserving logs and migration version and escalating to an owner-approved recovery procedure; do not improvise production SQL or claim restore availability.

See [Database Migrations](../database/migrations.md), [Database Risks](../database/database-risks.md), [Database Development](../development/database-development.md), [Backup and Restore](../operations/backup-and-restore.md), and [Rollback](rollback-and-recovery.md).

