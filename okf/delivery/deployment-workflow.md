---
okf_document_id: "delivery-deployment-workflow"
title: "Deployment Workflow"
project: "ZinharCMS"
category: "delivery"
phase: 10
status: "current"
review_status: "mixed"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "docker-compose.prod.yml"
  - "backend/Dockerfile.prod"
  - "frontend/Dockerfile.prod"
  - "backend/src/main.rs"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
related_documents:
  - "environment-promotion.md"
  - "database-deployment.md"
  - "rollback-and-recovery.md"
  - "../development/build-and-quality.md"
  - "../security/secrets-and-configuration.md"
  - "../operations/service-lifecycle.md"
related_diagrams:
  - "diagrams/deployment-flow.mmd"
---

# Deployment Workflow

## Verified Configuration

`docker-compose.prod.yml` can build and assemble PostgreSQL, Redis, backend, and frontend containers with named persistent volumes. The backend loads configuration, runs migrations, seeds bootstrap data when applicable, initializes Redis state, binds HTTP, and exposes health/readiness routes. The frontend is a static Nginx SPA image.

## Unverified Production Behavior

| Concern | Status |
| --- | --- |
| Deployment target/provider/cluster | `DEPLOYMENT_TARGET_UNCLEAR` |
| Trigger and approval | No deployment workflow or command; `DEPLOYMENT_TARGET_UNCLEAR` |
| Artifact source | Compose builds from source locally; production artifact registry is unknown |
| Configuration injection | Compose environment names/defaults are verified |
| Secret injection | Variable structure is verified; production secret manager/source is `SECRET_INJECTION_UNCLEAR` |
| Database migration | Backend startup automatically runs all pending embedded migrations |
| Service startup | Compose dependency health for PostgreSQL/Redis; app health gates absent |
| Traffic switching/TLS/DNS | `PRODUCTION_TOPOLOGY_UNKNOWN` |
| Restart policy | No Compose `restart` policy found |
| Zero downtime/rollout | Not defined; do not claim |
| Failure notifications | No integration found |
| Rollback | Manual narrative only; see rollback document |

## Documentation Conflict

V2/V3 runbooks describe ordered deployment, pre-migration backup, explicit migration deployment, application replacement, health checks, and rollback. Executable configuration does not implement that orchestrator: migrations run inside every backend startup, no backup command exists, no application health check is wired into Compose, and no previous artifact source is declared. The runbooks are operational intent, not an executable deployment contract (`DOCUMENTATION_CODE_CONFLICT`, `ROLLBACK_BEHAVIOR_UNCLEAR`).

No deployment command was executed in Phase 10. See [Database Deployment](database-deployment.md), [Service Lifecycle](../operations/service-lifecycle.md), [Secrets](../security/secrets-and-configuration.md), and [Deployment Diagram](diagrams/deployment-flow.mmd).

