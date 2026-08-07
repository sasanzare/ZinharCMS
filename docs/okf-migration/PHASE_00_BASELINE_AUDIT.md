# ZinharCMS Google OKF Migration — Phase 0 Baseline Audit

**Phase:** 0 — Baseline Audit, Evidence Collection, and Migration Boundary Definition  
**Audit date:** 2026-08-07 (Europe/London)  
**Status:** COMPLETE

## Objective

Establish a reproducible, evidence-backed baseline for replacing the repository's
legacy OKF convention with the official Google Open Knowledge Format (OKF) v0.2.
This report records what exists, how it is referenced, where it diverges from the
official specification, what executable coupling was found, and what knowledge
must be preserved before any later removal or conversion.

The legacy format is incorrect as a Google OKF implementation. That does not
make the project knowledge inside it incorrect. All preservation conclusions in
this phase are therefore candidates for verification, not migration decisions.

## Scope and exclusions

This audit inspected the tracked repository, the current worktree state, the
complete `okf/` and `okf-bootstrap/` trees, repository references, executable
configuration, and the official Google specification.

The following were explicitly excluded:

- deleting, renaming, rewriting, or converting `okf/` or `okf-bootstrap/`;
- replacing `okf/index.yaml` or creating a final Google OKF bundle;
- application, API, database, authentication, authorization, CI, or dependency
  changes;
- copying Google's reference agent, visualizer, examples, or tooling into the
  repository;
- selecting a target taxonomy or deciding KEEP/MIGRATE/MERGE/DELETE disposition.

Only this audit documentation and the required `HANDOFF.md` checkpoint are
Phase 0 changes. The pre-existing dirty worktree described below is preserved
and is not attributed to this phase.

## Evidence method

Repository conclusions were checked against current source/configuration first,
then migrations, tests, scripts, CI, and current documentation. Historical
claims in `okf-bootstrap/` and legacy OKF documents are identified as historical
or supporting evidence. The upstream source of truth was read directly from
Google's [`okf/SPEC.md`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
on the audit date.

## Git baseline

The baseline was captured before Phase 0 files were created.

| Item | Baseline evidence |
| --- | --- |
| Branch | `security/security-audit-fixes` |
| Starting HEAD | `518f74a1b0da5c4ee37c14e2a37a716707468410` (`security: complete phase 6 MFA, TOTP, and step-up hardening`) |
| Starting worktree | Dirty before this phase |
| Staged paths | None |
| Pre-existing unstaged modified paths | 23 |
| Pre-existing untracked paths | 7 |
| Pre-existing diff stat | 1,780 insertions, 632 deletions |
| Commit/push at baseline | None performed by this phase |

Pre-existing modified paths:

`.env.example`, `HANDOFF.md`, `backend/Cargo.lock`, `backend/Cargo.toml`,
`backend/Dockerfile.prod`, `backend/src/config.rs`,
`backend/src/middleware/security.rs`, `backend/src/routes/marketplace.rs`,
`backend/src/routes/media.rs`, `backend/src/routes/mod.rs`,
`backend/src/services/marketplace_validation.rs`,
`backend/src/services/media_processing.rs`, `backend/src/services/mod.rs`,
`backend/src/services/quota.rs`, `docker-compose.prod.yml`, `docs/API.md`,
`docs/ARCHITECTURE.md`, `frontend/src/i18n/messages.ts`,
`frontend/src/pages/MediaPage.tsx`, `frontend/src/services/api.ts`,
`frontend/src/types/api.ts`, `okf/backend/configuration-and-state.md`, and
`okf/security/overview.md`.

Pre-existing untracked paths:

`backend/migrations/0030_security_phase_seven_file_storage.sql`,
`backend/src/services/file_cleanup.rs`, `backend/src/services/file_security.rs`,
`backend/tests/security_phase7_files.rs`,
`backend/tests/security_phase7_migration.rs`,
`docs/security/PHASE_07_FILE_UPLOAD_STORAGE_HARDENING.md`, and
`frontend/src/pages/MediaPage.test.tsx`.

The ending HEAD remains the starting HEAD. Phase 0 does not stage, commit, push,
reset, clean, stash, or discard any path. The final status is recorded in
`HANDOFF.md` and in the completion validation below.

## Repository baseline

At the start of the audit, `git ls-files` reported 738 tracked files. Relevant
tracked counts were:

| Area | Tracked files | Evidence/role |
| --- | ---: | --- |
| `backend/` | 124 | Rust application, migrations, tests, and container assets |
| `frontend/` | 68 | React/TypeScript application and tests |
| `docs/` | 121 | Current, historical, operational, Marketplace, and diagram documentation |
| `okf/` | 390 | Legacy OKF bundle: 339 Markdown, 50 Mermaid, 1 YAML index |
| `okf-bootstrap/` | 14 | Historical Phase Zero inventory and planning reports |
| `scripts/` | 7 | Release, Marketplace, smoke, and consistency tooling |
| `.github/workflows/` | 2 | Backend and frontend CI |

Ignored/generated boundaries are defined in `.gitignore`: `backend/target`,
`frontend/node_modules`, `frontend/dist`, `marketplace-dist`, uploads, and local
environment/log artifacts are not treated as source knowledge.

The repository is a Rust/Axum/SQLx backend plus a React/Vite frontend with
PostgreSQL, Redis, local/reference filesystem storage, Docker Compose, GitHub
Actions, and extensive Markdown/Mermaid documentation. This technology summary
is corroborated by `backend/Cargo.toml`, `frontend/package.json`,
`backend/migrations/`, `docker-compose.yml`, `docker-compose.prod.yml`,
`.github/workflows/`, and `README.md`.

## Legacy OKF overview

The current `okf/` tree is a large project knowledge base organized by
architecture, backend, database, API, security, domain, extensibility, frontend,
delivery, development, operations, maintenance, project, and references. It is
not an implementation of Google's v0.2 contract:

- all 339 Markdown files begin with YAML frontmatter, but none has the required
  Google `type` field;
- every concept-like file uses a custom metadata model centered on
  `okf_document_id`, `category`, `phase`, `status`, `source_of_truth`,
  `last_verified_commit`, `last_verified_date`, `primary_sources`,
  `related_documents`, and uncertainty/status fields;
- the root navigation and registry are in a 411,888-byte, 8,527-line
  `okf/index.yaml` whose first field is `okf_version: "0.1"`;
- no `index.md` or `log.md` exists anywhere under `okf/`; those files are
  optional in Google OKF, but the current system relies on the YAML registry;
- 50 `.mmd` files are stored inside the tree as project diagrams. They are
  valuable artifacts, but they are not Markdown concepts and their treatment in
  a future bundle must be decided explicitly;
- the current validation model checks custom frontmatter schemas, YAML/index
  parity, metadata relations, repository evidence paths, status markers,
  secrets, and static Mermaid structure. The validation report states that this
  logic is not a tracked script or CI job.

The detailed path and field inventory is in
[`LEGACY_OKF_INVENTORY.md`](LEGACY_OKF_INVENTORY.md).

## `okf/` and `okf-bootstrap/` findings

### `okf/`

The 14 top-level areas and file counts are:

| Area | Files | Observed role |
| --- | ---: | --- |
| `api/` | 62 | Route, request/response, authentication, endpoint-family, and API-risk records |
| `architecture/` | 15 | Runtime boundaries, components, dependencies, decisions, and diagrams |
| `backend/` | 35 | Module, service, persistence, error, configuration, and test records |
| `database/` | 39 | Schema, entity, migration, relationship, RLS, lifecycle, and test records |
| `delivery/` | 14 | CI, containers, release, deployment, migration, and rollback records |
| `development/` | 11 | Prerequisites, commands, testing, contribution, and local workflow records |
| `domain/` | 50 | Business rules, invariants, state transitions, domains, and workflows |
| `extensibility/` | 46 | Plugins, Marketplace, extension points, permissions, adapters, and lifecycle records |
| `frontend/` | 36 | Routes, features, state, API client, builder, localization, and tests |
| `maintenance/` | 10 | Update policy, ownership, staleness, conflicts, validation, and completion records |
| `operations/` | 19 | Topology, configuration, observability, recovery, troubleshooting, and runbooks |
| `project/` | 4 | Overview, repository map, glossary, and navigation |
| `references/` | 1 | Source register |
| `security/` | 46 | Authentication, authorization, roles, permissions, tenancy, threats, and diagrams |

All of these areas contain potentially useful knowledge. The absence of Google
fields is a format gap, not evidence that the underlying claims can be deleted.

### `okf-bootstrap/`

`okf-bootstrap/` contains 14 plain Markdown reports with no frontmatter. They
cover repository inventory (`00`), technology (`01`), architecture (`02`),
modules (`03`), documentation (`04`), Mermaid (`05`), database (`06`), API
(`07`), conventions (`08`), knowledge gaps (`09`), proposed structure (`10`),
implementation phases (`11`), owner questions (`12`), and a summary. These
reports contain unique reasoning about unknowns, documentation conflicts,
owner decisions, proposed sequencing, and evidence boundaries.

The bootstrap summary still says that no final `okf/` directory had been
created. That statement is historical and is contradicted by the later Git
history (`docs(okf): complete phase one...` through Phase 10). It must be
preserved as history, not treated as current repository state.

## Official Google OKF v0.2 baseline

The current upstream `main` specification still identifies itself as **Open
Knowledge Format Version 0.2**. No upstream version change from the expected
v0.2 was detected on 2026-08-07. The authoritative document is the official
Google [`SPEC.md`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md);
the supporting repository tree is
[`GoogleCloudPlatform/knowledge-catalog/okf`](https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf).

The specification is intentionally minimal: a bundle is a self-contained
hierarchical collection of Markdown concepts with YAML frontmatter. It has no
schema registry, required runtime, required validator, fixed taxonomy, or
prescribed storage/serving infrastructure.

The exact requirement split is recorded in
[`GOOGLE_OKF_V02_BASELINE.md`](GOOGLE_OKF_V02_BASELINE.md). In brief:

- **MUST/conformance:** every non-reserved `.md` file has parseable YAML
  frontmatter and a non-empty `type`; reserved `index.md`/`log.md`, when
  present, follow their defined structures; consumers tolerate unknown types,
  unknown extension keys, broken links, missing optional fields, and missing
  index files;
- **SHOULD/recommended:** use descriptive `type`, `title`, `description`,
  `resource`, and `tags`; use standard Markdown links; record provenance in
  `sources`; use `generated`, `verified`, `status`, and `stale_after` when the
  corresponding trust/lifecycle facts matter; prefer Git distribution;
- **MAY/optional:** include `index.md`, `log.md`, bundle-root `okf_version`,
  `references/`, provenance credibility signals, verification events, and
  attested computation families;
- **non-goals:** fixed taxonomy, storage/query infrastructure, replacement of
  domain schemas, and a packaging/invocation standard for executors/attesters.

## Repository integration and reference audit

The important repository references are catalogued in
[`LEGACY_REFERENCE_MAP.md`](LEGACY_REFERENCE_MAP.md). The observed integration
is documentation and workflow coupling, not application runtime coupling:

| Area | Finding |
| --- | --- |
| Human navigation | `README.md` links to `okf/` entry points and describes both `okf/` and `okf-bootstrap/` as repository knowledge. |
| Legacy navigation | `okf/README.md` directs readers to `index.yaml`; all legacy documents use internal Markdown links and custom frontmatter relation arrays. |
| Historical handoff | `HANDOFF.md` contains active-looking and historical OKF Phase Zero/One instructions and references to the legacy paths. The new checkpoint must supersede them without deleting history. |
| Supporting documentation | Security phase documents and the bootstrap reports link to or name legacy OKF paths as evidence/navigation. |
| Tooling and CI | No exact `okf/`, `okf-bootstrap`, or `index.yaml` reference was found in Rust, frontend source, root scripts, package scripts, Docker files, or the two workflow definitions after excluding documentation and legacy trees. The custom validator is described as untracked and non-CI in `okf/maintenance/staleness-detection.md`. |

### Runtime coupling conclusion

**No executable dependency on the legacy OKF was found.** Deleting the legacy
tree in a future phase should not change application compilation, tests, API
runtime, authentication, database migrations, frontend behavior, Docker build
behavior, release scripts, or CI based on the inspected tracked sources. It
would change documentation navigation, handoff/history references, legacy
validation/review procedures, and any human or agent workflow that starts from
`README.md` or `okf/README.md`. This conclusion is limited to repository
evidence; it does not prove that an external operator or untracked local tool
does not read the files.

## Current versus official gap summary

| Severity | Legacy behavior | Google OKF v0.2 behavior | Evidence | Migration implication |
| --- | --- | --- | --- | --- |
| Critical | 339 Markdown files have custom frontmatter but no `type`. | `type` is the only always-required concept key; every non-reserved Markdown file must have it. | `okf/` field scan; Google SPEC §4.1 and §11 | Every candidate concept needs evidence-backed type mapping before it can be called conformant. |
| High | `okf/index.yaml` is the central registry and declares `okf_version: "0.1"`. | `index.md` is the optional directory listing; a root `index.md` may declare `okf_version: "0.2"`. | `okf/index.yaml`; Google SPEC §§3.1, 8, 12 | Do not replace the YAML registry until its navigation, status, evidence, and marker information has a preservation map. |
| High | Provenance is represented by `primary_sources`, `related_documents`, and a source register. | `sources` records per-concept derivation; `sources[].resource` is required and claim attribution uses source-ID footnotes. | `okf/README.md`, `okf/references/source-register.md`; Google SPEC §5.1 | Preserve source identity and claim scope before designing `sources` entries. |
| High | Trust/freshness uses `review_status`, `confidence`, `source_of_truth`, and verification commit/date. | `generated`, `verified`, `status`, and `stale_after` carry distinct optional semantics. | Legacy frontmatter scan; Google SPEC §§5.2–5.5 | Do not mechanically rename fields; separate authorship, verification, lifecycle, and staleness facts. |
| High | The legacy status vocabulary is mostly `current`, with phase-specific implementation/status fields. | Defined lifecycle examples are `draft`, `stable`, and `deprecated`; unknown extensions must be tolerated. | `okf/index.yaml`, legacy frontmatter scan; Google SPEC §5.4 and §11 | Decide whether legacy values are extensions, claims requiring verification, or historical labels. |
| Medium | Custom IDs (`okf_document_id`) and index IDs are authoritative alongside file paths. | Concept identity is the bundle file path without `.md`. | Legacy frontmatter/index; Google SPEC §2 | Check every ID/path relationship for duplicates or loss; do not assume IDs can be discarded. |
| Medium | 50 Mermaid diagrams are mixed into the legacy tree and indexed as first-class artifacts. | Concepts are Markdown; `references/` is a convention, not a requirement, and the spec does not define Mermaid semantics. | `okf/**.mmd`; Google SPEC §§3, 6.3 | Decide whether each diagram stays as a project artifact, is linked, or becomes a separately evidenced concept. |
| Informational | No `index.md` or `log.md` is present. | Both reserved files are optional. | Filesystem scan; Google SPEC §§3.1, 8, 9 | Absence alone is not a conformance failure; it is a navigation/history design decision. |

## Knowledge-preservation risk surface

Immediate deletion would remove or orphan more than format metadata. The main
preservation candidates are:

| Knowledge area | Candidate evidence | Loss risk |
| --- | --- | --- |
| Architecture and boundaries | `okf/architecture/`, `okf-bootstrap/02-architecture-observations.md` | Trust boundaries, dependency exceptions, runtime flows, and unresolved deployment limits |
| Backend modules and services | `okf/backend/`, `okf-bootstrap/03-module-inventory.md` | Module ownership observations, request/service/persistence relationships, error and test maps |
| Frontend | `okf/frontend/`, `okf-bootstrap/01-technology-inventory.md`, `04-documentation-audit.md` | Route/feature/state boundaries, builder and localization limits, client contract drift |
| API | `okf/api/`, `okf-bootstrap/07-api-inventory.md` | Route families, auth/tenant boundaries, error/contract caveats, OpenAPI gaps |
| Database and RLS | `okf/database/`, `okf-bootstrap/06-database-inventory.md` | Migration-defined schema, entity relationships, tenant controls, lifecycle and runtime-state unknowns |
| Security, authentication, authorization, tenancy | `okf/security/`, related `okf/api/` and `okf/database/` records | Roles, permissions, session flows, RLS assumptions, threat/risk markers, and current dirty security updates |
| Marketplace and extensibility | `okf/extensibility/`, `okf/domain/`, related API/backend/database records | Review, installation, entitlement, adapter, finance, feedback, and non-execution boundaries |
| Operations, deployment, release, development, and testing | `okf/operations/`, `okf/delivery/`, `okf/development/`, `okf/maintenance/` | Local commands, CI scope, container boundaries, recovery gaps, validation assumptions, and operational unknowns |
| External integrations and limitations | `okf/references/source-register.md`, `okf/project/glossary.md`, source paths throughout | Evidence provenance, terminology, provider boundaries, and explicit not-implemented/unknown claims |
| Historical bootstrap reasoning | All 14 `okf-bootstrap/*.md` reports, especially `09-knowledge-gaps.md` and `12-owner-questions.md` | Owner questions, conflict register, proposed sequencing, and why production claims remain unknown |

This is a risk surface only. The detailed disposition belongs to Phase 1.

## Migration risks

The prioritized register is in
[`MIGRATION_RISK_REGISTER.md`](MIGRATION_RISK_REGISTER.md). Highest risks are
knowledge loss, provenance loss, broken navigation after `index.yaml` removal,
stale claims being treated as current, and accidentally carrying the legacy
schema into a new directory without semantic review.

## Unresolved questions

Repository evidence does not answer the legacy-to-Google mapping decisions for
concept types, bundle scope, diagram handling, preservation of custom IDs,
source/claim attribution, generated-versus-curated authorship, verification
actors, lifecycle mapping, or whether a future `index.md`/`log.md` is useful.

The existing owner questions in `okf-bootstrap/12-owner-questions.md` remain
relevant, especially NOC-01 through NOC-15 and NOC-17/NOC-18. They cover public
routing, storage, recovery, observability, privacy/retention, deployment,
failure guarantees, Marketplace scope, supported toolchains, API policy,
documentation ownership, testing policy, module ownership, artifact retention,
and terminology. They are not blockers for the repository-derived preservation
map, but they must not be silently resolved by format conversion.

## Recommendations for Phase 1

The next phase is exactly:

**Phase 1 — Legacy OKF Inventory & Knowledge Preservation Map**

It should:

1. inventory every legacy document, diagram, index section, source-register
   entry, marker, and bootstrap report;
2. map each candidate claim to current code, migrations, tests, configuration,
   scripts, or authoritative documentation;
3. identify unique knowledge, duplicated knowledge, stale claims, historical
   records, and unresolved decisions without assigning final deletion status;
4. preserve custom IDs, evidence paths, dates, uncertainty markers, and links in
   a reviewable mapping before any conversion or deletion;
5. keep the old tree intact and avoid inventing a Google taxonomy until the
   evidence map is complete.

## Phase 0 completion status

**COMPLETE.** The legacy trees were inspected without deletion or conversion;
repository references and executable coupling were searched; the current
official Google specification was read directly and recorded; divergences,
preservation candidates, risks, unresolved questions, and the next phase are
documented. Production topology and owner-policy questions remain explicitly
unknown, as permitted by the Phase 0 boundary.

## Validation record

The following checks were performed for this report set:

- re-read `AGENTS.md` and `HANDOFF.md` before changes;
- captured branch, HEAD, staged/unstaged/untracked state, diff stat, and recent
  commits;
- enumerated tracked files and the complete `okf/`/`okf-bootstrap/` trees;
- counted Markdown, YAML, Mermaid, frontmatter, reserved-file, and custom-field
  inventories;
- searched exact legacy paths/terms and custom metadata names across tracked
  repository areas, excluding generated/dependency trees;
- inspected root scripts, package scripts, backend/frontend source patterns,
  Docker files, `.github/workflows/`, and `.gitignore` for executable coupling;
- read the official Google `SPEC.md` directly and compared its MUST/SHOULD/MAY
  and non-goal language to the legacy tree;
- confirmed no legacy file was deleted or intentionally rewritten by Phase 0.

Final link/path and Git checks are recorded in `HANDOFF.md` and must remain
truthful if the worktree changes after this handoff.
