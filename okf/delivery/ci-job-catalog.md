---
okf_document_id: "delivery-ci-job-catalog"
title: "CI Job Catalog"
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
  - "ci-architecture.md"
  - "../development/testing-workflow.md"
  - "../backend/testing-map.md"
  - "../frontend/testing-map.md"
  - "../operations/external-dependencies.md"
related_diagrams:
  - "diagrams/ci-pipeline.mmd"
---

# CI Job Catalog

| Job ID | Workflow | Purpose | Trigger | Runner | Dependencies/services | Main steps and commands | Artifacts/cache | Secrets | Failure behavior | Required status | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CI-BE-TEST | `Backend CI` | Format, lint, and test backend | Matching push/PR | `ubuntu-latest` | PostgreSQL 16 and Redis 7 service containers | Checkout; stable Rust with Clippy/rustfmt; Rust cache; `cargo fmt --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all-features` | Rust cache only; no uploaded artifact | No GitHub secret reference; synthetic env is inline | First nonzero run step fails job | External branch-protection status `CI_BEHAVIOR_UNCLEAR` | `VERIFIED` definition |
| CI-FE-TEST | `Frontend CI` | Install, lint, type-check, test, and build frontend | Matching push/PR | `ubuntu-latest` | Node 22; package registry access | Checkout; setup Node; `npm install`; `npm run lint`; `npm run typecheck`; `npm test`; `npm run build` | Build remains ephemeral; no explicit cache/artifact | No secret reference | First nonzero run step fails job | External branch-protection status `CI_BEHAVIOR_UNCLEAR` | `VERIFIED` definition |

## Review Findings

- Job IDs duplicate the generic name `test` across different workflows; GitHub distinguishes them by workflow. This is not a functional duplicate.
- No unused-looking job exists because there are only the two active jobs.
- Neither workflow declares that its job is required; external repository settings were not inspected.
- Backend CI does not build the production image or upload a binary. Frontend CI builds but does not upload the bundle.
- Missing areas include browser E2E, documentation/YAML/link/Mermaid checks, dependency/security/image scanning, API/OpenAPI parity, migration deployment smoke, and release/deploy validation.
- Action references are not immutable SHAs. Effective GitHub token permissions are unclear because `permissions` is omitted.
- Frontend uses `npm install`, not an explicitly immutable install command.

See [Testing Workflow](../development/testing-workflow.md), [Backend Testing](../backend/testing-map.md), [Frontend Testing](../frontend/testing-map.md), and [CI Architecture](ci-architecture.md).

