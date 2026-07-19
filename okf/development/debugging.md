---
okf_document_id: "development-debugging"
title: "Debugging"
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
  - "backend/src/main.rs"
  - "backend/src/error.rs"
  - "backend/src/routes/mod.rs"
  - "frontend/vite.config.ts"
  - "docker-compose.yml"
related_documents:
  - "local-environment.md"
  - "testing-workflow.md"
  - "../operations/logging-and-tracing.md"
  - "../operations/health-and-readiness.md"
  - "../operations/troubleshooting.md"
related_diagrams: []
---

# Debugging

| Target issue | Command or configuration | Working directory | Expected signal | Security caution | Evidence | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| Backend startup | Run repository backend command with `RUST_LOG` configured | Root or `backend/` | Configuration/migration/seed/bind context and listening log | Do not print URL credentials or JWT/provider secrets | `backend/src/main.rs`, `.env.example` | `VERIFIED` |
| Rust stack detail | `RUST_BACKTRACE` is a standard Rust variable but not defined by repository configuration | Process environment | Backtrace on eligible panic/error | Backtraces may expose paths/data | No tracked project definition | `COMMAND_STATUS_UNCLEAR` |
| HTTP request flow | `TraceLayer::new_for_http()` and generated `X-Request-Id` | Backend | Request trace events and propagated request ID | Redaction behavior is not verified | `backend/src/main.rs` | `VERIFIED` instrumentation; format/detail depends on filter |
| Health versus readiness | Request `/health` then `/ready` | Any | Process liveness separately from PostgreSQL/Redis status | Readiness error strings can reveal technical detail | `backend/src/routes/mod.rs` | `VERIFIED` |
| Backend tests | `cargo test <filter>` | `backend/` | Focused assertion and diagnostics | Test output can include fixture/provider text | Cargo test modules | `VERIFIED` |
| Frontend source/debug | `npm run dev` | `frontend/` | Vite development error overlay and browser devtools | Browser storage contains session state; do not copy tokens | Vite config and frontend code | `VERIFIED` dev server; browser procedure manual |
| Frontend tests | `npm test -- <pattern>` | `frontend/` | Vitest/jsdom failure output | Mocks may include synthetic auth data | Frontend manifest/Vitest config | `VERIFIED` |
| Production source maps | No explicit `build.sourcemap` setting | N/A | Vite default behavior only | Do not assume source maps are published or absent in every target | `frontend/vite.config.ts` | `OBSERVABILITY_STATUS_UNCLEAR` |
| Database connectivity | `/ready` and backend startup context | Any | PostgreSQL reachable/unreachable signal | Do not log `DATABASE_URL` | Routes/main | `VERIFIED` |
| Database query logging | No SQLx statement logging configuration found | N/A | No project-defined query log signal | Query values may be sensitive | Source/config scan | `NOT_IMPLEMENTED` as a defined facility |
| Container state/logs | Docker Compose service inspection is available through Docker, but no wrapper command exists | Root | Container health and stdout/stderr | Logs may contain operational data | Compose files | `DOCUMENTED_NOT_VERIFIED` |
| API errors | Inspect status plus `{error,message}` where application errors apply | Client | Stable high-level error code plus message | Internal/provider/database text may be exposed | `backend/src/error.rs` | `VERIFIED` with known inconsistency |
| Correlation | `X-Request-Id` is set/propagated | Client/backend logs | Same header can connect request handling observations | No retention/redaction policy | `backend/src/main.rs` | `VERIFIED`; end-to-end log correlation untested |

No debugger launch configuration, IDE profile, centralized tracing collector, frontend error-reporting provider, or production log query procedure is stored. See [Logging and Tracing](../operations/logging-and-tracing.md) and [Troubleshooting](../operations/troubleshooting.md).

