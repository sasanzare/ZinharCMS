---
okf_document_id: "operations-runbook-catalog"
title: "Runbook Catalog"
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
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
  - "scripts/v2-ga-check.ps1"
  - "scripts/marketplace-phase15-ga-check.ps1"
  - "okf/operations/troubleshooting.md"
related_documents:
  - "troubleshooting.md"
  - "alerts-and-incident-signals.md"
  - "backup-and-restore.md"
  - "../delivery/rollback-and-recovery.md"
  - "../security/security-risks.md"
  - "../architecture/architecture-risks.md"
related_diagrams: []
---

# Runbook Catalog

| ID | Scenario/trigger | Scope and preconditions | Diagnostic/recovery summary | Destructive risk/access | Verification | Owner | Status | Source |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| RB-V2-GA | V2 release/incident | Intended release environment and assigned owners | Freeze/checks; health/readiness; billing/access/RLS/migration triage; manual rollback narrative | Backup/restore and app replacement are high-risk; operational access | Manual post-action checks | `OPERATIONAL_OWNERSHIP_UNCLEAR` | `PARTIALLY_DEFINED` | `docs/V2_OPERATIONS_RUNBOOK.md` |
| RB-V3-MKT | Marketplace launch/incident | Approved products, support/admin access, owner assignments | Install/payment/report/kill-switch diagnosis and containment; manual rollback narrative | Product disable, rollback, backup restore require authority | Health/readiness and Marketplace surfaces | `OPERATIONAL_OWNERSHIP_UNCLEAR` | `PARTIALLY_DEFINED` | `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md` |
| RB-DEV-LOCAL | Local startup | Local templates/toolchains/services | Start infra/apps and verify health/readiness | Startup runs migrations/seed | Local endpoints | Developer | `DOCUMENTED_FROM_REPOSITORY` | `okf/development/local-environment.md` |
| RB-TROUBLE | Common application failures | Read-only diagnostics and safe local configuration | Symptom/cause/diagnostic/safe-action matrix | Explicit no-reset/no-secret warnings | Re-run smallest signal | Maintenance classes | `DOCUMENTED_FROM_REPOSITORY` | `okf/operations/troubleshooting.md` |
| RB-V2-CHECK | GA validation trigger | Rust/npm; optional API URL | Backend tests, frontend lint/build, optional health/readiness | Build/test side effects only | Script exit code | Release owner unknown | `VERIFIED_EXISTING` | `scripts/v2-ga-check.ps1` |
| RB-V3-CHECK | Marketplace GA validation | Rust/npm; optional API/auth/tenant | Contract/regression, frontend gates, optional read-only API checks | Authenticated reads; no deploy | Table/JSON and exit code | Release owner unknown | `VERIFIED_EXISTING` | `scripts/marketplace-phase15-ga-check.ps1` |
| RB-BACKUP | Database/files/config backup | Destination, access, encryption, retention | No steps available | High | None | Unknown | `RUNBOOK_NOT_FOUND` | No repository source |
| RB-RESTORE | Database/files/config restore | Valid backup and isolated validation target | No executable steps available | Critical/destructive | None | Unknown | `RUNBOOK_NOT_FOUND` | No repository source |
| RB-DR | Site/infrastructure disaster recovery | Recovery objectives, backup, provider, traffic, owner | No plan available | Critical | None | Unknown | `RUNBOOK_NOT_FOUND` | No repository source |
| RB-SEC-INC | General security incident | Evidence preservation and security ownership | Security docs identify threats/controls but no complete response runbook/contact | High | Undefined | `OPERATIONAL_OWNERSHIP_UNCLEAR` | `NEEDS_OWNER_CONFIRMATION` | `okf/security/security-risks.md` |

Runbooks must not be treated as verified for steps that depend on nonexistent backups, deployment artifacts, target infrastructure, or named owners. See [Troubleshooting](troubleshooting.md), [Backup and Restore](backup-and-restore.md), [Rollback](../delivery/rollback-and-recovery.md), [Security Risks](../security/security-risks.md), and [Architecture Risks](../architecture/architecture-risks.md).

