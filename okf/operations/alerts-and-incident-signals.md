---
okf_document_id: "operations-alerts-incident-signals"
title: "Alerts and Incident Signals"
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
  - "backend/src/routes/mod.rs"
  - "backend/src/services/webhooks.rs"
  - "backend/src/services/email.rs"
  - "backend/src/services/marketplace_feedback.rs"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
related_documents:
  - "health-and-readiness.md"
  - "logging-and-tracing.md"
  - "metrics-and-monitoring.md"
  - "runbook-catalog.md"
  - "../security/audit-and-security-events.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/observability-flow.mmd"
---

# Alerts and Incident Signals

No external alerting configuration, on-call integration, pager, notification channel, escalation roster, or automatic SLO evaluation was found.

| Signal | Source/detection | Alert integration | Severity | Response owner | Runbook | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| Backend startup/migration failure | Process exits with contextual error | None | Potentially HIGH | `OPERATIONAL_OWNERSHIP_UNCLEAR` | Troubleshooting; V2 runbook | `VERIFIED` signal |
| Health failure | Caller cannot get `/health` | Readiness scripts only when manually run | HIGH in intended deployment | Unknown | Troubleshooting | `VERIFIED` endpoint, no alert |
| Readiness degradation | `/ready` 503 for PostgreSQL/Redis | Manual scripts/possible external caller | HIGH | Unknown | Troubleshooting | `VERIFIED` |
| Authentication/rate failures | HTTP errors, database/audit state in selected flows | No automatic alert | Context-dependent | Security owner unknown | Security docs | `PARTIALLY_DEFINED` |
| CMS webhook failure | Warning log plus `webhook_deliveries` row | No retry/alert integration | MEDIUM | Tenant/operator unknown | Troubleshooting | `VERIFIED` persistence |
| Email failure | `email_deliveries` status/error; strict mode request failure | No external alert | MEDIUM | Unknown | V2 runbook | `VERIFIED` persistence |
| Critical Marketplace report | Internal database notification and moderation queue | No external delivery | HIGH | Marketplace/admin owner unknown | V3 runbook | `VERIFIED` internal signal |
| SaaS alert rule | Stored rule definitions/routes | No evaluator/delivery loop found | Unknown | Unknown | None complete | `IMPLEMENTATION_STATUS_UNCLEAR ISU-02` |
| Deployment/CI failure | GitHub job failure for CI; no deploy job | GitHub UI/provider defaults unknown | MEDIUM/HIGH | Maintainer unknown | CI docs | `VERIFIED` CI signal |
| Backup failure | No backup job | None | CRITICAL if expected | Unknown | `RUNBOOK_NOT_FOUND` | `BACKUP_STATUS_UNCLEAR` |

Runbook severity labels and monitoring lists are operational guidance, not proof of automatic detection. See [Runbooks](runbook-catalog.md), [Health](health-and-readiness.md), [Security Events](../security/audit-and-security-events.md), and [Troubleshooting](troubleshooting.md).

