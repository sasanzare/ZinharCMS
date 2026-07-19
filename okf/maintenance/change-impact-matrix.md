---
okf_document_id: "maintenance-change-impact"
title: "Change Impact Matrix"
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
  - "okf/index.yaml"
  - "okf/project/repository-map.md"
  - "okf/references/source-register.md"
related_documents:
  - "README.md"
  - "okf-update-policy.md"
  - "review-checklist.md"
  - "staleness-detection.md"
related_diagrams: []
---

# Change Impact Matrix

| Code/configuration change | Minimum OKF sections to review | Common secondary impact |
| --- | --- | --- |
| Add/remove backend module | Backend, architecture, project repository map, index | API, database, domain, security, tests |
| Change middleware/router composition | Backend, API, security, architecture | Frontend/API clients, operations health |
| Add/change endpoint | API, backend, index | Frontend if consumed, security, domain, testing |
| Change request/response/error DTO | API, backend, frontend contract map | Versioning, tests, security disclosure |
| Add frontend route/page/feature | Frontend, project map, index | API, security, domain, testing |
| Change frontend state/session/storage | Frontend, security | API, domain, troubleshooting |
| Add migration/table/index/constraint | Database, backend, index | Domain, security/RLS, delivery, operations |
| Change RLS/tenant ownership | Database, security, backend | API, domain, tests, risks |
| Change role/permission/auth/session | Security, API, backend, frontend | Domain, database, extensibility, threats |
| Change business rule/state transition | Domain, backend, API | Frontend, database constraints, tests/diagrams |
| Change plugin/manifest/hook/component contract | Extensibility, security, backend/API | Frontend, database, domain, compatibility |
| Change Marketplace lifecycle/finance/moderation | Extensibility, domain, API, security | Database, frontend, operations/runbooks |
| Change manifest/script command | Development | Delivery/operations and affected subsystem |
| Change CI workflow/action/job/trigger | Delivery, development | Security, maintenance validation, project map |
| Change Dockerfile/Compose/Nginx | Delivery, operations | Security, development prerequisites, architecture |
| Add/change environment variable | Operations configuration, security secrets | Backend/frontend config, delivery promotion |
| Change migration startup behavior | Delivery database deployment, operations lifecycle | Database migrations, development workflow |
| Change health/readiness/logging/metrics | Operations | Delivery, architecture, security, troubleshooting |
| Add backup/restore/DR behavior | Operations, delivery | Database, security, runbooks, owner questions |
| Add/remove documentation or diagram | Index, navigation, references, maintenance validation | Related subsystem and conflict/question registers |
| Resolve owner question/conflict | Owner-question/conflict register, all affected docs | Glossary, risks, final report/index uncertainty |

Apply [OKF Update Policy](okf-update-policy.md) and [Review Checklist](review-checklist.md); review transitive consumers rather than treating this as an exhaustive static dependency list.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

