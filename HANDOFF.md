# Project Handoff

> Persistent recovery and continuation document for Codex and human developers.
> The repository and Git state are the source of truth when this document becomes stale.

## 1. Handoff Metadata

- **Last updated:** 2026-07-10 17:23 +01:00 (Europe/London)
- **Updated by:** Codex
- **Repository:** ZinharCMS
- **Current branch:** `main`
- **Base branch:** `main` / `origin/main`
- **Latest relevant commit:** `7f18d7b feat(marketplace): implement phase 6 installation lifecycle`
- **Working tree:** Clean before this handoff; this setup intentionally adds untracked `AGENTS.md` and `HANDOFF.md` only.
- **Current version:** `0.1.0` in root, frontend, and backend manifests
- **Current phase:** V3 Marketplace Phase 6 — Installation Lifecycle
- **Current subphase:** Phase 6.3 update and rollback; implementation is complete
- **Overall status:** Completed for Phase 6; ready for the next verification/planning checkpoint

## 2. Project Overview

ZinharCMS is a headless CMS and multi-tenant SaaS administration product. It
serves organization owners and content teams through a React admin application
and a Rust/Axum API. The repository is a modular monolith with PostgreSQL as the
system of record, Redis for cache/rate-limit support, and local filesystem
storage for CMS media and Marketplace package artifacts.

The baseline includes the original CMS phases zero through ten, V2 organization,
billing, beta, and GA operations, and V3 Marketplace phases 0.1 through 6. The
current V3 implementation has reached the free Marketplace installation
lifecycle: reviewed free Component Packs and Design Templates can be installed,
managed, updated, and safely rolled back without executing uploaded package code.

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
| `docs/V3_MARKETPLACE_SCOPE.md` | V3 scope lock and MVP/out-of-scope rules. | Current product-scope authority. |
| `docs/V3_MARKETPLACE_GAP_LIST.md` | Resolved and deferred Marketplace gaps by phase. | Current gap/status record; verify against runtime. |
| `docs/V3_MARKETPLACE_POLICY.md` and `docs/V3_PRODUCT_TAXONOMY.md` | Review, moderation, product classification, and safety policy. | Current policy authority. |
| `docs/API.md` | Runtime route boundaries and Marketplace endpoint documentation. | Current, with older Marketplace routes manually documented. |
| `docs/ARCHITECTURE.md` | Runtime containers, tenant boundaries, RLS, and Marketplace architecture. | Mostly current; its sentence saying the schema is authoritative through migration `0018` is stale because Phase 6 adds `0019`. |
| `docs/diagrams/ARCHITECTURE_AUDIT.md`, `TRACEABILITY.md`, `FILE_EVIDENCE_INDEX.md`, `33-marketplace-installation-lifecycle.mmd` | Evidence links and visual Phase 6 implementation state. | Updated with Phase 6 evidence; static Mermaid validation is available, but no Mermaid parser is installed. |
| `D:\All projects\Zinhar_Doc\version_3_marketplace_proposal.html` | Original V3 Marketplace proposal and future lifecycle goals. | Planning authority; current migrations/routes/tests supersede it for implementation status. |
| `D:\All projects\Zinhar_Doc\version_2_proposal.html` | V2 SaaS/organization/billing proposal. | Historical planning authority for V2 dependencies. |
| `D:\All projects\Zinhar_Doc\headless_cms_proposal_polished.html` | Original CMS proposal. | Historical baseline; current repository documentation and code are newer. |

The proposals describe the complete future Marketplace lifecycle, including paid
products and executable/runtime concepts. Phase 6 intentionally implements only
the reviewed free install registry lifecycle; paid entitlements, payouts,
customer ratings, runtime permissions, and sandbox execution remain deferred.

## 6. Current Objective

No unfinished product implementation objective is evidenced after commit
`7f18d7b`. The active documentation objective is to preserve a truthful recovery
checkpoint for the completed V3 Marketplace Phase 6 implementation.

Phase 6 boundaries that must remain unchanged until their dedicated phases are
planned and authorized:

- only free `component_pack` and `design_template` products are installable;
- uploaded package code is never executed;
- paid purchase/entitlement and creator payout flows are not implemented;
- runtime permission enforcement, sandboxing, and emergency permission revocation are deferred;
- no background automatic update is enabled; installations remain explicitly pinned.

## 7. Completed and Verified Work

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

## 8. Completed but Not Verified

- [ ] Live application of migration `0019` and end-to-end API/browser installation smoke test.
  - **Files:** `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql`, `backend/src/routes/marketplace.rs`, `frontend/src/pages/MarketplacePage.tsx`
  - **Missing verification:** a running backend connected to the intended database, followed by authenticated tenant requests and artifact-backed install/update/rollback.
  - **Recommended validation:** start the backend with the project environment, verify `/health`, `/ready`, `/openapi.json`, and exercise the Phase 6 endpoints against a test organization without resetting data.
  - **Reason:** Docker infrastructure was visible with `docker compose ps`, but `docker compose exec` was denied Docker API access in this sandbox, and the local compose file does not include the backend service.

- [ ] Mermaid parser/render validation.
  - **Files:** `docs/diagrams/*.mmd`
  - **Missing verification:** no `mmdc`/Mermaid parser dependency is installed in the repository.
  - **Recommended validation:** use an approved Mermaid renderer in a separate documentation environment.
  - **Reason:** only static declaration/fence validation was available.

## 9. Work in Progress

### Active item

There is no partially implemented product item. Phase 6 is committed and the
current task adds only persistent recovery documentation.

### Exact stopping point

The implementation stopped at commit `7f18d7b`, after the Phase 6.3 update and
rollback flow and its frontend/backend tests were completed.

### Partially modified files

| File | Current state | Remaining work |
| --- | --- | --- |
| None in product code | No product file is partially modified after the Phase 6 commit. | Perform live migration/API smoke before starting a later phase. |

### Incomplete implementation markers

- Marketplace purchase, entitlement, payout, customer-rating, runtime permission, sandbox, and emergency permission-revocation items are intentionally planned/deferred, not stubs to finish in Phase 6.
- The frontend production build reports a chunk-size warning over 500 kB; this is non-blocking technical debt.

## 10. Current Git and Filesystem State

### Staged files

- None.

### Modified files

- None in the Phase 6 implementation before handoff creation.

### Untracked files

- `AGENTS.md` — new root-level persistent handoff protocol required by this task.
- `HANDOFF.md` — this repository-specific recovery document.

Both files should be reviewed and kept. They must not be committed unless the
user explicitly authorizes a commit.

### Deleted files

- None.

### Important diff observations

- Before this handoff setup, `git status --porcelain=v1 -uall` was clean and `HEAD` matched `origin/main`.
- Commit `7f18d7b` contains the Phase 6 product, test, API, and diagram changes; no product code is intentionally changed by this handoff task.
- No secrets or values from `.env` were copied into this document.

## 11. Tests and Validation

### Commands executed during this inspection

| Command | Executed? | Result | Notes |
| --- | ---: | --- | --- |
| `cargo fmt --manifest-path backend/Cargo.toml -- --check` | Yes | Passed | Rust formatting is clean. |
| `cargo test --manifest-path backend/Cargo.toml --all-features` | Yes | Passed | 87 passed, 0 failed; doc tests also completed. |
| `npm --prefix frontend run lint` | Yes | Passed | ESLint completed successfully. |
| `npm --prefix frontend run typecheck` | Yes | Passed | `tsc -b` completed successfully. |
| `npm --prefix frontend test` | Yes | Passed | 3 files, 8 tests passed. |
| `npm --prefix frontend run build` | Yes | Passed with warning | Vite build completed; one output chunk is over 500 kB. |
| Repository-local Mermaid structural check | Yes | Passed | 34 files, one declaration each, no fences. |
| `git diff --check` | Yes | Passed | No whitespace errors. |
| `docker compose ps` | Yes | Passed | PostgreSQL and Redis reported healthy; compose warned that `version` is obsolete. |
| `docker compose exec -T postgres psql ...` | Yes | Not run successfully | Docker API permission was denied by the sandbox before the query executed. |

### Other known validation

- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings` was run against the current Phase 6 tree and failed on pre-existing warnings in older backend modules. No new Phase 6-specific warning was identified in the reported output. Do not treat CI clippy as green until those legacy warnings are addressed or the policy changes.
- No destructive migration, database reset, dependency installation, deployment, or commit was performed during handoff setup.

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

## 13. Known Issues, Risks, and Technical Debt

### Blocking issues

- None for the committed Phase 6 code path.

### Non-blocking issues

- **Medium:** Live DB migration/API smoke is still unverified because Docker API exec access was denied from this sandbox; use a development environment with Docker access.
- **Medium:** `cargo clippy -D warnings` reports legacy warnings in older modules, so the backend CI clippy step is not currently a reliable green signal.
- **Low:** Mermaid files have only repository-local structural validation; no parser/render dependency is installed.
- **Low:** Vite emits a large output chunk warning (>500 kB).
- **Low:** `docs/ARCHITECTURE.md` still says schema authority ends at migration `0018`; reconcile this documentation-only sentence to `0019` in a future maintenance pass.

### Security risks

- **High if scope expands:** Marketplace package execution and runtime permission enforcement do not exist. Do not enable executable extensions before sandboxing, kill-switch, permission enforcement, and forced-RLS review are implemented.
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
- [x] Current manifests report version `0.1.0`.
- [x] The local development compose file provides PostgreSQL, Redis, and pgAdmin; it does not run the backend/frontend services.

### Unconfirmed assumptions

- [ ] The intended developer database has actually applied migration `0019`.
- [ ] A deployed environment has the expected `UPLOAD_DIR` contents and artifact files needed for end-to-end installation smoke tests.
- [ ] The exact V3 Phase 7/8 implementation order for runtime permissions, sandboxing, and emergency revocation has not been finalized in current repository docs.
- [ ] The user has not authorized committing the newly created `AGENTS.md` and `HANDOFF.md`.

## 15. Remaining Work

1. [ ] Verify Phase 6 against a running backend and test organization.
   - **Start at:** `backend/src/routes/marketplace.rs` Phase 6 handlers and `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql`.
   - **Prerequisites:** Docker API access or an equivalent PostgreSQL/Redis environment; safe test organization and approved artifact fixture.
   - **Required work:** Start the backend without resetting data, verify migration application, then exercise list/install/enable/disable/uninstall/updates/update/rollback with owner/admin and non-owner roles.
   - **Validation:** `/health`, `/ready`, `/openapi.json`, backend logs, and authenticated API assertions; record actual results here.
   - **Done when:** Migration and all Phase 6 lifecycle gates are confirmed in a live tenant-scoped flow.

2. [ ] Reconcile the stale schema-authority sentence in `docs/ARCHITECTURE.md`.
   - **Start at:** the “Data And Tenant Isolation” section.
   - **Prerequisites:** Confirm migration `0019` is the deployed/current latest migration.
   - **Required work:** Change only the migration range statement from `0018` to `0019` and update any related evidence wording if needed.
   - **Validation:** `git diff --check` and backend documentation static tests.
   - **Done when:** Architecture docs no longer contradict the current migration chain.

3. [ ] Decide how to handle legacy backend clippy warnings.
   - **Start at:** the files reported by `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`.
   - **Prerequisites:** Separate pre-existing warnings from any new Phase 6 diagnostics.
   - **Required work:** Either fix warnings in a separately scoped maintenance change or document an approved CI lint policy; do not mix unrelated fixes into Marketplace work.
   - **Validation:** The exact clippy command and backend CI.
   - **Done when:** The policy and CI result are explicit and reproducible.

4. [ ] Plan the next V3 runtime/permission phase without implementing it in this checkpoint.
   - **Start at:** `docs/V3_PHASE_SIX.md` “Deferred Boundaries” and `docs/V3_MARKETPLACE_GAP_LIST.md` “Permission Gaps”.
   - **Prerequisites:** Product decision on permission catalog, sandbox model, kill switch, and emergency revocation.
   - **Required work:** Produce an acceptance checklist and migration/API design before changing product code.
   - **Validation:** Review against the V3 proposal and scope lock; update `HANDOFF.md` before implementation.
   - **Done when:** A separately authorized Phase 7/8 objective is explicit.

5. [ ] Commit handoff documentation only after user authorization.
   - **Start at:** root `AGENTS.md` and `HANDOFF.md`.
   - **Prerequisites:** Review all paths and current Git state.
   - **Required work:** Stage only the handoff files if the user explicitly asks for a commit; never include unrelated product changes.
   - **Validation:** `git diff --check`, `git diff --stat`, and `git status --short` before committing.
   - **Done when:** The user has authorized and reviewed the exact commit scope.

## 16. Exact Next Action

Open `HANDOFF.md`, inspect `git status --short` and the latest commit, then begin
with a non-destructive live Phase 6 smoke check in an environment that has Docker
API access: start/verify PostgreSQL and Redis, start the backend with the existing
environment variable names, confirm `/health`, `/ready`, and `/openapi.json`, and
exercise the tenant-scoped installation list/install/lifecycle endpoints against
a safe test organization and approved artifact fixture. Do not reset the database,
change Phase 6 product code, enable paid/runtime Marketplace behavior, or create a
commit. Record the actual migration/API results in this file before planning any
Phase 7/8 work.

## 17. Acceptance Criteria for the Current Phase

- [x] Free Component Pack and Design Template installation is organization-owned and tenant-scoped.
- [x] Install requires approved listing/version, safe validation/risk, active compatibility, exact owner/admin permission approval, and intact artifact bytes.
- [x] Enable, disable, soft-uninstall, semantic-version update, explicit pinning, and safe same-listing rollback are implemented.
- [x] Lifecycle timestamps, preserved organization data, forced RLS, and transactional audit records are implemented.
- [x] Paid/custom products and executable package runtime remain blocked/deferred.
- [x] Backend formatting and 87 backend tests pass.
- [x] Frontend lint, typecheck, 8 tests, and production build pass.
- [x] Phase 6 API, architecture, gap, traceability, and Mermaid documentation is updated.
- [x] No unrelated application code was intentionally changed by the handoff setup.
- [x] `HANDOFF.md` and root `AGENTS.md` are present and describe recovery protocol.
- [ ] Live migration and authenticated end-to-end API/browser smoke are verified.
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
