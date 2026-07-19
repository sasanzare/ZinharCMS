---
okf_document_id: "maintenance-owner-questions"
title: "Unresolved Owner Questions"
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
  - "okf-bootstrap/12-owner-questions.md"
  - "okf/index.yaml"
  - "okf/project/overview.md"
  - "okf/maintenance/cross-phase-conflicts.md"
related_documents:
  - "README.md"
  - "cross-phase-conflicts.md"
  - "validation-report.md"
  - "final-completion-report.md"
  - "documentation-ownership.md"
related_diagrams: []
---

# Unresolved Owner Questions

Repository evidence was rechecked before carrying these questions forward. No question below has a complete owner-approved answer in the repository. There are no `BLOCKER` questions for continued evidence-based development, but several are `HIGH` for safe production operation.

| ID | Question | Category | Priority | Why it matters | Affected documents | Can work continue? | Recommended owner | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| NOC-01 | How should a public request select its production organization: default slug, host/domain, route, or another mapping? | Tenancy/delivery | HIGH | Prevents a verified public multi-tenant routing contract | Architecture, domain, API, operations | Yes, with current behavior marked | Project/backend owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-02 | What production storage serves media and Marketplace artifacts, and which assets require authorization? | Storage/security | HIGH | Affects scaling, access control, backup, and recovery | Architecture, security, extensibility, operations | Yes for local/reference behavior | Operations/security owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-03 | What backup schedule, restore procedure, RPO/RTO, retention, and drill cadence apply? | Recovery | HIGH | No recovery guarantee is supportable | Database, delivery, operations | Yes, without guarantee | Operations/database owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-04 | What logging, metrics, tracing, dashboards, alerts, SLOs, and on-call model are authoritative? | Observability | HIGH | Detection, evidence, and response are undefined | Operations, architecture, security | Yes, with unclear status | Operations/security owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-05 | What privacy, residency, audit, billing, artifact, log, and user-data retention policies apply? | Legal/data lifecycle | HIGH | Technical retention/deletion cannot establish policy | Database, security, domain, operations | Yes, without policy claims | Project/legal/security owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-06 | What environments, deployment, promotion, rollback, cadence, and release authority are intended? | Delivery | HIGH | Repository has no production deploy contract | Delivery, operations, architecture | Yes, without deployed claims | DevOps/project owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-07 | Which Rust, Node, npm, database, Redis, browser, and OS versions are supported? | Development | MEDIUM | CI/container/local drift affects reproducibility | Development, delivery, frontend | Yes | Maintainers/DevOps | `NEEDS_OWNER_CONFIRMATION` |
| NOC-08 | What API compatibility, versioning, deprecation, and support-window policy applies? | API | HIGH | Clients/plugins lack change guarantees | API, frontend, extensibility | Yes, mark current mechanics only | Project/backend owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-09 | What retry/compensation guarantee applies after DB success but file/cache/webhook/email/provider failure? | Consistency | HIGH | Partial completion is currently heterogeneous | Architecture, domain, operations | Yes, preserving gaps | Backend/domain owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-10 | Which Marketplace execution, appeal, cleanup, refund/dispute/tax/transfer/payout capabilities are planned or excluded? | Marketplace | MEDIUM | Current implementation cannot define roadmap | Extensibility, domain, security, operations | Yes, separate current/future | Marketplace/project owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-11 | What provider/process/renewal/failure/routing behavior governs organization-domain verification? | Tenancy | MEDIUM | Stored records do not prove live routing | Domain, API, operations | Yes, mark incomplete | Project/backend owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-12 | What policies govern schema evolution, workflow semantics, component compatibility, session recovery, accessibility, and frontend decomposition? | Product/engineering | MEDIUM | Long-term compatibility and UX expectations are undefined | Domain, frontend, API | Yes, document mechanics | Project/frontend/backend owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-13 | Which documents are canonical, who owns them, and how are stale documents retired? | Documentation | MEDIUM | Conflicting historical docs persist | Project, references, maintenance | Yes, use source priority | Project owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-14 | What branch, commit, review, required-check, coverage, E2E, migration-test, and docs policy applies? | Contribution | HIGH | Merge quality/governance is not tracked | Development, delivery, maintenance | Yes, label inference | Project/maintainer owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-15 | Who owns modules, Marketplace review, support, security response, operations, and final docs approval? | Ownership | HIGH | Escalation and approval are unavailable | All sections | Yes, ownership remains unclear | Project owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-16 | What product/repository license and distribution terms are authoritative? | Governance | MEDIUM | Backend crate license is not whole-repository policy | Project, contribution, release | Yes, avoid license inference | Project/legal owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-17 | What retention lifecycle applies to ignored Marketplace archives and local generated samples? | Tooling/artifacts | LOW | Local output cleanup/reproducibility is unclear | Project, extensibility, development | Yes | Marketplace/maintainer owner | `NEEDS_OWNER_CONFIRMATION` |
| NOC-18 | Which terms/abbreviations are preferred where code, UI, and historical docs differ? | Terminology | MEDIUM | Inconsistent language causes drift | Glossary and all sections | Yes, preserve identifiers | Project/documentation owner | `NEEDS_OWNER_CONFIRMATION` |

## Recommended Answer Order

Answer NOC-03, NOC-04, NOC-06, NOC-15, NOC-02, NOC-05, and NOC-09 before claiming production readiness or recovery guarantees. Record decisions in the affected source/policy and update OKF in the same change.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

