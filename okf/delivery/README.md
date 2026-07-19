---
okf_document_id: "delivery-readme"
title: "Delivery"
project: "ZinharCMS"
category: "delivery"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - "docker-compose.prod.yml"
  - "backend/Dockerfile.prod"
  - "frontend/Dockerfile.prod"
related_documents:
  - "../development/README.md"
  - "../database/README.md"
  - "../security/README.md"
  - "../operations/README.md"
  - "../maintenance/README.md"
related_diagrams:
  - "diagrams/ci-pipeline.mmd"
  - "diagrams/release-flow.mmd"
  - "diagrams/deployment-flow.mmd"
---

# Delivery

## Scope and Authority

This section covers current CI, build artifacts, container images, release evidence, production-like Compose configuration, migrations during startup, promotion uncertainty, and rollback guidance. Workflow files, Dockerfiles, Compose, startup code, and scripts outrank release prose.

## Pipeline Overview

GitHub Actions has two path-filtered CI workflows with one `test` job each. No tracked CD, release publication, image push, environment promotion, or production deployment workflow exists. `docker-compose.prod.yml` is a production-like assembly definition; it does not prove any live production topology or deployment.

## Build, Release, and Deployment

- **Build** is implemented through Cargo, Vite/TypeScript, and Dockerfiles.
- **Release** has milestone notes and readiness scripts, but no formal trigger, tag, publication, signing, or approval automation (`RELEASE_PROCESS_UNCLEAR`).
- **Deployment** has a production-like Compose model but no target/provider or deploy workflow (`DEPLOYMENT_TARGET_UNCLEAR`, `PRODUCTION_TOPOLOGY_UNKNOWN`).

## Reading Order

1. [CI Architecture](ci-architecture.md)
2. [CI Job Catalog](ci-job-catalog.md)
3. [Artifact Production](artifact-production.md)
4. [Container Builds](container-builds.md)
5. [Release Process](release-process.md)
6. [Deployment Workflow](deployment-workflow.md)
7. [Environment Promotion](environment-promotion.md)
8. [Database Deployment](database-deployment.md)
9. [Rollback and Recovery](rollback-and-recovery.md)
10. [Delivery Risks](delivery-risks.md)

Diagrams: [CI Pipeline](diagrams/ci-pipeline.mmd), [Release Flow](diagrams/release-flow.mmd), and [Deployment Flow](diagrams/deployment-flow.mmd).

Related context: [Development](../development/README.md), [Database](../database/README.md), [Security](../security/README.md), and [Operations](../operations/README.md).

