---
okf_document_id: "maintenance-documentation-ownership"
title: "Documentation Ownership"
project: "ZinharCMS"
category: "maintenance"
phase: 10
status: "current"
review_status: "mixed"
source_of_truth: false
implementation_view: "prescriptive"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "AGENTS.md"
  - "okf/index.yaml"
  - ".github/workflows"
related_documents:
  - "README.md"
  - "okf-update-policy.md"
  - "unresolved-owner-questions.md"
  - "review-checklist.md"
related_diagrams: []
---

# Documentation Ownership

No CODEOWNERS file, maintainer roster, team directory, on-call schedule, or approval map exists. The classes below identify the minimum expertise needed; they do not invent people or prove repository permissions.

| OKF section | Ownership classification | Required collaboration | Evidence/status |
| --- | --- | --- | --- |
| Project | `PROJECT_OWNER`, `NEEDS_OWNER_CONFIRMATION` | All maintainers for scope changes | Product intent and license require owner decisions |
| Architecture | `SHARED_OWNERSHIP` | Backend, frontend, database, security, DevOps | Cross-cutting evidence; no named architect |
| Backend | `BACKEND_MAINTAINERS` | Database/security/domain as affected | Source boundary is clear; people are unknown |
| Frontend | `FRONTEND_MAINTAINERS` | API/security/design/product as affected | Source boundary is clear; people are unknown |
| Database | `DATABASE_MAINTAINERS` | Backend/security/operations | Migration ownership is technical, not assigned |
| API | `SHARED_OWNERSHIP` | Backend plus frontend/external consumer reviewers | Manual/generated contracts span areas |
| Security | `SECURITY_REVIEWER`, `NEEDS_OWNER_CONFIRMATION` | Backend/frontend/database/operations | No named security owner or policy file |
| Domain | `PROJECT_OWNER`, `SHARED_OWNERSHIP` | Product and implementation maintainers | Code proves mechanics, not product rationale |
| Extensibility | `SHARED_OWNERSHIP` | Backend, frontend, security, Marketplace/product owner | Trust and compatibility cross boundaries |
| Development | `SHARED_OWNERSHIP` | Backend/frontend/database maintainers | Commands are component-owned; governance absent |
| Delivery | `DEVOPS_MAINTAINERS`, `NEEDS_OWNER_CONFIRMATION` | Component, database, security reviewers | CI exists; release/deployment ownership unknown |
| Operations | `OPERATIONAL_OWNERSHIP_UNCLEAR` | DevOps, database, security, support | No live topology or on-call roster |
| Maintenance | `SHARED_OWNERSHIP` | Every section owner class | This policy assigns no person |
| References | `SHARED_OWNERSHIP` | Maintainer of the evidenced area | Authority changes with source freshness |

## Escalation Rule

When a change needs an unavailable owner, preserve the current evidence-based statement, add `NEEDS_OWNER_CONFIRMATION`, identify the smallest decision, and do not convert a technical inference into policy. High-risk production, security, backup, restore, migration, secret, or data-retention decisions must not be approved solely by an AI agent.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

