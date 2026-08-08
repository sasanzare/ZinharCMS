# Project Handoff

> Persistent recovery and continuation document for Codex and human developers.
> The repository and Git state are the source of truth when this document becomes stale.

## 1. Handoff Metadata

- **Last updated:** 2026-08-08 (Europe/London)
- **Updated by:** Codex
- **Repository:** ZinharCMS
- **Current branch:** `security/security-audit-fixes`
- **Base branch:** `main`; current branch tracks `origin/security/security-audit-fixes`
- **Latest relevant commit:** `eb050a0010ccd721446f5d2ac4de4863679a9564` docs(okf): complete Google OKF v0.2 Phase 5 migration
- **Working tree:** Phase 6 adds the isolated staging Concept, one development index update, one staging-log update, five Phase 6 artifacts, and this handoff checkpoint; final validation passed and no staged paths are present
- **Current version:** `3.0.0` across root, frontend, backend, lockfile, Marketplace runtime, and dashboard release sources
- **Current phase:** Google OKF Migration Phase 6 — Operations, Delivery, Development & Governance Knowledge Migration
- **Current subphase:** Phase 6 construction, semantic reconciliation, staging validation, legacy-safety validation, and checkpoint documentation are complete.
- **Overall status:** Phase 6 is complete for the merge-based operations, delivery, development, governance, and documentation-maintenance boundary. The working tree contains only Phase 6 migration artifacts, the isolated staging Concept/index/log changes, and this handoff; no paths are staged. No legacy conversion/deletion, application change, dependency change, commit, push, reset, clean, stash, or history rewrite was performed.

> **Google OKF Migration Phase 6 safety override (2026-08-08):** This is the
> active continuation checkpoint for the migration task and supersedes older
> OKF phase next-action text below. The existing `okf/` and `okf-bootstrap/`
> trees remain legacy material and are untouched. The Phase 6 staging bundle is
> non-canonical. Do not delete, rename, rewrite, convert, replace, cut over,
> stage, commit, push, reset, clean, stash, or deploy without explicit owner
> authorization.

### Google OKF Migration Phase 0 Checkpoint

- **Objective:** Establish an evidence-backed baseline for migrating the
  repository's incorrect legacy OKF convention to the official Google OKF v0.2
  specification without losing project knowledge.
- **Status:** `COMPLETE` for the Phase 0 audit boundary. Production topology,
  owner-policy, and target-bundle decisions remain explicitly unresolved.
- **Branch and baseline HEAD:** `security/security-audit-fixes` at
  `518f74a1b0da5c4ee37c14e2a37a716707468410`.
- **Files created:** `docs/okf-migration/PHASE_00_BASELINE_AUDIT.md`,
  `docs/okf-migration/LEGACY_OKF_INVENTORY.md`,
  `docs/okf-migration/GOOGLE_OKF_V02_BASELINE.md`,
  `docs/okf-migration/LEGACY_REFERENCE_MAP.md`, and
  `docs/okf-migration/MIGRATION_RISK_REGISTER.md`.
- **Existing file modified by this phase:** `HANDOFF.md` only. The 23 other
  modified paths and 7 untracked paths listed in the Phase 0 report predate
  this audit and remain user-owned.
- **Key findings:** `okf/` contains 339 custom-frontmatter Markdown files, 50
  Mermaid diagrams, and a 411,888-byte custom `index.yaml`; no legacy Markdown
  file has the Google-required `type` field. `okf-bootstrap/` contains 14 plain
  historical audit/planning reports with unique gap and owner-question
  knowledge. Exact searches found documentation/navigation coupling but no
  executable dependency in Rust, frontend source, scripts, manifests, Docker,
  migrations, tests, or CI.
- **Authoritative target:** Google’s official
  [`okf/SPEC.md`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md),
  verified on 2026-08-07; it still specifies OKF v0.2.
- **Legacy warning:** The current `okf/` tree is still legacy and is not a
  Google OKF v0.2 bundle. Its content may still be accurate and valuable.
- **Deletion warning:** Do not delete `okf/`, `okf-bootstrap/`,
  `okf/index.yaml`, or legacy metadata before the Phase 1 preservation map is
  reviewed.
- **Worktree baseline:** Before Phase 0, 23 paths were modified, 7 were
  untracked, and none were staged. The two pre-existing modified legacy files
  are `okf/backend/configuration-and-state.md` and `okf/security/overview.md`.
- **Validation performed:** Repository instructions, Git state, full legacy
  trees, custom metadata, exact references, runtime/configuration coupling,
  generated boundaries, and the official specification were inspected. All five
  audit files are non-empty and were read back; 9 internal Markdown links
  resolve with 0 broken links; 45 explicitly cited repository paths exist; the
  executable/configuration legacy-reference search returned 0 matches; the
  legacy diff has only the 2 pre-existing `M` paths and 0 deletes/renames; and
  `git diff --check` passed. Git emitted only existing LF/CRLF normalization
  warnings for dirty files.
- **Unresolved issues:** Per-document claim verification; source/provenance
  mapping; path-ID reconciliation; diagram treatment; bundle boundary;
  optional index/log use; generated/verified/status mapping; and owner
  questions in `okf-bootstrap/12-owner-questions.md` remain open.
- **Final worktree:** HEAD remains
  `518f74a1b0da5c4ee37c14e2a37a716707468410`; 23 modified paths and 12
  untracked paths are present, none staged. The 5 new untracked paths are the
  Phase 0 audit documents; the other 7 untracked and 23 modified paths predate
  this phase.
- **Exact Next Action:** begin `Phase 1 — Legacy OKF Inventory & Knowledge
  Preservation Map`; inventory every legacy document and evidence relationship,
  verify claims against current repository evidence, and produce the
  KEEP/MIGRATE/MERGE/DELETE candidate map without deleting or converting legacy
  files.

### Google OKF Migration Phase 1 Checkpoint

- **Status:** COMPLETE for the preservation-map boundary. Phase 2 target
  architecture and bundle design has not started.
- **Branch:** security/security-audit-fixes.
- **Starting HEAD:** 518f74a1b0da5c4ee37c14e2a37a716707468410.
- **Ending HEAD:** 518f74a1b0da5c4ee37c14e2a37a716707468410.
- **Worktree at Phase 1 start:** 23 modified tracked paths, 12 untracked paths,
  no staged paths. The 23 modified paths and 7 original untracked paths
  predated the OKF work; the 5 Phase 0 reports were already untracked before
  this phase.
- **Phase 1 files created:** docs/okf-migration/PHASE_01_KNOWLEDGE_PRESERVATION.md,
  LEGACY_DOCUMENT_DISPOSITION_MATRIX.md, CLAIM_VERIFICATION_LEDGER.md,
  CONCEPT_CANDIDATE_MAP.md, LEGACY_METADATA_TRANSLATION.md,
  DIAGRAM_PRESERVATION_MAP.md, OWNER_DECISION_REGISTER.md,
  OWNER_QUESTION_DECISION_DEBT.md, and PHASE_01_EVIDENCE_MANIFEST.json.
- **Existing file modified by Phase 1:** HANDOFF.md. It was already modified by
  Phase 0 and remains unstaged; no other existing repository file was modified
  by this phase.
- **Legacy corpus:** 339 okf Markdown documents, 14 bootstrap Markdown
  documents, 50 Mermaid diagrams, one okf/index.yaml, and 404 material files.
- **Primary dispositions:** MIGRATE 168; MERGE 117; PRESERVE_HISTORICAL 19;
  REGENERATE 35; DELETE_LEGACY_ONLY 14. Review-required overlay: 75.
- **Merge groups:** 13 stable groups, with the largest being API 57, domain 44,
  security 40, extensibility 39, database 34, backend 31, and frontend 31.
- **Claim statuses:** VERIFIED_CURRENT 20; VERIFIED_HISTORICAL 6;
  PARTIALLY_VERIFIED 12; STALE 7; CONTRADICTED 2; UNVERIFIED 8;
  NON_FACTUAL 5.
- **Diagram dispositions:** PRESERVE 19; MERGE 19; REGENERATE 11; DROP 1.
- **Owner decisions:** 18 NOC records preserved; 17 remain open; NOC-16 is
  resolved by the root LICENSE decision. Fifteen UNKNOWN labels remain linked
  to the register.
- **Highest risks:** production topology/ingress, storage durability and
  authorization, backup/recovery, observability/on-call, legal retention,
  public tenant routing, API compatibility, Marketplace settlement/scope,
  and missing ownership.
- **Preservation confirmation:** no legacy files were deleted, renamed,
  rewritten, or converted; okf/index.yaml was not changed; no Google OKF
  replacement bundle was created; application code and dependencies were not
  changed; no commit or push occurred.
- **Validation checkpoint:** complete matrix and diagram coverage, controlled
  vocabulary, claim/diagram/disposition reconciliation, corpus hash, report
  hashes, new-document link/path checks, legacy-diff safety checks, and
  git diff --check are recorded in
  docs/okf-migration/PHASE_01_EVIDENCE_MANIFEST.json.
- **Final validation result (2026-08-07):** the deterministic Phase 1 validator
  exited 0; `git diff --check` exited 0 with only expected line-ending warnings;
  no paths are staged; the ending worktree is 23 modified tracked paths and 21
  untracked paths; the legacy diff still contains only the two pre-existing
  modified OKF files and no delete/rename summary.
- **Recommended next phase:** Phase 2 — Google OKF v0.2 Target Architecture &
  Bundle Design.
- **Exact Next Action:** review the Phase 1 preservation artifacts and owner
  decision register; then begin Phase 2 target concept hierarchy and bundle
  design without deleting or converting legacy content.

### Google OKF Migration Phase 2 Checkpoint (active)

- **Status:** COMPLETE for target architecture and bundle design only. No
  target Google OKF Concept or staging bundle was created.
- **Branch:** `security/security-audit-fixes`.
- **Starting HEAD:** `1cb6f43be6d46bee0bf8ac8b06c833967aae6786`.
- **Ending HEAD:** `1cb6f43be6d46bee0bf8ac8b06c833967aae6786`.
- **Phase 2 starting worktree:** clean; no staged, modified, or untracked
  paths. This is the actual state after the committed Phase 1 checkpoint, not
  the older dirty-worktree description in historical Phase 0/1 sections.
- **Phase 2 final worktree:** nine new untracked design artifacts under
  `docs/okf-migration/` and this existing `HANDOFF.md` modified; no staged
  paths. No unrelated path was changed.
- **Files created:**
  `docs/okf-migration/PHASE_02_TARGET_ARCHITECTURE.md`,
  `TARGET_BUNDLE_TREE.md`, `TARGET_CONCEPT_CATALOG.md`,
  `OKF_TYPE_AND_METADATA_POLICY.md`, `NAVIGATION_AND_LINKING_POLICY.md`,
  `LEGACY_TO_TARGET_MAPPING.md`, `MIGRATION_STAGING_AND_CUTOVER_PLAN.md`,
  `OKF_VALIDATION_CONTRACT.md`, and `PHASE_02_DECISION_RECORD.md`.
- **Existing file modified:** `HANDOFF.md` only.
- **Official OKF baseline:** Google `okf/SPEC.md` was read directly on
  2026-08-07 and still specifies OKF v0.2. No Phase 0/1 baseline change was
  detected.
- **Chosen bundle boundary:** future canonical `/okf/` containing 54 Concepts
  in 12 capability directories plus a root index and root log. Migration
  reports, source evidence, and detailed manifests remain outside the bundle
  under `docs/okf-migration/`.
- **Chosen staging boundary:**
  `docs/okf-migration/staging/google-okf-v0.2/`; it is design-only and does not
  exist yet.
- **Target design:** 54 Concepts, 19 descriptive types, 13 indexes, one root
  log, 33 embedded Mermaid visual blocks, and zero custom frontmatter
  extensions. Concept build strategies are 19 direct, 12 merged, 11
  regenerated, 7 owner-blocked, and 5 historical.
- **Mapping coverage:** 353/353 document rows, all 13 merge groups, all 50
  diagrams, all 60 claims, and all 18 NOC records are accounted for. Seventeen
  NOCs and 15 UNKNOWN labels remain open; NOC-16 remains resolved by LICENSE.
- **Validation performed:** required artifact presence, UTF-8 decoding,
  planning-document link resolution, trailing-whitespace scan, target catalog
  uniqueness, target tree filename/count reconciliation, Phase 1 matrix
  counts, claim/diagram/NOC row counts, staging absence, canonical-target
  absence, out-of-scope path scan, and legacy diff safety checks passed. The
  official specification version was re-verified. `git diff --check` exited 0;
  final Git inspection shows 0 staged paths, 0 legacy diff paths, 9 Phase 2
  untracked artifacts, and this handoff modified. Git emitted only the
  expected LF/CRLF normalization warning while inspecting the existing
  handoff.
- **Legacy confirmation:** legacy `okf/`, `okf-bootstrap/`, and
  `okf/index.yaml` were not deleted, moved, renamed, rewritten, or converted;
  the legacy index remains present and unchanged by Phase 2. No replacement
  bundle, application change, dependency change, commit, or push was made.
- **Migration readiness:** Phase 3 may begin only by constructing the
  high-confidence foundation in the isolated staging path, with owner blockers
  explicit and canonical `/okf/` untouched.
- **Recommended next phase:** `Phase 3 — Core Google OKF v0.2 Bundle Construction`.
- **Exact Next Action:** review this Phase 2 design checkpoint and, if
  authorized, construct only the catalog-approved foundational Concepts under
  the isolated staging path. Do not start cutover or resolve owner questions
  by assumption.

### Google OKF Migration Phase 3 Checkpoint (current)

- **Status:** `COMPLETE` for the core Google OKF v0.2 staging construction
  boundary. Phase 4 architecture/application expansion and any canonical
  cutover remain future work.
- **Branch:** `security/security-audit-fixes`.
- **Starting HEAD:** `b58840e9c227ff9d937b482eced5331122291f82`.
- **Ending HEAD:** `b58840e9c227ff9d937b482eced5331122291f82`.
- **Phase 3 starting worktree:** clean after the Phase 2 design checkpoint;
  no staged, modified, or untracked paths were present before construction.
- **Phase 3 final worktree:** four new top-level Phase 3 paths are untracked
  (`PHASE_03_BUILD_MANIFEST.json`, `PHASE_03_CONSTRUCTION_STATUS.md`,
  `PHASE_03_CORE_BUNDLE_CONSTRUCTION.md`, and `staging/`); this handoff is
  modified; no staged paths are present. No unrelated path is changed.
- **Staging contents:** `docs/okf-migration/staging/google-okf-v0.2/` contains
  13 indexes, one root `log.md`, and 19 Concepts. The root frontmatter is only
  `okf_version: "0.2"`; Concepts use the approved type/status/source policy
  with zero custom frontmatter keys. No standalone `.mmd` file was created.
- **Construction selection:** TARGET-001, TARGET-006, TARGET-007, TARGET-010,
  TARGET-011, TARGET-014, TARGET-015, TARGET-018, TARGET-020, TARGET-027,
  TARGET-028, TARGET-029, TARGET-030, TARGET-033, TARGET-034, TARGET-038,
  TARGET-039, TARGET-045, and TARGET-049 are `BUILT` from current
  repository-backed evidence. The remaining 35 catalog rows are explicitly
  recorded as 12 `DEFERRED_MERGE`, 11 `DEFERRED_REGENERATE`, 7
  `BLOCKED_OWNER_DECISION`, and 5 `HISTORICAL_DEFERRED`; no target is omitted.
- **Diagram construction:** 13 Phase 1-preserved visuals are embedded in
  their current direct owners: system/container architecture, authentication
  and session, authorization/RBAC, tenant, content, page-builder, Marketplace
  installation, and plugin data ownership. Merge/regenerate diagrams remain
  deferred.
- **Manifest:** `PHASE_03_BUILD_MANIFEST.json` records source HEAD, the staging
  paths, 19 built target IDs, 13 embedded diagrams, 35 deferred rows, source
  evidence, and SHA-256 hashes for all 33 staging files. The manifest itself is
  intentionally not self-hashed.
- **Validation performed:** JSON parse, UTF-8/whitespace scan, exact index/log/
  Concept counts, root and Concept frontmatter policy, approved type/status
  values, pinned source URLs, bundle-local link resolution with no parent
  traversal, Mermaid/diagram coverage, matrix coverage and status totals,
  manifest-to-file hash reconciliation, and `git cat-file` verification of
  source paths at the construction HEAD all passed. `git diff --check` passed.
- **Persistent lessons log:** Reviewed the relevant OKF, frontmatter, link,
  manifest, and whitespace lessons. Appended one reusable note for the
  transient duplicate-paragraph patching error caught and corrected during
  Phase 3.
- **Legacy and application safety:** `okf/`, `okf-bootstrap/`, and
  `okf/index.yaml` have no Phase 3 diff. No canonical `/okf/` cutover or
  reference redirect was made. Backend/frontend/database/CI/dependency paths
  have no Phase 3 diff.
- **Open issues:** 35 targets remain explicitly deferred; owner decisions
  NOC-01 through NOC-15, NOC-17, and NOC-18 remain outside this construction.
  The staged bundle is not canonical and must not be activated by assumption.
- **Recommended next phase:** `Phase 4 — Architecture & Application Knowledge
  Migration`.
- **Exact Next Action:** review the Phase 3 manifest, status matrix, and staged
  bundle, then expand the isolated staging root using Phase 1 MERGE and
  REGENERATE mappings for architecture/backend/frontend/API/database/security/
  domain/operations knowledge. Keep `/okf/` and `okf-bootstrap/` untouched and
  do not cut over without separate authorization.

### Google OKF Migration Phase 4 Checkpoint (current)

- **Status:** `COMPLETE` for the Architecture & Application Knowledge
  Migration boundary. The staging bundle remains non-canonical; Phase 5 and
  any cutover remain future work.
- **Branch:** `security/security-audit-fixes`.
- **Starting and ending HEAD:** `e37e94e2e6960a2547f33bf1ebb4225f818b3a4b`.
- **Starting worktree:** clean, with no staged, modified, or untracked paths.
- **Selected targets:** `TARGET-008`, `TARGET-012`, and `TARGET-016`, the only
  Phase 3 `DEFERRED_MERGE` targets whose primary domains were Architecture,
  Backend, or Frontend.
- **Phase 4 files created:**
  `docs/okf-migration/PHASE_04_ARCHITECTURE_APPLICATION_MIGRATION.md`,
  `PHASE_04_BUILD_MANIFEST.json`, `PHASE_04_CONSTRUCTION_STATUS.md`, and
  `PHASE_04_MERGE_LEDGER.md`; plus the three new staging Concepts under
  `architecture/`, `backend/`, and `frontend/`.
- **Staging changes:** the bundle grew from 19 to 22 Concepts. It retains 13
  indexes, one root log, 13 embedded Mermaid blocks, and zero standalone
  `.mmd` files. Three domain indexes and the root log were updated; no Phase 3
  Concept was modified.
- **Traceability:** the 54-target Phase 4 matrix has no duplicate or missing
  IDs. The merge ledger covers 72 unique legacy inputs from G002/G003/G008
  (10/31/31) with explicit routing, deferral, historical, or omission results.
- **Validation:** Concept frontmatter/type/source checks, local-link and root
  reachability checks, Mermaid fence checks, manifest hash reconciliation,
  matrix/ledger coverage, `git diff --check`, and safety scans passed. The
  post-Phase 4 manifest hashes all 36 staging files. No executable tests were
  run because this phase changed only documentation and staging artifacts.
- **Safety:** `okf/`, `okf-bootstrap/`, `okf/index.yaml`, Phase 3 artifacts,
  backend/frontend/database/dependency/CI/runtime paths, and canonical
  references have no Phase 4 diff. No cutover, redirect, deletion, commit, or
  push was performed.
- **Persistent lessons log:** relevant OKF, link, frontmatter, manifest, and
  PowerShell audit lessons were reviewed. No new reusable lesson was identified
  during Phase 4, so the cross-project mistakes log was not changed.
- **Remaining disposition:** 9 `DEFERRED_MERGE`, 11 `DEFERRED_REGENERATE`, 7
  `BLOCKED_OWNER_DECISION`, and 5 `HISTORICAL_DEFERRED` targets remain.
- **Recommended next phase:** `Phase 5 — Data, API, Security & Domain
  Knowledge Migration`.
- **Exact Next Action:** preserve this checkpoint, review the Phase 4 report,
  status matrix, ledger, and full manifest, then begin only the approved Phase
  5 Data/API/Security/Domain selection. Keep `/okf/` and `okf-bootstrap/`
  untouched and do not cut over without separate authorization.

### Google OKF Migration Phase 5 Checkpoint (current)

- **Status:** `COMPLETE` for the Data, API, Security, and Domain staging
  boundary. The staging bundle remains non-canonical; Phase 6 and cutover are
  future work.
- **Branch:** `security/security-audit-fixes`.
- **Starting HEAD:** `6f51612c5716c7d9c3365553811053fd24a03947`, matching the
  origin branch at the start of Phase 5.
- **Starting worktree:** clean, with no staged, modified, or untracked paths.
- **Selected targets:** `TARGET-021`, `TARGET-022`, `TARGET-025`, `TARGET-031`,
  `TARGET-032`, `TARGET-036`, `TARGET-037`, and `TARGET-040`.
- **Out-of-scope target:** `TARGET-047` was evaluated and routed to Phase 6;
  it was not constructed in the Data/API/Security/Domain phase.
- **Phase 5 files created:**
  `docs/okf-migration/PHASE_05_DATA_API_SECURITY_DOMAIN_MIGRATION.md`,
  `PHASE_05_CONSTRUCTION_STATUS.md`, `PHASE_05_MERGE_LEDGER.md`,
  `PHASE_05_SECURITY_DATA_VERIFICATION.md`, and
  `PHASE_05_BUILD_MANIFEST.json`; plus eight new staging Concepts.
- **Staging changes:** the bundle grew from 22 to 30 Concepts. It retains 13
  indexes, one root log, 17 embedded Mermaid blocks, zero standalone `.mmd`
  files, and 44 staging files. Four domain indexes and the root log were
  updated; no existing Concept was modified.
- **Traceability:** the Phase 5 status matrix has 54 unique target IDs. The
  merge ledger covers the selected primary legacy categories and shared group
  routing. The security/data ledger contains 15 stable verification IDs.
- **Validation:** the staging validator passed frontmatter/type/source/link/
  index coverage, Mermaid fence, manifest hash, status-matrix, source-path,
  and safety checks. The manifest contains all 44 staging-file SHA-256 hashes
  with zero mismatches. No executable tests ran because only documentation and
  staging artifacts changed.
- **Final verification rerun (2026-08-07):** the dependency-free OKF text
  validator exited 0 with 30 Concepts, 13 indexes, one log, 17 Mermaid blocks,
  44 manifest hashes, and no errors; the 54-target matrix has 54 unique IDs;
  the Phase 5 report's staging count is reconciled to 44; and `git diff --check`
  exited 0 with only expected LF/CRLF normalization warnings. An optional
  PyYAML-based validator was not available in the environment, so the final
  result uses the repository-compatible text validator and direct JSON/hash
  checks.
- **Safety:** `okf/`, `okf-bootstrap/`, `okf/index.yaml`, application source,
  migrations, dependencies, CI, and runtime/configuration paths have no Phase
  5 diff. No cutover, redirect, deletion, commit, or push was performed.
- **Persistent lessons log:** relevant OKF, provenance, link, frontmatter,
  manifest, and PowerShell lessons were reviewed. No new cross-project lesson
  was added.
- **Remaining disposition:** 11 `DEFERRED_REGENERATE`, 7
  `BLOCKED_OWNER_DECISION`, 1 `OUT_OF_SCOPE`, and 5
  `HISTORICAL_DEFERRED` targets remain.
- **Exact Next Action:** preserve and review the Phase 5 artifacts, then begin
  a separately selected Phase 6 development/documentation/operations boundary.
  Keep the legacy trees and canonical cutover boundary unchanged.

### Google OKF Migration Phase 6 Checkpoint (complete)

- **Status:** `COMPLETE` for the merge-based operations, delivery, development,
  governance, and documentation-maintenance boundary; `TARGET-047` is the sole
  Phase 6 Concept.
- **Branch:** `security/security-audit-fixes`.
- **Starting and ending HEAD:** `eb050a0010ccd721446f5d2ac4de4863679a9564`;
  no commit or push is part of this phase.
- **Initial worktree:** clean, with no staged, modified, or untracked paths.
- **Phase 5 baseline:** 30 Concepts, 13 indexes, one log, 17 embedded Mermaid
  blocks, zero standalone `.mmd` files, 44 staging files, and 44/44 manifest
  hashes valid.
- **TARGET-047:** `PHASE6_ELIGIBLE`; path
  `development/documentation-maintenance.md`; type `Development Guide`;
  merge group `MERGE-GROUP-011`; eight mapped primary legacy inputs; no
  maintenance diagram dependency. NOC-13, NOC-14, and NOC-17 remain open.
- **Phase 6 staging result:** 31 Concepts, 13 indexes, one log, 17 embedded
  Mermaid blocks, zero standalone `.mmd` files, and 45 staging files.
  One new Concept and the Development index/root log were changed; no existing
  Concept was modified.
- **Evidence findings:** current source/configuration/tests/CI/documentation
  were separated from documented policy, historical maintenance snapshots,
  owner decisions, and unknown production operations. CI covers backend and
  frontend validation only; no tracked general-purpose OKF validator or
  documentation CI gate exists.
- **Safety:** `/okf/`, `/okf-bootstrap/`, `okf/index.yaml`, application source,
  migrations, dependencies, CI workflows, and runtime behavior remain outside
  the change scope. Staging remains non-canonical.
- **Artifacts:** `PHASE_06_OPERATIONS_DELIVERY_GOVERNANCE_MIGRATION.md`,
  `PHASE_06_CONSTRUCTION_STATUS.md`, `PHASE_06_MERGE_LEDGER.md`,
  `PHASE_06_OPERATIONAL_VERIFICATION.md`, and
  `PHASE_06_BUILD_MANIFEST.json`.
- **Final validation:** Google OKF and local metadata/source checks passed with
  31 Concepts, 13 indexes, one log, 14 distinct approved type values, zero
  frontmatter errors, 150 pinned resource lines, and zero missing source
  objects. The link check found 126 internal links, zero broken links, and zero
  concept-index coverage errors. Mermaid fences matched 17/17. The 45/45
  staging manifest hashes matched; the 54-target matrix has 54 unique IDs;
  the merge ledger has 8 rows; and the operational ledger has 19 unique
  verification records. `git diff --check` exited 0 with only expected
  LF/CRLF normalization warnings.
- **Target disposition:** `BUILT_PHASE_3=19`, `BUILT_PHASE_4=3`,
  `BUILT_PHASE_5=8`, `BUILT_PHASE_6=1`, `UPDATED_PHASE_6=0`,
  `DEFERRED_REGENERATE=11`, `BLOCKED_OWNER_DECISION=7`,
  `HISTORICAL_DEFERRED=5`, `INSUFFICIENT_EVIDENCE=0`, and
  `OUT_OF_SCOPE=0`. No deferred merge target remains.
- **Legacy safety:** `/okf/`, `/okf-bootstrap/`, `okf/index.yaml`, application
  source, migrations, dependencies, CI workflows, runtime/configuration paths,
  and staged paths have zero Phase 6 changes. No executable tests ran because
  this phase changed documentation and migration artifacts only.
- **Exact Next Action:** `Phase 7 — Regenerated Knowledge Reconstruction` may
  begin only as a separately authorized phase; preserve the legacy trees and
  canonical cutover boundary.

> **Security Audit Phase 4 override (2026-07-28):** This completed checkpoint
> supersedes earlier Phase 1-3 and product-phase exact-next-action text for the
> current working tree. Phase 3 is committed at `b2e34c37`. Preserve all Phase
> 1-4 hardening. Do not commit, stage, push, reset, clean, or deploy without
> explicit owner authorization.

> **Security Audit Phase 3 override (2026-07-27):** This checkpoint
> supersedes earlier exact-next-action text for the current uncommitted tree.
> Phase 2 is committed at `ff148ff9`. Preserve its session-family, SSRF, RLS,
> and non-superuser database hardening. Do not commit, stage, push, reset, clean,
> or deploy.

### Phase 3 Completed Checkpoint

- **Branch and starting commit:** `security/security-audit-fixes` at
  `ff148ff9 fix(security): complete phase 2 session and RLS hardening`.
- **Objective:** remove browser-readable authentication persistence, restore
  sessions safely from the Phase 2 refresh cookie, coordinate refresh across
  requests/tabs without weakening family reuse detection, and replace preview
  URL credentials with one-time scope-bound WebSocket tickets.
- **Inherited findings:** `SEC-P01-003` (High, Confirmed) and `SEC-P01-009`
  (Medium, Confirmed) are closed. Phase 2 closures for `SEC-P01-006`,
  `SEC-P01-007`, and `SEC-P01-008` are preserved.
- **Browser model:** access tokens are volatile; legacy access/refresh storage
  keys are deleted and ignored. `SessionBootstrap` restores authority from the
  `HttpOnly` cookie before protected rendering and preserves the full requested
  route. Cached identity projections are cleared on failed bootstrap/logout.
- **Refresh design:** one promise single-flights a tab. Web Locks provide the
  primary cross-tab critical section; BroadcastChannel carries transient
  session/logout messages and supplies a bounded election fallback. Without
  either safe primitive, automatic refresh fails closed. Only a stable
  `401 access_token_invalid` response receives one refresh and one replay.
- **Cookie/Origin boundary:** refresh and logout accept the refresh credential
  only from the narrow `HttpOnly`, `SameSite=Lax` cookie. Browser Origin is
  exact when present; null, malformed, duplicate, and untrusted values fail.
  Missing Origin remains valid for non-browser clients. Logout requires no
  bearer token and clears the cookie deterministically.
- **Preview ticket design:** authenticated tenant
  `POST /api/pages/{id}/preview-ticket` requires preview-reader permission.
  Redis stores only a SHA-256-derived key plus user/organization/page/audience/
  authentication-version/time scope, uses a 30-second default and maximum
  60-second TTL, applies a separate issuance limit, fails closed, and consumes
  atomically with `GETDEL`.
- **WebSocket boundary:** `GET /api/preview/{page_id}` is outside bearer/tenant
  middleware, rejects every query string, requires exactly one configured
  canonical Origin plus `zinhar.preview.v1` and one ticket protocol, and
  selects only the stable application protocol. Handshake authorization and
  every configured 30–60-second tick reload current user/version, active
  organization membership/role, preview permission, and page access using a
  fresh tenant context. Denial or unavailable authoritative state closes with
  policy code and a generic reason.
- **Frontend preview lifecycle:** every connection/reconnect requests a new
  ticket. URLs contain no credentials or organization IDs. Reconnect is
  bounded and stops on logout, protocol/payload error, policy rejection, or a
  definitive ticket API response.
- **New findings:** `SEC-P03-001` (Medium, Confirmed) cross-tab refresh-family
  availability race, `SEC-P03-002` (Medium, Confirmed) missing preview Origin
  boundary, and `SEC-P03-003` (Medium, Confirmed) stale open-socket
  authorization. All three are closed with regression evidence. No new
  Critical or High source finding was confirmed.
- **Files created:**
  `backend/src/services/preview_tickets.rs`,
  `backend/tests/docker-compose.phase3.yml`,
  `frontend/src/components/SessionBootstrap.tsx`,
  `frontend/src/components/RequireAuth.test.tsx`,
  `frontend/src/services/authSession.ts`,
  `frontend/src/services/authSession.test.ts`,
  `frontend/src/services/previewSocket.ts`,
  `frontend/src/services/previewSocket.test.ts`, and
  `docs/security/PHASE_03_BROWSER_AUTH_PREVIEW_WS_HARDENING.md`.
- **Files modified:** root environment/production Compose templates and this
  handoff; backend configuration/error/auth/tenant/pages/router/service/RBAC
  modules; frontend route guard/bootstrap/auth/page/API/store/types and API
  tests; `docs/API.md`, `docs/ARCHITECTURE.md`, historical phase compatibility
  notes, and current architecture diagrams/inventories; current OKF
  authentication, session, preview, API, architecture, frontend, domain, risk,
  and diagram documents. `git status` is the authoritative exact path list.
- **No migration:** preview ticket state is ephemeral in Redis; Phase 3 creates
  no database migration.
- **Compatibility:** legacy persisted tokens are deleted/ignored; reload
  requires a valid refresh cookie; body refresh tokens remain unsupported;
  generic `401` and all `403` responses do not trigger refresh; safe automatic
  refresh requires Web Locks or BroadcastChannel; legacy preview query/bearer
  clients stop working; every connection/reconnect needs a fresh ticket; exact
  frontend origins must be configured; Redis 7 `GETDEL` is required.
- **Validation:**
  - `cargo fmt --manifest-path backend/Cargo.toml -- --check` passed.
  - `cargo clippy --manifest-path backend/Cargo.toml --all-targets
    --all-features -- -D warnings` passed.
  - `cargo test --manifest-path backend/Cargo.toml --all-features` passed:
    159 library tests, conditional integration harnesses, and doc tests.
  - focused access-claim error mapping, cookie-Origin, and preview fail-closed
    revalidation tests passed.
  - focused preview-ticket tests passed 6 tests.
  - the same 6 preview-ticket tests passed against isolated disposable Redis
    7, including hash-only/TTL, concurrent one-success consumption, reuse,
    expiry, rate limiting, and unavailable-store failure.
  - `npm --prefix frontend run lint` and `npm --prefix frontend run typecheck`
    passed.
  - `npm --prefix frontend test` passed 8 files and 32 tests.
  - `npm --prefix frontend run build` passed with the existing non-fatal
    large-chunk advisory.
  - `docker compose config --quiet` passed with the existing obsolete-version
    warning. Production Compose interpolation/config validation passed with a
    temporary placeholder env file that was deleted immediately afterward.
  - `git diff --check`, the exact Phase 3 heading/5-diagram checks, the
    changed-file Persian scan, the active token/ticket URL/logging scan, and
    the production-shaped secret-pattern scan passed.
- **Browser verification:** isolated local PostgreSQL/Redis/backend/frontend
  confirmed no protected-content flash, preserved-target login,
  refresh-cookie reload bootstrap, an actual connected one-time-ticket preview
  WebSocket, logout redirect, and no browser console warning/error. An initial
  mixed `localhost`/`127.0.0.1` attempt correctly failed SameSite delivery and
  was repeated successfully with one canonical local origin.
- **Failed/unavailable checks:** no installed pinned dependency advisory
  scanner; no real two-tab browser race harness; no live mid-socket database
  authorization mutation. Unit/focused/live normal-path evidence is recorded
  without treating these unexecuted exhaustive cases as passed.
- **Operational actions:** disposable PostgreSQL/Redis projects used only
  generated local data and were removed with networks/volumes. Temporary
  validation configuration was deleted. No real provider, staging, or
  production service was contacted.
- **Git state:** the Phase 3 tree is uncommitted and unstaged. No commit, push,
  reset, stash, clean, history rewrite, or deployment occurred.
- **Exact Next Action:** begin Phase 4 with tests-first CSP/Trusted Types and
  rich-text browser mutation-corpus hardening while preserving the Phase 2–3
  session, Origin, and one-time preview-ticket boundaries.

### Phase 3 Implementation Checkpoint (Superseded)

- **Branch and starting commit:** `security/security-audit-fixes` at
  `ff148ff9 fix(security): complete phase 2 session and RLS hardening`.
- **Implemented browser model:** access tokens start empty and remain in module
  memory only; both legacy browser token keys are removed. Bootstrap rotates the
  HttpOnly refresh cookie before protected rendering. The store exposes
  `unknown`, `refreshing`, `authenticated`, and `unauthenticated` states and
  preserves the requested route.
- **Refresh coordination:** one promise single-flights a tab. Web Locks provide
  the primary cross-tab critical section; BroadcastChannel carries transient
  session/logout events and supplies a bounded election fallback. Tokens are not
  written to localStorage, sessionStorage, URLs, or storage events.
- **API retry policy:** only a `401` with stable code
  `access_token_invalid` triggers one refresh and one replay. Generic `401`,
  every `403`, refresh itself, and second failures are not retried. Bearer
  credentials are attached only to the configured API origin.
- **Cookie boundary:** refresh and logout allow a missing Origin for non-browser
  clients but reject null, malformed, duplicate, or non-configured browser
  origins. Logout is cookie-authenticated and deterministically clears the
  narrow `/api/auth` cookie.
- **Preview ticket design:** `POST /api/pages/{id}/preview-ticket` requires a
  current tenant-authenticated user and preview-reader permission. It issues a
  random 32-byte base64url ticket with a 30-second default/max-60-second
  lifetime. Redis stores only the SHA-256-derived key and scoped JSON record,
  applies a per-user issuance limit, fails closed, and consumes with `GETDEL`.
- **WebSocket handshake:** `/api/preview/{page_id}` is outside bearer/tenant
  middleware, rejects all query parameters, requires one exact allowed Origin,
  one `zinhar.preview.v1` protocol, and one `zinhar.ticket.<opaque>` protocol.
  Only the stable application protocol is offered back. Consumed records are
  audience/user/organization/page/version/time scoped and rechecked against
  current database state.
- **Open-connection freshness:** every configured 30–60 seconds, the connection
  rechecks active user/authentication version, active organization membership,
  preview permission, and page access. A failed check closes with policy code
  and a generic reason.
- **Frontend preview lifecycle:** each connection and reconnect requests a new
  ticket; URLs never contain credentials or organization IDs. Reconnect uses
  bounded exponential backoff and stops on logout, protocol/payload errors,
  policy rejection, or definitive ticket API rejection.
- **Focused results so far:** frontend auth/preview/page/auth-page test selection
  passed 12 tests; `cargo test preview_tickets` passed 4 unit tests; `cargo check
  --all-targets --all-features` passed. A Redis integration test exists but has
  not yet run with `PHASE3_TEST_REDIS_URL`.
- **Files created so far:**
  `backend/src/services/preview_tickets.rs`,
  `frontend/src/components/SessionBootstrap.tsx`,
  `frontend/src/services/authSession.ts`,
  `frontend/src/services/authSession.test.ts`,
  `frontend/src/services/previewSocket.ts`, and
  `frontend/src/services/previewSocket.test.ts`.
- **No migration:** Phase 3 preview-ticket state is ephemeral in Redis; no
  database schema change is required.
- **Unverified/in progress:** expanded Redis concurrency/expiry evidence,
  cookie/route focused tests, full frontend suite, lint/build, backend
  fmt/clippy/all tests, both Compose configs, browser verification, scans,
  Phase 3 report, API/architecture/security-document updates, final diff review,
  and resource cleanup.
- **Superseded Exact Next Action:** run formatting and focused backend tests, correct any
  failures, then execute the conditional Redis single-use test against an
  isolated disposable Redis 7 container before writing the Phase 3 report and
  completing full validation.

> **Security Audit Phase 2 override (2026-07-26):** This current checkpoint
> supersedes the Phase 1 exact-next-action text for the uncommitted working tree.
> Phase 1 is committed at `eaf90c43`. Phase 2 now includes the pinned,
> dispatch-time SSRF-safe webhook transport; transactional refresh-token
> families and reuse response; authentication-version freshness; explicit
> trusted-proxy CIDRs; cookie-only browser refresh compatibility; migration
> `0027`; non-superuser Compose database initialization; live session, RLS, and
> migration evidence; the English Phase 2 report; full repository validation;
> and cleanup verification. Existing PostgreSQL volumes require owner migration to a verified
> non-superuser, `NOBYPASSRLS` application role. Do not commit or push.

### Phase 2 Current Checkpoint

- **Branch:** `security/security-audit-fixes`
- **Starting commit:** `eaf90c43 fix(security): complete phase 1 audit and baseline hardening`
- **Objective:** close the inherited tenant-webhook SSRF, trusted-proxy,
  access-freshness, and refresh-rotation findings; execute disposable live
  PostgreSQL RLS and migration evidence; document limitations without expanding
  into the deferred frontend/deployment phases.
- **Inherited findings:** `SEC-P01-002`, `SEC-P01-005`, `SEC-P01-007`,
  `SEC-P01-008`, and `SEC-P01-017`. Owner-side `SEC-P01-001` remains unresolved.
- **New finding:** `SEC-P02-001` (High, Confirmed). The tracked Compose
  application role was the PostgreSQL bootstrap superuser and bypassed forced
  RLS. Source initialization is corrected for fresh volumes; existing
  initialized volumes require owner migration or safe recreation.
- **Migration created:** `backend/migrations/0027_security_phase_two_sessions.sql`.
  It adds authentication versions and token-family state. Legacy refresh rows
  are preserved as explicitly revoked one-token families, intentionally signing
  out existing sessions.
- **Files created:**
  `backend/migrations/0027_security_phase_two_sessions.sql`,
  `backend/src/services/outbound_http.rs`,
  `backend/src/services/sessions.rs`,
  `backend/tests/security_phase2_rls.rs`,
  `backend/tests/docker-compose.phase2.yml`,
  `docker/postgres-init-app-user.sh`,
  `frontend/src/services/api.test.ts`, and
  `docs/security/PHASE_02_OUTBOUND_SESSION_RLS_HARDENING.md`.
- **Files modified:** `.env.example`, `env.example`, `backend/Cargo.toml`,
  `backend/Cargo.lock`, `backend/src/config.rs`, `backend/src/main.rs`,
  `backend/src/state.rs`, `backend/src/middleware/auth.rs`,
  `backend/src/middleware/tenant.rs`, `backend/src/routes/auth.rs`,
  `backend/src/routes/mod.rs`, `backend/src/services/jwt.rs`,
  `backend/src/services/mod.rs`, `backend/src/services/security.rs`,
  `backend/src/services/webhooks.rs`, `docker-compose.yml`,
  `docker-compose.prod.yml`, `frontend/src/components/AppShell.tsx`,
  `frontend/src/pages/AuthPage.tsx`,
  `frontend/src/pages/MarketplacePage.test.tsx`,
  `frontend/src/pages/PagesPage.test.tsx`,
  `frontend/src/pages/SettingsPage.tsx`, `frontend/src/services/api.ts`,
  `frontend/src/stores/useAppStore.ts`, `frontend/src/types/api.ts`,
  `docs/API.md`, `docs/ARCHITECTURE.md`, and this handoff.
- **SSRF design:** one reusable webhook client reparses and resolves on every
  dispatch; denies any forbidden or mixed DNS result; supplies the validated
  addresses to the real reqwest connection; preserves the hostname for HTTP/TLS;
  disables redirects, environment proxies, and idle per-host reuse; uses
  3-second connect and 10-second total timeouts; reads at most 64 KiB; and
  returns generic errors. Existing signatures/events are preserved.
- **Session-family design:** login/registration create one absolute-lifetime
  family. Raw random tokens are issued only as HttpOnly cookies and hashes are
  stored. Refresh locks token/family/user rows, rotates once in one transaction,
  links the successor, and commits before cookie issuance. Reused rotated tokens
  compromise and revoke the whole family. Logout revokes only the current
  family and clears the cookie.
- **Access-token design:** JWTs include `ver`. Auth and tenant middleware perform
  one indexed current-user/global-role/version query. Database triggers bump
  `auth_version` for active-state, password-hash, email, and global-role changes;
  reactivation cannot revive an obsolete token. Organization roles remain
  independent in `TenantContext`.
- **Trusted-proxy policy:** `TRUSTED_PROXY_CIDRS` is empty by default, supports
  IPv4/IPv6 CIDRs, and rejects malformed startup configuration. Socket peer is
  authoritative unless it is trusted. Precedence is `Forwarded`,
  `X-Forwarded-For`, then `X-Real-IP`; chains are walked nearest-to-original,
  only configured proxies are removed, and malformed selected headers fall back
  to the socket peer.
- **RLS environment:** uniquely named Docker Compose project
  `zinharcms-phase2-rls-019f9f34`, PostgreSQL 16 Alpine, tmpfs storage,
  generated test-only values, and a dedicated `NOSUPERUSER NOBYPASSRLS`
  application role. No developer volume, remote database, or real provider/user
  data was used.
- **RLS result:** migration 27; 34 tenant-keyed tables; 32 RLS-enabled; 32
  forced-RLS; 118 policies; operation counts 32 INSERT, 25 DELETE, 32 SELECT,
  and 29 UPDATE; 20 recorded matrix cases and 20 passed. The two tenant-keyed
  non-RLS tables are the intentional membership bootstrap/control-plane tables
  `organization_invitations` and `organization_members`.
- **Bypass review:** 17 application call sites were inspected. Beta,
  moderation, admin analytics, and global runtime paths require administrative
  authority; creator analytics/finance paths perform ownership checks before
  protected data use; Stripe verifies the provider signature before bypass;
  catalog paths are authenticated and constrain published/current-tenant data.
- **Findings closed:** `SEC-P01-002`, `SEC-P01-005`, `SEC-P01-007`, and
  `SEC-P01-008`. `SEC-P01-017` is reclassified and closed for the executed
  source/migration boundary after the corrected live matrix.
- **Deferred:** owner response for `SEC-P01-001`; browser access-token
  `localStorage`; preview WebSocket query tokens; OpenAPI security contracts;
  rich-text browser corpus testing; advisory enforcement; operator email
  webhook contract; production ingress/egress/TLS/secrets/backups/logs/runtime
  hardening; exhaustive dynamic coverage of every handler and tenant row state.
- **Compatibility:** migration 27 revokes legacy refresh sessions; access tokens
  without `ver` are rejected; refresh/logout no longer accept JSON refresh
  tokens; every protected request adds an indexed identity lookup; proxy
  deployments must configure exact trusted CIDRs; webhook redirects and
  responses over 64 KiB are rejected; production Compose requires separate
  bootstrap/application credentials; existing volumes do not rerun init scripts.

Validation commands and actual results:

- `cargo fmt --manifest-path backend/Cargo.toml -- --check` — passed.
- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings` — passed.
- `cargo test --manifest-path backend/Cargo.toml --all-features` — passed:
  150 backend unit tests, the conditional integration harness, and doc tests.
  The general run had no Phase 2 database environment, so live behavior is
  evidenced by the separately configured runs below.
- `cargo test --manifest-path backend/Cargo.toml services::outbound_http::tests` —
  passed 12 deterministic outbound tests.
- `cargo test --manifest-path backend/Cargo.toml services::webhooks::tests` —
  passed 3 tests.
- `cargo test --manifest-path backend/Cargo.toml services::sessions::tests -- --nocapture`
  with the disposable Phase 2 database environment — passed 3 live tests.
- `cargo test --manifest-path backend/Cargo.toml --test security_phase2_rls -- --nocapture`
  with the disposable Phase 2 database environment — passed 2 integration
  tests; the RLS test printed 20 of 20 passing matrix cases and the migration
  test completed the `0026` to `0027` upgrade path.
- `npm --prefix frontend run lint` — passed.
- `npm --prefix frontend run typecheck` — passed after correcting the partial
  `Response` test-double type boundary.
- `npm --prefix frontend test` — final run passed 5 files and 17 tests.
- `npm --prefix frontend run build` — passed; Vite retained a non-fatal
  large-chunk advisory.
- `docker compose config --quiet` — passed with the existing obsolete-version
  warning.
- `docker compose -f docker-compose.prod.yml config --quiet` with placeholder
  process-only required values — passed.
- `git diff --check` — passed.
- Phase 2 report exact-heading comparison — passed.
- Changed source/Markdown Persian-range scan — passed with no matches.
- Sensitive-pattern review — only the explicit placeholder connection templates
  in `.env.example` and `env.example` matched; no real credential was identified.

Failed or unavailable checks:

- Expected tests-first Rust compilation and frontend compatibility regressions
  failed before their respective implementations and passed afterward.
- The first live RLS run exposed cross-tenant visibility because the application
  role was a PostgreSQL superuser. It became `SEC-P02-001`; the role boundary was
  corrected and the complete matrix passed on rerun.
- The first final frontend run had one existing Marketplace test exceed its
  five-second timeout. Its focused rerun passed, and the final full suite passed
  all 17 tests.
- The first new API test fixture failed full TypeScript validation because it
  directly cast a partial object to `Response`; the explicit test-double
  boundary was corrected and typecheck passed.
- Rust dependency advisory scanning was unavailable because no pinned scanner
  is installed. Production deployment controls and live provider requests were
  unavailable and not authorized.

Cleanup:

- The temporary migration-upgrade database was dropped only after its generated
  name was verified.
- The Compose project was stopped with its orphan resources removed. Final
  project-label queries returned no container, network, or volume.
- No real webhooks, Stripe calls, emails, metadata calls, arbitrary public
  service calls, staging access, or production access occurred.

**Exact Next Action:** begin Phase 3 by writing failing frontend and backend
regression tests for removal of `zinhar.access_token` from browser-readable
persistent storage and replacement of preview WebSocket query tokens, while
preserving the Phase 2 cookie-family and authoritative-access checks.

> **Security Audit Phase 1 override (2026-07-26):** This completed security
> checkpoint supersedes the older source-release and OKF exact-next-action text
> for the current branch. Preserve all historical content below. Do not create
> a commit or push without explicit user authorization. The exact next action is
> the owner-side SEC-P01-001 deployment inventory/credential response described
> in section 16, followed by an explicitly authorized Phase 2.

## 2. Project Overview

> **OKF Phase Zero override (2026-07-17 13:03 +01:00):** The repository-wide
> knowledge inventory and implementation plan are complete in fourteen English
> reports under `okf-bootstrap/`. This is the active source-of-truth checkpoint;
> older V3 phase actions below are retained as history. No final `okf/` directory
> or product implementation change was created.

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
billing, beta, and GA operations, and V3 Marketplace phases 0.1 through 15. The
current V3 implementation includes installation lifecycle, runtime security
policy, host-owned adapters, one-time purchases/entitlements, feedback/abuse
moderation, read-only analytics, creator-side packaging tooling, Marketplace
security/performance QA gates, beta readiness evidence gates, and launch/GA
operations readiness. Uploaded
package code is still never executed.

## 3. Technology Stack

- **Backend:** Rust 2024, Axum 0.8, Tokio, modular route/service architecture.
- **Frontend:** React 19.2, TypeScript, Vite 7, React Router 8, Zustand, React Hook Form, Zod.
- **Database:** PostgreSQL 16 accessed through SQLx migrations and queries.
- **Authentication:** Argon2id password hashing, HMAC-SHA256 JWT access tokens, hashed refresh tokens in HttpOnly cookies.
- **Authorization:** Global roles plus organization membership roles, tenant middleware, PostgreSQL forced RLS.
- **API:** JSON HTTP APIs, authenticated WebSocket preview, `utoipa` OpenAPI generation.
- **Cache and limits:** Redis 7 for Delivery API cache and rate-limit counters; quota checks use organization plans.
- **Storage:** Local filesystem under `UPLOAD_DIR`; no S3/CDN implementation is present.
- **Testing:** Rust unit/static contract tests, Vitest, Testing Library, ESLint, TypeScript build/typecheck.
- **Build and deployment:** Cargo, npm, Docker Compose, Nginx production frontend image, GitHub Actions CI.
- **Documentation:** Markdown phase/API/architecture documents and 43 Mermaid diagrams.
- **Not implemented:** Durable queue/worker, search service, separately deployed gateway, automatic backups, monitoring vendor, executable Marketplace sandbox/runtime.

## 4. Repository Structure

| Path | Purpose |
| --- | --- |
| `backend/src/` | Rust/Axum routes, middleware, services, plugins, configuration, and application startup. |
| `backend/migrations/` | SQLx migrations through `0027_security_phase_two_sessions.sql`. |
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
| `docs/security/PHASE_02_OUTBOUND_SESSION_RLS_HARDENING.md` | Phase 2 outbound request, session-family, access-freshness, trusted-proxy, live RLS, migration, compatibility, and residual-risk evidence. | Current Phase 2 source and disposable-local-test authority; deployment evidence still outranks it. |
| `docs/security/PHASE_01_SECURITY_BASELINE.md` | Repository-wide Phase 1 attack-surface baseline, stable findings, fixes, validation, limitations, and next-phase recommendation. | Current security-audit authority for branch `security/security-audit-fixes`; live deployment evidence still outranks source assumptions. |
| `README.md` | Current repository scope and quick-start commands through V3 Phase 15. | Current summary; source code and migrations outrank it. |
| `docs/V3_PHASE_SIX.md` | Phase 6 acceptance, install gates, lifecycle rules, update/rollback behavior, and deferred boundaries. | Current Phase 6 authority. |
| `docs/V3_PHASE_SEVEN.md` | Phase 7 permission catalog, sandbox policy, runtime authorization, kill switch, and acceptance. | Current Phase 7 authority. |
| `docs/V3_PHASE_TEN.md` | Phase 10 customer review/rating and abuse-reporting acceptance. | Current Phase 10 authority. |
| `docs/V3_PHASE_ELEVEN.md` | Phase 11 creator analytics and Marketplace admin analytics acceptance, data sources, and deliberate boundaries. | Current Phase 11 authority. |
| `docs/V3_PHASE_TWELVE.md`, `docs/MARKETPLACE_CREATOR_GUIDE.md`, `scripts/marketplace-cli.mjs`, `docs/marketplace-samples/*`, `docs/diagrams/39-marketplace-creator-tooling.mmd` | Phase 12 creator CLI, packaging workflow, submit handoff, guide, samples, and visual traceability. | Current Phase 12 authority. |
| `docs/V3_PHASE_THIRTEEN.md`, `backend/src/services/marketplace_phase_thirteen.rs`, `backend/src/services/marketplace_performance.rs`, `backend/migrations/0026_v3_phase_thirteen_marketplace_qa_performance.sql`, `scripts/marketplace-phase13-load-smoke.ps1`, `docs/diagrams/40-marketplace-qa-performance.mmd` | Phase 13 Marketplace security QA, index/cache performance contracts, load-smoke baseline, and visual traceability. | Current Phase 13 authority. |
| `docs/V3_PHASE_FOURTEEN.md`, `backend/src/services/marketplace_phase_fourteen.rs`, `scripts/marketplace-phase14-beta-readiness.ps1`, `docs/diagrams/41-marketplace-beta.mmd` | Phase 14 Private Creator Beta and Customer Beta evidence gates over existing beta and Marketplace APIs. | Current Phase 14 authority. |
| `docs/V3_PHASE_FIFTEEN.md`, `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md`, `docs/V3_MARKETPLACE_RELEASE_NOTES.md`, `backend/src/services/marketplace_phase_fifteen.rs`, `scripts/marketplace-phase15-ga-check.ps1`, `docs/diagrams/42-marketplace-launch-ga.mmd` | Phase 15 launch readiness, final policy, support workflow, incident/rollback runbook, release notes, GA checks, and visual traceability. | Current Phase 15 authority. |
| `docs/V3_MARKETPLACE_SCOPE.md` | V3 scope lock and MVP/out-of-scope rules. | Current product-scope authority. |
| `docs/V3_MARKETPLACE_GAP_LIST.md` | Resolved and deferred Marketplace gaps by phase. | Current gap/status record; verify against runtime. |
| `docs/V3_MARKETPLACE_POLICY.md` and `docs/V3_PRODUCT_TAXONOMY.md` | Review, moderation, product classification, and safety policy. | Current policy authority. |
| `docs/API.md` | Runtime route boundaries and Marketplace endpoint documentation. | Current, with older Marketplace routes manually documented. |
| `docs/ARCHITECTURE.md` | Runtime containers, tenant boundaries, RLS, and Marketplace architecture. | Updated through Phase 15 launch/GA readiness. |
| `docs/diagrams/ARCHITECTURE_AUDIT.md`, `TRACEABILITY.md`, `FILE_EVIDENCE_INDEX.md`, `33-marketplace-installation-lifecycle.mmd`, `34-marketplace-security-runtime.mmd`, `35-marketplace-runtime-adapters.mmd`, `36-marketplace-finance-lifecycle.mmd`, `37-marketplace-feedback-abuse.mmd`, `38-marketplace-analytics.mmd`, `39-marketplace-creator-tooling.mmd`, `40-marketplace-qa-performance.mmd`, `41-marketplace-beta.mmd`, `42-marketplace-launch-ga.mmd` | Evidence links and visual Marketplace implementation state. | Updated through Phase 15; static Mermaid validation is available, but no Mermaid parser is installed. |
| `D:\All projects\Zinhar_Doc\version_3_marketplace_proposal.html` | Original V3 Marketplace proposal and future lifecycle goals. | Planning authority; current migrations/routes/tests supersede it for implementation status. |
| `D:\All projects\Zinhar_Doc\version_2_proposal.html` | V2 SaaS/organization/billing proposal. | Historical planning authority for V2 dependencies. |
| `D:\All projects\Zinhar_Doc\headless_cms_proposal_polished.html` | Original CMS proposal. | Historical baseline; current repository documentation and code are newer. |

The proposals describe the complete future Marketplace lifecycle, including paid
products and executable/runtime concepts. Phase 7 established the permission and
containment boundary; Phase 8 supplies host-owned Component Pack, Template, and
public Hook adapters. Phase 9 supplies one-time purchases/entitlements and payout
onboarding, Phase 10 supplies customer feedback/abuse reporting, Phase 11
supplies read-only analytics, Phase 12 supplies creator-side packaging tooling
plus sample packages, Phase 13 supplies security QA and performance gates,
Phase 14 supplies Private Creator Beta and Customer Beta evidence/readiness
gates over existing APIs, and Phase 15 supplies launch readiness, incident
response, rollback, release notes, monitoring/support planning, and GA check
coverage over existing APIs.
External execution, runtime error telemetry, automated payout transfer
execution, and arbitrary package execution remain deferred.

## 6. Current Objective

> **Google OKF Phase 5 current objective (2026-08-07):** The Data, API,
> Security, and Domain staging migration is complete for eight source-backed
> targets. Preserve the isolated staging boundary, keep `TARGET-047` for Phase
> 6, and leave `/okf/`, `okf-bootstrap/`, `okf/index.yaml`, and all executable
> source outside scope. No cutover, deletion, redirect, staging, commit, or
> push is authorized by this checkpoint.

> **Security Audit Phase 1 override (2026-07-26):** The requested Phase 1
> repository-wide audit, low-risk hardening, baseline document, and local
> validation are complete in the working tree. No commit or push is authorized.
> Immediate remaining work is owner-side response for any administrator created
> by the former deterministic bootstrap path; later code work begins only as a
> separately authorized Phase 2.

> **OKF Phase Zero override (2026-07-17):** The user explicitly requested a
> repository-wide, evidence-based OKF bootstrap analysis. Create only the
> required English analytical reports under `okf-bootstrap/`, do not create the
> final `okf/` directory, and do not change product behavior. The current Git
> state at `61ed3b38` supersedes the older uncommitted Phase 15 wording below.

> **Phase 15 override (2026-07-12 16:27):** Phase 14 is committed at `87bc6d0e`.
> The active objective is V3 Marketplace Phase 15: 15.1 Launch Readiness
> and 15.2 General Availability. Implementation and local validation are
> complete as an operational launch/GA readiness layer over existing
> Marketplace APIs and documentation. The remaining action is user-authorized
> review/stage/commit, plus optional live GA smoke only when a safe target
> environment, token, organization, and approved products are available.

> **Phase 14 override (2026-07-12 11:38):** Phase 13 is committed at `70d8f12`.
> The active objective is V3 Marketplace Phase 14: 14.1 Private Creator Beta
> and 14.2 Customer Beta. Implementation and local validation are complete as a
> read-only beta evidence/readiness layer over existing V2 beta and V3
> Marketplace APIs. The remaining action is user-authorized review/stage/commit.

> **Phase 13 override (2026-07-12 10:52):** Phase 12 is committed at `19f6673`.
> The active objective is V3 Marketplace Phase 13: 13.1 Marketplace security QA
> and 13.2 load/performance. Implementation, local backend validation, SQLx
> migration `0026`, and release-mode catalog/search/listing performance smoke
> are complete. The remaining action is user-authorized review/stage/commit.

> **Phase 12 override (2026-07-12 09:24):** Phase 11 is committed at `beb4cf2`.
> The active objective is V3 Marketplace Phase 12: 12.1 CLI/SDK packaging and
> 12.2 documentation/sample packages. Implementation and local validation are
> complete. The remaining action is user-authorized review/stage/commit, or an
> optional live API submit smoke if the user provides/authorizes a safe approved
> creator listing.

> **Phase 11 override (2026-07-11 18:58):** Phase 10 is committed at `e77e2f7`.
> The active objective is V3 Marketplace Phase 11: 11.1 creator analytics and
> 11.2 Marketplace admin analytics. The implementation and local validation are
> complete, and authenticated live API smoke passed on 2026-07-12 after Docker
> became available. The remaining action is user-authorized review/stage/commit.

> **Phase 10 override (2026-07-11 14:54):** Phase 9 is committed at `dffe515`.
> The active objective is V3 Marketplace Phase 10: 10.1 customer rating/review
> after install or purchase, and 10.2 abuse reporting with evidence, moderation
> queue, and critical internal notification. The implementation and validation
> are complete; the remaining action is user-authorized review/stage/commit.

The historical Phase 8 objective below is superseded by the Phase 15 override.

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

### Security Audit and Hardening Phase 1 checkpoint (2026-07-26)

- [x] Reconciled `AGENTS.md`, this handoff, branch/status/diff, recent commits,
  repository state, and relevant persistent lessons before changing code.
- [x] Mapped backend/frontend architecture, 168 handler-method endpoints, public
  static media, preview WebSocket, authentication/session, global/tenant RBAC,
  RLS, Redis, file/package storage, webhooks, Stripe, Marketplace, Compose, CI,
  scripts, dependency/tooling, logging, and error surfaces.
- [x] Scanned the current tracked tree and Git history for selected
  provider-token and private-key signatures without copying values; no real
  committed secret was confirmed by the available patterns.
- [x] Replaced deterministic privileged bootstrap with an explicit validated
  configuration pair; public registration now always receives the author role,
  and the login UI no longer prefills a deterministic identity/password.
- [x] Restricted public static uploads to generated media paths, excluding the
  Marketplace package namespace; unit and router-level allow/deny tests pass.
- [x] Applied constant-time JWT signature verification, generic internal and
  readiness error responses, placeholder-secret rejection, CSPRNG webhook-secret
  fallback, loopback-only development service ports, and secure production
  refresh-cookie Compose default.
- [x] Added `docs/security/PHASE_01_SECURITY_BASELINE.md` with the exact required
  sections, stable `SEC-P01-*` IDs, severities, confidence, evidence, status,
  unverified risks, validation, limitations, and next-phase recommendation.
- [x] Updated active setup/phase/diagram documentation and the external
  recurring-mistakes log without adding secret values to repository documents.
- [x] Completed the final validation matrix recorded in section 11.

### Phase 15 checkpoint override (2026-07-12 16:27)

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth supersedes
  the stale handoff: Phase 14 is committed at `87bc6d0e`, and the working tree
  was clean before Phase 15 implementation.
- Extracted Phase 15 from the V3 proposal: 15.1 Launch Readiness and 15.2
  General Availability.
- Implemented Phase 15 as an operational launch/GA readiness layer over existing
  Marketplace APIs rather than a new schema, migration, or parallel launch API.
- Added `backend/src/services/marketplace_phase_fifteen.rs` with static backend
  contract tests that verify Phase 15 docs, final policy, operations runbook,
  release notes, GA check script, diagram, and existing install/rollback/report/
  finance/admin analytics/beta blocker route coverage.
- Added `scripts/marketplace-phase15-ga-check.ps1`, a GA readiness script that
  runs backend Marketplace checks, frontend lint/build gates, and optional
  read-only live smoke against `/health`, `/ready`, installations, purchases,
  reports, admin analytics, and beta blocker endpoints.
- Added `docs/V3_PHASE_FIFTEEN.md`,
  `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md`,
  `docs/V3_MARKETPLACE_RELEASE_NOTES.md`, and
  `docs/diagrams/42-marketplace-launch-ga.mmd`; updated README, API,
  architecture, policy, gap list, repository inventory, diagram status map,
  traceability, evidence index, architecture audit, and diagram catalog.
- Validation passed: `cargo fmt --manifest-path backend/Cargo.toml`,
  `cargo test --manifest-path backend/Cargo.toml marketplace_phase_fifteen`,
  PowerShell script parse check, `git diff --check`, Mermaid diagram count
  check, and `cargo test --manifest-path backend/Cargo.toml marketplace`
  (79 tests).
- No migration, database mutation, live API smoke, staged files, commit, reset,
  or uploaded package execution was performed.

### Phase 14 checkpoint override (2026-07-12 11:38)

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth supersedes
  the stale handoff: Phase 13 is committed at `70d8f12`, and the working tree
  was clean before Phase 14 implementation.
- Extracted Phase 14 from the V3 proposal: 14.1 Private Creator Beta and 14.2
  Customer Beta.
- Implemented Phase 14 as an operational beta evidence/readiness layer over
  existing APIs rather than a new schema or parallel Marketplace beta route
  group.
- Added `backend/src/services/marketplace_phase_fourteen.rs` with static
  backend contract tests that verify the Phase 14 docs, readiness script,
  diagram, and existing beta/Marketplace/finance/analytics routes.
- Added `scripts/marketplace-phase14-beta-readiness.ps1`, a read-only
  readiness script that queries existing beta dashboard/feedback/blockers,
  Marketplace installations/purchases/reports, and creator/admin analytics. It
  supports `-ReportOnly` and `-AdminMode`.
- Added `docs/V3_PHASE_FOURTEEN.md` and
  `docs/diagrams/41-marketplace-beta.mmd`; updated README, API, architecture,
  gap list, repository inventory, diagram status map, traceability, evidence
  index, architecture audit, and diagram catalog.
- Validation passed: `cargo fmt --manifest-path backend/Cargo.toml`,
  `cargo test --manifest-path backend/Cargo.toml marketplace_phase_fourteen`,
  PowerShell script parse check, `git diff --check`, and
  `cargo test --manifest-path backend/Cargo.toml marketplace` (75 tests).
- No migration, database mutation, live API smoke, staged files, commit, reset,
  or uploaded package execution was performed.

### Phase 13 checkpoint override (2026-07-12 10:52)

- Phase 13.1 is implemented: backend security QA covers the main Marketplace
  abuse paths from the proposal: IDOR on creator/listing access, permission
  bypass, malicious package blocking, refund abuse/idempotency, and review/abuse
  reporting guards.
- Phase 13.2 is implemented: migration
  `0026_v3_phase_thirteen_marketplace_qa_performance.sql` adds catalog/search,
  latest-version, active-install, entitlement, and checkout indexes; catalog and
  listing-detail responses emit a private bounded cache policy.
- Added `backend/src/services/marketplace_phase_thirteen.rs`,
  `backend/src/services/marketplace_performance.rs`,
  `scripts/marketplace-phase13-load-smoke.ps1`,
  `docs/V3_PHASE_THIRTEEN.md`, and
  `docs/diagrams/40-marketplace-qa-performance.mmd`.
- Updated README, API, architecture, gap list, repository inventory, diagram
  status map, traceability, evidence index, audit, and diagram catalog docs for
  Phase 13.
- Local validation passed: `cargo fmt --manifest-path backend/Cargo.toml`,
  `cargo test --manifest-path backend/Cargo.toml marketplace_phase_thirteen`,
  `cargo test --manifest-path backend/Cargo.toml marketplace_performance`,
  `cargo test --manifest-path backend/Cargo.toml marketplace`, and
  `git diff --check`.
- Live/release-mode performance validation passed against local Docker
  PostgreSQL/Redis with SQLx migration `26` applied: catalog P95 144 ms, catalog
  search P95 195 ms, and listing detail P95 162 ms. A temporary
  `phase13-smoke-*` fixture was created only for listing-detail smoke and then
  removed; verification showed zero remaining fixture rows.
- Debug-build smoke was intentionally not used as the final performance gate:
  it exceeded the absolute latency budgets on local Windows, while `/health`
  showed the debug/runtime environment itself had high baseline overhead.
- No files were staged or committed. Uploaded Marketplace package code remains
  unexecuted.

### Phase 12 checkpoint override (2026-07-12 09:24)

- Phase 12.1 is implemented: `scripts/marketplace-cli.mjs` provides a
  dependency-free Node CLI with `validate`, `pack`, and `submit` commands. The
  CLI validates Marketplace manifests, package file trees, entry points, assets,
  permissions, compatibility, Phase 8 adapter declarations, and security
  findings before upload.
- The `pack` command creates ZIP artifacts with SHA-256 reporting under the
  ignored `marketplace-dist/` output directory by default. Generated ZIPs were
  verified readable with `tar -tf` and by validating the generated archives with
  the CLI.
- The `submit` command targets the existing
  `POST /api/marketplace/listings/{listing_id}/versions/upload` API and sends
  the same multipart `manifest` and `file` fields used by the frontend upload
  flow. It requires token, organization id, and listing id from flags or
  environment variables.
- Phase 12.2 is implemented: added `docs/V3_PHASE_TWELVE.md`,
  `docs/MARKETPLACE_CREATOR_GUIDE.md`, Component Pack and Integration Plugin
  sample packages under `docs/marketplace-samples/`, and Mermaid diagram
  `docs/diagrams/39-marketplace-creator-tooling.mmd`.
- Updated README, API, architecture, gap list, diagram status map, traceability,
  evidence index, and diagram catalog to include Phase 12.
- Validation passed: `node --check scripts/marketplace-cli.mjs`, CLI help,
  sample-package validation for both samples, sample-package packing for both
  samples, generated-archive validation for both ZIPs, `tar -tf` archive listing,
  package/sample JSON parsing, `git diff --check`, and 40-file Mermaid static
  validation. The Integration Plugin sample reports one expected medium finding
  for `webhook.send`; this is non-blocking and matches review-policy behavior.
- No backend runtime code, migration, database row, stage, commit, reset, or
  destructive action was performed for Phase 12. Uploaded package code remains
  unexecuted.

### Phase 11 checkpoint override (2026-07-11 18:58)

- Phase 11.1 is implemented: creator owners can read product analytics only for
  their own creator profile via `/api/marketplace/creators/{creator_id}/analytics`.
  Metrics include listing count, total installs, active installs, purchase
  attempts, completed/refunded purchases, gross revenue, creator revenue,
  conversion rate, ratings, reports, and persisted error signals.
- Phase 11.2 is implemented: global admins/super admins can read internal
  Marketplace health analytics via `/api/marketplace/analytics/admin`. Metrics
  include 30-day submission count/rate, average approval time, installs, refunds,
  reports, critical reports, blocked packages, and a ranked risky/repetitive
  product list.
- No new migration was required. Phase 11 aggregates existing tables from phases
  1 through 10: installs, purchases, revenue ledger, product reviews, abuse
  reports, versions/package risk, submissions, and review events.
- Frontend Marketplace UI now renders a creator analytics panel and a global-admin
  Marketplace health/risk panel, with API methods, TypeScript types, translations,
  and Phase 11 test coverage.
- Added `backend/src/routes/marketplace_analytics.rs`,
  `backend/src/services/marketplace_analytics.rs`, `docs/V3_PHASE_ELEVEN.md`,
  and `docs/diagrams/38-marketplace-analytics.mmd`; updated API, architecture,
  gap, ambiguity, traceability, diagram index, README, frontend API/types/UI/tests,
  and OpenAPI registration.
- Validation passed: `cargo fmt -- --check`, `cargo test marketplace_analytics`,
  `cargo test --all-features` (105 backend tests plus doc tests), `npm run lint`,
  `npm run typecheck`, `npm test -- MarketplacePage`, full `npm test` (3 files,
  14 tests), `npm run build`, `git diff --check`, and 39-file Mermaid static
  validation. Frontend Vitest/build required sandbox escalation because esbuild
  otherwise failed with `spawn EPERM`; the Vite >500 kB chunk warning remains
  pre-existing/non-blocking.
- Live PostgreSQL/API smoke passed on 2026-07-12: Docker PostgreSQL/Redis were
  healthy; backend `/health` returned 200; `/ready` returned 200 with PostgreSQL
  and Redis reachable; `/openapi.json` contained both Phase 11 analytics paths.
  Authenticated creator-owner analytics returned 200, author access to admin
  analytics returned 403, global-admin analytics returned 200, and non-owner
  access to creator analytics returned 403. The temporary test user/creator data
  was removed, the backend process was stopped, and no stage or commit was
  performed.

### Phase 10 final checkpoint override (2026-07-11 14:54)

- Phase 10.1 is implemented: owner/admin customer reviews are gated by
  organization install or completed purchase, include 1-5 rating and review text,
  return to `pending` on resubmission, and publish only through global-admin
  moderation. Published reviews feed catalog averages and listing detail.
- Phase 10.2 is implemented: authenticated organization members can submit abuse
  reports with violation type, severity, description, and JSON-object evidence;
  global admins get pending review and abuse queues and can investigate, resolve,
  dismiss, publish, or reject as applicable.
- Critical abuse reports now create both the report/audit record and a persisted
  unread `marketplace_internal_notifications` admin notification in the same
  transaction; resolving or dismissing acknowledges the notification.
- Follow-up hardening is included: review list responses are sanitized for
  tenant/catalog users, abuse queues show only actionable statuses, Unicode text
  length is counted by characters, and stale diagram/doc claims were corrected.
- Added/updated migrations `0024` and `0025`, backend feedback service/routes,
  OpenAPI registration, Marketplace frontend forms/admin queues/API types, Phase
  10 documentation, and Mermaid diagram `37`.
- Validation passed after the final changes: backend format check, backend tests
  (102 tests plus doc tests), frontend lint/typecheck/Vitest/build, `git
  diff --check`, 38-file Mermaid static validation, Docker PostgreSQL/Redis
  readiness, SQLx migrations `24` and `25`, and backend `/health`, `/ready`, and
  `/openapi.json` smoke with all six Phase 10 paths present.

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

- [ ] Owner-side SEC-P01-001 response for every existing deployment.
  - **Missing verification:** no deployment/account inventory, login/audit
    history, or secret-manager access was available.
  - **Required action:** identify and disable/rotate any administrator created
    by the former deterministic bootstrap path, review relevant activity, and
    decide whether Git-history rewriting is warranted.
- [ ] Live PostgreSQL tenant/RLS and bypass matrix (SEC-P01-017).
  - **Missing verification:** source/static policy tests passed, but this phase
    did not start a populated database or execute cross-tenant requests.
- [ ] Rust advisory assessment (SEC-P01-018).
  - **Missing verification:** `cargo-audit` and `cargo-deny` were unavailable.
- [ ] Deployment/edge/email-webhook/rich-text runtime checks (SEC-P01-019 through
  SEC-P01-021).
  - **Missing verification:** no live ingress, secret manager, egress policy,
    receiver, or browser E2E environment was in scope.

- [ ] Optional Phase 15 live GA smoke against a safe target environment.
  - **Files:** `scripts/marketplace-phase15-ga-check.ps1`, `docs/V3_PHASE_FIFTEEN.md`, `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md`.
  - **Missing verification:** an authenticated production/staging-like environment with a safe organization, approved products, and support/incident evidence.
  - **Recommended validation:** run the GA check script with `-ApiBaseUrl`, `-AccessToken`, `-OrganizationId`, and optionally `-AdminMode`; do not reset data or create destructive fixtures.
  - **Reason:** Phase 15 local validation verifies the operational contract, script syntax, route coverage, and documentation. A meaningful GA pass/fail requires real launch data and credentials.

- [ ] Mermaid parser/render validation.
  - **Files:** `docs/diagrams/*.mmd`
  - **Missing verification:** no `mmdc`/Mermaid parser dependency is installed in the repository.
  - **Recommended validation:** use an approved Mermaid renderer in a separate documentation environment.
  - **Reason:** only static declaration/fence validation was available.

## 9. Work in Progress

### Google OKF Phase 5 current work state

Phase 5 implementation and documentation validation are complete. There is no
partially implemented product code. The uncommitted worktree contains only
the Phase 5 report, construction matrix, merge ledger, security/data ledger,
manifest, eight staging Concepts, four domain-index updates, the staging log
update, and this handoff update. No paths are staged.

### Security Audit Phase 1 active checkpoint override

Phase 1 implementation, documentation, and local validation are complete. There
is no partially implemented Phase 1 product code. The working tree is
intentionally uncommitted. The next action requires owner/deployment authority,
not additional source changes in this phase.

### Phase 15 active checkpoint override

Phase 15 implementation and local validation are complete. The only active work
is handoff maintenance, review, and user-authorized staging/commit. Optional live
GA smoke is deferred until a safe authenticated environment is available.

### Active item

Review the uncommitted Phase 15 diff and commit only after explicit user
authorization.

### Exact stopping point

The Phase 15 implementation stopped after the launch readiness/GA docs, final
policy update, operations runbook, release notes, GA check script, static backend
contract tests, Mermaid diagram `42`, traceability/index updates, and local
validation passed on top of commit `87bc6d0e`.

### Partially modified files

No partially implemented Phase 15 product code remains. All current uncommitted
files are Phase 15 docs, static contract tests, script, diagram/index updates,
and this handoff update.

### Incomplete implementation markers

- Optional live GA smoke was not run because it requires real target
  environment credentials and launch data.
- Uploaded Marketplace package code remains intentionally unexecuted.
- No Phase 15 migration or new GA/launch API was added; the phase uses existing
  operational surfaces.

## 10. Current Git and Filesystem State

### Actual state at Google OKF Phase 5 checkpoint

- `HEAD` is `6f51612c5716c7d9c3365553811053fd24a03947` on
  `security/security-audit-fixes`, tracking the matching origin branch.
- The worktree was clean at Phase 5 start and has no staged paths now.
- Current changed/untracked paths are limited to `HANDOFF.md`, the five Phase 5
  migration artifacts under `docs/okf-migration/`, and the eight new Concepts
  plus four index/log updates under the isolated staging root.
- `okf/`, `okf-bootstrap/`, `okf/index.yaml`, Phase 4 artifacts, backend,
  frontend, database migrations, dependency, CI, and runtime/configuration paths
  have zero Phase 5 diff. No files were deleted, moved, renamed, or converted.
- The cross-project mistakes log was reviewed at
  `D:\All projects\Mistakes\mistakes.md` and was not changed during Phase 5.

### Actual state at Security Audit Phase 1 checkpoint

- `HEAD` is `64d780b6` on `security/security-audit-fixes`, tracking
  `origin/security/security-audit-fixes`.
- The working tree was clean at Phase 1 start. Nothing is staged, committed,
  pushed, deleted, reset, or cleaned by this phase.
- Modified tracked files are `.env.example`,
  `.github/workflows/backend-ci.yml`, `HANDOFF.md`, `README.md`,
  `backend/src/config.rs`, `backend/src/error.rs`, `backend/src/main.rs`,
  `backend/src/routes/auth.rs`, `backend/src/routes/mod.rs`,
  `backend/src/services/jwt.rs`,
  `backend/src/services/rbac.rs`, `docker-compose.prod.yml`,
  `docker-compose.yml`, `docs/PHASE_ONE.md`, `docs/V2_PHASE_ONE.md`,
  `docs/V2_PHASE_ZERO.md`,
  `docs/diagrams/05-local-development-runtime.mmd`,
  `docs/diagrams/06-production-deployment.mmd`,
  `docs/diagrams/07-backend-component-architecture.mmd`,
  `docs/diagrams/31-observability-and-failure-recovery.mmd`,
  `docs/diagrams/REPOSITORY_INVENTORY.md`, `env.example`,
  `frontend/src/pages/AuthPage.tsx`, and
  `frontend/src/pages/SettingsPage.tsx`.
- New untracked files are
  `docs/security/PHASE_01_SECURITY_BASELINE.md` and
  `frontend/src/pages/AuthPage.test.tsx`.
- The ignored local `.env` was not modified. It still contains a JWT
  placeholder and will be rejected by the hardened configuration until the
  operator replaces it. Its value is intentionally absent from this handoff.
- `D:\All projects\Mistakes\mistakes.md` was updated outside the repository as
  required by `AGENTS.md`; it is not part of Git status.

### Actual state at Phase 15 checkpoint

- `HEAD` is `87bc6d0e` (`feat(marketplace): add v3 phase 14 beta readiness`) on
  `main`, matching `origin/main` at inspection time.
- No files are staged, deleted, reset, or committed for Phase 15.
- Modified tracked files are `README.md`, `backend/src/services/mod.rs`,
  `docs/API.md`, `docs/ARCHITECTURE.md`, `docs/V3_MARKETPLACE_GAP_LIST.md`,
  `docs/V3_MARKETPLACE_POLICY.md`,
  `docs/diagrams/00-implementation-status-map.mmd`,
  `docs/diagrams/01-project-scope.mmd`,
  `docs/diagrams/32-end-to-end-traceability.mmd`,
  `docs/diagrams/ARCHITECTURE_AUDIT.md`,
  `docs/diagrams/FILE_EVIDENCE_INDEX.md`,
  `docs/diagrams/README.md`, `docs/diagrams/REPOSITORY_INVENTORY.md`,
  `docs/diagrams/TRACEABILITY.md`, and this handoff update.
- New Phase 15 files are `backend/src/services/marketplace_phase_fifteen.rs`,
  `docs/V3_PHASE_FIFTEEN.md`,
  `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md`,
  `docs/V3_MARKETPLACE_RELEASE_NOTES.md`,
  `docs/diagrams/42-marketplace-launch-ga.mmd`, and
  `scripts/marketplace-phase15-ga-check.ps1`.
- Phase 15 added no SQL migration, no new GA/launch API, and performed no
  database mutation. Live API smoke was not run because it requires real
  production/staging-like launch data and credentials.

### Actual state at Phase 12 checkpoint

- `HEAD` is `beb4cf2` (`feat(marketplace): complete v3 phase 11 creator and
  admin analytics`) on `main`, matching `origin/main` at inspection time.
- No files are staged, deleted, reset, or committed for Phase 12.
- Modified tracked files are `.gitignore`, `package.json`, `README.md`,
  `docs/API.md`, `docs/ARCHITECTURE.md`, `docs/V3_MARKETPLACE_GAP_LIST.md`,
  `docs/diagrams/00-implementation-status-map.mmd`,
  `docs/diagrams/32-end-to-end-traceability.mmd`,
  `docs/diagrams/ARCHITECTURE_AUDIT.md`,
  `docs/diagrams/FILE_EVIDENCE_INDEX.md`, `docs/diagrams/README.md`,
  `docs/diagrams/TRACEABILITY.md`, and this handoff update.
- New Phase 12 files are `scripts/marketplace-cli.mjs`,
  `docs/V3_PHASE_TWELVE.md`, `docs/MARKETPLACE_CREATOR_GUIDE.md`,
  `docs/diagrams/39-marketplace-creator-tooling.mmd`,
  `docs/marketplace-samples/component-pack/*`, and
  `docs/marketplace-samples/integration-plugin/*`.
- Generated ZIPs were written under `marketplace-dist/`; this directory is
  ignored by Git and is not source of truth.

### Actual state at Phase 11 checkpoint

- `HEAD` is `e77e2f7` (`feat(marketplace): complete v3 phase 10 feedback and
  abuse reporting`) on `main`, matching `origin/main` at inspection time.
- No files are staged, deleted, reset, or committed for Phase 11.
- Modified tracked files are Phase 11 implementation/documentation/UI/test files
  plus this handoff update.
- New Phase 11 files are `backend/src/routes/marketplace_analytics.rs`,
  `backend/src/services/marketplace_analytics.rs`, `docs/V3_PHASE_ELEVEN.md`,
  and `docs/diagrams/38-marketplace-analytics.mmd`.
- Modified Phase 11 files are `README.md`, `backend/src/routes/mod.rs`,
  `backend/src/services/mod.rs`, `docs/API.md`, `docs/ARCHITECTURE.md`,
  `docs/V3_MARKETPLACE_GAP_LIST.md`, `docs/diagrams/00-implementation-status-map.mmd`,
  `docs/diagrams/01-project-scope.mmd`, `docs/diagrams/19-marketplace-data-model.mmd`,
  `docs/diagrams/32-end-to-end-traceability.mmd`, `docs/diagrams/AMBIGUITIES.md`,
  `docs/diagrams/ARCHITECTURE_AUDIT.md`, `docs/diagrams/FILE_EVIDENCE_INDEX.md`,
  `docs/diagrams/README.md`, `docs/diagrams/TRACEABILITY.md`,
  `frontend/src/i18n/messages.ts`, `frontend/src/pages/MarketplacePage.test.tsx`,
  `frontend/src/pages/MarketplacePage.tsx`, `frontend/src/services/api.ts`, and
  `frontend/src/types/api.ts`.
- Phase 11 live DB/API smoke passed on 2026-07-12 against local Docker
  PostgreSQL/Redis. The temporary backend process was stopped; Docker
  PostgreSQL/Redis remained healthy at the end of the smoke.

### Actual state at Phase 10 final checkpoint

- `HEAD` is `dffe515` (`feat(marketplace): complete v3 phase 9 monetization`) on
  `main`, matching `origin/main` at inspection time.
- No files are staged, deleted, reset, or committed for Phase 10.
- Modified tracked files are Phase 10 implementation/documentation/UI files plus
  this handoff update.
- New Phase 10 files are `backend/migrations/0024_v3_phase_ten_ratings_abuse.sql`,
  `backend/migrations/0025_v3_phase_ten_internal_notifications.sql`,
  `backend/src/services/marketplace_feedback.rs`, `docs/V3_PHASE_TEN.md`, and
  `docs/diagrams/37-marketplace-feedback-abuse.mmd`.
- Docker PostgreSQL/Redis were started for validation and remain healthy; the
  temporary backend process was stopped after smoke verification.

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

### Google OKF Phase 5 validation results (2026-08-07)

- The Phase 4 baseline manifest was rechecked: 31 of 36 staged file hashes
  remained unchanged and the five changed paths were exactly the four selected
  domain indexes plus the staging log.
- Final staging validation confirmed 30 Concepts, 13 indexes, one log, 17
  embedded Mermaid blocks, zero standalone `.mmd` files, zero frontmatter/type/
  source/link/index errors, and 44 staging files.
- `PHASE_05_BUILD_MANIFEST.json` parsed successfully and reconciled all 44
  staging-file SHA-256 entries with zero mismatches.
- `PHASE_05_CONSTRUCTION_STATUS.md` contains 54 unique target IDs with no
  missing rows. Counts are 19 `BUILT_PHASE_3`, 3 `BUILT_PHASE_4`, 8
  `BUILT_PHASE_5`, 11 `DEFERRED_REGENERATE`, 7
  `BLOCKED_OWNER_DECISION`, 1 `OUT_OF_SCOPE`, and 5
  `HISTORICAL_DEFERRED`.
- The Phase 5 merge ledger covers every primary source category path and
  records shared group routing; the security/data ledger contains 15 stable
  `P5-VERIFY-*` entries.
- Source-path validation confirmed every new Phase 5 provenance URL points to
  an existing path at `6f51612c5716c7d9c3365553811053fd24a03947`. Carried
  Phase 3/4 Concepts retain their own immutable construction provenance.
- Legacy safety scans returned no diff for `okf/`, `okf-bootstrap/`,
  `okf/index.yaml`, backend/frontend/migrations/dependencies/CI/runtime paths;
  no paths are staged.
- `git diff --check` exited 0 with only expected LF/CRLF normalization
  warnings. No executable tests were run because Phase 5 changed only
  documentation and staging artifacts.

### Google OKF Phase 4 validation results (2026-08-07)

- Phase 3 baseline validation confirmed 19 Concepts, 13 indexes, one log, 13
  embedded Mermaid blocks, zero standalone `.mmd` files, and 33 manifest-hash
  entries with zero mismatches.
- Final staging validation confirmed 22 Concepts, 13 indexes, one log, 13
  embedded Mermaid blocks, zero standalone `.mmd` files, zero frontmatter/type/
  source errors, zero broken links, zero parent traversals, and zero orphan
  Concepts.
- `PHASE_04_BUILD_MANIFEST.json` parsed successfully and reconciled all 36
  staging-file SHA-256 entries with zero mismatches.
- `PHASE_04_CONSTRUCTION_STATUS.md` contains 54 unique target IDs with no
  missing rows. Final counts are 19 `BUILT_PHASE_3`, 3 `BUILT_PHASE_4`, 9
  `DEFERRED_MERGE`, 11 `DEFERRED_REGENERATE`, 7
  `BLOCKED_OWNER_DECISION`, and 5 `HISTORICAL_DEFERRED`.
- `PHASE_04_MERGE_LEDGER.md` contains 72 unique source inputs: 10 G002, 31
  G003, and 31 G008.
- `git diff --check` passed with only expected LF/CRLF normalization warnings.
- No executable tests were run because Phase 4 changed only documentation and
  staging artifacts; no application, database, dependency, CI, or runtime
  source changed.

### Security Audit Phase 1 validation results (2026-07-26)

- `cargo fmt --manifest-path backend/Cargo.toml -- --check`: passed.
- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`: passed.
- `cargo test --manifest-path backend/Cargo.toml --all-features`: passed, 124
  tests, 0 failed.
- Focused config, registration-role, JWT-tampering, internal-error, upload-path,
  upload-router, and AuthPage regression tests: passed.
- `npm --prefix frontend run lint`: passed.
- `npm --prefix frontend run typecheck`: passed.
- `npm --prefix frontend test`: passed, 15 tests, 0 failed.
- `npm --prefix frontend run build`: passed; Vite retained the existing
  >500-kB chunk warning.
- `npm --prefix frontend audit --omit=dev --audit-level=low`: passed, 0 reported
  vulnerabilities.
- `npm --prefix frontend audit --audit-level=low`: passed, 0 reported
  vulnerabilities.
- `npm --prefix frontend ls --omit=dev --depth=0`: passed.
- `cargo metadata --manifest-path backend/Cargo.toml --locked --offline --no-deps --format-version 1`: passed.
- Development and production Compose `config --quiet`: passed. Rendered
  PostgreSQL, Redis, and pgAdmin host bindings are `127.0.0.1`.
- `git diff --check`: passed with line-ending warnings only.
- Sanitized current/history selected-token/private-key scans: no matches.
- Targeted source pattern scans found no Rust unsafe blocks, process execution,
  dangerous React HTML API, or disabled TLS verification pattern.
- Rust advisory/license scanning was not run because `cargo-audit` and
  `cargo-deny` are unavailable. `cargo-geiger`, `actionlint`, `gitleaks`,
  `trufflehog`, and `semgrep` are also unavailable.
- Live PostgreSQL/Redis tenant integration, browser E2E, payment, outbound
  webhook/email, and deployed-edge checks were not run in this source-only
  phase.
- An initial focused Rust compile found a missing `StatusCode` import introduced
  by the new upload middleware. It was fixed before the final matrix; no final
  validation failure remains.

### Phase 15 validation results (2026-07-12 16:27)

- `cargo fmt --manifest-path backend/Cargo.toml`: passed.
- `cargo test --manifest-path backend/Cargo.toml marketplace_phase_fifteen`:
  passed, 4 targeted launch/GA contract tests with 0 failures.
- PowerShell script parse check for
  `scripts/marketplace-phase15-ga-check.ps1`: passed.
- `git diff --check`: passed with line-ending warnings only.
- Mermaid diagram count check: passed, 43 `.mmd` files (`00` through `42`).
- `cargo test --manifest-path backend/Cargo.toml marketplace`: passed, 79
  Marketplace tests with 0 failures.
- Live GA smoke was not run because it requires a safe target environment,
  access token, organization id, approved product data, and support/incident
  evidence.

### Phase 14 validation results (2026-07-12 11:38)

- `cargo fmt --manifest-path backend/Cargo.toml` passed.
- `cargo test --manifest-path backend/Cargo.toml marketplace_phase_fourteen`
  passed: 3 tests, 0 failed.
- PowerShell script parse check for
  `scripts/marketplace-phase14-beta-readiness.ps1` passed.
- `git diff --check` passed with line-ending warnings only.
- `cargo test --manifest-path backend/Cargo.toml marketplace` passed: 75 tests,
  0 failed.
- Mermaid diagram count is 42 (`00` through `41`). No Mermaid parser is
  installed, so render/parser validation was not performed.

### Phase 13 validation results (2026-07-12 10:52)

- `cargo fmt --manifest-path backend/Cargo.toml`: passed.
- `cargo test --manifest-path backend/Cargo.toml marketplace_phase_thirteen`:
  passed, 3 targeted security QA tests with 0 failures.
- `cargo test --manifest-path backend/Cargo.toml marketplace_performance`:
  passed, 2 targeted performance/cache/script contract tests with 0 failures.
- `cargo test --manifest-path backend/Cargo.toml marketplace`: passed, 72
  Marketplace tests with 0 failures.
- `git diff --check`: passed with line-ending notices only.
- Docker infrastructure: `docker compose ps` showed PostgreSQL and Redis
  healthy before live validation.
- SQLx migration check: `_sqlx_migrations` reports version `26`
  (`v3 phase thirteen marketplace qa performance`) applied successfully.
- Release-mode backend smoke: a release backend was compiled and run in an
  isolated target directory on port `8084`; `/ready` returned 200.
- Phase 13 load smoke against the release backend passed:
  - catalog: 10 requests, HTTP `200:10`, average 135.2 ms, P95 144 ms, max
    144 ms, budget 300 ms;
  - catalog search: 10 requests, HTTP `200:10`, average 152.1 ms, P95 195 ms,
    max 195 ms, budget 300 ms;
  - listing detail: 10 requests, HTTP `200:10`, average 145.4 ms, P95 162 ms,
    max 162 ms, budget 250 ms.
- Temporary smoke fixture: inserted `phase13-smoke-creator` and
  `phase13-smoke-listing` only to exercise listing detail, then deleted both;
  verification query returned count `0` for both slugs.
- Earlier debug-build smoke was not treated as final acceptance: the debug
  backend on local Windows exceeded budgets, and public `/health`/`/ready`
  calls showed high environment/runtime overhead. The final gate used the
  release backend.

### Phase 12 local validation results (2026-07-12 09:24)

- `node --check scripts\marketplace-cli.mjs`: passed.
- `npm run marketplace -- --help`: passed and printed validate/pack/submit usage.
- `npm run marketplace -- validate docs/marketplace-samples/component-pack`:
  passed; 4 files, low risk, 0 errors, 0 warnings, 0 findings.
- `npm run marketplace -- validate docs/marketplace-samples/integration-plugin`:
  passed; 3 files, medium risk, 0 errors, 0 warnings, 1 expected finding for
  sensitive permission `webhook.send`.
- `npm run marketplace -- pack docs/marketplace-samples/component-pack --force`:
  passed; created `marketplace-dist/demo-component-pack-1.0.0.zip` with SHA-256
  `c8ec262783ecea58671922ef931c45c481c528437d460b78448438b73e9a453f`.
- `npm run marketplace -- pack docs/marketplace-samples/integration-plugin
  --force`: passed; created
  `marketplace-dist/demo-webhook-adapter-1.0.0.zip` with SHA-256
  `377d60ef18594523dc8389b40d085d70fc0c36714c3b6f4bcc6df539644d8cb7`.
- `npm run marketplace -- validate marketplace-dist\demo-component-pack-1.0.0.zip
  --manifest docs\marketplace-samples\component-pack\manifest.json`: passed.
- `npm run marketplace -- validate
  marketplace-dist\demo-webhook-adapter-1.0.0.zip --manifest
  docs\marketplace-samples\integration-plugin\manifest.json`: passed with the
  same expected medium `webhook.send` finding.
- `tar -tf` on both generated ZIP files: passed; listed expected package entries.
- Node JSON parse check for `package.json` and both sample manifests: passed.
- `git diff --check`: passed with line-ending notices only.
- Mermaid static validation: passed for 40 `.mmd` files, one declaration each
  and no Markdown fences.
- Live `submit` against a real approved creator listing was not run because it
  requires a safe authenticated creator/listing fixture. The command path is
  implemented and documented, and uses the existing version upload API.

### Phase 11 local validation results (2026-07-11 18:58)

- `cargo fmt -- --check` from `backend/`: passed.
- `cargo test marketplace_analytics` from `backend/`: passed, 3 targeted
  analytics tests with 0 failures.
- `cargo test --all-features` from `backend/`: passed, 105 backend tests plus
  doc tests with 0 failures.
- `npm run lint` from `frontend/`: passed.
- `npm run typecheck` from `frontend/`: passed.
- `npm test -- MarketplacePage` from `frontend/`: passed under approved
  escalation, 1 file and 12 tests. Escalation was required because Vitest/esbuild
  failed in the sandbox with `spawn EPERM`.
- `npm test` from `frontend/`: passed under approved escalation, 3 files and
  14 tests.
- `npm run build` from `frontend/`: passed under approved escalation with the
  existing >500 kB Vite chunk warning.
- `git diff --check`: passed with line-ending notices only.
- Mermaid static validation: passed, 39 `.mmd` files, one declaration each and no
  Markdown fences.
- Environment smoke: `docker compose ps` failed because Docker Desktop was not
  running (`dockerDesktopLinuxEngine` pipe missing). No live PostgreSQL/API smoke
  was run for Phase 11.

### Phase 10 final validation results (2026-07-11 14:54)

- `cargo fmt --manifest-path backend/Cargo.toml -- --check`: passed.
- `cargo test --manifest-path backend/Cargo.toml --all-features`: passed, 102
  backend tests plus doc tests with 0 failures.
- `npm --prefix frontend run lint`: passed.
- `npm --prefix frontend run typecheck`: passed.
- `npm --prefix frontend test`: passed, 3 files and 13 tests.
- `npm --prefix frontend run build`: passed with the existing >500 kB Vite chunk
  warning.
- `git diff --check`: passed with line-ending notices only.
- Mermaid static validation: passed, 38 `.mmd` files, one declaration each and no
  Markdown fences.
- Docker smoke: PostgreSQL and Redis healthy; backend `/health` 200, `/ready` 200
  with PostgreSQL/Redis reachable, `/openapi.json` 200.
- SQLx migration smoke: `_sqlx_migrations` reports version `24` (`v3 phase ten
  ratings abuse`) and version `25` (`v3 phase ten internal notifications`) with
  `success = true`.
- OpenAPI smoke: all six Phase 10 routes are present:
  `/api/marketplace/listings/{listing_id}/reviews`,
  `/api/marketplace/reviews`,
  `/api/marketplace/reviews/{review_id}/moderation`,
  `/api/marketplace/listings/{listing_id}/reports`,
  `/api/marketplace/reports`, and `/api/marketplace/reports/{report_id}`.

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

### Security Audit Phase 1 override

- **Critical owner action:** SEC-P01-001 is fixed in source but existing
  deployments and Git history are not automatically remediated.
- **High confirmed:** tenant webhook DNS/redirect SSRF exposure
  (SEC-P01-002) and browser-readable token persistence (SEC-P01-003).
- **Medium confirmed/deferred:** trusted-proxy identity, stale access-token role,
  refresh rotation/reuse, preview query tokens, and missing CI security gates.
- **High unverified:** live tenant/RLS isolation matrix and Rust advisory status.
- The complete stable finding/risk register and evidence are in
  `docs/security/PHASE_01_SECURITY_BASELINE.md`.
- Historical statements below about Clippy failing are superseded: the exact
  all-target/all-feature Clippy command passed at this checkpoint.

### Blocking issues

- None for the locally validated Phase 15 work.

### Non-blocking issues

- **Medium:** Optional live Phase 15 GA smoke has not been run because it requires a safe target environment, authenticated token, organization id, approved products, and launch/support evidence.
- **Medium:** `cargo clippy -D warnings` reports legacy warnings in older modules, so the backend CI clippy step is not currently a reliable green signal.
- **Low:** Mermaid files have only repository-local structural validation; no parser/render dependency is installed.
- **Low:** Vite emits a large output chunk warning (>500 kB).

### Security risks

- **High if scope expands:** Phase 7 supplies policy decisions but not concrete package execution. Do not enable executable extensions before an isolated adapter/runtime, kill-switch, permission enforcement, and forced-RLS review are implemented.
- **Medium operational:** Local filesystem artifact storage and non-atomic filesystem/database behavior require backup and cleanup procedures outside this repository; Phase 15 documents response/rollback workflow but does not provision external monitoring or backups.
- **Low/known:** The frontend hides controls by role, but backend middleware/handler checks remain the security authority.

### Compatibility risks

- Phase 15 adds no migration and no new GA/launch API; it validates and documents existing operational surfaces.
- All Marketplace routes are tenant-aware and require authentication plus `X-Organization-Id`; older planning language that calls the catalog public must not be used as a runtime contract.
- No anonymous catalog behavior, automatic updates, S3/CDN, durable queue, monitoring vendor integration, or multi-replica preview broadcast is implemented.

### Technical debt

- Legacy clippy warnings across pre-Phase-6 modules.
- No durable webhook retry worker, monitoring exporter, automatic backup, or vendor-backed dashboard provisioning.
- Automated payout transfer execution, partial refunds, external runtime telemetry, and executable Marketplace runtime remain deferred.

## 14. Assumptions and Unknowns

### Confirmed facts

- [x] The Phase 1 branch, starting commit, clean starting tree, and origin
  tracking were verified before edits.
- [x] No real committed secret was confirmed by the available current/history
  signature scans; scanner limitations are documented.
- [x] Phase 1 source fixes and the complete final local validation matrix pass.
- [x] No commit, push, history rewrite, deployed credential rotation, or live
  deployment mutation occurred.
- [x] `main` and `origin/main` point to `87bc6d0e` at the Phase 15 checkpoint.
- [x] The repository was clean before Phase 15 implementation began.
- [x] Phase 14 is committed at `87bc6d0e`.
- [x] Phase 15 docs, static contract tests, GA check script, final policy update, operations runbook, release notes, and diagram `42` are present in the current working tree.
- [x] Phase 15 adds no migration and no new GA/launch API.
- [x] Current manifests report version `0.1.0`.
- [x] The local development compose file provides PostgreSQL, Redis, and pgAdmin; it does not run the backend/frontend services.

### Unconfirmed assumptions

- [ ] Whether any deployed environment still contains an administrator created
  by the former deterministic bootstrap path.
- [ ] Whether real ingress, egress, TLS, secret injection, backups, monitoring,
  and email webhook configuration enforce stronger controls than the repository
  demonstrates.
- [ ] Whether every tenant/RLS policy and bypass path behaves correctly against
  representative live data.
- [ ] A production/staging-like environment has real approved products, support evidence, monitoring ownership, and credentials for meaningful Phase 15 live GA smoke.
- [ ] A deployed environment has the expected `UPLOAD_DIR` contents and artifact files needed for end-to-end install/update/rollback smoke tests.
- [ ] External monitoring/dashboard, backup, and support tooling are operationally provisioned outside this repository.
- [ ] The user has not authorized committing the current Phase 15 working tree.

## 15. Remaining Work

### Google OKF Phase 5 current remaining work

1. Preserve and review the Phase 5 report, status matrix, merge ledger,
   security/data ledger, and full build manifest. No commit or staging is
   authorized by this checkpoint.
2. Keep the 23 remaining targets explicitly deferred: 11 regeneration, 7
   owner-blocked, 1 out-of-scope Phase 6 target, and 5 historical.
3. Resolve owner and deployment evidence before promoting public routing,
   compatibility, retention, monitoring, storage, settlement, or ownership
   statements.
4. Keep the legacy `/okf/`, `okf-bootstrap/`, and `okf/index.yaml` trees
   untouched until a later preservation-backed cutover decision.

### Security Audit Phase 1 remaining-work override

1. Owner/deployment operators must execute the SEC-P01-001 account inventory,
   disable/rotate response, activity review, local/deployed placeholder
   replacement, and Git-history decision without committing replacement
   credentials.
2. Review the uncommitted Phase 1 diff. Stage/commit/push only after explicit
   user authorization.
3. Start Phase 2 only after separate authorization. Prioritize SSRF-safe
   outbound requests, transactional refresh families/reuse detection,
   trusted-proxy handling, and a live RLS/tenant authorization matrix.
4. Add Rust advisory and repository security scanners through a separately
   reviewed CI/tooling change; do not force audit fixes or major upgrades.

### Phase 15 remaining-work override

1. Review the uncommitted Phase 15 diff.
2. If and only if the user explicitly authorizes it, stage and commit the Phase
   15 implementation.
3. Do not repeat Phases 9 through 14 or already completed Phase 15 validation
   unless review changes code, script, docs, or diagrams.
4. When a safe production/staging-like environment exists, optionally run
   `scripts/marketplace-phase15-ga-check.ps1` with the appropriate API base URL,
   access token, organization id, and optional admin mode.

### Phase 14 remaining-work override

Phase 14 is committed at `87bc6d0e`; no Phase 14 remaining implementation work
is active.

### Phase 13 remaining-work override

Phase 13 is committed at `70d8f12`; no Phase 13 remaining implementation work is
active.

### Phase 12 remaining-work override

1. Review the uncommitted Phase 12 diff.
2. If and only if the user explicitly authorizes it, stage and commit the Phase
   12 implementation.
3. Optionally run a live `submit` smoke against a safe approved creator/listing
   fixture if credentials and fixture scope are explicitly provided/authorized.
4. Do not repeat Phases 9, 10, 11, or already completed Phase 12 local
   validation unless code changes.

### Phase 11 remaining-work override

1. Review the uncommitted Phase 11 diff.
2. If and only if the user explicitly authorizes it, stage and commit the Phase
   11 implementation.
3. Do not repeat Phases 9, 10, or already completed Phase 11 implementation and
   validation.

### Phase 10 remaining-work override

1. Review the uncommitted Phase 10 diff.
2. If and only if the user explicitly authorizes it, stage and commit the Phase
   10 implementation.
3. Do not repeat Phases 9 or 10 and do not rerun completed validation unless code
   or migrations change.

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

Review the Phase 5 report, construction status, merge ledger, security/data
verification ledger, and full build manifest. Then preserve the 23 deferred or
out-of-scope targets and open NOCs, keep `/okf/`, `okf-bootstrap/`, and
`okf/index.yaml` untouched, and do not stage, commit, push, cut over, or delete
legacy material. The next separately authorized boundary is Phase 6.

The older security, product-phase, and Phase 4 actions below are historical and
superseded by the current Google OKF Phase 5 checkpoint.

Have the deployment owner inventory every ZinharCMS environment for an
administrator created by the former deterministic bootstrap path; disable or
rotate any such account, review relevant authentication/audit activity, replace
the ignored local `.env` JWT placeholder and any deployed placeholders, and
decide whether the five matching historical commits require history rewriting.
Do not copy replacement values into Git, this handoff, logs, or chat. After that
owner-side response, obtain explicit authorization before staging/committing
Phase 1 or beginning the Phase 2 SSRF/session/RLS work.

The older OKF and Phase 7 instructions below are historical and superseded by
the Security Audit Phase 1 action above.

Wait for explicit owner authorization before starting OKF Phase One. Once
authorized, create `okf/README.md` and `okf/index.yaml`, then create
`okf/project/overview.md`, `repository-map.md`, `glossary.md`, and
`documentation-map.md` plus the initial reference registers. Import every Phase
Zero UNKNOWN, NEEDS_OWNER_CONFIRMATION, and DOCUMENTATION_CODE_CONFLICT marker
without guessing an answer. Do not modify product code, APIs, migrations, or
existing Mermaid files as part of that start.

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

### Google OKF Phase 4 acceptance override

- [x] Only the three eligible Architecture/Backend/Frontend `DEFERRED_MERGE`
  targets were constructed.
- [x] Each new Concept has valid required frontmatter, an approved type,
  current source provenance, explicit NOC caveats, and no custom extension.
- [x] Architecture, Backend, and Frontend indexes plus the staging log were
  updated without modifying Phase 3 Concept files.
- [x] The 54-target matrix and 72-input merge ledger reconcile without
  duplicates or silent omissions.
- [x] The post-Phase 4 staging manifest contains deterministic hashes for all
  36 staging files.
- [x] Local links, root reachability, Mermaid fence structure, frontmatter,
  source provenance, and no-orphan checks passed.
- [x] Legacy trees, canonical index, Phase 3 artifacts, application/runtime
  paths, database, dependencies, and CI were not modified.
- [x] No cutover, redirect, deletion, commit, push, or deployment occurred.
- [x] Remaining deferred work and the exact Phase 5 next action are recorded.

### Security Audit and Hardening Phase 1 acceptance override

- [x] Entire tracked repository and relevant ignored local configuration were
  inventoried without copying secret values.
- [x] Required branch/state/handoff/recent-commit checks were completed and
  existing work was preserved.
- [x] `docs/security/PHASE_01_SECURITY_BASELINE.md` exists in English with every
  required heading and stable findings.
- [x] Confirmed findings distinguish severity, confidence, evidence, impact,
  status, owner-side remediation, and deferred work.
- [x] Only low-risk, source-supported hardening changes were applied.
- [x] Final backend/frontend quality, test, dependency, Compose, secret-pattern,
  and diff checks completed as recorded; unavailable/live checks are explicit.
- [x] No claim of production readiness, penetration-test coverage, complete
  security, or vulnerability-free status is made.
- [x] No commit or push was created.

### OKF Phase Zero acceptance override

- [x] The repository, technology, architecture, module, documentation,
  Mermaid, database, API, and convention inventories are complete.
- [x] Knowledge gaps, owner questions, proposed OKF structure, and the
  twelve-phase implementation plan are recorded.
- [x] All fourteen required English files exist under `okf-bootstrap/`.
- [x] UNKNOWN, NEEDS_OWNER_CONFIRMATION, and DOCUMENTATION_CODE_CONFLICT
  registers are explicit and reproduced in the summary.
- [x] The generated reports contain no Persian/Arabic script and no trailing
  whitespace; the required file set and twelve summary sections were checked.
- [x] Source counts were revalidated against Git, migrations, and route source.
- [x] No final `okf/` directory, product code, API, migration, or Mermaid source
  was created or changed.
- [x] Git diff validation passed; no files were staged or committed.

### Phase 15 acceptance override

- [x] 15.1 Launch Readiness is documented with a runbook, final policy,
  support workflow, rollback plan, and incident checklist.
- [x] The launch runbook covers broken install, malicious product, wrong
  payment, refund/dispute/payout issue, abuse/review attack, and emergency block
  scenarios.
- [x] 15.2 General Availability is documented with release notes, public docs,
  monitoring dashboard sources, support plan, and GA go/no-go checks.
- [x] GA readiness uses existing install, rollback, purchase, report, admin
  analytics, beta blocker, health, and ready endpoints rather than a new launch
  API.
- [x] Static backend tests verify docs, final policy, script, diagram, and route
  coverage for both Phase 15 subphases.
- [x] Diagram `42-marketplace-launch-ga.mmd` and traceability/index docs are
  updated.
- [x] Uploaded Marketplace package code remains unexecuted.
- [ ] Optional live GA smoke is verified against a safe target environment.

### Phase 14 acceptance override

- [x] 14.1 Private Creator Beta evidence contract is documented for 5 to 10
  real products, creator feedback, bug list/blockers, and creator analytics.
- [x] 14.2 Customer Beta evidence contract is documented for install,
  uninstall, purchase, support issue, and report data.
- [x] Readiness script uses existing beta and Marketplace endpoints only and is
  read-only.
- [x] Static backend tests verify docs, script, diagram, and existing route
  coverage for both Phase 14 subphases.
- [x] Diagram `41-marketplace-beta.mmd` and traceability/index docs are updated.
- [x] Uploaded Marketplace package code remains unexecuted.

### Phase 13 acceptance override

- [x] Security QA covers IDOR, permission bypass, malicious package, refund
  abuse, and review/abuse-reporting abuse paths.
- [x] No targeted Phase 13 P0 security test fails.
- [x] Catalog and listing-detail responses expose a private bounded cache
  policy.
- [x] Migration `0026` adds catalog/search/latest-version/installation/
  entitlement/checkout performance indexes.
- [x] Load-smoke script measures catalog, search, listing detail, and optional
  install mutation with P95 budgets.
- [x] Load-smoke script fails non-2xx/3xx samples and disables localhost proxy
  overhead.
- [x] Release-mode local smoke passed catalog, search, and listing-detail P95
  budgets against PostgreSQL/Redis with migration `26` applied.
- [x] Phase 13 README/API/architecture/gap/Mermaid traceability is updated.
- [x] Uploaded Marketplace package code remains unexecuted.

### Phase 12 acceptance override

- [x] Creator CLI exposes `validate`, `pack`, and `submit` commands.
- [x] Local validation reports manifest, permission, compatibility, file-tree,
  adapter declaration, and security findings before upload.
- [x] Pack creates readable ZIP artifacts and reports SHA-256.
- [x] Submit targets the existing Marketplace version upload API with multipart
  `manifest` and `file` fields.
- [x] Creator guide documents manifest, permissions, review policy, CLI workflow,
  and sample packages.
- [x] Component Pack and Integration Plugin sample packages are present.
- [x] Both sample packages pass local validation and packing.
- [x] Phase 12 API/architecture/gap/README/Mermaid traceability is updated.
- [x] Uploaded Marketplace package code remains unexecuted.
- [ ] Optional live submit smoke is verified against a safe approved
  creator/listing fixture.

### Phase 11 acceptance override

- [x] Creator analytics expose installs, active installs, revenue, conversion,
  ratings, reports, and persisted error signals.
- [x] Creator analytics are scoped to the creator owner; non-owners receive
  forbidden behavior from the backend ownership check.
- [x] Marketplace admin analytics expose submission rate, approval time, installs,
  refunds, reports, critical reports, blocked packages, and risky/repetitive
  products.
- [x] Admin analytics require global admin/super admin authorization.
- [x] Frontend Marketplace UI renders creator analytics and admin health/risk
  analytics with typed API clients and translations.
- [x] Phase 11 API, architecture, gap, traceability, ambiguity, README, and
  Mermaid documentation is updated.
- [x] Backend formatting/tests and frontend lint/typecheck/tests/build passed.
- [x] No new migration was required and uploaded package code remains unexecuted.
- [x] Live authenticated analytics API smoke is verified against a running local
  PostgreSQL/Redis-backed backend.

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
- **Backend runtime:** Rust 1.96; run Cargo commands with `--manifest-path backend/Cargo.toml` from the repository root.
- **Frontend runtime:** Node 24/npm; frontend commands can be run with `npm --prefix frontend ...`.
- **Local infrastructure:** PostgreSQL 16, Redis 7, and pgAdmin from `docker-compose.yml`.
- **Production-like infrastructure:** `docker-compose.prod.yml` also runs backend and Nginx-served frontend images, using environment variable names from `.env.example`.
- **Safe setup:** copy `.env.example` to `.env`, then start only the required local infrastructure with `docker compose up -d postgres redis pgadmin`. Do not expose or copy `.env` values into documentation.
- **Backend development:** `cargo run --manifest-path backend/Cargo.toml` after required environment variables and database/Redis are available.
- **Frontend development:** `npm ci --prefix frontend`, then `npm --prefix frontend run dev`.
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

### 2026-07-11 09:25 +01:00 - V3 Phase 10 feedback and abuse-reporting checkpoint

- Verified Git source of truth: Phase 9 is committed at `dffe515`; no Phase 9 implementation was repeated.
- Implemented 10.1 customer ratings/reviews with one organization review per listing, 1–5 rating, review text, install-or-completed-purchase ownership gate, pending/published/rejected moderation, published catalog aggregation, and audit records.
- Implemented 10.2 abuse intake with typed severity/evidence, forced-RLS report storage, global-admin severity-prioritized moderation queue, investigate/resolve/dismiss states, and an atomic critical-notification handoff/audit record.
- Added migration `0024`, feedback validation service/contract test, routes/OpenAPI, Marketplace UI/API client/forms, Phase 10 docs, and Mermaid diagram `37`.
- Validation passed: `cargo fmt --check`, `cargo test --all-features` (101 tests), frontend lint/typecheck, Vitest (3 files/10 tests), production build, `git diff --check`, PostgreSQL migration 24, and backend `/health` plus OpenAPI route smoke. The temporary backend process was stopped; PostgreSQL/Redis remain healthy.
- **Exact Next Action:** review the uncommitted Phase 10 diff and only after explicit user authorization stage/commit it. Do not repeat completed work.

### 2026-07-11 14:54 +01:00 - V3 Phase 10 final validation checkpoint

- Verified the handoff against actual Git state and treated the repository as the
  source of truth where the earlier 09:25 handoff was stale.
- Completed Phase 10 hardening after review: sanitized customer-review list DTO,
  global pending-review queue, actionable abuse queue, persisted internal
  notifications for critical abuse reports, notification acknowledgement,
  Unicode character-count validation, frontend admin queues/API types/tests, and
  stale diagram/doc corrections.
- Final validation passed: backend format, 102 backend tests plus doc tests,
  frontend lint/typecheck/build, 13 frontend tests, `git diff --check`, 38
  Mermaid files, migration versions `24` and `25`, and backend health/ready/OpenAPI
  smoke with all six Phase 10 paths present.
- The temporary backend process was stopped. Docker PostgreSQL/Redis remain
  healthy. No commit, reset, cleanup, or destructive action was performed.
- **Exact Next Action:** review the uncommitted Phase 10 diff and only after
  explicit user authorization stage/commit it. Do not repeat completed work.

### 2026-07-11 18:58 +01:00 - V3 Phase 11 analytics checkpoint

- Verified Git source of truth: Phase 10 is committed at `e77e2f7`; no Phase 10
  implementation was repeated.
- Implemented Phase 11.1 creator analytics with owner-only backend access and
  product-level installs, active installs, revenue, conversion, ratings, reports,
  and persisted error signals.
- Implemented Phase 11.2 global-admin Marketplace analytics with submission
  rate, approval time, installs, refunds, reports, critical reports, blocked
  packages, and ranked risky/repetitive products.
- Added analytics route/service modules, frontend API/types/UI/test coverage,
  i18n keys, `docs/V3_PHASE_ELEVEN.md`, diagram `38`, and related API,
  architecture, gap, ambiguity, traceability, and README updates.
- Local validation passed: backend format, 105 backend tests plus doc tests,
  frontend lint/typecheck/build, 14 frontend tests, `git diff --check`, and
  39-file Mermaid static validation.
- Live DB/API smoke was not run because Docker Desktop was not running. No stage,
  commit, reset, cleanup, destructive command, or database mutation was performed.
- **Exact Next Action:** review the uncommitted Phase 11 diff and only after
  explicit user authorization stage/commit it. If live validation is requested
  first, start Docker Desktop/PostgreSQL/Redis and smoke the two analytics
  endpoints against safe test data.

### 2026-07-12 05:10 +01:00 - Phase 11 live smoke retry still blocked

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth remains
  Phase 10 commit `e77e2f7` plus uncommitted Phase 11 analytics changes.
- Rechecked Docker for the remaining live analytics smoke. `docker compose ps`
  failed because the Docker Desktop Linux engine pipe is missing
  (`dockerDesktopLinuxEngine` not running).
- No source code, database, staging, commit, reset, cleanup, or destructive action
  was performed during this retry.
- **Exact Next Action:** either start Docker Desktop and run the live Phase 11
  analytics API smoke, or review/stage/commit the already locally validated Phase
  11 implementation if live environment smoke is not required before commit.

### 2026-07-12 05:30 +01:00 - Phase 11 live analytics smoke completed

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth remains
  Phase 10 commit `e77e2f7` plus uncommitted Phase 11 analytics changes.
- Started/verified Docker PostgreSQL and Redis with `docker compose up -d
  postgres redis`; both services were healthy.
- Started a temporary backend on port `8082` with local development environment
  values. `/health` returned 200, `/ready` returned 200 with PostgreSQL and
  Redis reachable, and `/openapi.json` contained both Phase 11 analytics paths.
- Authenticated live analytics smoke passed: creator-owner analytics returned
  200 with zero-count safe test data; author access to admin analytics returned
  403; global-admin analytics returned 200; global-admin/non-owner access to the
  creator-owned analytics endpoint returned 403.
- The temporary smoke user/creator data was removed from the local database and
  verified absent. The temporary backend process was stopped; port `8082` was
  closed. Docker PostgreSQL/Redis remain running and healthy.
- No source code was changed during smoke validation. `HANDOFF.md` was updated
  to record the completed live smoke. No files were staged, committed, reset, or
  discarded.
- **Exact Next Action:** review the uncommitted Phase 11 diff and only after
  explicit user authorization stage/commit it. Do not repeat completed Phase 11
  validation unless review changes code.

### 2026-07-12 09:24 +01:00 - V3 Phase 12 creator tooling checkpoint

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth supersedes
  the stale handoff: Phase 11 is committed at `beb4cf2`, and the working tree
  was clean before Phase 12 implementation.
- Extracted Phase 12 from the V3 proposal: 12.1 CLI/SDK packaging and 12.2
  documentation/sample packages.
- Implemented `scripts/marketplace-cli.mjs` with `validate`, `pack`, and
  `submit`. The CLI performs local manifest, permission, compatibility, file-tree,
  adapter-declaration, and security preflight checks; `pack` writes ZIP artifacts
  with SHA-256; `submit` targets the existing version upload API.
- Added `docs/V3_PHASE_TWELVE.md`, `docs/MARKETPLACE_CREATOR_GUIDE.md`, sample
  Component Pack and Integration Plugin packages, and
  `docs/diagrams/39-marketplace-creator-tooling.mmd`; updated README, API,
  architecture, gap, status map, traceability, evidence, and diagram catalog docs.
- Validation passed: Node syntax check, CLI help, sample validate/pack, generated
  ZIP validation and listing, JSON parsing, `git diff --check`, and 40-file
  Mermaid static validation. Live submit smoke was not run because it requires a
  safe authenticated approved creator/listing fixture.
- Recorded a lesson in `D:\All projects\Mistakes\mistakes.md` for a SemVer parser
  bug caught and fixed in the new CLI.
- No files were staged or committed. No backend runtime code, migration, database
  reset, or uploaded-code execution was performed.
- **Exact Next Action:** review the uncommitted Phase 12 diff and only after
  explicit user authorization stage/commit it. Do not repeat completed Phase 12
  validation unless code changes.

### 2026-07-12 10:52 +01:00 - V3 Phase 13 security QA and performance checkpoint

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth supersedes
  the stale handoff: Phase 12 is committed at `19f6673`, and the working tree
  contains only uncommitted Phase 13 changes.
- Extracted Phase 13 from the V3 proposal: 13.1 Marketplace security QA and
  13.2 load/performance.
- Implemented targeted security QA for IDOR, permission bypass, malicious
  package blocking, refund abuse/idempotency, and review/abuse-reporting abuse
  contracts.
- Implemented performance contracts: migration `0026`, catalog cache headers,
  load-smoke script, Phase 13 docs, diagram `40`, and updated README/API/
  architecture/gap/traceability/evidence/inventory docs.
- Fixed two smoke-script validation mistakes discovered during live checks:
  Windows PowerShell now loads `System.Net.Http`, and the script fails any HTTP
  sample outside the 2xx/3xx range.
- Validation passed: targeted Phase 13 backend tests, wider Marketplace backend
  tests, `git diff --check`, SQLx migration `26`, and release-mode load smoke
  for catalog/search/listing detail. The temporary `phase13-smoke-*` fixture was
  removed and temporary backend processes were stopped.
- No files were staged or committed. Uploaded package code remains unexecuted.
- **Exact Next Action:** review the uncommitted Phase 13 diff and only after
  explicit user authorization stage/commit it. Do not repeat completed Phase 13
  validation unless code, migration, script, or docs change.

### 2026-07-12 11:38 +01:00 - V3 Phase 14 beta readiness checkpoint

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth supersedes
  the stale handoff: Phase 13 is committed at `70d8f12`, and the working tree was
  clean before Phase 14 implementation.
- Extracted Phase 14 from the V3 proposal: 14.1 Private Creator Beta and 14.2
  Customer Beta.
- Implemented Phase 14 as a read-only evidence/readiness layer over existing V2
  beta and V3 Marketplace APIs, without a new migration or parallel beta API.
- Added Phase 14 static backend contract tests, readiness script, Markdown
  phase document, Mermaid diagram `41`, and README/API/architecture/gap/
  traceability/evidence/inventory updates.
- Validation passed: backend format, targeted Phase 14 tests, PowerShell script
  parse check, `git diff --check`, and wider Marketplace backend regression
  (75 tests).
- No live DB/API smoke was run because the script requires real beta
  cohort/product/support evidence to produce meaningful readiness results. No
  database mutation, staging, commit, reset, or uploaded-code execution was
  performed.
- **Exact Next Action:** review the uncommitted Phase 14 diff and only after
  explicit user authorization stage/commit it. Do not repeat completed Phase 14
  validation unless code, script, docs, or diagrams change.

### 2026-07-12 16:27 +01:00 - V3 Phase 15 launch readiness and GA checkpoint

- Re-read `AGENTS.md` and `HANDOFF.md`; verified Git source of truth supersedes
  the stale handoff: Phase 14 is committed at `87bc6d0e`, and the working tree
  was clean before Phase 15 implementation.
- Extracted Phase 15 from the V3 proposal: 15.1 Launch Readiness and 15.2
  General Availability.
- Implemented Phase 15 as launch/GA operations readiness over existing
  Marketplace APIs: final policy update, operations runbook, incident checklist,
  rollback/support workflow, release notes, public-doc references, monitoring
  dashboard sources, GA check script, static contract tests, and diagram `42`.
- Validation passed: backend format, targeted Phase 15 tests, PowerShell script
  parse check, `git diff --check`, 43-diagram count check, and wider Marketplace
  backend regression (79 tests).
- No migration, new launch/GA API, live API smoke, database mutation, staging,
  commit, reset, or uploaded-code execution was performed.
- **Exact Next Action:** review the uncommitted Phase 15 diff and only after
  explicit user authorization stage/commit it. Do not repeat completed Phase 15
  validation unless code, script, docs, or diagrams change.

### 2026-07-17 13:03 +01:00 - OKF Phase Zero completed

- Reconciled the stale handoff with Git source of truth on branch
  `docs/okf-phase-zero` at `61ed3b38`; the session started from a clean working
  tree.
- Completed all fourteen required English analytical reports under
  `okf-bootstrap/`, covering repository, technology, architecture, 22 modules,
  66 Markdown documents, 43 Mermaid sources, 51 database tables, 140 route
  declarations, conventions, knowledge gaps, the proposed OKF structure,
  twelve implementation phases, owner questions, and the final summary.
- Recorded 15 UNKNOWN items, 18 NEEDS_OWNER_CONFIRMATION decisions, and 10
  DOCUMENTATION_CODE_CONFLICT items. None blocks repository-derived Phase One
  work, but affected policy and production claims must remain draft.
- Validation confirmed the exact 14-file set, non-empty English headings, no
  Persian/Arabic script, no trailing whitespace, twelve numbered summary
  sections, source counts, absence of the final `okf/` directory, and a clean
  `git diff --check` result apart from a line-ending notice.
- Parser-level and rendered validity of the 43 Mermaid sources remains UNKNOWN
  because no Mermaid parser/renderer is installed; static declaration,
  delimiter, and traceability checks passed.
- Product tests were not rerun because Phase Zero changed only analytical
  Markdown and this mandatory handoff. No staging, commit, reset, database
  mutation, dependency installation, or internet access was used.
- **Modified/untracked files:** `HANDOFF.md` and the fourteen files under
  `okf-bootstrap/`.
- **Exact Next Action:** wait for explicit owner authorization, then start OKF
  Phase One with the root OKF index and project overview files listed in section
  16. Do not start later phases automatically.

### 2026-07-25 - V3 release-facing root README rewrite

- Verified Git source of truth: `main`, `origin/main`, and the completed OKF
  branch all point to `cff48071`; the working tree was clean before this change.
- Replaced the phase-log-oriented root `README.md` with a release-facing project
  entry point covering current V3 status, core/SaaS/Marketplace capabilities,
  architecture, prerequisites, executable local startup, validation commands,
  Marketplace tooling, GA readiness, runtime boundaries, documentation links,
  repository layout, security, and governance gaps.
- Corrected the stale Page Builder roadmap claim and the misleading Quick Start
  that previously started infrastructure while presenting the API and frontend
  as available.
- Preserved the evidence-based distinction between completed repository
  implementation and an actual production GA launch. Root, frontend, and
  backend manifests remain at `0.1.0`, and no release tag exists.
- README validation found 35 Markdown links with zero broken local targets, no
  Persian/Arabic script, and all twelve checked command/endpoint references.
  Product tests were not rerun because this change only rewrites documentation.
- No file was staged, committed, tagged, published, or deployed.
- **Modified files:** `README.md`, `HANDOFF.md`.
- **Exact Next Action:** review the README diff and, only after explicit user
  authorization, commit it with this handoff. Then decide the V3 semantic
  version/tag and run the documented release go/no-go process.

### 2026-07-25 - V3 release-candidate hardening checkpoint

- Reconciled the stale handoff with Git source of truth at `201058d1` on
  `main`; the session began from a clean working tree.
- Reproduced the failed backend CI gate locally and fixed all Rust 1.96 Clippy
  errors with behavior-preserving refactors, then aligned Cargo, CI, and both
  backend Dockerfiles on Rust 1.96.
- Refactored transactional email and subscription update parameters into input
  structs, implemented the standard `AsMut<PgConnection>` trait for tenant
  connections, removed needless borrows/dereferences, and simplified linted
  validation paths without changing their behavior.
- Backend formatting, exact Clippy with denied warnings, and the all-features
  backend test command completed successfully after the fixes.
- Aligned application release sources to `3.0.0`, added
  `scripts/check-version-consistency.mjs`, wired it into both CI workflows, and
  changed frontend CI and both frontend Dockerfiles from `npm install` to
  lockfile-enforced `npm ci`.
- Aligned the supported build toolchains on Rust 1.96 and Node 24. Upgraded the
  frontend security baseline to React 19.2.7, React Router 8.3, Vite 7.3.6,
  ESLint 10, and compatible plugins/types. Migrated router imports according to
  the React Router 8 package boundary.
- Added a high-severity npm audit CI gate. The clean install and final audit
  reported zero vulnerabilities.
- Final local validation passed: eight-source version consistency; backend
  format, denied-warning Clippy, and 117 tests; frontend lint, typecheck, 14
  tests, and production build; complete Phase 15 report-only checks with four
  contract tests, 79 Marketplace tests, frontend lint/build, and both readiness
  booleans true; Git whitespace; YAML/JSON parsing; local Markdown links;
  changed-file English-language enforcement; and static Mermaid declarations.
- The frontend production bundle completed with a non-blocking chunk-size
  warning: 613.44 kB minified and 169.04 kB gzip for the main JavaScript chunk.
- Production Compose interpolation/config validation passed with non-secret
  validation-only values. The backend/frontend image build could not start
  because Docker Hub DNS authorization lookups failed, and none of the four
  required base images was cached locally.
- Updated the release-facing README, V3 release notes, diagrams, and affected
  OKF delivery/development/frontend/project documents while preserving the
  distinction between a local release candidate and production GA.
- No files were staged, committed, tagged, pushed, published, or deployed.
- **Modified/untracked files:** backend lint fixes; root/backend/frontend
  version and toolchain sources; frontend dependency/runtime migration; both CI
  workflows; Docker/Compose definitions; release documentation; diagrams; OKF
  knowledge documents; this handoff; and
  `scripts/check-version-consistency.mjs`.
- **Exact Next Action:** review the complete uncommitted release-candidate diff.
  Only after explicit authorization, stage and commit it. After push, require
  both GitHub CI workflows to pass, retry the two production image builds when
  Docker Hub is reachable, obtain the license and production go/no-go owner
  decisions, and only then create the approved `v3.0.0` tag/release.

### 2026-07-25 - Pushed release-candidate README review

- Reconciled Git state after the owner committed and pushed the release
  candidate. Local `HEAD`, `origin/main`, and the public HTTPS `main` reference
  all resolve to `1aafdcddc80196d865b1e32bd69e0dd50cc19337`.
- Verified that Backend CI and Frontend CI both completed successfully for the
  pushed commit.
- Reviewed the root README against current manifests, workflows, public tags,
  and repository paths. All 37 Markdown links resolve and no stale Vite 6,
  `npm install`, `0.1.0`, or Phase 5 wording remains.
- Applied two small accuracy corrections: CI and supported local development
  use Node.js 24 with npm 11, not Node.js 22; and the tag row now states
  specifically that the V3 tag is unpublished while public `v1.0.0` and
  `v2.0.0` tags already exist.
- No source, workflow, tag, release, or remote state was changed.
- **Modified files:** `README.md` and `HANDOFF.md`.
- **Exact Next Action:** review the two-file documentation diff and, only after
  explicit owner authorization, commit and push it before creating `v3.0.0`.

### 2026-07-25 - GPLv3 repository licensing checkpoint

- Reconciled the clean pushed state at `4e93657d` before starting the licensing
  change. The project owner selected GNU General Public License version 3 only,
  represented by the SPDX identifier `GPL-3.0-only`.
- Added the complete 674-line GNU GPL version 3 text as root `LICENSE`. The
  normalized file exactly matches the GNU-distributed reference text and has
  SHA-256
  `8b1ba204bb69a0ade2bfcf65ef294a920f6bb361b317dba43c7ef29d96332b9b`.
- Replaced the backend crate's `MIT OR Apache-2.0` metadata and aligned root,
  frontend, and frontend lockfile project metadata on `GPL-3.0-only`.
  Third-party dependency license metadata remains unchanged.
- Added the README license section, removed the obsolete no-license warning,
  registered `LICENSE` in repository inventories, and resolved UNKNOWN U-10
  and NOC-16 in the current OKF index and governance documents. Historical
  Phase Zero rows remain with explicit resolution notices.
- Validation passed for JSON parsing and project license metadata, Cargo
  metadata, eight-source release-version consistency, exact GPLv3 text
  comparison, changed Markdown local links, English-only changed files, and Git
  whitespace. The `okf/index.yaml` diff only removes the resolved entries and
  passed static indentation/quote checks; parser-level YAML validation was not
  rerun because no local parser was available and the temporary dependency
  install was unavailable offline.
- Product tests were not rerun because this change affects license metadata and
  documentation only. No dependency license was rewritten.
- No file was staged, committed, pushed, tagged, released, or deployed.
- **Modified/untracked files:** root `LICENSE`; root/frontend/backend manifest
  metadata; frontend lockfile root metadata; README; repository inventories;
  affected OKF current and historical governance records; and this handoff.
- **Exact Next Action:** review the GPLv3 licensing diff and, only after
  explicit owner authorization, commit and push it before retrying production
  image builds or creating `v3.0.0`.

### 2026-07-25 - GitHub Actions Node.js 24 runtime checkpoint

- Verified through the public GitHub Actions API that Frontend CI run
  `30163197148` completed successfully on rerun attempt 2 for commit
  `04dc10e5c3ffb9bfebd71f0b744dcfdbdbe75dc3`.
- Upgraded `actions/checkout` from v4 to v5 in both backend and frontend CI,
  and upgraded `actions/setup-node` from v4 to v5 in frontend CI.
- Preserved the explicit Node.js 24 application toolchain and disabled the new
  setup-node package-manager cache behavior to keep the existing no-cache CI
  semantics.
- Static validation passed with `git diff --check`, and all workflow action
  references resolve to the intended v5 majors. Local `actionlint` validation
  was not run because `actionlint` is not installed.
- No product source, dependency, manifest, tag, release, or remote state was
  changed. No files were staged or committed.
- **Modified files:** `.github/workflows/backend-ci.yml`,
  `.github/workflows/frontend-ci.yml`, and `HANDOFF.md`.
- **Exact Next Action:** review the three-file diff, then commit and push it
  only with explicit owner authorization. Require Backend CI and Frontend CI to
  pass without the Node.js 20 deprecation annotation before proceeding.

### 2026-07-25 - Docker Hub recovery and production image validation

- Reconciled the repository at clean pushed commit `c7876099` before the
  Docker investigation. The workflow runtime upgrade was already committed and
  pushed by the owner.
- Confirmed that Windows DNS resolves `registry-1.docker.io`,
  `auth.docker.io`, `production.cloudflare.docker.com`, and
  `production.cloudfront.docker.com`. Docker Desktop 29.5.3 is running the
  Linux engine with the `overlay2` driver.
- Pulled the production base images `rust:1.96-bookworm`,
  `debian:bookworm-slim`, `node:24-alpine`, `nginx:1.27-alpine`,
  `postgres:16-alpine`, and `redis:7-alpine` successfully.
- The first PostgreSQL refresh encountered one transient TLS handshake timeout
  at `auth.docker.io`. A retry authenticated successfully, Docker recovered one
  layer transfer through its internal retry mechanism, and the pull completed
  with digest
  `sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777`.
  No Docker DNS, proxy, registry mirror, or Windows network setting was changed
  because the failure was transient and DNS resolution was healthy.
- Built `zinharcms-frontend:latest` successfully as image
  `sha256:92a246b5ee2580d5fd6317b9164574f4b91338e02288cd10c304831b868b4bf8`.
  The container build completed `npm ci`, reported zero vulnerabilities, and
  completed the Vite production build. The known non-blocking 613.44 kB main
  chunk warning remains.
- Built `zinharcms-backend:latest` successfully as image
  `sha256:aff5eec6c3d54c70070b1d4d906af81d3fab0a12ea9fdaaf24205529741cdb53`.
  The clean containerized Cargo dependency download and optimized release build
  completed in 4 minutes 1 second.
- Production Compose interpolation and image resolution passed with
  non-secret validation-only values. No service container was started, no
  database or volume was mutated, and nothing was deployed or published.
- **Modified files:** `HANDOFF.md` only. Docker's local image and build caches
  changed as a result of the authorized pulls and builds.
- **Exact Next Action:** verify Backend CI and Frontend CI for pushed commit
  `c7876099`, including absence of the Node.js 20 deprecation annotation. Then
  complete target-environment go/no-go validation and owner sign-off before
  creating the approved `v3.0.0` tag and GitHub release.

### 2026-07-25 - Final V3 release go/no-go audit

- Reconciled local `HEAD`, `origin/main`, and public GitHub `main` at
  `c7876099c35f3ec1a8b6994d6f7327c4e0e6ed2e`. The release version validator
  passed across all eight `3.0.0` sources.
- Backend CI run `30165038600` passed on attempt 1.
- Frontend CI run `30165038602` failed at `npm test` on attempts 1 and 2,
  then passed all steps on attempt 3. The two latest successful check runs
  report zero annotations, confirming that the Node.js 20 deprecation warning
  is resolved.
- Public releases remain `v1.0.0` and `v2.0.0`; no `v3.0.0` release was
  present during this audit.
- `API_BASE_URL`, `ACCESS_TOKEN`, and `ORGANIZATION_ID` are not configured in
  the current environment, so target-environment Phase 15 API smoke and final
  operational go/no-go evidence remain unverified.
- **Release decision:** no-go until the repeated Frontend CI test failure is
  diagnosed and a fresh Frontend CI run passes on its first attempt, the
  target-environment go/no-go validation is completed, and the release,
  support, rollback, and communication owners sign off.
- No tag, release, deployment, source change, dependency change, or external
  mutation was performed.
- **Modified files:** `HANDOFF.md` only.
- **Exact Next Action:** diagnose and fix the Frontend CI `npm test`
  flakiness using the failed attempt logs or a faithful CI reproduction, then
  require a new first-attempt green Frontend CI run before completing the
  target-environment Phase 15 validation and owner sign-off.

### 2026-07-26 - Frontend CI teardown race fix

- Reproduced the intermittent Frontend CI failure in a disposable
  Linux/Node.js 24 container. The fourth consecutive run completed all 14 test
  assertions but Vitest then failed with `ReferenceError: window is not
  defined`, identified `DashboardPage.test.tsx` as the source, and reported
  that the exception occurred after the test environment was torn down.
- Confirmed the root cause: the Dashboard test ended after synchronous
  foundation-card assertions while the component's asynchronous content,
  media, page, entry, and billing requests were still pending.
- Replaced the test's live API behavior with deterministic service mocks and
  made the test await both the resolved dashboard statistics and billing
  usage before completion. The fix does not add retries, ignore unhandled
  errors, or weaken the CI gate.
- Validation passed: the focused Dashboard test; 10 consecutive complete
  Frontend test runs on Windows; 10 consecutive complete Frontend test runs
  in a fresh Linux/Node.js 24 container; and the complete Linux Frontend CI
  sequence (`npm ci`, high-severity audit with zero vulnerabilities, lint,
  typecheck, 14 tests, and production build).
- Release-version validation remains consistent at `3.0.0` across all eight
  sources. The known non-blocking 613.44 kB minified main-chunk build warning
  remains unchanged.
- No file was staged or committed, and nothing was pushed, tagged, released,
  published, or deployed.
- **Modified files:** `frontend/src/pages/DashboardPage.test.tsx` and
  `HANDOFF.md`.
- **Exact Next Action:** review the two-file diff and, only after explicit
  owner authorization, commit and push it. Require the resulting Frontend CI
  run to pass on attempt 1 before starting target-environment Phase 15
  validation and owner sign-off.

### 2026-07-26 - Fresh first-attempt Frontend CI verification

- Reconciled the repository after the owner committed and pushed the teardown
  race fix. Local `main`, `origin/main`, and `origin/HEAD` resolve to
  `4396b556a6e722adbdd818db9fb19074c46ee3fb`; the working tree was clean
  before this handoff update.
- Verified through the public GitHub Actions API that Frontend CI run
  `30207051877` completed successfully on `run_attempt: 1`.
- Every required job step passed: version consistency, `npm ci`, high-severity
  audit, lint, typecheck, all 14 tests, and the production build.
- Check run `89806841336` reports `annotations_count: 0`; no Node.js runtime
  deprecation or other check annotation remains.
- Backend CI was not expected to run because the pushed commit changed only
  the frontend test and this handoff, which do not match the backend workflow's
  path filters. The most recent backend-affecting commit remains covered by its
  previously verified successful Backend CI run.
- The repeated Frontend CI failure gate is closed. No tag, GitHub release,
  deployment, or publication was created.
- **Modified files:** `HANDOFF.md` only.
- **Exact Next Action:** complete the read-only Phase 15 go/no-go validation
  against the approved target environment using `API_BASE_URL`,
  `ACCESS_TOKEN`, and `ORGANIZATION_ID`, optionally with admin checks. Record
  the evidence and obtain explicit release, support, rollback, and
  communication owner sign-off before creating `v3.0.0`.

### 2026-07-26 - Target-environment Phase 15 validation blocked

- Reconciled the repository at `4396b556` with local `main` matching
  `origin/main`. The only pre-existing working-tree change was the uncommitted
  handoff update from the successful first-attempt Frontend CI verification.
- Checked only whether the required target inputs were configured; no secret
  value was printed or copied. `API_BASE_URL`, `ACCESS_TOKEN`, and
  `ORGANIZATION_ID` are absent from both the process environment and the root
  `.env` file.
- Inspected `scripts/marketplace-phase15-ga-check.ps1`. Without
  `API_BASE_URL`, it skips every API smoke check, so running it now would only
  repeat previously completed local backend/frontend validation and could not
  constitute target-environment go/no-go evidence.
- No HTTP request was sent, no local validation was misrepresented as a target
  pass, and no database, deployment, tag, release, or remote state was changed.
- **Modified files:** `HANDOFF.md` only.
- **Exact Next Action:** configure `API_BASE_URL`, `ACCESS_TOKEN`, and
  `ORGANIZATION_ID` outside the repository for the approved staging or
  production-like environment. If global-admin access is approved, also enable
  the script's `-AdminMode`. Then rerun the read-only Phase 15 GA check and
  record its endpoint-level results before requesting owner sign-off.

### 2026-07-26 - GitHub source-release scope and documentation preflight

- The owner explicitly limited `v3.0.0` to a GitHub source-code release.
  Target-environment API smoke, production support ownership, deployment
  rollback, and production communication gates are not applicable to this
  source publication and remain separate prerequisites for any future
  production General Availability claim.
- Verified application version `3.0.0` across all eight release sources and
  confirmed `GPL-3.0-only` in the root, frontend, and backend metadata.
- The public GitHub API returned HTTP 404 for both the `v3.0.0` tag reference
  and the `v3.0.0` Release endpoint. Existing public Releases remain `v1.0.0`
  and `v2.0.0`.
- Reworked the root README and V3 release notes to define a source-only release
  without implying a hosted service, binary/container publication, target
  health, or production GA. Kept the Phase 15 deployment gates as explicit
  future requirements.
- Updated the repository inventory and OKF release process with the selected
  scope, observed semantic tag format, manual source-release gate, and
  annotated-tag/GitHub-Release sequence.
- Validation passed: four targeted Phase 15 backend contract tests, eight-source
  version consistency, GPLv3 metadata checks, changed-document local links,
  changed-file English-language scan, and Git whitespace validation.
- The source-release files use timeless release identity; transient pre-tag
  state is recorded only in this handoff.
- No file was staged or committed, and no tag, GitHub Release, deployment,
  binary, container image, or package was published.
- **Modified files:** `README.md`,
  `docs/V3_MARKETPLACE_RELEASE_NOTES.md`,
  `docs/diagrams/REPOSITORY_INVENTORY.md`,
  `okf/delivery/release-process.md`, and `HANDOFF.md`.
- **Exact Next Action:** review and commit these five source-release
  documentation files, then push `main`. Confirm the release commit is clean
  and present on `origin/main`; documentation-only path filters are not
  expected to trigger product CI. After explicit final publication approval,
  create and push annotated tag `v3.0.0`, publish a non-prerelease GitHub
  Release using `docs/V3_MARKETPLACE_RELEASE_NOTES.md`, and verify the tag
  target plus GitHub-generated ZIP/tar source archives.

### 2026-07-26 - Security Audit and Hardening Phase 1 completed

- Verified the required `security/security-audit-fixes` branch at `64d780b6`
  with a clean starting tree, then inventoried the complete repository security
  surface and relevant Git history/local configuration without copying secret
  values.
- Created `docs/security/PHASE_01_SECURITY_BASELINE.md` with sixteen confirmed
  findings, five unverified risks, applied changes, validation evidence,
  limitations, and a Phase 2 recommendation.
- Fixed the deterministic privileged bootstrap/public-registration path,
  public Marketplace artifact exposure through the static upload root,
  placeholder-secret acceptance, ordinary JWT signature comparison, raw
  internal/readiness errors, browser non-CSPRNG fallback, production cookie
  configuration, and development data-service host exposure.
- Final Rust formatting, Clippy, and all-feature tests passed (124 tests).
  Frontend lint, typecheck, tests (15 tests), and build passed. npm audits,
  locked Rust metadata, Compose rendering, sanitized secret/source scans, and
  `git diff --check` passed. Unavailable and live-environment checks are
  explicitly documented.
- No commit, push, history rewrite, deployed credential change, or live
  environment mutation was performed.
- **Modified/untracked files:** the Phase 1 files listed in section 10.
- **Exact Next Action:** the deployment owner must complete the SEC-P01-001
  account/credential/activity inventory and local/deployed placeholder
  replacement without putting values in Git, then explicitly authorize either
  review/stage/commit or Phase 2 SSRF/session/RLS work.

### 2026-07-28 - Security Audit and Hardening Phase 4 completed

- Resumed on `security/security-audit-fixes` at starting commit `b2e34c37`.
  Preserved and completed the owner's pre-existing uncommitted Phase 4 work;
  no reset, clean, stash, stage, commit, push, migration, or deployment was
  performed.
- Implemented the parser-based backend rich-content boundary with Ammonia,
  write/read sanitation for current and historical entry/page/delivery paths,
  legacy Page Builder schema support, bounded document/URL complexity, safe
  Marketplace URLs, Preview WebSocket sanitation, and a versioned delivery
  cache namespace.
- Implemented the frontend DOMPurify boundary with the branded
  `SanitizedRichHtml` type, one approved `SafeRichText` sink, element-specific
  attribute enforcement, numeric attribute bounds, centralized URL handling,
  strict CSP/security headers, production Trusted Types enforcement, legacy
  editor parity, and an AST sink policy.
- Added the shared 20-malicious/5-safe XSS corpus and Phase 4 security tests.
  Confirmed and remediated `SEC-P04-001` (High), `SEC-P04-002` (Medium), and
  `SEC-P04-003` (Medium), and closed inherited `SEC-P01-021`. No Critical
  finding was confirmed.
- Created
  `docs/security/PHASE_04_CSP_TRUSTED_TYPES_RICH_TEXT_HARDENING.md` with all 30
  required sections and five English Mermaid sequence diagrams. Updated API,
  architecture, README, environment, deployment, and OKF documentation.
- Final code validation completed successfully before this pause:
  `cargo fmt --all -- --check`; Clippy offline with all targets/features and
  warnings denied; 166 backend unit tests plus 2 integration tests; frontend
  lint and typecheck; 44 frontend tests in 11 files; production frontend build;
  one-approved-sink AST policy; local and production Compose interpolation; and
  browser login/reload/navigation/logout, stored/published rich text, Preview
  WebSocket, CSP, and Trusted Types enforcement checks.
- The production build retained only the known non-blocking large-chunk warning.
  `cargo audit` was unavailable because the subcommand is not installed.
  `npm audit --omit=dev` was not run because external dependency-metadata
  transmission was not authorized. A production Nginx container was not
  rebuilt or started.
- Browser-test services on ports 8080 and 5173 were stopped, Phase 4 temporary
  artifacts were removed, the disposable `cms_phase4_20260727_2040` database
  was dropped, and Redis database 15 reported zero keys. A next-day recheck
  could not query PostgreSQL or Redis because Docker Desktop was no longer
  running; it still confirmed no temporary files and no listeners on the test
  ports. This unavailable redundant check is not represented as passed.
- **Created files:** `backend/src/services/rich_content.rs`,
  `docs/security/PHASE_04_CSP_TRUSTED_TYPES_RICH_TEXT_HARDENING.md`,
  `frontend/nginx.conf.template`, `frontend/scripts/check-html-sinks.mjs`,
  `frontend/securityHeaders.ts`, `frontend/src/components/SafeRichText.tsx`,
  `frontend/src/security/richContent.ts`,
  `frontend/src/security/richContent.test.tsx`,
  `frontend/src/security/securityHeaders.test.ts`,
  `frontend/src/security/trustedTypes.test.ts`, and
  `security/phase4-xss-corpus.json`.
- **Modified/deleted files:** `.env.example`, `README.md`, backend Cargo
  manifests and security/content/delivery/page/Marketplace routes/services,
  `docker-compose.prod.yml`, `docs/API.md`, `docs/ARCHITECTURE.md`,
  `frontend/Dockerfile.prod`, deleted `frontend/nginx.conf`, frontend package
  manifests, Billing/Marketplace/Page Builder sources and tests,
  `frontend/vite.config.ts`, the Phase 4-related OKF delivery/development/
  security documents, and this `HANDOFF.md`.
- **Final static closure:** the report has the exact 30 required headings and
  five English Mermaid sequence diagrams; changed/untracked files contain no
  Persian-script text; the production-shaped token/private-key/live-provider
  scan found no match; and `git diff --check` passed. The working tree remains
  intentionally unstaged and uncommitted.
- **Exact Next Action:** review the complete Phase 4 diff. Stage, commit, or
  push it only after explicit owner authorization. If a later Phase 5 is
  authorized, begin by inventorying CI and dependency-provenance controls,
  obtain approval for external advisory metadata transmission, and add the AST
  sink policy, shared malicious corpus, and production CSP/Trusted Types
  browser checks to CI without weakening the completed Phase 1-4 boundaries.

### 2026-07-28 - Security Audit and Hardening Phase 5 completed

- Verified `security/security-audit-fixes` at clean starting commit
  `5c3f4d110f807e66239fec8bbf37c56f9cbb92aa`, read the Phase 1-4 reports, and
  preserved the earlier finding IDs and closed boundaries.
- Replaced the single no-`kid` JWT secret with a strict HS256 key ring:
  exactly one active signer, deterministic `kid` selection, bounded previous
  verification, retired/unknown/legacy rejection, exact protected headers,
  clock-skew/lifetime limits, and fail-closed configuration.
- Added migration `0028` and implemented opaque logical-session inventory,
  owned/current-session revocation, logout-all with `auth_version` invalidation,
  exact-super-admin privileged revocation, and advisory-lock serialization
  against refresh/revocation races.
- Added a hash-only, purpose/user/binding-bound internal recovery and
  verification token foundation with bounded issue rate/TTL, supersession,
  revocation, atomic single-use consumption, reuse handling, and controlled
  security events. No public password-reset, email-verification, or email-change
  product flow is claimed.
- Pinned Argon2id v19 parameters, random salt and output length, and rejected
  passwords containing NUL or exceeding 1,024 UTF-8 bytes before expensive
  work.
- Removed raw organization invitation links from administrative responses and
  stored delivery payloads, redacted history in migration `0028`, erased
  finalized hashes, serialized acceptance and shared organization capacity,
  enforced recipient/active organization/server role boundaries, and removed
  the invitation query from browser history after capture.
- Added configurable stable-ID/`SKIP LOCKED` cleanup for sessions, security
  tokens/events, invitation hashes, and login attempts. It is bounded,
  idempotent, concurrent-safe, and transactionally rolls back on failure. The
  callable service has no repository-owned scheduler.
- Added Settings session management with current-session labeling,
  per-session revoke, logout-all confirmation, duplicate-submit prevention,
  plain-text rendering, and no browser persistence.
- Confirmed `SEC-P05-001`, `SEC-P05-002`, `SEC-P05-003`, and `SEC-P05-007`
  (High); `SEC-P05-004`, `SEC-P05-005`, and `SEC-P05-006` (Medium);
  `SEC-P05-008` (Low); and `SEC-P05-009` (Informational). No Critical finding
  was confirmed. The existing local `cms_user` role remains an open owner action
  because read-only inspection showed `SUPERUSER BYPASSRLS`.
- Final backend validation passed:
  `cargo fmt --all -- --check`;
  `cargo clippy --all-targets --all-features -- -D warnings`; and
  `PHASE2_TEST_DATABASE_URL=<LOCAL_TEST_DATABASE_URL>
  PHASE2_UPGRADE_TEST_DATABASE_URL=<LOCAL_UPGRADE_TEST_DATABASE_URL>
  PHASE5_UPGRADE_TEST_DATABASE_URL=<LOCAL_PHASE5_UPGRADE_TEST_DATABASE_URL>
  cargo test --all-features`. Results were 180 unit tests, two live Phase 2
  integration tests, one Phase 5 migration integration test, and no doc-test
  failure, all through a verified `NOSUPERUSER NOBYPASSRLS` application role.
- Final frontend validation passed: `npm run lint`, `npm run typecheck`,
  `npm run check:sinks`, `npm test` (47 tests in 12 files), and
  `npm run build`. The build retained only the known non-blocking chunk-size
  warning.
- Local and production `docker compose config --quiet` passed. Production-bundle
  browser checks passed for registration, invitation URL removal, bounded
  previous-key access, A-to-B key rotation/refresh bootstrap, two-session
  inventory without browser persistence, individual revoke, and logout-all.
  The Browser plugin runtime was unavailable, so the equivalent disposable
  headless Edge/CDP fallback was used. Vite's development React-refresh
  bootstrap remains incompatible with the strict Phase 4 CSP; the production
  bundle passed without weakening the policy.
- Final static closure passed: the report contains exactly 34 required headings
  and six English Mermaid diagrams; 42 changed/untracked files contain no
  Persian-script text; the production-shaped credential/secret and persisted
  test-value scans found no match; the report has no mojibake; and
  `git diff --check` passed.
- `cargo audit` was unavailable because the subcommand is not installed.
  `npm audit --omit=dev` was not run because external advisory-metadata
  transmission was not authorized. No production/staging environment was
  accessed and no live credential/key was rotated.
- Temporary browser scripts/profiles were removed; services/listeners on ports
  8080, 5173, and 9333 were stopped. Disposable databases
  `cms_phase5_fresh_20260728` and `cms_phase5_upgrade_20260728` plus roles
  `cms_phase5_app_20260728` and `cms_phase5_bootstrap_20260728` were dropped and
  verified absent. The PostgreSQL service was returned to its stopped state.
- **Created files:** `backend/migrations/0028_security_phase_five_key_session_recovery.sql`,
  `backend/src/services/invitations.rs`,
  `backend/src/services/security_audit.rs`,
  `backend/src/services/security_cleanup.rs`,
  `backend/src/services/security_tokens.rs`,
  `backend/tests/security_phase5_migration.rs`,
  `docs/security/PHASE_05_KEY_SESSION_RECOVERY_HARDENING.md`, and
  `frontend/src/pages/SettingsPage.test.tsx`.
- **Modified files:** `.env.example`, `.github/workflows/backend-ci.yml`,
  `HANDOFF.md`, `README.md`, `backend/src/config.rs`,
  `backend/src/routes/auth.rs`, `backend/src/routes/mod.rs`,
  `backend/src/routes/organizations.rs`, `backend/src/services/email.rs`,
  `backend/src/services/jwt.rs`, `backend/src/services/mod.rs`,
  `backend/src/services/password.rs`, `backend/src/services/sessions.rs`,
  `backend/tests/security_phase2_rls.rs`, `docker-compose.prod.yml`,
  `docs/API.md`, `docs/ARCHITECTURE.md`, `docs/V2_PHASE_EIGHT.md`,
  `docs/diagrams/06-production-deployment.mmd`, `env.example`,
  `frontend/src/pages/OrganizationPage.tsx`,
  `frontend/src/pages/SettingsPage.tsx`, `frontend/src/services/api.ts`,
  `frontend/src/types/api.ts`,
  `okf/api/endpoints/authentication-and-session.md`,
  `okf/backend/configuration-and-state.md`,
  `okf/frontend/features/authentication-and-session.md`,
  `okf/operations/environment-configuration.md`,
  `okf/security/audit-and-security-events.md`,
  `okf/security/authentication-architecture.md`,
  `okf/security/diagrams/session-token-lifecycle.mmd`,
  `okf/security/password-and-credential-handling.md`,
  `okf/security/secrets-and-configuration.md`, and
  `okf/security/session-and-token-lifecycle.md`.
- No file is staged or committed; no push, migration on an owner environment,
  deployment, external message, or production/staging mutation was performed.
- **Exact Next Action:** the owner must back up each existing environment,
  inspect and correct the application database role to
  `NOSUPERUSER NOBYPASSRLS` through an approved change procedure, provision a
  real `JWT_KEY_RING` through the deployment secret path, and assign cleanup
  scheduling/retention ownership. Then review the complete Phase 5 diff and
  explicitly authorize any stage/commit/push before beginning Phase 6 with the
  highest-priority remaining inherited finding.

### 2026-07-28 - Security Audit and Hardening Phase 6 completed

- Verified `security/security-audit-fixes` at clean starting commit
  `0f9fc4e9e927089cf2ebef9183237d4955c16921`; Phase 5 is committed and the
  staging area was empty.
- Read the Phase 6 requirements, project instructions, persistent lessons, and
  Phase 1-5 security reports. The repository has no existing MFA, TOTP,
  recovery-code, pre-authentication, or Step-Up implementation. The Phase 5
  password-reset service remains an internal token foundation without a public
  reset flow.
- Inventoried login/register/refresh/session paths and the account, global-role,
  organization-ownership, webhook credential, billing, and Marketplace
  sensitive-operation surfaces.
- Selected an assurance model of password/refresh `AAL1`, password plus TOTP or
  recovery-code `AAL2`, and short-lived session/scope-bound `STEP_UP`.
- Selected standards-compatible TOTP (`SHA-1`, six digits, 30-second period,
  160-bit OS-random secret, one adjacent time step each direction) through a
  maintained library. Accepted time steps are stored transactionally to
  prevent replay.
- Selected a dedicated `AES-256-GCM` MFA encryption key ring, separate from JWT
  keys, with one active encryptor, bounded previous decryptors, unique nonces,
  record-bound associated data, fail-closed decryption, and lazy re-encryption
  under the active key.
- Durable MFA state and recovery-code hashes will use PostgreSQL. Hash-keyed
  short-lived pre-authentication challenges, Step-Up challenges/grants, and
  distributed rate limits will use Redis and fail closed.
- Existing `super_admin` and `admin` accounts will retain normal login and MFA
  enrollment access, but selected privileged actions will require completed MFA
  enrollment and recent scope-specific Step-Up instead of silently locking
  existing accounts out.
- Test-first evidence is recorded: the first focused MFA test failed on the
  intentionally missing primitives/configuration, then passed after the
  maintained `totp-rs` and RustCrypto `aes-gcm` implementation was added.
- Added migration `0029` for durable MFA state, Argon2-verified recovery-code
  records, accepted-TOTP replay state, and session AAL/AMR/authentication
  context. Existing families upgrade to AAL1 with their current auth version.
- Added fail-fast dedicated MFA encryption-key configuration, bounded
  previous-key decryption, AES-256-GCM record-bound encryption, standard
  TOTP provisioning/QR/manual setup, and lazy key rotation after successful
  verification.
- Added password-confirmed enrollment, atomic confirmation, ten one-time
  recovery codes, exact-once recovery-code consumption, MFA disable and
  recovery-code regeneration, session revocation on MFA state changes, and
  expired-pending-enrollment cleanup.
- Added Redis hash-keyed pre-auth, Step-Up challenges, one-time session/scope
  grants, distributed attempt locks, bounded failure invalidation, and
  per-subject rate limits. Password login no longer creates any session when
  enabled MFA is pending.
- Access JWTs and refresh families now carry and validate `sid`, AAL, AMR,
  password-authentication time, MFA time, and auth version. Revoked,
  compromised, expired, stale-version, or context-mismatched families
  invalidate access and refresh.
- Added centralized Step-Up policy for high-impact session, MFA,
  organization, webhook, billing, plugin, Marketplace administration, and
  payout mutations. Grants are one-time and bound to the authenticated user,
  logical session, auth version, and exact scope.
- Added frontend MFA login, enrollment QR/manual fallback, display-once
  recovery-code acknowledgment, recovery fallback, and a reusable Step-Up
  dialog. Pre-auth, codes, and Step-Up values remain component/process memory
  only and are never stored in browser storage. Client-side pre-auth expiry now
  clears the in-memory challenge, failed recovery submissions clear the code,
  and logout clears pending MFA/Step-Up material.
- Local disposable PostgreSQL validation passed under
  `cms_phase6_app_20260728` (`NOSUPERUSER`, `NOBYPASSRLS`): fresh migration
  through 29, upgrade from 28 to 29, encrypted-secret/replay/concurrent
  recovery-code/key-rotation/session-revocation tests. Live Redis
  single-consumer/replay/absence tests also passed. The final live Redis
  expansion additionally passed distributed-worker TOTP/recovery bucket
  limits, TTL, hash-key privacy, fail-closed connection failure, and cleanup.
- Final validation after the disable-lifecycle fix passed:
  `cargo fmt --manifest-path backend/Cargo.toml -- --check`;
  `cargo clippy --offline --manifest-path backend/Cargo.toml --all-targets
  --all-features -- -D warnings`; and
  `cargo test --offline --manifest-path backend/Cargo.toml --all-features`.
  Results were 189 unit tests, two Phase 2 integration tests, one Phase 5
  migration test, one Phase 6 migration test, and doc tests. Final frontend
  `lint`, `typecheck`, 53 tests in 12 files, production build, and the
  one-approved-sink policy passed. The known non-blocking bundle-size warning
  remains.
- Additional focused tests passed for standard TOTP vectors, leading-zero
  preservation, adjacent/excessive clock skew, tampered ciphertext/nonce,
  wrong/unknown encryption keys, client pre-auth expiry, recovery input
  clearing, enrollment-secret non-persistence, one-time recovery display, and
  MFA-disable Step-Up. A fresh disposable live test proved that concurrent use
  of the same accepted adjacent-step TOTP produces exactly one success.
- Live in-app-browser validation passed registration, password-confirmed
  pending enrollment, QR/manual fallback, one-time recovery-code display,
  password-only pre-authentication with zero active sessions, TOTP login,
  TOTP replay denial, recovery login, recovery reuse denial, recovery-based
  Step-Up, scoped recovery replacement, TOTP-based disable Step-Up, session
  invalidation, return to login, and empty local/session browser storage. The
  production bundle was used because the development React-refresh preamble is
  incompatible with the strict Phase 4 CSP.
- Browser/postcondition verification discovered that disable deleted the MFA
  row and revoked sessions but left independently keyed recovery-code hashes.
  A live regression first failed with 10 remaining rows. Disable now deletes all
  recovery credentials in the same transaction; the focused live test passes
  with zero MFA/recovery rows and a revoked session.
- Created the exact 35-section English Phase 6 report with seven English Mermaid
  sequence diagrams and updated API, architecture, README, environment, OKF,
  security, frontend, and operations documentation.
- Confirmed and remediated `SEC-P06-001` (High: missing MFA for the selected
  privileged threat model), `SEC-P06-002` (High: missing Step-Up), and
  `SEC-P06-003` (Medium: disable left recovery hashes). No Critical, Low, or
  Informational Phase 6 finding was confirmed. The recent-reauthentication
  residual in `SEC-P05-002` is closed for the selected matrix; `SEC-P05-005`
  public recovery and `SEC-P05-007` existing-role correction remain open.
- Local and production Compose rendering passed. The report has exactly 35 H2
  sections and seven English Mermaid sequence diagrams. Changed/untracked files
  contain no Persian text and no production-shaped or browser-test credential
  artifacts. `git diff --check` passed.
- `cargo audit` was not run because the subcommand is unavailable.
  `npm audit --omit=dev` was not run because external advisory-metadata
  transmission was not authorized. The first production-browser tab attach was
  retried successfully; no final product test remains failed.
- One focused frontend assertion incorrectly required all browser storage to be
  empty and was corrected to reject credential values while permitting benign
  preferences. A disposable PostgreSQL setup was first blocked by the sandbox
  and then used a stale superuser assumption; no resource was created by those
  attempts. Production Compose also initially rejected absent required env
  values. All three checks passed after correction with scoped, validation-only
  inputs. The first compile of the expanded Redis test reused a moved Rust
  `String`; it was corrected before the focused live and final full Backend
  reruns.
- Disposable databases `cms_phase6_fresh_20260728` and
  `cms_phase6_upgrade_20260728` plus role `cms_phase6_app_20260728` were dropped
  and verified absent. The follow-up database `cms_phase6_focus_20260728` and
  role `cms_phase6_app_focus_20260728` were also dropped and verified absent.
  Redis contains no `zinhar:mfa:*` test key. Browser-test
  listeners on 8080/5173, tabs, the temporary TOTP helper, and its empty
  directory were removed. Pre-existing PostgreSQL, Redis, and pgAdmin services
  were left in their original running state.
- **Created files:** `backend/migrations/0029_security_phase_six_mfa_step_up.sql`,
  `backend/src/middleware/step_up.rs`, `backend/src/services/mfa.rs`,
  `backend/src/services/mfa_accounts.rs`,
  `backend/src/services/mfa_challenges.rs`,
  `backend/tests/security_phase6_migration.rs`,
  `docs/security/PHASE_06_MFA_TOTP_STEP_UP_HARDENING.md`, and
  `frontend/src/components/StepUpDialog.tsx`.
- **Modified files:** `.env.example`, `.github/workflows/backend-ci.yml`,
  `HANDOFF.md`, `README.md`, backend Cargo manifests, configuration, main,
  auth/tenant middleware, auth/OpenAPI/webhook routes, JWT/session/audit/cleanup
  services and the Phase 2 migration test; `docker-compose.prod.yml`,
  `docs/API.md`, `docs/ARCHITECTURE.md`, `env.example`; frontend auth/settings
  pages, tests, API/types; and the Phase 6-related API/backend/frontend/
  operations/security OKF documents.
- No file is staged or committed. No push, deployment, production/staging
  access, owner-environment migration, live key rotation, or external message
  occurred.
- **Exact Next Action:** the owner should back up each environment, correct the
  application database role to `NOSUPERUSER NOBYPASSRLS`, provision independent
  production JWT and MFA key rings through the approved secret manager, apply
  migration 0029 in an approved non-production environment, enroll privileged
  accounts, and review the complete Phase 6 diff. Stage/commit/push only after
  explicit authorization. Phase 7 should define mandatory privileged
  enrollment, administrative MFA recovery, rotation drills, alert ownership,
  and production performance/chaos validation.

### 2026-07-29 - Security Audit and Hardening Phase 7 inventory checkpoint

- Verified `security/security-audit-fixes` at clean starting commit
  `518f74a1b0da5c4ee37c14e2a37a716707468410`; Phase 6 is committed, the index
  was empty, and no branch, commit, push, deployment, or owner environment was
  changed.
- Read `AGENTS.md`, this complete handoff, the Phase 7 requirements, the
  persistent lessons, and the complete Phase 1-6 security reports before
  implementation.
- Inventoried repository file surfaces. Runtime uploads are limited to tenant
  media multipart upload and creator-owned Marketplace ZIP upload. Local
  filesystem storage under `UPLOAD_DIR` also holds generated image variants
  and Marketplace artifacts. `avatar_url`, Marketplace screenshots, template
  asset mappings, backup/restore documentation, and creator CLI archives are
  references or tooling surfaces, not additional server upload handlers.
- Confirmed the current media/package handlers buffer complete multipart file
  parts in memory. The image decoder has no explicit dimension/allocation
  limits, and Marketplace validation uses a custom central-directory parser.
- Confirmed the public `ServeDir` path admits generated organization/UUID media
  names regardless of stored media policy; PDF and text uploads therefore use
  public unguessable URLs rather than authenticated download authorization.
- Confirmed original bytes are written before relational commit, deletion is
  best-effort after row deletion, there is no quarantine/cleanup reconciliation
  model, and `ensure_media_capacity` is a read-then-check quota vulnerable to
  concurrent over-commit.
- Existing positive controls remain: tenant middleware and forced RLS protect
  metadata routes; generated object names avoid direct client path selection;
  Marketplace object keys validate slugs/version/checksum; package code is
  never extracted or executed; install rechecks size and SHA-256; the
  production backend runs as UID 10001 with a writable uploads volume.
- **Exact Next Action:** add failing Phase 7 backend contract tests for central
  upload policy, bounded streaming, path/filename handling, image limits,
  archive containment/bomb limits, private/public response policy, atomic
  quotas, and cleanup state; then implement migration `0030` and the central
  file-security/storage services without weakening Phase 1-6 controls.

### 2026-07-30 - Security Audit and Hardening Phase 7 implementation checkpoint

- Added central, purpose-specific upload policies and disk-backed bounded
  multipart staging. Client filenames are display metadata only; generated
  tenant/UUID storage keys, containment checks, existing symlink/reparse-point
  rejection, create-new publication, secure temporary permissions, stale-temp
  cleanup, SHA-256 metadata, and explicit scanner-unavailable state now protect
  the file lifecycle.
- Raster media is decoded under explicit dimension/pixel/allocation limits and
  re-encoded to WebP before public publication. PDF and UTF-8 text are
  restricted attachments served only by authenticated tenant-scoped download;
  PDF structure and the complete streamed text encoding are validated. SVG,
  HTML, mismatched types, malformed images, and active content are rejected.
- Removed broad `ServeDir` delivery. Public image lookup now requires an active,
  verified public database record; restricted downloads apply safe disposition,
  `nosniff`, sandbox CSP, private no-store caching, and reject Range requests.
- Marketplace ZIP intake now streams to quarantine and uses a bounded parser
  that rejects traversal, absolute/Windows/encoded paths, links and special
  files, duplicate/case-colliding/non-ASCII paths, nested archives, unsupported
  compression/encryption/ZIP64, CRC/size mismatches, excessive entries, depth,
  expansion ratio, total output, and processing time. Package bytes are not
  extracted or executed by the application.
- Added migration `0030` for media/variant checksums, visibility, verification
  and lifecycle state, Marketplace artifact state, and forced-RLS durable
  cleanup jobs. Legacy media/packages are restricted and unverified. Quota
  reservations now lock the tenant row and include publishing/deletion-pending
  bytes in the same transaction; delete/publish failures use idempotent cleanup
  jobs rather than silent best effort.
- Frontend upload UX now has advisory file filters, limits, duplicate-submit
  prevention, cancellation, authenticated Blob downloads, public inline
  rendering only for verified raster media, and safe React text rendering.
  Deployment configuration adds non-root read-only runtime controls and
  bounded upload settings.
- Current focused validation passed: `cargo check --offline --all-targets
  --all-features`; all 16 Phase 7 filesystem/archive/image tests; frontend
  typecheck; and all five focused MediaPage tests. Fresh and upgrade migration
  tests both passed against disposable databases owned by a
  `NOSUPERUSER/NOBYPASSRLS` role, including cross-tenant non-enumeration,
  forced RLS, legacy classification, and concurrent quota reservation.
  Both temporary databases and the temporary role were dropped in `finally`.
- Docker Desktop was started for local validation; PostgreSQL and Redis project
  services are running. No commit, stage, push, deployment, production access,
  or owner-environment migration occurred.
- **Exact Next Action:** run the complete backend/frontend/static/deployment
  regression matrix, perform disposable in-app-browser upload/download and
  cross-tenant checks, then create the exact 41-section Phase 7 report and
  finalize this handoff with cleanup evidence and the Phase 8 recommendation.

### 2026-07-30 - Security Audit and Hardening Phase 7 final checkpoint

- Phase 7 source implementation and non-browser validation are complete.
  Confirmed/remediated `SEC-P07-001` (High: public raw document/static delivery),
  `SEC-P07-002` (High: multipart/image/archive resource exhaustion), and
  `SEC-P07-003` (Medium: quota and file/database lifecycle races). No Critical,
  Low, or Informational Phase 7 finding was confirmed. The earlier
  `SEC-P01-004` closure is strengthened by removing the shared static root and
  requiring database state for every public object.
- Final media behavior streams to secure quarantine; validates byte type, PDF
  structure, complete UTF-8, image decode limits, and source size; re-encodes
  public images to checked WebP; forces PDF/text through tenant-authenticated
  attachment delivery; rejects SVG/HTML and Range; applies safe disposition,
  cache, `nosniff`, and sandbox CSP headers; and preserves the stricter
  route-specific CSP through global middleware.
- Final archive behavior streams Marketplace ZIPs to nonpublic quarantine and
  rejects traversal, encoded/Windows/absolute paths, links/special files,
  duplicates/case collisions/non-ASCII ambiguity, nested archives,
  encryption/ZIP64/multidisk/unsupported compression, overlapping/mismatched
  records, CRC/actual-size mismatch, excess entry/path/depth/ratio/expanded
  bytes, and processing timeout. Packages remain data only and require reviewed
  artifact state before install/update/rollback.
- Migration `0030` adds file checksums, visibility/verification/scanner and
  lifecycle state, variants, Marketplace artifact state, constraints/indexes,
  and forced-RLS cleanup jobs. Legacy media/artifacts become restricted and
  unverified. Quota reservation is locked/atomic. Deletion, publish rollback,
  stale source/processing cleanup, and stale-publishing reconciliation are
  bounded and idempotent; a trusted scheduled worker remains an owner action.
- Frontend behavior now has advisory accept types, a 25 MiB UX ceiling,
  cancellation/unmount cleanup, duplicate-submit prevention, authenticated Blob
  downloads and URL revocation, verified-raster-only inline rendering, and safe
  React text rendering. Production Compose has a read-only root filesystem,
  capability drop, `no-new-privileges`, `init`, restricted tmpfs, and a
  dedicated writable upload volume.
- Final Backend validation passed: `cargo fmt --all -- --check`;
  `cargo clippy --offline --all-targets --all-features -- -D warnings`; and
  `cargo test --offline --all-features`. Results were 192 unit tests, two Phase
  2 integration tests, one Phase 5 migration test, one Phase 6 migration test,
  17 Phase 7 filesystem/archive/image/cleanup tests, two Phase 7 migration tests,
  and doc tests.
- Live Phase 7 PostgreSQL validation passed fresh and 0029-to-0030 upgrade
  databases owned by a disposable `NOSUPERUSER NOBYPASSRLS` role. It verified
  schema/constraints, forced RLS, two-way tenant non-enumeration, concurrent
  near-limit quota, legacy classification, and stale-publishing reconciliation.
- Frontend lint, typecheck, all 58 tests in 13 files, production build, and the
  one-approved-HTML-sink policy passed. The known nonblocking bundle-size
  warning remains. Five focused MediaPage tests passed.
- Disposable live HTTP validation passed health, private text upload,
  malicious-filename handling, anonymous private denial, private headers/body,
  Range 416, SVG rejection, MIME mismatch, public image decode/re-encode and
  headers, two-way cross-tenant IDOR denial, and zero remaining `.part` files.
  This testing found and fixed the global-middleware CSP overwrite before final
  validation.
- Local and production Compose rendering, BuildKit Dockerfile policy checking,
  and `git diff --check` passed. The exact report
  `docs/security/PHASE_07_FILE_UPLOAD_STORAGE_HARDENING.md` has exactly 41
  required H2 sections and seven English Mermaid diagrams.
- Changed/untracked source was scanned without printing values: zero
  production-shaped token/private-key matches, zero Phase 7 test-credential
  artifacts, and no Persian text outside the explicitly allowed frontend i18n
  file. A local deterministic development-config value surfaced during an
  earlier diagnostic; it was not reproduced and remains tracked by
  `POTENTIAL_SECRET_EXPOSURE PSE-01`. Rotate it if reused outside disposable
  local development.
- Browser verification is **unavailable/partial**, not passed: the mandatory
  in-app Browser runtime failed before commands with an internal kernel-asset
  path error and still failed after its documented reset/retry. Standalone
  browser automation was not substituted. Browser cookies/storage/profile
  inspection is also prohibited by the selected Browser skill. Unit and live
  HTTP results are not mislabeled as browser evidence.
- `cargo audit` is unavailable because the subcommand is not installed. A new
  ZIP dependency could not be retrieved because registry DNS/network access
  failed; locked cached `flate2`/`crc32fast` support the strict reader.
  `npm audit --omit=dev` was not run because external advisory-metadata
  transmission lacked separate authorization. No real scanner, object store,
  signed URL, CDN, ingress, backup/restore, or owner filesystem was tested.
- Cleanup is complete: every disposable Phase 7 database and role was dropped
  and verified at count zero; the HTTP backend, port 18087, temporary harness,
  test storage/log root, and browser binding were removed. Docker Desktop was
  started only for validation and returned to its initial stopped state. No
  commit, stage, push, deployment, production/staging access, owner migration,
  or external message occurred.
- **Created files:** `backend/migrations/0030_security_phase_seven_file_storage.sql`,
  `backend/src/services/file_cleanup.rs`,
  `backend/src/services/file_security.rs`,
  `backend/tests/security_phase7_files.rs`,
  `backend/tests/security_phase7_migration.rs`,
  `docs/security/PHASE_07_FILE_UPLOAD_STORAGE_HARDENING.md`, and
  `frontend/src/pages/MediaPage.test.tsx`.
- **Modified files:** `.env.example`, `HANDOFF.md`, backend Cargo manifests,
  `Dockerfile.prod`, config/security middleware, Marketplace/media routes,
  route registration, Marketplace/media/file/quota services;
  `docker-compose.prod.yml`, `docs/API.md`, `docs/ARCHITECTURE.md`; frontend
  i18n/MediaPage/API/types; and OKF backend configuration/security overview.
- **Exact Next Action:** the owner should review the complete Phase 7 diff,
  back up database and storage, verify every application role is
  `NOSUPERUSER NOBYPASSRLS`, test migration 0030 and legacy private-download/
  Marketplace-review compatibility in approved non-production, provision the
  hardened upload volume, wire and monitor a trusted cleanup/scanner worker,
  and repeat browser/ingress/backup validation. Stage/commit/push only after
  explicit authorization. Phase 8 should address audit logging, redaction,
  observability, alert ownership, scheduled lifecycle workers, dependency/
  container advisory gates, and upload/storage performance and chaos testing.
