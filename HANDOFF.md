# Project Handoff

> Persistent recovery and continuation document for Codex and human developers.
> The repository and Git state are the source of truth when this document becomes stale.

## 1. Handoff Metadata

- **Last updated:** 2026-07-11 07:40 +01:00 (Europe/London)
- **Updated by:** Codex
- **Repository:** ZinharCMS
- **Current branch:** `main`
- **Base branch:** `main` / `origin/main`
- **Latest relevant commit:** `b52f81c feat(marketplace): implement v3 phase 8 runtime adapters`
- **Working tree at session start:** Clean at Phase 8 commit `b52f81c`; current tree contains only the uncommitted Phase 9 implementation and documentation.
- **Current version:** `0.1.0` in root, frontend, and backend manifests
- **Current phase:** V3 Marketplace Phase 9 — Monetization
- **Current subphase:** 9.1–9.4 implementation, local validation, and authenticated smoke complete
- **Overall status:** Phase 9 complete in the local environment; changes remain uncommitted pending explicit user authorization

## 2. Project Overview

> **Phase 9 override (2026-07-10 21:20):** Phase 8 is committed at `b52f81c`.
> The active objective is Phase 9 monetization. Migration `0022`, free/paid
> checkout, entitlements, paid lifecycle gates, revenue/refund ledger, payout
> onboarding/verification, frontend purchase/onboarding surfaces, Phase 9 docs,
> and diagram `36` are present in the uncommitted working tree. Older Phase 7/8
> status and exact-next-action text below is historical.

> **Phase 8 override (2026-07-10 18:55):** Phase 7 is complete and committed at
> `1231613`; the active objective is V3 Marketplace Phase 8 (8.1 Component Pack
> Runtime, 8.2 Template Import, and 8.3 Plugin Hook MVP). The clean Git state at
> session start is the source of truth over older Phase 7 wording below.

ZinharCMS is a headless CMS and multi-tenant SaaS administration product. It
serves organization owners and content teams through a React admin application
and a Rust/Axum API. The repository is a modular monolith with PostgreSQL as the
system of record, Redis for cache/rate-limit support, and local filesystem
storage for CMS media and Marketplace package artifacts.

The baseline includes the original CMS phases zero through ten, V2 organization,
billing, beta, and GA operations, and V3 Marketplace phases 0.1 through 7. The
current V3 implementation has reached the Phase 7 security boundary: reviewed
free Component Packs and Design Templates can be installed and managed, while a
permission catalog, allowlisted runtime policy, bounded sandbox decision, and
global/organization kill switch prevent unauthorized or unsafe runtime actions.
Uploaded package code is still never executed.

## 3. Technology Stack

- **Backend:** Rust 2024, Axum 0.8, Tokio, modular route/service architecture.
- **Frontend:** React 19, TypeScript, Vite 6, React Router, Zustand, React Hook Form, Zod.
- **Database:** PostgreSQL 16 accessed through SQLx migrations and queries.
- **Authentication:** Argon2id password hashing, HMAC-SHA256 JWT access tokens, hashed refresh tokens in HttpOnly cookies.
- **Authorization:** Global roles plus organization membership roles, tenant middleware, PostgreSQL forced RLS.
- **API:** JSON HTTP APIs, authenticated WebSocket preview, `utoipa` OpenAPI generation.
- **Cache and limits:** Redis 7 for Delivery API cache and rate-limit counters; quota checks use organization plans.
- **Storage:** Local filesystem under `UPLOAD_DIR`; no S3/CDN implementation is present.
- **Testing:** Rust unit/static contract tests, Vitest, Testing Library, ESLint, TypeScript build/typecheck.
- **Build and deployment:** Cargo, npm, Docker Compose, Nginx production frontend image, GitHub Actions CI.
- **Documentation:** Markdown phase/API/architecture documents and 34 Mermaid diagrams.
- **Not implemented:** Durable queue/worker, search service, separately deployed gateway, automatic backups, monitoring vendor, executable Marketplace sandbox/runtime.

## 4. Repository Structure

| Path | Purpose |
| --- | --- |
| `backend/src/` | Rust/Axum routes, middleware, services, plugins, configuration, and application startup. |
| `backend/migrations/` | SQLx migrations; `0019_v3_phase_six_installation_lifecycle.sql` is the current Phase 6 migration. |
| `frontend/src/` | React routes/pages, API client, state, types, translations, styles, and frontend tests. |
| `docs/` | API, architecture, phase, V2/V3 Marketplace, operations, and localization documentation. |
| `docs/diagrams/` | Evidence-based Mermaid architecture set, audit, traceability, and ambiguity records. |
| `scripts/` | Release and smoke-check PowerShell scripts. |
| `.github/workflows/` | Backend and frontend CI definitions. |
| `docker-compose.yml` | Local PostgreSQL, Redis, and pgAdmin infrastructure. |
| `docker-compose.prod.yml` | Production-like PostgreSQL, Redis, backend, frontend, and uploads volumes. |
| `.env.example`, `env.example` | Non-secret environment variable templates. |

Generated/dependency directories such as `backend/target`, `frontend/node_modules`,
and `frontend/dist` are not source-of-truth directories.

## 5. Authoritative Documents

| Document | Role | Authority / freshness |
| --- | --- | --- |
| `README.md` | Current repository scope and quick-start commands through V3 Phase 6. | Current summary; source code and migrations outrank it. |
| `docs/V3_PHASE_SIX.md` | Phase 6 acceptance, install gates, lifecycle rules, update/rollback behavior, and deferred boundaries. | Current Phase 6 authority. |
| `docs/V3_PHASE_SEVEN.md` | Phase 7 permission catalog, sandbox policy, runtime authorization, kill switch, and acceptance. | Current Phase 7 authority. |
| `docs/V3_MARKETPLACE_SCOPE.md` | V3 scope lock and MVP/out-of-scope rules. | Current product-scope authority. |
| `docs/V3_MARKETPLACE_GAP_LIST.md` | Resolved and deferred Marketplace gaps by phase. | Current gap/status record; verify against runtime. |
| `docs/V3_MARKETPLACE_POLICY.md` and `docs/V3_PRODUCT_TAXONOMY.md` | Review, moderation, product classification, and safety policy. | Current policy authority. |
| `docs/API.md` | Runtime route boundaries and Marketplace endpoint documentation. | Current, with older Marketplace routes manually documented. |
| `docs/ARCHITECTURE.md` | Runtime containers, tenant boundaries, RLS, and Marketplace architecture. | Updated through migration `0021` and Phase 8 host-owned adapters. |
| `docs/diagrams/ARCHITECTURE_AUDIT.md`, `TRACEABILITY.md`, `FILE_EVIDENCE_INDEX.md`, `33-marketplace-installation-lifecycle.mmd`, `34-marketplace-security-runtime.mmd`, `35-marketplace-runtime-adapters.mmd` | Evidence links and visual Phase 6/7/8 implementation state. | Updated with Phase 8 evidence; static Mermaid validation is available, but no Mermaid parser is installed. |
| `D:\All projects\Zinhar_Doc\version_3_marketplace_proposal.html` | Original V3 Marketplace proposal and future lifecycle goals. | Planning authority; current migrations/routes/tests supersede it for implementation status. |
| `D:\All projects\Zinhar_Doc\version_2_proposal.html` | V2 SaaS/organization/billing proposal. | Historical planning authority for V2 dependencies. |
| `D:\All projects\Zinhar_Doc\headless_cms_proposal_polished.html` | Original CMS proposal. | Historical baseline; current repository documentation and code are newer. |

The proposals describe the complete future Marketplace lifecycle, including paid
products and executable/runtime concepts. Phase 7 established the permission and
containment boundary; Phase 8 now supplies host-owned Component Pack, Template,
and public Hook adapters. Paid entitlements, payouts, customer ratings, external
execution, and arbitrary package execution remain deferred.

## 6. Current Objective

The active objective is to implement and validate V3 Marketplace Phase 8 without
repeating the committed Phase 7 boundary. The implementation target is the
Component Pack registry, Template preview/import with tenant asset mapping, and
public Plugin Hook MVP contracts.

Phase 6 boundaries that must remain unchanged until their dedicated phases are
planned and authorized:

- only free `component_pack` and `design_template` products are installable;
- uploaded package code is never executed;
- paid purchase/entitlement and creator payout flows are not implemented;
- external runtime execution and fine-grained permission revocation remain deferred; Phase 7 policy decisions and Phase 8 host-owned adapters are implemented;
- no background automatic update is enabled; installations remain explicitly pinned.

## 7. Completed and Verified Work

### Phase 8 checkpoint override (2026-07-10 19:10)

- Phase 8.1 Component Pack registry is implemented: active installed manifest components are namespaced and materialized into the organization Page Builder palette.
- Phase 8.2 Template Import is implemented: preview and import validate runtime state, media ownership, page JSON, page version, import record, and audit event.
- Phase 8.3 Plugin Hook MVP is implemented: only `sidebar.item`, `dashboard.widget`, `form.field`, and `webhook.adapter` are exposed/authorized; execution remains `not_executed`.
- Migration `0021`, adapter routes/service, frontend API/types/UI, docs, and diagram 35 are present in the current working tree.
- Phase 7 remains the committed baseline at `1231613`; no prior completed work was repeated or reset.

- [x] Implemented the additive Phase 6 migration and installation lifecycle schema.
  - **Files:** `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql`, `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`
  - **Verification:** `cargo test --manifest-path backend/Cargo.toml --all-features`
  - **Result:** 87 backend tests passed; migration and route/service static contract tests passed.

- [x] Implemented tenant-aware list/install/enable/disable/soft-uninstall/update-check/update/rollback APIs.
  - **Files:** `backend/src/routes/marketplace.rs`, `backend/src/services/marketplace_installation.rs`, `backend/src/services/rbac.rs`, `backend/src/routes/mod.rs`
  - **Verification:** backend tests, `cargo fmt --manifest-path backend/Cargo.toml -- --check`, and `cargo test --manifest-path backend/Cargo.toml --all-features`
  - **Result:** format check passed; 87 tests passed with 0 failures.

- [x] Enforced Phase 6 install gates for review state, risk, compatibility, free MVP product type, exact owner/admin permission approval, artifact existence/size/SHA-256, forced RLS, and atomic lifecycle audit records.
  - **Files:** `backend/src/routes/marketplace.rs`, `backend/src/services/marketplace_installation.rs`, `backend/src/services/rls.rs`, `backend/src/services/audit.rs`
  - **Verification:** artifact, permission, semantic-version, lifecycle, rollback, migration, route, and documentation contract tests.
  - **Result:** relevant backend tests passed; paid/custom products remain deterministically blocked.

- [x] Implemented Marketplace installation management UI and API client methods, including permission approval, changelog confirmation, update permission reapproval, rollback, and soft-uninstall controls.
  - **Files:** `frontend/src/pages/MarketplacePage.tsx`, `frontend/src/services/api.ts`, `frontend/src/types/api.ts`, `frontend/src/i18n/messages.ts`, `frontend/src/styles/index.css`
  - **Verification:** `npm --prefix frontend run lint`, `npm --prefix frontend run typecheck`, `npm --prefix frontend test`, `npm --prefix frontend run build`
  - **Result:** lint and typecheck passed; 3 test files and 8 tests passed; production build passed with an existing large-chunk warning.

- [x] Added Phase 6 frontend coverage and preserved existing Pages coverage.
  - **Files:** `frontend/src/pages/MarketplacePage.test.tsx`, `frontend/src/pages/PagesPage.test.tsx`
  - **Verification:** `npm --prefix frontend test`
  - **Result:** Dashboard, Pages, and Marketplace suites passed (8 tests total).

- [x] Updated Phase 6 API, architecture, gap, inventory, audit, ambiguity, traceability, and Mermaid documentation.
  - **Files:** `docs/V3_PHASE_SIX.md`, `docs/API.md`, `docs/ARCHITECTURE.md`, `docs/V3_MARKETPLACE_GAP_LIST.md`, `docs/diagrams/*`
  - **Verification:** repository-local Mermaid structural validation and `git diff --check`.
  - **Result:** 34 `.mmd` files each contain one standalone Mermaid declaration and no Markdown fences; diff check passed.

- [x] Implemented Phase 7.1 permission catalog and runtime permission model.
  - **Files:** `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql`, `backend/src/services/marketplace_runtime.rs`, `backend/src/services/rbac.rs`
  - **Verification:** backend unit/static contract tests.
  - **Result:** Permission catalog, risk/product/runtime metadata, operation mappings, and bounded reason validation are covered by the backend suite.

- [x] Implemented Phase 7.2 allowlisted sandbox host API policy without executing uploaded code.
  - **Files:** `backend/src/routes/marketplace_runtime.rs`, `backend/src/services/marketplace_runtime.rs`, `frontend/src/services/api.ts`, `frontend/src/types/api.ts`
  - **Verification:** runtime policy tests and backend route/OpenAPI compilation.
  - **Result:** 93 backend tests passed; inactive/blocked installations, unknown operations, permission escalation, unsafe entry points, and oversized payloads are denied; successful decisions report `execution = not_executed`.

- [x] Implemented Phase 7.3 global and organization kill switches.
  - **Files:** `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql`, `backend/src/routes/marketplace_runtime.rs`, `backend/src/routes/marketplace.rs`, `frontend/src/pages/MarketplacePage.tsx`
  - **Verification:** backend contract tests, frontend Phase 7 UI test, lint/typecheck/build.
  - **Result:** Owner/admin organization controls and global admin controls block runtime state, installation, and re-enable; status/lift/audit paths are present; frontend test suite passes 9 tests.

- [x] Updated Phase 7 API, architecture, gap, manifest, traceability, repository inventory, ambiguity, and Mermaid documentation.
  - **Files:** `docs/V3_PHASE_SEVEN.md`, `README.md`, `docs/API.md`, `docs/ARCHITECTURE.md`, `docs/V3_MARKETPLACE_GAP_LIST.md`, `docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md`, `docs/diagrams/*`
  - **Verification:** repository-local Mermaid structural/evidence validation and `git diff --check`.
  - **Result:** 35 `.mmd` files each contain one standalone Mermaid declaration with existing evidence paths and no Markdown fences.

## 8. Completed but Not Verified

- [ ] Live application of migration `0019` and end-to-end API/browser installation smoke test.
  - **Files:** `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql`, `backend/src/routes/marketplace.rs`, `frontend/src/pages/MarketplacePage.tsx`
  - **Missing verification:** a running backend connected to the intended database, followed by authenticated tenant requests and artifact-backed install/update/rollback.
  - **Recommended validation:** start the backend with the project environment, verify `/health`, `/ready`, `/openapi.json`, and exercise the Phase 6 endpoints against a test organization without resetting data.
  - **Reason:** Docker infrastructure was visible with `docker compose ps`, but `docker compose exec` was denied Docker API access in this sandbox, and the local compose file does not include the backend service. Migration `0020` has therefore only static/compile coverage so far.

- [ ] Mermaid parser/render validation.
  - **Files:** `docs/diagrams/*.mmd`
  - **Missing verification:** no `mmdc`/Mermaid parser dependency is installed in the repository.
  - **Recommended validation:** use an approved Mermaid renderer in a separate documentation environment.
  - **Reason:** only static declaration/fence validation was available.

## 9. Work in Progress

### Phase 8 active checkpoint override

Phase 8 product code is implemented and statically validated. The remaining
verification is live migration/API/browser smoke for migration `0021`; this is
not a request to reimplement Phase 7.

### Active item

Phase 7 product code is implemented; the remaining work is final validation and
handoff maintenance, not another implementation pass.

### Exact stopping point

The Phase 7 implementation stopped after the 7.3 kill-switch routes/UI and
documentation were added on top of commit `b1b3d05`. Backend tests and frontend
tests/build have passed; live migration/API smoke is still pending.

### Partially modified files

| File | Current state | Remaining work |
| --- | --- | --- |
| `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql` | Additive permission catalog, runtime status, and kill-switch schema is present. | Apply and inspect it in a running test database. |
| `backend/src/routes/marketplace_runtime.rs` | Runtime status, authorization decision, kill-switch activation/lift handlers are present. | Run authenticated tenant/global-admin API smoke. |
| `frontend/src/pages/MarketplacePage.tsx` | Phase 7 runtime safety panel and kill-switch controls are present. | Verify against a live API and browser session. |

### Incomplete implementation markers

- Marketplace purchase, entitlement, payout, customer-rating, concrete runtime adapters, arbitrary package execution, and fine-grained permission revocation remain intentionally planned/deferred, not stubs to finish in Phase 7.
- The frontend production build reports a chunk-size warning over 500 kB; this is non-blocking technical debt.

## 10. Current Git and Filesystem State

### Actual state at Phase 8 checkpoint

- `HEAD` is `1231613` (`feat(marketplace): implement v3 phase 7 runtime security controls`) and the working tree contains only Phase 8 implementation/documentation changes plus this handoff update.
- No files are staged, deleted, reset, or committed for Phase 8.
- New Phase 8 files are `backend/migrations/0021_v3_phase_eight_runtime_adapters.sql`, `backend/src/routes/marketplace_adapters.rs`, `backend/src/services/marketplace_adapters.rs`, `docs/V3_PHASE_EIGHT.md`, and `docs/diagrams/35-marketplace-runtime-adapters.mmd`.

### Staged files

- None.

### Modified files

- `README.md`, `backend/src/routes/marketplace.rs`, `backend/src/routes/mod.rs`, `backend/src/services/mod.rs`, `backend/src/services/rbac.rs` — Phase 7 route integration, kill-switch gates, service registration, and RBAC.
- `docs/API.md`, `docs/ARCHITECTURE.md`, `docs/V3_MARKETPLACE_GAP_LIST.md`, `docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md` — Phase 7 API, scope, architecture, and manifest updates.
- `docs/diagrams/01-project-scope.mmd`, `03-identity-and-authorization-boundaries.mmd`, `AMBIGUITIES.md`, `ARCHITECTURE_AUDIT.md`, `FILE_EVIDENCE_INDEX.md`, `README.md`, `REPOSITORY_INVENTORY.md`, `TRACEABILITY.md` — Phase 7 evidence and diagram updates.
- `frontend/src/i18n/messages.ts`, `frontend/src/pages/MarketplacePage.tsx`, `frontend/src/pages/MarketplacePage.test.tsx`, `frontend/src/services/api.ts`, `frontend/src/types/api.ts` — Phase 7 status, permission catalog, kill-switch UI/API, and tests.

### Historical handoff files and current untracked files

- `AGENTS.md` — new root-level persistent handoff protocol required by this task.
- `HANDOFF.md` — this repository-specific recovery document.

The original handoff files remain tracked by commit `b1b3d05`; the Phase 7
implementation files listed below are the current uncommitted work. No commit
should be created unless the user explicitly authorizes it.

- `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql` — Phase 7 schema and seeded permission catalog.
- `backend/src/routes/marketplace_runtime.rs` — Phase 7 runtime and kill-switch routes.
- `backend/src/services/marketplace_runtime.rs` — Phase 7 pure runtime policy service and tests.
- `docs/V3_PHASE_SEVEN.md` — Phase 7 authority and acceptance.
- `docs/diagrams/34-marketplace-security-runtime.mmd` — Phase 7 Mermaid evidence diagram.

### Deleted files

- None.

### Important diff observations

- At session start, `HEAD` was `b1b3d05` and matched `origin/main`; Phase 7 changes are currently unstaged.
- Commit `7f18d7b` contains the Phase 6 product, test, API, and diagram changes; commit `b1b3d05` contains the handoff protocol. Phase 7 changes are not committed.
- No Phase 6 files were discarded or reset; all current modifications are Phase 7 implementation/documentation plus this handoff update.
- No secrets or values from `.env` were copied into this document.

## 11. Tests and Validation

### Phase 8 validation results (2026-07-10 19:10)

- `cargo fmt --manifest-path backend/Cargo.toml -- --check`: passed.
- `cargo test --manifest-path backend/Cargo.toml --all-features`: passed, 96 tests and doc tests with 0 failures; Phase 8 route/service contract tests included.
- `npm --prefix frontend run lint`: passed.
- `npm --prefix frontend run typecheck`: passed.
- `npm --prefix frontend test`: passed, 3 files and 9 tests.
- `npm --prefix frontend run build`: passed with the existing >500 kB chunk warning; sandbox `spawn EPERM` was resolved by the approved escalated rerun.
- Mermaid static/evidence validation: passed, 36 files (`00` through `35`), one declaration each, no fences, all evidence paths present.
- `git diff --check`: passed.
- `cargo clippy -D warnings` remains blocked by legacy warnings outside Phase 8; no new Phase 8 warning was introduced by the test build.

### Commands executed during this inspection

| Command | Executed? | Result | Notes |
| --- | ---: | --- | --- |
| `cargo fmt --manifest-path backend/Cargo.toml -- --check` | Yes | Passed | Rust formatting is clean. |
| `cargo test --manifest-path backend/Cargo.toml --all-features` | Yes | Passed | 93 passed, 0 failed; Phase 7 runtime and route contract tests included; doc tests also completed. |
| `npm --prefix frontend run lint` | Yes | Passed | ESLint completed successfully. |
| `npm --prefix frontend run typecheck` | Yes | Passed | `tsc -b` completed successfully. |
| `npm --prefix frontend test` | Yes | Passed | 3 files, 9 tests passed, including the Phase 7 runtime safety control test. |
| `npm --prefix frontend run build` | Yes | Passed with warning | Vite build completed; one output chunk is over 500 kB. Sandbox first returned esbuild `spawn EPERM`; escalated rerun passed. |
| Repository-local Mermaid structural check | Yes | Passed | 35 files, one declaration each, no fences, all evidence paths exist. |
| `git diff --check` | Yes | Passed | No whitespace errors. |
| `docker compose ps` | Yes | Passed | PostgreSQL and Redis reported healthy; compose warned that `version` is obsolete. |
| `docker compose exec -T postgres psql ...` | Yes | Not run successfully | Docker API permission was denied by the sandbox before the query executed. |

### Other known validation

- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings` was rerun after Phase 7 and still fails on 29 pre-existing warnings in older backend modules. No Phase 7-specific warning remains; do not treat CI clippy as green until the legacy warnings are addressed or the policy changes.
- No destructive migration, database reset, dependency installation, deployment, or commit was performed during this Phase 7 checkpoint.

### Discovered but not run now

- `scripts/v2-ga-check.ps1` — runs backend tests plus frontend lint/build and optionally health/readiness; use only when the intended environment is available.
- GitHub Actions run commands in `.github/workflows/backend-ci.yml` and `.github/workflows/frontend-ci.yml` — CI definitions were inspected, not triggered locally.
- Full authenticated browser/API smoke — not available without a running backend and seeded test data.

## 12. Architecture and Implementation Decisions

### Decision: Keep Marketplace Phase 6 as a registry lifecycle, not executable runtime

- **Decision:** Install records manage approved package metadata and lifecycle state; uploaded package code is never executed.
- **Evidence:** `docs/V3_PHASE_SIX.md`, `docs/V3_MARKETPLACE_SCOPE.md`, `backend/src/services/marketplace_installation.rs`.
- **Reason:** The V3 MVP explicitly limits installation to reviewed free Component Packs and Design Templates until sandbox/runtime phases.
- **Affected areas:** Marketplace routes/services, frontend Marketplace page, package storage and validation.
- **Do not change unless:** A separately authorized V3 runtime/sandbox phase defines permission enforcement and kill-switch behavior.

### Decision: Require exact owner/admin permission approval at install and permission-changing update

- **Decision:** The submitted approval array must exactly match the manifest snapshot; updates that change permissions require fresh approval.
- **Evidence:** `backend/src/routes/marketplace.rs`, `backend/src/services/rbac.rs`, `frontend/src/pages/MarketplacePage.tsx`, Phase 6 tests.
- **Reason:** Permission consent must be explicit and auditable before a product enters the organization lifecycle.
- **Affected areas:** Install/update DTOs, installation metadata, audit events, frontend confirmation modal.
- **Do not change unless:** The Marketplace permission catalog and runtime enforcement contract are intentionally revised.

### Decision: Use tenant transactions and forced RLS for lifecycle mutations

- **Decision:** Installation writes and lifecycle audit records commit in the same tenant-scoped transaction; tenant-owned queries use forced RLS.
- **Evidence:** `backend/src/services/rls.rs`, `backend/src/services/audit.rs`, `backend/src/routes/marketplace.rs`, migrations `0015` and `0019`.
- **Reason:** Installation state and audit history must not diverge across organizations.
- **Affected areas:** All installation lifecycle handlers and moderation emergency-block path.
- **Do not change unless:** A reviewed cross-tenant platform operation explicitly requires a narrow bypass transaction.

### Decision: Soft-uninstall preserves organization data

- **Decision:** Uninstall changes installation state to `uninstalled` and retains organization content/data under `cleanup_policy = preserve_organization_data`.
- **Evidence:** `docs/V3_PHASE_SIX.md`, migration `0019`, lifecycle handlers/tests.
- **Reason:** Phase 6 has no safe product-specific data migration or cleanup contract.
- **Affected areas:** Uninstall handler, installed-app listing, timestamps, audit records.
- **Do not change unless:** A later product-data ownership and cleanup policy is approved.

### Decision: Explicit pinned semver updates and same-listing safe rollback

- **Decision:** Updates require a strictly newer semantic version and changelog confirmation; the previous same-listing version is retained for controlled rollback, including a deprecated safe target.
- **Evidence:** `marketplace_installation.rs`, `marketplace.rs`, migration `0019`, Phase 6 tests.
- **Reason:** Avoid background changes and preserve a reversible version path.
- **Affected areas:** `version_id`, `rollback_version_id`, version gates, frontend update/rollback controls.
- **Do not change unless:** An explicit automatic-update policy and migration strategy are approved.

### Decision: Local filesystem artifact integrity is a hard gate

- **Decision:** Install/update/enable/rollback recheck object-key safety, file existence, stored size, and SHA-256 before changing state.
- **Evidence:** `backend/src/services/marketplace_installation.rs`, `backend/src/routes/marketplace.rs`, `docs/V3_PACKAGE_STORAGE.md`, artifact tests.
- **Reason:** The repository has no S3/CDN or durable artifact service; the approved local bytes must be reverified.
- **Affected areas:** Marketplace package storage and lifecycle mutations.
- **Do not change unless:** A durable storage contract replaces local artifact storage.

### Decision: Phase 7 runtime is a policy-only allowlisted host API

- **Decision:** Runtime requests are authorized against an operation allowlist, product type, declared safe entry point, approved permission snapshot, and 64 KiB JSON payload limit; the backend returns a decision and never executes uploaded code.
- **Evidence:** `backend/src/services/marketplace_runtime.rs`, `backend/src/routes/marketplace_runtime.rs`, `docs/V3_PHASE_SEVEN.md`, migration `0020`.
- **Reason:** The proposal requires sandbox containment before runtime expansion, while Phase 8 owns concrete Component Pack/Template/Hook adapters.
- **Affected areas:** Runtime authorization endpoint, permission catalog, installation runtime status, frontend safety panel.
- **Do not change unless:** A separately reviewed sandbox adapter defines execution isolation, host APIs, and permission enforcement.

### Decision: Kill switches are independent runtime state with global/org scopes

- **Decision:** Global and organization switches set installation `runtime_status = blocked`, prevent new install/re-enable/runtime authorization, preserve reasons/timestamps, and can be lifted under matching global/org authority.
- **Evidence:** `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql`, `backend/src/routes/marketplace_runtime.rs`, `backend/src/routes/marketplace.rs`.
- **Reason:** Emergency blocking must stop runtime policy decisions without deleting installation history or conflating a platform kill switch with Phase 6 listing moderation status.
- **Affected areas:** Kill-switch table/RLS, runtime status, audit logs, Marketplace UI.
- **Do not change unless:** A later incident/runbook phase defines a more granular revocation model.

## 13. Known Issues, Risks, and Technical Debt

### Blocking issues

- None for the committed Phase 6 code path.

### Non-blocking issues

- **Medium:** Live DB migration/API smoke for migration `0020` is still unverified because Docker API exec access was denied from this sandbox; use a development environment with Docker access.
- **Medium:** `cargo clippy -D warnings` reports legacy warnings in older modules, so the backend CI clippy step is not currently a reliable green signal.
- **Low:** Mermaid files have only repository-local structural validation; no parser/render dependency is installed.
- **Low:** Vite emits a large output chunk warning (>500 kB).
- **Low:** `docs/ARCHITECTURE.md` still says schema authority ends at migration `0018`; reconcile this documentation-only sentence to `0019` in a future maintenance pass.

### Security risks

- **High if scope expands:** Phase 7 supplies policy decisions but not concrete package execution. Do not enable executable extensions before an isolated adapter/runtime, kill-switch, permission enforcement, and forced-RLS review are implemented.
- **Medium operational:** Local filesystem artifact storage and non-atomic filesystem/database behavior require backup and cleanup procedures outside this repository.
- **Low/known:** The frontend hides controls by role, but backend middleware/handler checks remain the security authority.

### Compatibility risks

- Migration `0019` is additive but must be applied before calling Phase 6 routes in a deployed database.
- All Marketplace routes are tenant-aware and require authentication plus `X-Organization-Id`; older planning language that calls the catalog public must not be used as a runtime contract.
- No anonymous catalog behavior, automatic updates, S3/CDN, durable queue, or multi-replica preview broadcast is implemented.

### Technical debt

- Legacy clippy warnings across pre-Phase-6 modules.
- No durable webhook retry worker, monitoring exporter, automatic backup, or operational incident runbook for the complete Marketplace lifecycle.
- Marketplace purchases, entitlements, payouts, customer ratings, and executable runtime are future work.

## 14. Assumptions and Unknowns

### Confirmed facts

- [x] `main` and `origin/main` point to `7f18d7b`.
- [x] The repository was clean before adding this handoff documentation.
- [x] Phase 6 routes, migration, service, frontend UI, tests, and docs are present in the latest commit.
- [x] Phase 7 migration, runtime policy service/routes, frontend safety controls, tests, and docs are present in the current working tree.
- [x] Current manifests report version `0.1.0`.
- [x] The local development compose file provides PostgreSQL, Redis, and pgAdmin; it does not run the backend/frontend services.

### Unconfirmed assumptions

- [ ] The intended developer database has actually applied migrations `0019` and `0020`.
- [ ] A deployed environment has the expected `UPLOAD_DIR` contents and artifact files needed for end-to-end installation smoke tests.
- [ ] The exact V3 Phase 8 implementation order for concrete component/template/integration adapters has not been finalized in current repository docs.
- [ ] The user has not authorized committing the newly created `AGENTS.md` and `HANDOFF.md`.

## 15. Remaining Work

### Phase 8 remaining-work override

1. Apply migration `0021` in a safe development database and run authenticated
   API/browser smoke for Component Pack registry, Template preview/import, asset
   ownership rejection, and Plugin Hook authorization.
2. Fetch generated `/openapi.json` and confirm all five Phase 8 paths and schemas
   agree with `docs/API.md`.
3. Keep paid entitlements, external network execution, arbitrary package code,
   and customer ratings deferred to their proposal phases.
4. Do not create a commit unless the user explicitly authorizes it.

1. [ ] Verify Phase 7 against a running backend and test organization.
   - **Start at:** `backend/src/routes/marketplace_runtime.rs` and `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql`.
   - **Prerequisites:** Docker API access or an equivalent PostgreSQL/Redis environment; safe test organization and approved artifact fixture.
   - **Required work:** Start the backend without resetting data, verify migration application, list the permission catalog, exercise runtime authorization allow/deny cases, activate/lift organization and global kill switches, and confirm install/re-enable gates stop while blocked.
   - **Validation:** `/health`, `/ready`, `/openapi.json`, backend logs, and authenticated tenant/global-admin API assertions; record actual results here.
   - **Done when:** Migration `0020` and all Phase 7 permission/sandbox/kill-switch gates are confirmed in a live tenant-scoped flow.

2. [ ] Review generated OpenAPI and documentation against Phase 7 routes.
   - **Start at:** `backend/src/routes/mod.rs` OpenAPI registration and `docs/API.md` Marketplace section.
   - **Prerequisites:** Backend compiles and `/openapi.json` is reachable.
   - **Required work:** Confirm all six Phase 7 paths and schemas appear in generated OpenAPI; reconcile any path/schema drift without changing product scope.
   - **Validation:** Fetch `/openapi.json`, backend static contract tests, and `git diff --check`.
   - **Done when:** Runtime OpenAPI and manual API documentation agree.

3. [ ] Decide how to handle legacy backend clippy warnings.
   - **Start at:** the files reported by `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`.
   - **Prerequisites:** Separate pre-existing warnings from any new Phase 6 diagnostics.
   - **Required work:** Either fix warnings in a separately scoped maintenance change or document an approved CI lint policy; do not mix unrelated fixes into Marketplace work.
   - **Validation:** The exact clippy command and backend CI.
   - **Done when:** The policy and CI result are explicit and reproducible.

4. [ ] Plan V3 Phase 8 concrete runtime adapters without implementing them in this checkpoint.
   - **Start at:** `docs/V3_PHASE_SEVEN.md`, `docs/V3_MARKETPLACE_SCOPE.md`, and the Phase 8 proposal sections for Component Pack Runtime, Template Import, and Plugin Hook MVP.
   - **Prerequisites:** Product decision on permission catalog, sandbox model, kill switch, and emergency revocation.
   - **Required work:** Produce an acceptance checklist and isolated adapter/API design before changing product code; preserve the policy-only Phase 7 boundary.
   - **Validation:** Review against the V3 proposal and scope lock; update `HANDOFF.md` before implementation.
   - **Done when:** A separately authorized Phase 8 objective is explicit.

5. [ ] Commit handoff documentation only after user authorization.
   - **Start at:** root `AGENTS.md` and `HANDOFF.md`.
   - **Prerequisites:** Review all paths and current Git state.
   - **Required work:** Stage only the handoff files if the user explicitly asks for a commit; never include unrelated product changes.
   - **Validation:** `git diff --check`, `git diff --stat`, and `git status --short` before committing.
   - **Done when:** The user has authorized and reviewed the exact commit scope.

## 16. Exact Next Action

For Phase 8, the exact next action is a non-destructive live smoke check in an
environment with Docker API access: verify PostgreSQL/Redis, apply migration
`0021` without reset, start the backend, confirm `/health`, `/ready`, and
`/openapi.json`, then exercise Component Pack registry listing, Template preview
and import with a same-organization media mapping plus one cross-tenant rejection,
and public Plugin Hook listing/authorization. Record actual results here. Do not
execute uploaded package code, enable paid products, reset data, or create a
commit.

The older Phase 7 instruction below is historical and superseded by the Phase 8
action above.

Open `HANDOFF.md`, inspect `git status --short` and the latest commit, then run a
non-destructive live Phase 7 smoke check in an environment with Docker API access:
verify PostgreSQL and Redis, start the backend with the existing environment
variable names, confirm `/health`, `/ready`, and `/openapi.json`, list the Phase 7
permission catalog, authorize one allowed and one denied runtime operation, and
activate/lift organization and global kill switches against a safe test
organization. Do not reset the database, execute uploaded package code, enable
paid products, or create a commit. Record the actual migration `0020` and API
results in this file before planning Phase 8 adapters.

## 17. Acceptance Criteria for the Current Phase

### Phase 8 acceptance override

- [x] Installed Component Pack definitions are namespaced and available to the same organization’s Page Builder palette.
- [x] Design Template preview/import creates an independent page/version and verifies organization-owned asset mappings.
- [x] Integration Plugin hooks are restricted to the four public contract types and return policy-only authorization.
- [x] Phase 8 migration uses tenant RLS for template imports and plugin hooks; package code is never executed.
- [x] Backend 96 tests, frontend 9 tests, lint/typecheck/build, Mermaid 36-file validation, and diff check pass.
- [ ] Live migration `0021` and authenticated end-to-end adapter smoke are verified.

- [x] Free Component Pack and Design Template installation is organization-owned and tenant-scoped.
- [x] Install requires approved listing/version, safe validation/risk, active compatibility, exact owner/admin permission approval, and intact artifact bytes.
- [x] Enable, disable, soft-uninstall, semantic-version update, explicit pinning, and safe same-listing rollback are implemented.
- [x] Lifecycle timestamps, preserved organization data, forced RLS, and transactional audit records are implemented.
- [x] Paid/custom products and executable package runtime remain blocked/deferred.
- [x] Backend formatting and 87 backend tests pass.
- [x] Frontend lint, typecheck, 8 tests, and production build pass.
- [x] Phase 6 API, architecture, gap, traceability, and Mermaid documentation is updated.
- [x] Phase 7 permission catalog, allowlisted sandbox policy, and global/organization kill switches are implemented.
- [x] Phase 7 runtime authorization denies inactive/blocked installations, unknown operations, unapproved permissions, unsafe entry points, and oversized payloads.
- [x] Runtime authorization explicitly does not execute uploaded package code.
- [x] Phase 7 API, architecture, gap, manifest, traceability, and Mermaid documentation is updated.
- [x] No unrelated application code was intentionally changed by the Phase 7 implementation.
- [x] `HANDOFF.md` and root `AGENTS.md` are present and describe recovery protocol.
- [ ] Live migration `0020` and authenticated end-to-end API/browser smoke are verified.
- [ ] Any legacy clippy policy/CI failure is resolved or explicitly accepted.

## 18. Environment and Setup Notes

- **Platform observed:** Windows PowerShell, repository at `D:\All projects\ZinharCMS`.
- **Backend runtime:** Rust stable toolchain; run Cargo commands with `--manifest-path backend/Cargo.toml` from the repository root.
- **Frontend runtime:** Node/npm; CI specifies Node 22. Frontend commands can be run with `npm --prefix frontend ...`.
- **Local infrastructure:** PostgreSQL 16, Redis 7, and pgAdmin from `docker-compose.yml`.
- **Production-like infrastructure:** `docker-compose.prod.yml` also runs backend and Nginx-served frontend images, using environment variable names from `.env.example`.
- **Safe setup:** copy `.env.example` to `.env`, then start only the required local infrastructure with `docker compose up -d postgres redis pgadmin`. Do not expose or copy `.env` values into documentation.
- **Backend development:** `cargo run --manifest-path backend/Cargo.toml` after required environment variables and database/Redis are available.
- **Frontend development:** `npm install --prefix frontend`, then `npm --prefix frontend run dev`.
- **Validation:** `cargo fmt --manifest-path backend/Cargo.toml -- --check`, `cargo test --manifest-path backend/Cargo.toml --all-features`, `npm --prefix frontend run lint`, `npm --prefix frontend run typecheck`, `npm --prefix frontend test`, `npm --prefix frontend run build`.
- **Required variable names:** `DATABASE_URL`, `REDIS_URL`, `JWT_SECRET`, `JWT_ACCESS_EXPIRY`, `JWT_REFRESH_EXPIRY`, `UPLOAD_DIR`, `MAX_UPLOAD_SIZE`, `CORS_ORIGIN`, `PORT`, `VITE_API_URL`, and the optional billing/email/rate-limit names listed in `.env.example`.

## 19. Resume Instructions

At the beginning of the next session:

1. Read AGENTS.md completely.
2. Read HANDOFF.md completely.
3. Inspect git status, git diff, and recent commits.
4. Compare the repository state with HANDOFF.md.
5. Treat the repository as the source of truth if they differ.
6. Summarize completed work, incomplete work, and the exact next action.
7. Continue from “Exact Next Action.”
8. Do not restart completed work or discard existing changes.
9. Update HANDOFF.md after each meaningful milestone and before stopping.

## 20. Suggested Resume Prompt

```text
Read AGENTS.md and HANDOFF.md completely.

Inspect the actual repository state using git status, git diff, and recent commits. Compare it with HANDOFF.md and correct stale information when necessary.

Continue from the “Exact Next Action” section. Preserve all existing work, avoid repeating completed tasks, and do not expand the scope.

After each meaningful milestone, update HANDOFF.md with the files changed, work completed, test results, remaining tasks, and the new exact next action. Before stopping for any reason, leave the repository at the safest available checkpoint and update HANDOFF.md.
```

## 21. Handoff History

### 2026-07-10 17:23 +01:00 — Handoff system initialized

- Repository inspected.
- Current Git and implementation state documented.
- Known completed and incomplete work recorded.
- Next action identified.
- No product code intentionally modified by handoff setup.

### 2026-07-10 18:28 +01:00 — V3 Phase 7 security runtime checkpoint

- Re-read `AGENTS.md` and `HANDOFF.md`; verified `HEAD` `b1b3d05` matches `origin/main` before implementation.
- Implemented Phase 7.1 permission catalog, 7.2 policy-only sandbox authorization, and 7.3 global/organization kill switches.
- Backend 93 tests, frontend 9 tests, lint/typecheck/build, and 35-diagram static validation passed.
- Live migration/API smoke remains the exact next action; no Phase 7 commit was created.

### 2026-07-10 18:35 +01:00 — Phase 7 validation checkpoint

- Backend formatting and 93 tests passed.
- Frontend lint, typecheck, 9 tests, and production build passed; Vite retained the existing large-chunk warning.
- Mermaid structural/evidence validation passed for 35 files.
- Working tree remains uncommitted for Phase 7; live migration/API smoke is still pending.

### 2026-07-10 18:45 +01:00 — Phase 7 final code validation checkpoint

- Fixed the remaining Phase 7 Clippy findings (route borrow/condition, contract-test placement, and explicit policy exception for the multi-gate authorizer).
- `cargo fmt --check` and `cargo test --all-features` passed: 93 backend tests plus doc tests; Clippy remains blocked only by legacy warnings outside Phase 7.
- The exact next action remains the non-destructive live migration/API smoke for migration `0020`; no commit was created.

### 2026-07-10 19:10 +01:00 - V3 Phase 8 runtime adapters checkpoint

- Verified Git source of truth: clean Phase 7 commit `1231613` before Phase 8 changes.
- Implemented 8.1 Component Pack registry/Page Builder integration, 8.2 Template preview/import/clone with tenant media mapping, and 8.3 public Plugin Hook registry/authorization.
- Added migration `0021`, adapter routes/service, frontend API/UI/test updates, Phase 8 docs, and Mermaid diagram `35`.
- Backend 96 tests, frontend 9 tests, lint/typecheck/build, Mermaid 36-file static validation, and `git diff --check` passed.
- Exact next action is live migration `0021` and authenticated adapter smoke; no Phase 8 commit was created.

### 2026-07-10 19:25 +01:00 - Phase 8 final backend checkpoint

- Added automatic Component Pack registry synchronization before Template Import validation so templates can use active installed components without a prior UI visit.
- `cargo fmt --check`, `cargo check --all-features`, and `cargo test --all-features` passed; 96 backend tests and doc tests completed with 0 failures.
- Clippy still reports only legacy warnings outside the Phase 8 adapter module; live migration/API smoke remains pending.

### 2026-07-10 19:35 +01:00 - Phase 8 frontend validation checkpoint

- Added Page Builder and Marketplace UI assertions for installed Component Pack and public Hook surfaces.
- Frontend lint, typecheck, and Vitest passed: 3 files, 9 tests; live Vite build had already passed after the Phase 8 UI implementation.
- The exact next action remains the non-destructive live migration `0021` and authenticated adapter API/browser smoke; no commit was created.

### 2026-07-10 21:20 +01:00 - V3 Phase 9 monetization checkpoint

- Verified Git source of truth: Phase 8 is committed at `b52f81c`; no Phase 8 work was repeated.
- Implemented 9.1 free purchase/entitlement, 9.2 paid one-time Stripe checkout and paid lifecycle entitlement gates, 9.3 idempotent purchase/refund revenue ledger, and 9.4 payout onboarding plus admin verification/eligibility.
- Added migration `0022`, finance routes/service, Stripe Marketplace event handling, frontend purchase/payout surfaces, `docs/V3_PHASE_NINE.md`, and Mermaid diagram `36`.
- Backend tests passed at the first checkpoint (98); final rerun includes the new Phase 9 contract test. Frontend typecheck/lint/build pass; one historical Phase 6 assertion was updated for the new paid-checkout behavior and requires the final rerun.
- **Exact Next Action:** run final backend/frontend suites and Mermaid/diff checks; then apply migration `0022` in a safe development database and perform signed Stripe checkout/refund plus payout-verification smoke. Do not create a commit unless the user explicitly requests it.

### 2026-07-10 21:29 +01:00 - User-requested pause checkpoint

- Final local validation completed: backend formatting and 99 tests passed; frontend lint, typecheck, production build, and 9 tests passed. Vitest still reports two pre-existing post-teardown Dashboard async warnings despite all tests passing.
- Mermaid structural validation passed for 37 files (`00` through `36`), and `git diff --check` passed with only line-ending notices.
- Because the original debug executable was locked, an isolated Phase 9 binary was compiled under `backend/target/phase9` and started on port `8081` without modifying the existing service. The isolated process was stopped at the user's request.
- PostgreSQL confirms SQLx migration `22 v3 phase nine marketplace finance` applied successfully; migration 21 is also present and successful.
- Working tree remains uncommitted and unstaged. No reset, deletion, commit, or branch operation was performed.
- **Exact Next Action:** resume with authenticated Phase 9 API smoke against migration `0022`: free checkout/install, paid checkout with signed Stripe completion, paid install entitlement gate, full-refund reversal/revocation, payout onboarding, and admin payout verification. Then update this handoff with smoke results. Do not repeat implementation or local test suites unless code changes.

### 2026-07-11 07:25 +01:00 - Phase 9 smoke blocker

- Read `AGENTS.md`, this handoff, Git status, diff summary, and recent commits; repository state still matches the Phase 9 uncommitted checkpoint on `b52f81c`.
- Backend smoke was attempted twice. The process compiled but could not complete migrations because PostgreSQL was unavailable (`pool timed out while waiting for an open connection`); `/health`, `/ready`, and `/openapi.json` therefore could not be reached.
- `Test-NetConnection` confirmed localhost ports `5432` and `6379` are closed. `docker compose up -d postgres redis` failed because Docker Desktop daemon `//./pipe/dockerDesktopLinuxEngine` is not running.
- No source files were changed during this smoke attempt; no commit, reset, cleanup, or destructive action was performed.
- **Exact Next Action:** start Docker Desktop (or provide an equivalent PostgreSQL/Redis environment), run `docker compose up -d postgres redis`, then start the backend and execute the authenticated Phase 9 smoke matrix. Do not repeat local implementation/tests unless code changes.

### 2026-07-11 07:40 +01:00 - Phase 9 authenticated smoke completed

- Docker PostgreSQL/Redis were started successfully; migrations 20, 21, and 22 are present and successful.
- Backend smoke passed: `/health` 200, `/ready` 200 with PostgreSQL/Redis reachable, `/openapi.json` 200; all five Phase 9 paths and purchase/ledger/payout schemas were present.
- 9.1 passed: free checkout returned `201`, created a completed purchase and active entitlement, and free installation returned `200 active` with artifact verification.
- 9.2 passed: paid checkout without configured Stripe secret returned `503` and persisted `failed`; paid install without entitlement returned `409`. With a locally signed `checkout.session.completed`, purchase became `completed`, entitlement was granted, and paid installation returned `200 active`.
- 9.3 passed: purchase ledger split recorded platform fee `980` and creator share `3920` for a `4900` purchase. A locally signed full `charge.refunded` returned `200`; purchase became `refunded`, entitlement became `revoked`, and exactly two ledger entries (`purchase`, `refund`) remained.
- 9.4 passed: payout onboarding returned `pending`; verification without submitted details returned `409`; provider-attested verification with all readiness flags returned `200 verified` and `payouts_enabled=true`.
- All temporary fixture rows and artifact files were removed; existing database rows were not reset. Backend process was stopped; PostgreSQL/Redis remain healthy under Docker.
- **Exact Next Action:** review the uncommitted Phase 9 diff and, only after explicit user authorization, stage/commit the Phase 9 implementation. No further implementation or test repetition is required unless review identifies a change.
