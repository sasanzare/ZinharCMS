---
okf_document_id: "development-prerequisites"
title: "Development Prerequisites"
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
  - "backend/Cargo.toml"
  - "frontend/package.json"
  - "frontend/package-lock.json"
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - "backend/Dockerfile"
  - "frontend/Dockerfile"
  - ".env.example"
related_documents:
  - "README.md"
  - "local-environment.md"
  - "commands.md"
  - "../backend/overview.md"
  - "../frontend/configuration-and-build.md"
related_diagrams: []
---

# Development Prerequisites

No owner-approved minimum local toolchain or browser support matrix exists. Versions below are exact only where a workflow, image, manifest, or lock file declares them.

| Tool | Purpose | Requirement | Version | Version source | Installation source in repository | Area | Verification command defined | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Git | Checkout and change history | Required for contribution; runtime does not require it | `UNKNOWN` | No version file | Not documented | Repository | `git status --short` is required by OKF workflow | `ENVIRONMENT_REQUIREMENT_UNCLEAR` |
| Rust toolchain and Cargo | Backend build, format, lint, test, run | Required for non-container backend work | CI uses `stable`; development image uses Rust `1.87`; crate edition is `2024` | Workflow, Dockerfile, Cargo manifest | `dtolnay/rust-toolchain@stable` in CI | Backend | `rustc --version`; `cargo --version` are conventional checks, not scripts | `VERIFIED` declarations; support range unknown |
| Node.js | Frontend and Marketplace CLI | Required for frontend and CLI | CI uses `22`; development and production builders use `24-alpine` | Workflow and Dockerfiles | `actions/setup-node@v4` in CI | Frontend, tooling | `node --version` is not wrapped by a repository script | `VERIFIED` declarations; local support range unknown |
| npm | Frontend dependency and script runner | Required for frontend | No npm engine constraint; lockfile version `3` | `frontend/package-lock.json` | Bundled with selected Node image/runtime | Frontend | `npm --version` is not wrapped | `ENVIRONMENT_REQUIREMENT_UNCLEAR` |
| PostgreSQL | System of record and migrations | Required for backend startup | `16-alpine` in Compose and CI | Compose and backend workflow | Container image reference | Backend, database | `pg_isready` in container health checks | `VERIFIED` for configured environments |
| Redis | Cache and rate-limit state | Required for `/ready`; some cache paths fall back | `7-alpine` in Compose and CI | Compose and backend workflow | Container image reference | Backend | `redis-cli ping` in health checks | `VERIFIED` for configured environments |
| Docker Engine | Container builds and local infrastructure | Optional if PostgreSQL and Redis are provided separately | No version constraint | No version file | Not documented | Infrastructure | No repository wrapper | `ENVIRONMENT_REQUIREMENT_UNCLEAR` |
| Docker Compose | Local and production-like assembly | Optional for application code; required for documented container setup | Compose schema says `3.9`; CLI version is not pinned | Compose files | Not documented | Infrastructure | `docker compose` commands are in `package.json` and README | `VERIFIED` use; binary version unknown |
| PowerShell | GA/readiness/load scripts | Optional unless those scripts are used | No version constraint | `.ps1` files | Not documented | Operational scripts | Script parsing/execution is environment dependent | `VERIFIED` use; version unknown |
| Nginx | Production frontend static serving and SPA fallback | Required only for the production frontend image | `1.27-alpine` | `frontend/Dockerfile.prod` | Container image reference | Frontend delivery | No standalone verification command | `VERIFIED` image |
| SQLx embedded migrator | Apply ordered backend migrations | Built into backend binary; SQLx CLI is not required | SQLx `0.8` dependency | `backend/Cargo.toml` | Cargo dependency | Database | Backend startup calls `sqlx::migrate!` | `VERIFIED` |
| Mermaid parser/renderer | Diagram parser/render validation | Optional and currently absent | Not declared | Repository dependency scan | Not documented | Documentation | `mmdc` was not found during Phase 10 | `COMMAND_STATUS_UNCLEAR` |
| Browser | Use and debug the React admin UI | Required for interactive UI use | No supported-browser matrix | No browserslist or E2E matrix found | Not documented | Frontend | No browser verification command | `ENVIRONMENT_REQUIREMENT_UNCLEAR` |

## Environment Files

The tracked templates are `.env.example` and `env.example`. The real root `.env` is ignored. Never copy real values into documentation or commit them. The backend uses `dotenvy::dotenv()` and `Config::from_env()`; the frontend consumes `VITE_API_URL` at build/runtime-development configuration boundaries.

## Native and Build Dependencies

The Dockerfiles provide complete Linux build images. No separate owner-maintained list of local compiler libraries or platform SDKs was found. Treat local native build requirements as `ENVIRONMENT_REQUIREMENT_UNCLEAR` and prefer the declared container images when reproducibility is more important than local toolchain speed.

See [Local Environment](local-environment.md), [Backend Overview](../backend/overview.md), and [Frontend Configuration and Build](../frontend/configuration-and-build.md).

