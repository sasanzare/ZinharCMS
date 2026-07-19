---
okf_document_id: "development-workflow"
title: "Development Workflow"
project: "ZinharCMS"
category: "development"
phase: 10
status: "current"
review_status: "mixed"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - "package.json"
  - "backend/migrations"
related_documents:
  - "commands.md"
  - "build-and-quality.md"
  - "testing-workflow.md"
  - "contribution-workflow.md"
  - "../maintenance/okf-update-policy.md"
related_diagrams: []
---

# Development Workflow

| Stage | Observed behavior | Classification | Enforcement |
| --- | --- | --- | --- |
| Branch creation | Recent OKF work uses `docs/okf-phase-zero`; no general branch naming rule exists | `INFERRED_CONVENTION` | Not repository-enforced |
| Environment startup | Compose infrastructure plus separate backend and frontend processes | `EXPLICIT_CONVENTION` in manifests/README | Developer-controlled |
| Code changes | Backend, frontend, migrations, tests, and docs are stored in separate top-level areas | `INFERRED_CONVENTION` from structure | Review-controlled; ownership unknown |
| Backend formatting | `cargo fmt --check` | `EXPLICIT_CONVENTION` | Backend CI |
| Backend linting | `cargo clippy --all-targets --all-features -- -D warnings` | `EXPLICIT_CONVENTION` | Backend CI; known legacy warnings may keep it red |
| Backend tests | `cargo test --all-features` | `EXPLICIT_CONVENTION` | Backend CI |
| Frontend linting | `npm run lint` | `EXPLICIT_CONVENTION` | Frontend CI |
| Frontend type checking | `npm run typecheck` and as part of build | `EXPLICIT_CONVENTION` | Frontend CI |
| Frontend tests | `npm test` (`vitest --run`) | `EXPLICIT_CONVENTION` | Frontend CI |
| Frontend build | `npm run build` | `EXPLICIT_CONVENTION` | Frontend CI |
| Database changes | Ordered SQL files under `backend/migrations`; application startup applies them | `EXPLICIT_CONVENTION` | Compile/runtime; no migration-specific CI job |
| Documentation changes | Markdown and Mermaid are tracked; no docs CI workflow exists | `INFERRED_CONVENTION` | Manual validation only |
| Commit preparation | Inspect status/diff and run relevant gates | `EXPLICIT_CONVENTION` for Codex through `AGENTS.md`; otherwise not governed | Local process |
| Pull request | CI runs on path-filtered pull requests | `EXPLICIT_CONVENTION` | GitHub Actions only when backend/frontend paths match |
| Review and merge | No CODEOWNERS, PR template, branch protection, approval count, or merge strategy is stored | `NEEDS_OWNER_CONFIRMATION NOC-14` | Unknown external settings |

## Recommended Evidence-Based Sequence

1. Read the affected OKF subsystem and [Prerequisites](prerequisites.md).
2. Start only required dependencies.
3. Make the smallest coherent implementation and test change.
4. Run the smallest relevant formatter, linter, type check, or filtered test.
5. For a migration, inspect ordering, tenant policy, backward compatibility, and startup behavior.
6. Update affected documentation and `okf/index.yaml` in the same change when implementation makes them stale.
7. Run the wider repository-defined gates for changed areas.
8. Inspect `git status --short`, `git diff`, and `git diff --check` before review.
9. Prepare a PR only after reporting any failing or unexecuted checks honestly.

This sequence is a documented recommendation, not an owner-approved merge policy. See [Contribution Workflow](contribution-workflow.md), [Backend Testing](../backend/testing-map.md), and [Frontend Testing](../frontend/testing-map.md).

