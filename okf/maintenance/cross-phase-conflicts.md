---
okf_document_id: "maintenance-cross-phase-conflicts"
title: "Cross-Phase Conflicts"
project: "ZinharCMS"
category: "maintenance"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "okf-bootstrap/09-knowledge-gaps.md"
  - "okf/project/overview.md"
  - "okf/references/source-register.md"
  - "docs/diagrams"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
related_documents:
  - "README.md"
  - "validation-report.md"
  - "unresolved-owner-questions.md"
  - "final-completion-report.md"
  - "../architecture/architecture-risks.md"
  - "../delivery/deployment-workflow.md"
  - "../operations/backup-and-restore.md"
related_diagrams: []
---

# Cross-Phase Conflicts

The register preserves source conflicts instead of rewriting historical artifacts. Current code/configuration/migrations outrank older prose and diagrams.

| ID | Topic | Documents involved | Source-code/config evidence | Conflicting claims | Best-supported interpretation | Confidence | Owner confirmation | Recommended resolution | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| DCC-01 | Visual Page Builder status | Historical Phase Three; frontend/architecture OKF | `frontend/src/pages/PagesPage.tsx` | Historical text can imply future; implementation exists | Visual builder is implemented inside Pages | High | No for current state | Mark historical doc superseded | `OKF_CROSS_PHASE_CONFLICT` open in source history |
| DCC-02 | Localization coverage | `docs/I18N.md`; frontend OKF | `frontend/src/i18n` | Older doc understates coverage | Current first-party i18n is broader but incomplete | High | Yes for target coverage | Update/retire old coverage statement | Open |
| DCC-03 | Marketplace finance/feedback entities | V3 domain model; database/extensibility OKF | Migrations `0022`–`0025` and services | Narrative calls implemented entities future | Current migrations/code are authoritative | High | No for current state | Update historical model status | Open |
| DCC-04 | Marketplace finance architecture status | Legacy architecture audit; architecture OKF | Finance routes/services/migrations | Audit said finance absent | Finance is implemented within documented limits | High | No | Correct audit entry | Open |
| DCC-05 | OpenAPI/manual API staleness | Diagram 00; API OKF | Router/OpenAPI comparison | Diagram groups distinct staleness states | Manual API and generated OpenAPI have different gaps | High | No | Narrow diagram labels | Open |
| DCC-06 | Paid purchase status | Diagram 02; backend/domain/extensibility OKF | Marketplace finance implementation | Diagram says deferred | One-time purchase/entitlement exists; execution remains deferred | High | No | Update diagram status | Open |
| DCC-07 | Marketplace lifecycle sequence status | Diagram 30; domain/extensibility OKF | Install, finance, feedback code | Diagram labels implemented flows unimplemented | Current route/service/migration status wins | High | No | Update diagram | Open |
| DCC-08 | Paid entitlement install gate | Diagram 33; extensibility OKF | Installation/finance services | Diagram treats paid entitlement as future/rejected | Paid entitlement gate is implemented | High | No | Update diagram | Open |
| DCC-09 | Billing webhook path | `docs/API.md`; API OKF | `backend/src/routes/billing.rs` | One manual paragraph uses `/api/billing/webhook` | Registered path is `/api/billing/stripe/webhook` | High | No | Correct manual API paragraph | Open |
| DCC-10 | Marketplace diagram visual class | Diagram 20; architecture OKF | Current marketplace implementation | Node text says implemented/partial while class is planned | Text/current source wins; executable runtime remains planned | High | No | Correct class/status split | Open |
| DCC-11 | Frontend form libraries | Phase Zero inventory; frontend OKF | `frontend/package.json`, no verified imports under `frontend/src` | Inventory attributed active React Hook Form/Zod use | Dependencies are declared but source use is unverified | High | Yes for intended convention | Correct inventory or adopt/remove libraries separately | Open |
| DCC-12 | RLS verification coverage | Database OKF; hardening service | Migrations after 0015; `hardening.rs` | Helper enumerates fewer RLS tables than migration intent | Helper is stale/incomplete; runtime applied schema still needs verification | High | No for count; yes for policy | Update verification helper in separate implementation phase | Open |
| P10-DCC-01 | Backup-dependent rollback | V2/V3 runbooks; delivery/operations OKF | No backup/restore script/workflow | Runbooks require backup/restore | Recovery input is not repository-evidenced | High | Yes | Implement/test process, then reconcile runbooks | Open |
| P10-DCC-02 | Migration deployment orchestration | V2 runbook; delivery/operations OKF | `main.rs`, `db/mod.rs` | Runbook lists deploy migrations separately | Current binary migrates during every startup | High | Yes | Choose one-shot or startup policy and update runbook/config | Open |
| P10-DCC-03 | Monitoring dashboard/alerts | V2/V3 runbooks; operations OKF | Health routes/business analytics; no exporter/alerts | Runbooks imply monitoring activities | Manual signals exist; continuous monitoring/alerts are not implemented | High | Yes | Define observability platform and precise runbook signals | Open |
| P10-DCC-04 | Deployment and health gating | Runbooks/Compose/operations OKF | Compose checks DB/Redis only; no app health check/CD | Runbooks require app health as rollout gate | Endpoints exist but deployment wiring is unknown | High | Yes | Wire target probes after topology decision | Open |

**Conflict count: 16.** None is silently resolved by Phase 10; current-state OKF uses the best-supported interpretation while retaining owner work.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

