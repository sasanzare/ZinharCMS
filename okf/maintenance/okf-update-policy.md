---
okf_document_id: "maintenance-update-policy"
title: "OKF Update Policy"
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
  - "AGENTS.md"
  - "okf/index.yaml"
  - "okf/README.md"
related_documents:
  - "README.md"
  - "change-impact-matrix.md"
  - "review-checklist.md"
  - "staleness-detection.md"
  - "validation-report.md"
related_diagrams: []
---

# OKF Update Policy

OKF documentation should normally change in the same pull request as the implementation, configuration, migration, contract, or operational change that makes it stale. Executable repository evidence remains authoritative; update OKF claims rather than changing product behavior to satisfy prose.

| Change category | OKF sections to review | Required reviewer class | Minimum validation | Timing |
| --- | --- | --- | --- | --- |
| Architecture or runtime boundary | Project, architecture, affected modules, operations, index | `SHARED_OWNERSHIP`; architecture owner needs confirmation | Source paths, links, diagram, status/unknown review | Same change |
| Backend module/service/middleware | Backend, architecture, API, database/domain/security as affected | `BACKEND_MAINTAINERS` | Relevant tests; module/catalog/index/link checks | Same change |
| Frontend feature/state/route | Frontend, API, security, domain | `FRONTEND_MAINTAINERS` | Lint/typecheck/tests/build references; route/client links | Same change |
| Route/request/response/error | API, backend, frontend consumers, security | Backend plus consumer maintainers | Route/OpenAPI/client parity and link/path checks | Same change |
| Database migration/schema/index/RLS | Database, backend, domain, security, delivery/operations | `DATABASE_MAINTAINERS` plus backend/security where relevant | Migration path/order, evidence, tenant/rollback unknowns | Same change before deployment |
| Authentication/authorization/role/permission | Security, API, backend, frontend, domain, extensibility | `SECURITY_REVIEWER` plus maintainers | Access boundary, ownership, tenant, test, threat/risk review | Same change |
| Business rule/state transition | Domain, backend, API, frontend, database | Product/project owner plus maintainers | Rule evidence, enforcement layers, state diagram/tests | Same change |
| Plugin/Marketplace contract | Extensibility, security, API, backend/frontend, domain | Shared backend/frontend/security | Manifest, permission, tenant, compatibility, trust, test review | Same change |
| Build/command/toolchain | Development, delivery, affected subsystem | Maintainer/DevOps owner | Manifest/workflow command trace and local/CI status | Same change |
| CI/workflow/release | Delivery, development, operations, security | `DEVOPS_MAINTAINERS` or `NEEDS_OWNER_CONFIRMATION` | YAML parse, job/trigger/action/artifact/secret review | Same change |
| Container/deployment/environment variable | Delivery, operations, security, development | DevOps/security/affected maintainer | Compose/Docker/config path, secret, lifecycle, health, rollback review | Same change |
| Logging/metrics/alerts/runbook/backup | Operations, delivery, security, database | `OPERATIONAL_OWNERSHIP_UNCLEAR` until assigned | Evidence, no-secret, failure/recovery and owner-question review | Same change |
| Documentation-only correction | Affected docs, references, index, conflict register | Section owner class | YAML, links, source paths, metadata, terminology, scope | Same change |

## Update Rules

1. Preserve `UNKNOWN`, `NEEDS_OWNER_CONFIRMATION`, conflict, planned, partial, and unverified markers until evidence or an owner decision resolves them.
2. Set verification commit/date to the evidence snapshot actually reviewed; never copy a future or guessed hash.
3. Register every added/removed document and diagram in `okf/index.yaml`.
4. Resolve relative links and evidence paths and keep IDs unique.
5. Update risk, conflict, owner-question, navigation, glossary, and source registers when their scope changes.
6. Do not copy secret values, private data, or production outputs into OKF.
7. Record executed and unexecuted validation honestly.
8. Do not use an OKF edit as authorization to deploy, migrate, publish, restore, or change infrastructure.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

