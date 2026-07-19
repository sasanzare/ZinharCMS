---
okf_document_id: "development-risks"
title: "Development Risks"
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
  - "package.json"
  - "README.md"
  - ".github/workflows"
  - "backend/src/main.rs"
  - "frontend/package.json"
related_documents:
  - "prerequisites.md"
  - "commands.md"
  - "testing-workflow.md"
  - "contribution-workflow.md"
  - "../maintenance/staleness-detection.md"
related_diagrams: []
---

# Development Risks

| Risk ID | Title | Evidence | Impact | Likelihood | Severity | Existing mitigation | Recommended follow-up | Owner confirmation required | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| DEV-R01 | Toolchain support is inconsistent | CI Node 22; Docker Node 24; CI stable Rust; Docker Rust 1.87 | Local/CI/container drift | Medium | HIGH | Lock files and images | Define supported Rust, Node, npm, OS, and browser matrix | Yes | `ENVIRONMENT_REQUIREMENT_UNCLEAR` |
| DEV-R02 | Root `dev` name can imply a full stack | Local Compose has only infrastructure | Backend/frontend may be assumed running | Medium | MEDIUM | Separate backend/frontend scripts | Rename or clarify script in a future implementation change | Yes | `DOCUMENTATION_CODE_CONFLICT` risk |
| DEV-R03 | Backend startup mutates schema and seeds | `main.rs` runs migrations and bootstrap before bind | Unexpected data/schema mutation during debugging | Medium | HIGH | Startup fails on migration error | Separate explicit migration/seed policy and remove deterministic bootstrap credentials for non-development use | Yes | `VERIFIED` risk |
| DEV-R04 | No safe reset or downgrade workflow | No reset/down command or down migrations | Ad hoc destructive recovery | Medium | HIGH | Documentation warns against deletion | Define disposable database and reset process | Yes | `COMMAND_STATUS_UNCLEAR` |
| DEV-R05 | Backend test isolation is incomplete | No general database fixture/reset harness | Environment-dependent failures and limited integration coverage | High | HIGH | Pure/static tests and CI services | Add isolated integration harness | Yes | `TEST_ISOLATION_UNCLEAR TIU-01` |
| DEV-R06 | Frontend test surface is narrow | Three test files; no browser E2E | UI regressions may escape | High | MEDIUM | Lint/typecheck/build/Vitest | Define browser and accessibility test strategy | Yes | `TEST_COVERAGE_GAP` |
| DEV-R07 | Documentation has no CI gate | CI paths cover backend/frontend only | Broken OKF links/metadata can merge | High | HIGH | Phase 10 validation checklist | Add non-mutating docs validation workflow later | Yes | `PLANNED_NOT_IMPLEMENTED` |
| DEV-R08 | Contribution governance is absent | No contribution guide, templates, CODEOWNERS, or branch policy | Inconsistent review and validation | High | HIGH | Observable commit/CI conventions | Owner-approved contribution and ownership policy | Yes | `NEEDS_OWNER_CONFIRMATION NOC-14/NOC-15` |
| DEV-R09 | Frontend dependency install is mutable | CI and images use `npm install` with a lock file | Resolver drift or unnoticed lock changes | Medium | MEDIUM | Tracked lockfile | Decide immutable installation policy | Yes | `INFERRED_FROM_WORKFLOWS` |
| DEV-R10 | Clippy is a strict CI gate with legacy debt | CI denies all warnings; prior verified runs report legacy warnings | Backend PRs may remain red | High | HIGH | Format/tests can still run | Fix warnings or approve a scoped policy | Yes | `CI_BEHAVIOR_UNCLEAR` until current run |
| DEV-R11 | Generated/API contract drift is manual | No client generation or drift gate | Frontend/OpenAPI mismatch | Medium | HIGH | Phase 6 comparison docs/tests | Add route/OpenAPI/client parity validation | Yes | `DUPLICATED_CONTRACT DC-01` |
| DEV-R12 | Platform-specific scripts | Operational scripts are PowerShell and include Windows npm resolution | Non-Windows developer friction | Medium | MEDIUM | Core Cargo/npm commands are cross-platform | Define cross-platform script support | Yes | `INFERRED_FROM_SCRIPTS` |

See [Commands](commands.md), [Testing Workflow](testing-workflow.md), [Contribution Workflow](contribution-workflow.md), and [Staleness Detection](../maintenance/staleness-detection.md).

