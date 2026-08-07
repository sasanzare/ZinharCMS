# Legacy OKF Reference Map

This map records repository locations that depend on or refer to the existing
legacy OKF convention. It distinguishes human/documentation coupling from
executable coupling. The inventory is based on exact searches for `okf/`,
`okf-bootstrap`, `index.yaml`, and discovered legacy field names; broad
substring matches such as `WebhookFailed` were excluded.

For the structure and field details behind these references, see
[`LEGACY_OKF_INVENTORY.md`](LEGACY_OKF_INVENTORY.md). For the overall conclusion,
see [`PHASE_00_BASELINE_AUDIT.md`](PHASE_00_BASELINE_AUDIT.md).

## Reference map

| Location | Reference | Category | Impact | Evidence/Notes |
| --- | --- | --- | --- | --- |
| `README.md:98`, `README.md:110`, `README.md:195-196`, `README.md:232-234`, `README.md:317-319`, `README.md:352-353` | Links to `okf/architecture/README.md`, development/operations/API/maintenance documents, and security documents | Documentation navigation | High for human/agent discovery; no runtime effect | Root README presents the legacy tree as the complete Open Knowledge Format knowledge base |
| `README.md:335-336` | Describes `okf/` as structured/indexed knowledge and `okf-bootstrap/` as original inventory/planning analysis | Documentation/navigation | Medium | Deleting either directory without replacement would break the repository layout explanation |
| `HANDOFF.md:390-393` | “OKF Phase Zero” override names `okf-bootstrap/` and says no final `okf/` directory existed | Operational handoff/history | High for continuation safety | Historical wording is contradicted by later Git history and must be superseded by the current checkpoint, not erased |
| `HANDOFF.md:504-507`, `HANDOFF.md:1550-1552`, `HANDOFF.md:1590-1602`, `HANDOFF.md:2024-2050` | Historical Phase Zero acceptance and old next-action instructions | Operational handoff/history | High for future agents | These sections can cause an agent to restart or create the wrong bundle if the active checkpoint is not clear |
| `HANDOFF.md:2571-2580` and current security checkpoint sections | Lists legacy OKF files updated alongside security phases | Documentation synchronization | Medium/High | Shows the legacy tree is maintained as part of security documentation workflow; current dirty paths must be preserved |
| `okf/README.md` | Links to `index.yaml`, all legacy section entry points, source register, maintenance policy, and old phase navigation | Legacy navigation | High | This is the main legacy reader/agent entry point and contains custom authority/uncertainty rules |
| `okf/index.yaml` | Declares `okf_version: "0.1"`, lists 390 path entries, evidence paths, related documents, markers, current/planned sections, and legacy metadata | Legacy tooling/index | Critical | Central nonstandard registry; replacement without a preservation map risks orphaning the entire corpus |
| All 339 `okf/**/*.md` files | Custom frontmatter fields including `okf_document_id`, `category`, `phase`, `status`, `primary_sources`, `related_documents`, verification commit/date, and uncertainty markers | Legacy metadata/validation | Critical | No executable consumer was found, but every document's identity and evidence model depends on these conventions |
| `okf/references/source-register.md` | Source/evidence register referenced by the root README, project documents, maintenance documents, and index | Provenance | High | Candidate source for a later Google `sources` preservation map |
| `okf/maintenance/README.md` | Maintenance navigation and governance | Documentation workflow | High | Deleting it removes how legacy maintenance sections are discovered |
| `okf/maintenance/okf-update-policy.md` | Same-change update policy and custom metadata/index requirements | Documentation workflow | High | It describes rules for the old schema, not Google OKF requirements |
| `okf/maintenance/staleness-detection.md` | Bespoke checks for `index.yaml`, frontmatter, IDs, links, evidence paths, status, and Mermaid | Validation workflow | High | Explicitly says validation logic is not a tracked script or CI job |
| `okf/maintenance/review-checklist.md` | Legacy review checklist for frontmatter, IDs, index parity, evidence paths, Mermaid, terminology, and secrets | Review workflow | Medium/High | Useful process knowledge but not a normative Google validator |
| `okf/maintenance/validation-report.md` | Historical validation results at legacy source commit `131c4f30...` | Validation evidence | High | Records 339 Markdown, 50 Mermaid, 390 index entries, custom passes, and unavailable parser/render checks |
| `okf/maintenance/cross-phase-conflicts.md`, `unresolved-owner-questions.md`, `final-completion-report.md` | Legacy conflict, owner, and completion registers | Historical/governance | High | Contains unresolved product/operations questions and historical best-supported interpretations |
| `okf-bootstrap/00-04*.md` | Repository, technology, architecture, module, and documentation audits | Historical audit | High | Unique baseline reasoning and source-selection/preservation observations |
| `okf-bootstrap/05-mermaid-audit.md`, `06-database-inventory.md`, `07-api-inventory.md` | Diagram, database/RLS, and API evidence inventories | Historical audit | High | Includes limitations and counts that should be checked against current source before migration |
| `okf-bootstrap/08-conventions-inventory.md` | Inferred naming, branch, documentation, testing, and validation conventions | Historical audit | Medium | Distinguishes inferred conventions from enforced rules; valuable anti-assumption record |
| `okf-bootstrap/09-knowledge-gaps.md`, `12-owner-questions.md` | Unknowns, conflicts, owner decisions, and operational gaps | Governance/uncertainty | Critical | Deleting them would erase why claims must remain UNKNOWN or NEEDS_OWNER_CONFIRMATION |
| `okf-bootstrap/10-proposed-okf-structure.md`, `11-implementation-phases.md`, `phase-zero-summary.md` | Legacy target structure and prior OKF phase plan | Historical planning | High | Not normative for Google v0.2, but records design history and sequencing rationale |
| `docs/security/PHASE_01_SECURITY_BASELINE.md:22`, `:57`, `:279` | Names the “OKF route/security baseline” and links `okf/api/openapi-consistency.md` | Documentation evidence | Medium | Documentation-only reference to legacy API/security evidence |
| `docs/security/PHASE_03_BROWSER_AUTH_PREVIEW_WS_HARDENING.md:410` | Lists current API/architecture/OKF documents among updated evidence | Documentation synchronization | Medium | No runtime consumer |
| `docs/security/PHASE_05_KEY_SESSION_RECOVERY_HARDENING.md:24`, `:344` | Treats older OKF as part of repository/Git source-of-truth review and documents updated OKF areas | Documentation synchronization | Medium | Historical/security workflow reference |
| `okf/backend/configuration-and-state.md` | Pre-existing unstaged legacy document modification for file-ingress configuration | Dirty-worktree preservation | High | This change predates Phase 0; do not overwrite or attribute it to this audit |
| `okf/security/overview.md` | Pre-existing unstaged legacy document modification for input protection and storage security | Dirty-worktree preservation | High | This change predates Phase 0 and contains current security knowledge that would be lost by careless cleanup |
| `.gitignore` | Excludes generated/dependency trees but does not exclude `okf/` or `okf-bootstrap/` | Repository boundary | Low/Medium | The legacy trees are tracked source, not generated output |
| `package.json`, `frontend/package.json`, `backend/Cargo.toml` | No exact legacy OKF path, field, or index command | Tooling | None found | Package/build scripts do not consume the legacy tree |
| `scripts/*` | No exact legacy OKF path, field, or index command | Tooling | None found | Seven tracked scripts are release/Marketplace/smoke/version tools |
| `.github/workflows/backend-ci.yml`, `.github/workflows/frontend-ci.yml` | No exact legacy OKF path or validation step; path filters target application/manifests/scripts | CI | None found | Docs-only legacy changes do not trigger these workflow path filters |
| `backend/**/*.rs`, `frontend/src/**/*.{ts,tsx}`, Docker files, Compose files | No exact legacy OKF path, index, or field consumer | Runtime/build | None found | Exact searches found no executable dependency; future deletion should not affect runtime based on repository evidence |

## Search summary

The exact tracked searches produced these useful counts:

| Search | Result |
| --- | --- |
| Literal `okf-bootstrap` | 15 tracked files, concentrated in `HANDOFF.md`, `README.md`, legacy maintenance/project/index files, and all relevant bootstrap references |
| Literal `index.yaml` | 25 tracked files, concentrated in legacy navigation/index/bootstrap/maintenance references and `HANDOFF.md` |
| Exact `okf/` outside `okf/**` | Root README, handoff, selected security documents, and bootstrap planning/history |
| Exact legacy terms in executable/config paths | No matches in Rust, frontend source, scripts, package manifests, Docker/Compose, or workflow files |
| Legacy custom field names outside `okf/**`/`okf-bootstrap/**` | No legacy metadata consumer found; unrelated application fields such as Marketplace `review_status` are not OKF coupling |

The search intentionally used exact path/field terms and excluded generated or
dependency directories. A bare substring search for `okf` is not evidence of
coupling because it matches unrelated text such as `WebhookFailed` and package
names containing `hookform`.

## Coupling classification

| Coupling class | Result |
| --- | --- |
| Documentation-only | Confirmed for README, HANDOFF, security-phase, bootstrap, and legacy-tree references |
| Navigation/linking | Confirmed for root README, `okf/README.md`, internal Markdown links, frontmatter relation arrays, and `index.yaml` |
| Tooling/validation | Legacy maintenance documents describe custom checks, but no tracked executable validator or package script was found |
| CI | No legacy path or validation step found in the two tracked workflows |
| Runtime/application | No exact dependency found in Rust, frontend, Docker, Compose, migrations, or tests |
| Development workflow | Human/agent handoff and maintenance process depend on legacy documentation being discoverable |
| Historical record | Confirmed for `okf-bootstrap/`, handoff history, phase records, conflicts, and owner questions |

## Migration implication

The future removal of the legacy format is primarily a knowledge/navigation
change, not a runtime refactor. Before removal, Phase 1 must preserve the
document graph, source evidence, uncertainty/owner markers, historical context,
and current dirty OKF changes. Later phases must update root navigation and
handoff instructions together with any new bundle boundary. No Phase 0 action
should remove these references yet.
