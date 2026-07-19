---
okf_document_id: "development-build-quality"
title: "Build and Quality"
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
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - "backend/Dockerfile.prod"
  - "frontend/Dockerfile.prod"
  - "frontend/eslint.config.js"
  - "frontend/tsconfig.json"
related_documents:
  - "commands.md"
  - "testing-workflow.md"
  - "../backend/testing-map.md"
  - "../frontend/configuration-and-build.md"
  - "../delivery/ci-architecture.md"
related_diagrams: []
---

# Build and Quality

| Gate | Tool and command | Scope | Configuration | CI enforcement | Failure behavior | Local use | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Backend development build | `cargo build` | Backend crate | Cargo manifest/lock | Indirect through Clippy/tests | Nonzero; diagnostics | From `backend/` | `VERIFIED` definition |
| Backend production build | `cargo build --release` | Backend binary | Production Dockerfile | Not in GitHub Actions | Docker build fails | From `backend/` or image build | `VERIFIED` definition |
| Rust format | `cargo fmt --check` | Backend Rust | rustfmt from stable toolchain | Yes | Job fails | Run before tests | `VERIFIED` |
| Rust static analysis | `cargo clippy --all-targets --all-features -- -D warnings` | All backend targets/features | Clippy on CI stable | Yes | Any warning fails job | Run after format | `VERIFIED`; legacy-warning state requires current run |
| Backend tests | `cargo test --all-features` | Unit/static contract/doc tests | Cargo test | Yes | Any failing test fails job | Filter first, then widen | `VERIFIED` |
| Frontend lint | `npm run lint` | TS/TSX except `dist` | `eslint.config.js` | Yes | ESLint nonzero | Run from `frontend/` | `VERIFIED` |
| Frontend type check | `npm run typecheck` | TS project references | strict TS configs | Yes | TypeScript nonzero | Run from `frontend/` | `VERIFIED` |
| Frontend tests | `npm test` | Three jsdom test files | `vitest.config.ts` | Yes | Vitest nonzero | Filters may be passed to Vitest | `VERIFIED` |
| Frontend production build | `npm run build` | Type check plus Vite bundle | Vite and TS configs | Yes | Either stage fails | Writes ignored `dist` | `VERIFIED` |
| Container builds | Docker build through Compose or Dockerfiles | Backend/frontend images | Four Dockerfiles, two Compose files | No | Docker build fails | Environment dependent | `VERIFIED` configuration |
| Dependency integrity | Cargo lock and frontend lock are tracked | Backend/frontend dependency graph | Lock files | Used by tools, but frontend CI runs `npm install`, not `npm ci` | Resolver/install failure | Review lock diffs | `PARTIALLY_DEFINED` |
| Security scanning | No SAST, dependency audit, image scan, or secret-scan job found | Repository | None | No | Not applicable | Manual/owner decision | `PLANNED_NOT_IMPLEMENTED` |
| Generated-code drift | No generated client/schema check found | OpenAPI/frontend contracts | None | No | Not applicable | Manual comparison | `PLANNED_NOT_IMPLEMENTED` |
| Documentation links/YAML | No repository CI command before Phase 10 | `okf/` | Validation policy in maintenance docs | No | Manual validator reports failures | Run Phase 10 validation method | `VERIFIED` absence |
| Mermaid parser | No parser dependency or `mmdc` executable found | `.mmd` files | Static checks only | No | Parser-level result unavailable | Structural validation only | `COMMAND_STATUS_UNCLEAR` |

## CI and Local Gaps

- Production release builds and container builds exist locally/configurationally but are not GitHub Actions jobs.
- Backend CI includes Clippy, while root package scripts do not expose it.
- Frontend CI installs with `npm install` even though a lock file exists; no immutable-install gate is declared.
- Path filters mean documentation-only, root-manifest-only, Compose-only, and many script-only changes do not trigger either workflow.
- There is no workspace build command because the repository is not a Cargo or npm workspace; root scripts orchestrate selected subprojects.

See [CI Architecture](../delivery/ci-architecture.md), [Backend Testing](../backend/testing-map.md), and [Frontend Build](../frontend/configuration-and-build.md).

