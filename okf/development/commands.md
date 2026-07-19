---
okf_document_id: "development-commands"
title: "Command Catalog"
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
  - "frontend/package.json"
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - "scripts"
  - "backend/src/main.rs"
related_documents:
  - "local-environment.md"
  - "build-and-quality.md"
  - "testing-workflow.md"
  - "database-development.md"
  - "../delivery/ci-job-catalog.md"
related_diagrams: []
---

# Command Catalog

Commands are reproduced only from current manifests, workflows, scripts, Dockerfiles, or tracked documentation. `Safe` describes the command's normal local effect; it is not a production authorization.

| ID | Group | Command | Working directory | Purpose | Source | Inputs | Output or side effects | Safe | Destructive or special status | Required environment | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CMD-ROOT-01 | Root, Docker | `npm run dev:infra` | Root | Start PostgreSQL, Redis, pgAdmin detached | `package.json` | Compose engine | Creates/starts containers and named volumes | Yes for local | Mutates local container state | Docker | `ENVIRONMENT_DEPENDENT` |
| CMD-ROOT-02 | Root, development | `npm run dev` | Root | Run local Compose in foreground | `package.json` | Compose engine | Starts infrastructure only; no app services in local Compose | Yes for local | Persistent DB/cache volumes | Docker | `VERIFIED` definition |
| CMD-ROOT-03 | Backend, development | `npm run dev:backend` | Root | Run backend through Cargo | `package.json` | Environment, database | Applies migrations, may seed, starts API | Conditional | **Runs migrations** and may insert bootstrap records | PostgreSQL, configuration; Redis URL | `ENVIRONMENT_DEPENDENT` |
| CMD-ROOT-04 | Frontend, development | `npm run dev:frontend` | Root | Start Vite dev server | `package.json` | Installed dependencies | Development server on configured port | Yes | None | Node/npm | `ENVIRONMENT_DEPENDENT` |
| CMD-ROOT-05 | Testing | `npm run test:backend` | Root | Full backend feature tests | `package.json` | Rust toolchain | Test output, build artifacts | Yes | Creates `backend/target` artifacts | Rust | `VERIFIED` definition |
| CMD-ROOT-06 | Testing | `npm run test:frontend` | Root | Frontend Vitest run | `package.json` | Installed dependencies | Test output | Yes | None | Node/npm | `VERIFIED` definition |
| CMD-ROOT-07 | Build | `npm run build:frontend` | Root | Production frontend build | `package.json` | Installed dependencies | Writes `frontend/dist` | Yes | Replaces generated build output | Node/npm | `VERIFIED` definition |
| CMD-BE-01 | Backend, build | `cargo build` | `backend/` | Debug backend build | `backend/Dockerfile` | Cargo manifest/lock | `target/debug` artifacts | Yes | Generated files | Rust | `VERIFIED` definition |
| CMD-BE-02 | Backend, production build | `cargo build --release` | `backend/` | Optimized binary | `backend/Dockerfile.prod` | Cargo manifest/lock | `target/release/cms-backend` | Yes | Generated files | Rust | `VERIFIED` definition |
| CMD-BE-03 | Formatting | `cargo fmt --check` | `backend/` | Check Rust formatting | Backend CI | Rust source | Nonzero on drift | Yes | No source rewrite | Rustfmt | `VERIFIED` |
| CMD-BE-04 | Linting | `cargo clippy --all-targets --all-features -- -D warnings` | `backend/` | Treat Clippy warnings as failures | Backend CI | All targets/features | Nonzero on warning | Yes | Build artifacts | Clippy | `VERIFIED`; current green status must be checked |
| CMD-BE-05 | Testing | `cargo test --all-features` | `backend/` | Backend tests | Backend CI | PostgreSQL/Redis service variables in CI | Test output/build artifacts | Yes in test environment | Tests may use environment-dependent paths | Rust; CI services | `ENVIRONMENT_DEPENDENT` |
| CMD-FE-01 | Dependencies | `npm install` | `frontend/` | Install declared dependencies | Frontend CI/Dockerfiles/README | package files, registry | Writes `node_modules`; may update lock | Conditional | **May modify lock file** | Node/npm, registry | `ENVIRONMENT_DEPENDENT` |
| CMD-FE-02 | Development server | `npm run dev` | `frontend/` | Vite dev server | `frontend/package.json` | Source, environment | Listens on `0.0.0.0:5173` | Yes | None | Node/npm | `VERIFIED` definition |
| CMD-FE-03 | Linting | `npm run lint` | `frontend/` | ESLint TypeScript/React | Frontend manifest/CI | Source | Nonzero on lint error | Yes | None | Installed dependencies | `VERIFIED` |
| CMD-FE-04 | Type checking | `npm run typecheck` | `frontend/` | TypeScript project build without JS emit | Frontend manifest/CI | Source/config | `.tsbuildinfo` under ignored dependency path | Yes | Generated cache | Installed dependencies | `VERIFIED` |
| CMD-FE-05 | Testing | `npm test` | `frontend/` | Vitest run mode | Frontend manifest/CI | jsdom tests | Test output | Yes | None | Installed dependencies | `VERIFIED` |
| CMD-FE-06 | Production build | `npm run build` | `frontend/` | Type check and Vite build | Frontend manifest/CI | Source/config | Writes `dist` | Yes | Replaces generated output | Installed dependencies | `VERIFIED` |
| CMD-FE-07 | Preview | `npm run preview` | `frontend/` | Serve built bundle for preview | Frontend manifest | Existing `dist` | Preview server | Yes | None | Prior build | `DOCUMENTED_NOT_VERIFIED` |
| CMD-DB-01 | Database | Backend startup (`cargo run`) | Root via manifest or `backend/` | Apply embedded migrations in order | `backend/src/main.rs`, `backend/src/db/mod.rs` | `DATABASE_URL` | **Mutates schema** | Development only after review | **Migration command** | PostgreSQL | `ENVIRONMENT_DEPENDENT` |
| CMD-DOC-01 | Documentation | `git diff --check` | Root | Detect whitespace errors | Project protocol/history | Working tree | Nonzero on whitespace errors | Yes | None | Git | `VERIFIED` process command |
| CMD-TOOL-01 | Marketplace CLI | `npm run marketplace -- --help` | Root | Show CLI usage | Root manifest/script | None | Console help | Yes | None | Node | `VERIFIED` definition |
| CMD-TOOL-02 | Marketplace CLI | `npm run marketplace -- validate <target>` | Root | Validate package directory/ZIP | CLI script | Package target | Reads package; reports findings | Yes | None | Node | `ENVIRONMENT_DEPENDENT` |
| CMD-TOOL-03 | Marketplace CLI | `npm run marketplace -- pack <directory> [--force]` | Root | Create Marketplace ZIP | CLI script | Package directory | Writes `marketplace-dist`; `--force` overwrites | Conditional | **Overwrite when `--force`** | Node | `ENVIRONMENT_DEPENDENT` |
| CMD-TOOL-04 | Marketplace CLI | `npm run marketplace -- submit <target> --listing-id <uuid>` | Root | Upload a package | CLI script | Token, organization, listing, target | **Network mutation/upload** | No without explicit authorization | **Publishes/submits data** | Authenticated API | `ENVIRONMENT_DEPENDENT` |
| CMD-OPS-01 | Testing, readiness | `powershell -ExecutionPolicy Bypass -File scripts/v2-ga-check.ps1` | Root | Backend tests, frontend lint/build, optional health | Script | Optional API URL | Builds/tests; optional read-only HTTP | Conditional | No deployment | PowerShell/toolchains | `ENVIRONMENT_DEPENDENT` |
| CMD-OPS-02 | Testing, readiness | `powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase15-ga-check.ps1 -ReportOnly` | Root | Marketplace readiness report | Script | Optional API and auth inputs | Tests/builds and optional GET requests | Conditional | No deployment; may access tenant data | PowerShell/toolchains | `ENVIRONMENT_DEPENDENT` |

## Commands Not Defined

No repository command was found for release publication, container push, production deployment, database downgrade, database reset, backup, restore, Mermaid parsing, API code generation, or generated-code drift checking. Do not invent or run these operations from this catalog. See [Database Development](database-development.md), [Release Process](../delivery/release-process.md), and [Backup and Restore](../operations/backup-and-restore.md).

