---
okf_document_id: "delivery-ci-architecture"
title: "CI Architecture"
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
related_documents:
  - "ci-job-catalog.md"
  - "../development/build-and-quality.md"
  - "../database/database-testing.md"
  - "../security/security-testing.md"
  - "../operations/health-and-readiness.md"
related_diagrams:
  - "diagrams/ci-pipeline.mmd"
---

# CI Architecture

## Workflow Summary

| Workflow | File | Triggers and paths | Jobs | Services | Cache | Artifacts |
| --- | --- | --- | --- | --- | --- | --- |
| `Backend CI` | `.github/workflows/backend-ci.yml` | `push` and `pull_request` when `backend/**` or the workflow file changes | `test` | PostgreSQL 16, Redis 7 | `Swatinem/rust-cache@v2` | None uploaded |
| `Frontend CI` | `.github/workflows/frontend-ci.yml` | `push` and `pull_request` when `frontend/**` or the workflow file changes | `test` | None | No explicit cache | None uploaded |

## Trigger and Execution Model

- No branch-name restriction is declared; the path filters govern push/PR eligibility.
- No `workflow_dispatch`, scheduled trigger, release trigger, tag filter, or deployment event exists.
- Each workflow has one job, so there are no job dependencies or matrices.
- Both use `ubuntu-latest`; runner image immutability is not guaranteed by the repository.
- No workflow-level or job-level `concurrency`, cancellation, timeout, environment, or approval configuration is declared.
- No explicit `permissions` block exists; effective token permissions depend on GitHub defaults and external repository settings (`CI_BEHAVIOR_UNCLEAR`).
- No required-check or branch-protection configuration is tracked; job existence does not prove it is mandatory for merge.

## Dependencies, Caches, Services, and Secrets

Backend CI checks out source, installs the stable Rust toolchain with rustfmt/Clippy, restores a Rust cache, and supplies synthetic CI configuration for PostgreSQL, Redis, JWT, upload, CORS, logging, and port variables. No real secret value is required by the tracked workflow. Frontend CI selects Node 22 and uses `npm install`; it has no explicit cache or secret.

External actions use major/stable references (`actions/checkout@v4`, `actions/setup-node@v4`, `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`) rather than immutable commit SHAs. This is a verifiable supply-chain configuration fact, not evidence of compromise.

## Coverage and Failure Behavior

Any nonzero step stops its sequential job. Workflows are independent. Documentation, root scripts/manifests, Compose, Dockerfiles outside matching component paths, and most operational scripts can change without either CI workflow running. No application artifact, test report, coverage report, OpenAPI file, image, checksum, or documentation bundle is uploaded.

See [CI Job Catalog](ci-job-catalog.md), [Build and Quality](../development/build-and-quality.md), and [CI Pipeline Diagram](diagrams/ci-pipeline.mmd).

