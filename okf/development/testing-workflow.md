---
okf_document_id: "development-testing-workflow"
title: "Testing Workflow"
project: "ZinharCMS"
category: "development"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src"
  - "frontend/src"
  - "frontend/vitest.config.ts"
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - "scripts"
related_documents:
  - "build-and-quality.md"
  - "../backend/testing-map.md"
  - "../frontend/testing-map.md"
  - "../database/database-testing.md"
  - "../api/api-testing.md"
  - "../security/security-testing.md"
  - "../extensibility/extensibility-testing.md"
  - "../domain/business-rule-testing.md"
related_diagrams: []
---

# Testing Workflow

| Test type | Command | Working directory | Dependencies | Environment | Data setup | CI execution | Output | Known limitations |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Backend unit and static contract | `cargo test --all-features` | `backend/` | Rust/Cargo | Backend CI provides PostgreSQL and Redis variables | Most inspected tests are pure/static; no universal fixture harness | Yes | Cargo test report | Test categories are colocated; no separate coverage report |
| Filtered backend test | `cargo test <filter>` | `backend/` | Rust/Cargo | Test-specific | Test-specific | No dedicated filtered CI | Filtered report | Filter strings are source test/module names, not a stable public catalog |
| Backend integration/API | No separate repository command | N/A | Running services would be required | Environment dependent | No general fixture/reset harness | No | N/A | `TEST_ISOLATION_UNCLEAR TIU-01`; static route contracts are not full HTTP integration |
| Frontend unit/component | `npm test` | `frontend/` | Installed dependencies | jsdom | Test-local mocks and setup | Yes | Vitest report | Only Dashboard, Marketplace, and Pages test files are tracked |
| Frontend filtered test | `npm test -- <file-or-pattern>` | `frontend/` | Installed dependencies | jsdom | Test-local | No dedicated filtered CI | Vitest filtered report | The script already includes `--run`; do not append another `--run` |
| Browser end-to-end | No command found | N/A | Browser/runtime | Unknown | Unknown | No | N/A | No Playwright/Cypress/browser matrix |
| Database schema | `cargo test --all-features` plus startup migration behavior | `backend/` | Rust; PostgreSQL for live startup | CI database available, but test use varies | No dedicated disposable database harness | Indirect | Test/migration errors | Applied schema equivalence is environment-specific |
| API contracts | Backend tests and optional readiness scripts | Root or `backend/` | Toolchain; optional running API | Scripts accept synthetic URL/token context | No general fixture | Backend tests only | Test or JSON report | OpenAPI omits known registered handlers/security metadata |
| Security | Backend service/route tests under all-features | `backend/` | Rust | Mostly pure/static | Test-local | Indirect | Cargo output | No complete IDOR, cross-tenant, replay, browser, or fuzz suite |
| Plugin/Marketplace | `cargo test marketplace`; `cargo test plugins::seo` | `backend/` | Rust | Test-specific | Static/pure tests; provider smoke needs credentials | Included only through full backend job | Filtered Cargo output | Uploaded code is not executed; end-to-end adapter coverage is incomplete |
| Domain workflow | `cargo test workflow` and broader backend tests | `backend/` | Rust | Test-specific | Pure transition examples and static contracts | Included through full job | Filtered Cargo output | Not every route-state combination has an integration test |
| GA/readiness | `scripts/v2-ga-check.ps1`; `scripts/marketplace-phase15-ga-check.ps1` | Root | PowerShell, Rust, npm; optional API | Optional credentials and organization | Safe target data required | No | Console/table/JSON | These are readiness checks, not deployment jobs |
| Load smoke | `scripts/phase8-load-smoke.ps1`; `scripts/marketplace-phase13-load-smoke.ps1` | Root | PowerShell and running API | URL and optional auth | Optional install mutation is explicitly gated | No | Latency/status report | Results are environment-specific; mutation flag requires explicit authorization |

## Parallelism, Cleanup, and Failure Debugging

Cargo and Vitest may use their tool defaults; the repository does not define a project-specific parallelism policy. CI jobs are in separate workflows and may run independently when both path filters match. No universal test database creation/reset/cleanup command exists. Avoid destructive cleanup; remove only fixtures created by the current test and verify their absence.

On failure, rerun the smallest test filter, preserve complete diagnostics, inspect environment/service health, and do not convert an environment failure into a passing test claim. See [Debugging](debugging.md), [Database Testing](../database/database-testing.md), [API Testing](../api/api-testing.md), and [Security Testing](../security/security-testing.md).

