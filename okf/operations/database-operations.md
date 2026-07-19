---
okf_document_id: "operations-database-operations"
title: "Database Operations"
project: "ZinharCMS"
category: "operations"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/db/mod.rs"
  - "backend/src/main.rs"
  - "backend/migrations"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "../database/README.md"
  - "../database/migrations.md"
  - "../delivery/database-deployment.md"
  - "../security/tenant-access-control.md"
  - "backup-and-restore.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/backup-restore-flow.mmd"
---

# Database Operations

| Concern | Verified status |
| --- | --- |
| Connection pool | Lazy SQLx PostgreSQL pool; maximum 10 connections; no configured minimum/acquire timeout in project code |
| Connection monitoring | `/ready` executes `SELECT 1`; no pool metrics/dashboard |
| Migration execution | Embedded ordered migrations run on every backend startup before serving |
| Schema verification | `_sqlx_migrations` records applied history; no repository command compares full runtime catalog to expected schema |
| RLS/privileges | Migrations enable/force tenant policies; actual deployed roles/grants and cross-tenant behavior require environment validation |
| Maintenance | No VACUUM, ANALYZE, REINDEX, partition, bloat, or statistics maintenance script/config found |
| Backup/restore | No command, schedule, destination, or test found |
| Retention | No platform data-retention/purge policy |
| Test cleanup | No general fixture/reset harness; prior smoke scripts/processes used task-specific cleanup, not a reusable contract |
| Failure handling | Startup fails on migration/seed error; request SQL errors map to application errors, sometimes with technical detail |
| Operational scripts | Readiness/load scripts call API surfaces; none performs database maintenance |
| Required privileges | Migration/application role must create/alter objects and execute runtime queries; exact production grants unknown |

Do not run ad hoc maintenance, reset, backup, restore, or direct data correction based on this document. Those actions require target-specific authorization, backups, tested commands, and ownership not present in the repository.

See [Database Architecture](../database/README.md), [Migrations](../database/migrations.md), [Database Deployment](../delivery/database-deployment.md), [Tenant Access](../security/tenant-access-control.md), [Backup and Restore](backup-and-restore.md), and [Troubleshooting](troubleshooting.md).

