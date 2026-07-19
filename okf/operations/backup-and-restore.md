---
okf_document_id: "operations-backup-restore"
title: "Backup and Restore"
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
  - "docker-compose.prod.yml"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
  - "backend/src/config.rs"
related_documents:
  - "database-operations.md"
  - "disaster-recovery.md"
  - "../delivery/rollback-and-recovery.md"
  - "../database/database-risks.md"
  - "../security/secrets-and-configuration.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/backup-restore-flow.mmd"
---

# Backup and Restore

## Status Matrix

| Asset | Backup status | Schedule/retention/encryption/destination | Restore status | Restore validation | Tenant/PITR status |
| --- | --- | --- | --- | --- | --- |
| PostgreSQL | `BACKUP_STATUS_UNCLEAR`; no command/job | Not defined | `RESTORE_STATUS_UNCLEAR`; runbooks only say restore a backup | No test/drill evidence | Tenant restore and PITR not defined |
| Redis | `NOT_IMPLEMENTED` as backup workflow; production-like Redis uses AOF volume | Volume persistence is not backup | `RESTORE_STATUS_UNCLEAR` | None | Not defined |
| Upload/media volume | `BACKUP_STATUS_UNCLEAR` | Not defined | `RESTORE_STATUS_UNCLEAR` | None | Tenant-level mapping not defined |
| Marketplace artifacts | Shares local/upload storage behavior | Not defined | `RESTORE_STATUS_UNCLEAR` | Integrity checks exist during install, not restore | Not defined |
| Configuration | `BACKUP_STATUS_UNCLEAR`; production source unknown | Not defined | `RESTORE_STATUS_UNCLEAR` | None | N/A |
| Secrets | No backup/recovery process in repository | Must not be copied into docs | `SECRET_INJECTION_UNCLEAR` | None | N/A |
| Plugin/tenant data | Stored across PostgreSQL and files | Depends on absent database/file backup | `RESTORE_STATUS_UNCLEAR` | None | No product-specific restore contract |

Named Docker volumes provide persistence across ordinary container recreation; they are not backups and do not establish off-host copies, encryption, retention, immutability, or recovery.

## Documentation Conflict

V2/V3 runbooks require a database backup before deployment and instruct operators to restore it when integrity is unsafe. No `pg_dump`, `pg_restore`, sidecar, scheduled workflow, destination, retention rule, encryption rule, or restore drill exists. The safest interpretation is that backup and restore are external owner responsibilities that are not evidenced (`DOCUMENTATION_CODE_CONFLICT`, `UNKNOWN U-04`).

No backup or restore command was executed in Phase 10. See [Backup Diagram](diagrams/backup-restore-flow.mmd), [Database Operations](database-operations.md), [Rollback](../delivery/rollback-and-recovery.md), [Database Risks](../database/database-risks.md), and [Troubleshooting](troubleshooting.md).

