---
okf_document_id: "operations-risks"
title: "Operational Risks"
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
  - "backend/src/main.rs"
  - "backend/src/error.rs"
  - "docker-compose.prod.yml"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
related_documents:
  - "runtime-topology.md"
  - "logging-and-tracing.md"
  - "metrics-and-monitoring.md"
  - "backup-and-restore.md"
  - "disaster-recovery.md"
  - "../delivery/delivery-risks.md"
  - "../security/security-risks.md"
related_diagrams:
  - "diagrams/observability-flow.mmd"
  - "diagrams/backup-restore-flow.mmd"
---

# Operational Risks

| Risk ID | Title | Evidence | Impact | Likelihood | Severity | Existing mitigation | Recommended follow-up | Owner confirmation required | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| OPS-R01 | Production topology is unknown | No deployment provider/edge/replica evidence | Security, availability, scaling cannot be assessed | High | CRITICAL | Production-like Compose reference | Document and review actual topology | Yes | `PRODUCTION_TOPOLOGY_UNKNOWN` |
| OPS-R02 | No backup automation or restore evidence | No commands/jobs/tests | Irrecoverable database/file loss | High | CRITICAL | Named volumes only; runbook warnings | Implement encrypted off-host backups and restore drills | Yes | `BACKUP_STATUS_UNCLEAR` |
| OPS-R03 | Disaster recovery undefined | No RTO/RPO/site/traffic/owner plan | Prolonged outage and data loss | High | CRITICAL | Rebuildable source/images only | Owner-approved DR plan and exercise | Yes | `DISASTER_RECOVERY_UNDEFINED` |
| OPS-R04 | No application metrics/alerts | No exporter/collector/alerts | Failures detected late or manually | High | HIGH | Health/readiness, logs, business analytics | Select metrics, dashboards, alert thresholds, SLOs | Yes | `METRICS_STATUS_UNCLEAR` |
| OPS-R05 | Log collection/redaction/retention unknown | stdout formatter only | Lost evidence or sensitive disclosure | Medium | HIGH | Request IDs and selected traces | Define structured logging, redaction, collector, retention | Yes | `LOG_RETENTION_UNKNOWN` |
| OPS-R06 | Application containers lack health checks | Compose only checks DB/Redis | Unready app may be treated as started | Medium | HIGH | Public health/readiness endpoints | Wire target-specific probes | Yes | `HEALTH_CHECK_STATUS_UNCLEAR` |
| OPS-R07 | In-process webhook/background work is not durable | `tokio::spawn`, no queue/retry drain | Lost side effects on failure/shutdown | Medium | HIGH | Delivery result records for completed attempts | Add durable outbox/worker/retry policy | Yes | `PARTIALLY_DEFINED` |
| OPS-R08 | Database/filesystem operations are non-atomic | Media/artifact storage split | Orphaned/missing files or rows | Medium | HIGH | Handler validation and integrity checks | Define reconciliation and backup process | Yes | `TRANSACTION_BOUNDARY_UNCLEAR TBU-01` |
| OPS-R09 | Startup migrations couple deploy and schema mutation | `main.rs` | Failed/contending rollouts | Medium | HIGH | Fail-before-bind | Define migration orchestration/compatibility | Yes | `INFERRED_FROM_CODE` |
| OPS-R10 | Secret management and bootstrap credential risk | Env names only; deterministic dev bootstrap source | Credential exposure/misuse | High | CRITICAL | Ignored `.env`, validation, security docs | Remove/guard bootstrap credential and select secret manager/rotation | Yes | `POTENTIAL_SECRET_EXPOSURE PSE-01` |
| OPS-R11 | Error/rediness messages may expose internals | Shared errors include technical/provider/database text | Information disclosure | Medium | HIGH | High-level error code | Central redaction and tests | Yes | `ERROR_DISCLOSURE_RISK EDR-01` |
| OPS-R12 | Operational ownership is absent | No CODEOWNERS/on-call/escalation roster | Slow or unsafe incident response | High | HIGH | Conceptual roles in runbooks | Assign named role ownership outside secrets | Yes | `OPERATIONAL_OWNERSHIP_UNCLEAR` |
| OPS-R13 | External dependency failure semantics vary | DB, Redis fallback, provider calls, spawned webhooks | Partial completion/inconsistent recovery | Medium | HIGH | Transactions in selected flows and persisted delivery records | Define retry/compensation matrix | Yes | `NEEDS_OWNER_CONFIRMATION NOC-09` |
| OPS-R14 | Runbooks overstate unavailable recovery inputs | Restore/previous deployment steps without artifacts/backups | False confidence during incident | High | HIGH | Phase 10 status markers | Reconcile runbooks after recovery implementation | Yes | `DOCUMENTATION_CODE_CONFLICT` |

See [Delivery Risks](../delivery/delivery-risks.md), [Security Risks](../security/security-risks.md), [Backup](backup-and-restore.md), and [Disaster Recovery](disaster-recovery.md).

