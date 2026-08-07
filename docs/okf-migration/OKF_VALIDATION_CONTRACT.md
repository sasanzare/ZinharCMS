# ZinharCMS Google OKF v0.2 Validation Contract

**DESIGN ONLY - NOT YET IMPLEMENTED**

This contract defines checks that later construction and cutover phases can
automate. It distinguishes official Google OKF conformance from stricter
ZinharCMS policy and from migration-preservation reconciliation. Phase 2 does
not implement a validator or create a target bundle.

## Validation scope

The future validator receives:

- a candidate bundle root, initially
  `docs/okf-migration/staging/google-okf-v0.2/`;
- the target tree and [TARGET_CONCEPT_CATALOG.md](TARGET_CONCEPT_CATALOG.md);
- the Phase 1 matrix, claim ledger, diagram map, owner register, and evidence
  manifest;
- the source snapshot commit and repository root;
- an optional output path for a machine-readable report.

It must not modify the candidate, legacy files, source code, or Git index. A
future CI wrapper may call it in read-only mode; it must not download external
content or require a network connection to parse a bundle.

## Result classes and exit codes

The process emits a human-readable summary and a JSON report containing check
ID, class, severity, path, message, and evidence. It exits with the first
applicable non-zero class, while still reporting all discovered failures.

| Exit code | Class | Meaning |
| ---: | --- | --- |
| 0 | PASS | All requested checks passed. |
| 10 | GOOGLE_CONFORMANCE | Official OKF v0.2 structural/conformance failure. |
| 20 | ZINHAR_POLICY | Local type, metadata, path, navigation, lifecycle, or diagram-policy failure. |
| 30 | KNOWLEDGE_INTEGRITY | Unsupported, stale, contradicted, or unresolved claims are presented incorrectly. |
| 40 | MIGRATION_RECONCILIATION | Phase 1 rows, groups, claims, diagrams, NOCs, or target tree do not reconcile. |
| 50 | SAFETY_SCOPE | Legacy/application/Git scope changed or the candidate is not isolated. |
| 60 | TOOL_ERROR | Input cannot be read or the validator itself cannot complete; not a knowledge pass/fail. |

An implementation may support `--class` to run one class, but a cutover gate
must run all classes and receive exit code 0.

## Google OKF v0.2 conformance checks

These checks answer only whether the candidate follows the official minimal
contract.

| ID | Check | Required behavior |
| --- | --- | --- |
| G-001 | UTF-8 | Read every candidate Markdown file as valid UTF-8; report path and byte offset for invalid input. |
| G-002 | Concept frontmatter | Every non-reserved `.md` begins with an opening `---`, has a closing `---`, and contains parseable YAML. |
| G-003 | Required type | Every non-reserved `.md` has a non-empty top-level `type` value. |
| G-004 | Reserved names | Exact `index.md` and `log.md` files are never parsed as Concepts. |
| G-005 | Index structure | Present indexes are directory listings; a root index may carry `okf_version`, and non-root indexes do not require frontmatter. |
| G-006 | Log structure | Present logs use date-grouped entries with ISO `YYYY-MM-DD` headings, newest first. |
| G-007 | Optional-family shape | If present, `sources[].resource`, `generated.by/at`, `verified.by/at`, `status`, and `stale_after` follow Google v0.2 shape. |
| G-008 | Actor syntax | Actors in `generated`/`verified` use `<producer>/<version>`, `human:<id>`, or `process:<id>`. |
| G-009 | Computation shape | If an `Attested Computation` is ever introduced, its required runtime/computation contract is validated; the current policy forbids this family. |

The official consumer-permissive rules are not changed by local checks:
unknown types/keys, missing optional fields, missing indexes, and broken links
are not Google conformance failures. A local policy check may still reject them
for the ZinharCMS target.

## ZinharCMS project-policy checks

These are deliberately stronger local rules and must be reported as such.

### Scope and tree

| ID | Check | Rule |
| --- | --- | --- |
| P-001 | Bundle root | Candidate root is the isolated staging root or final `/okf/`; it is never the legacy tree during construction. |
| P-002 | Tree reconciliation | All 54 catalog Concept paths and 13 indexes exist exactly as planned; root log exists once. |
| P-003 | Directory depth | No unapproved deeper directory or ad hoc Concept path is introduced without updating the catalog/tree decision. |
| P-004 | Legacy artifacts | No `index.yaml`, legacy `okf_document_id`, legacy relation arrays, `.mmd`, or copied `okf-bootstrap` file occurs inside the candidate bundle. |
| P-005 | Root version | Only root `index.md` may contain `okf_version`, and its value is exactly `"0.2"`. |

### Types and metadata

| ID | Check | Rule |
| --- | --- | --- |
| P-010 | Type vocabulary | Every Concept type is one of the 19 approved values in [OKF_TYPE_AND_METADATA_POLICY.md](OKF_TYPE_AND_METADATA_POLICY.md). |
| P-011 | Local required fields | Every Concept has non-empty `title`, one-sentence `description`, `status`, and a non-empty `sources` list. |
| P-012 | Sources | Every source entry has a meaningful `resource`; source IDs are unique within a Concept and footnote labels match when used. |
| P-013 | Immutable repository provenance | Repository sources use a checked immutable commit URL; unpinned `main` URLs are not accepted as Concept evidence. |
| P-014 | No extensions | No frontmatter key outside the Google families adopted by policy is present. The target extension count is zero. |
| P-015 | Status | Only `draft`, `stable`, and `deprecated` occur; historical scope is expressed through type/body, not a custom status. |
| P-016 | Freshness | `stale_after`, when used, is an ISO date with a recorded owner-defined review rule; Phase 2 supplies no date. |
| P-017 | Actors | No synthetic human actor is used; human actors must be owner-supplied `human:<id>`, and process actors identify an actual process. |
| P-018 | Verification | `verified` is present only when a real human/process check was performed against current content and sources. |
| P-019 | Generated | `generated` is present only when the named producer actually wrote the current content and the timestamp is known. |

### Navigation and relationships

| ID | Check | Rule |
| --- | --- | --- |
| P-020 | Root index coverage | Root index lists all 12 major directories in deterministic order. |
| P-021 | Domain index coverage | Each direct Concept appears exactly once in its domain index; descriptions match frontmatter unless a reviewed wording exception is recorded. |
| P-022 | Internal links | Intended Concept links resolve inside the candidate bundle; no final Concept link escapes the bundle with `..`. |
| P-023 | Link form | Concept relationships use bundle-root-relative or ordinary relative Markdown links; surrounding prose explains material relationship meaning. |
| P-024 | Orphans | Every Concept is reachable from the root through the index hierarchy; unused Concepts are reported, not silently removed. |
| P-025 | Root log | The single root log uses newest-first ISO date headings and records semantic changes only. |

### Diagram policy

| ID | Check | Rule |
| --- | --- | --- |
| P-030 | Embedded visuals | Planned diagrams occur as named Mermaid code blocks in owning Concept bodies; no standalone `.mmd` is in the bundle. |
| P-031 | Diagram reconciliation | The mapping reports 19 preserved, 8 merged visual groups, 6 regenerated visual groups, and one dropped legacy path. |
| P-032 | Diagram evidence | Each visual has source/evidence wording and does not assert deployment/owner facts unsupported by current evidence. |

## Knowledge-integrity checks

These checks prevent a structurally valid bundle from becoming misleading.

| ID | Check | Rule |
| --- | --- | --- |
| K-001 | Source existence | Every repository source path/URL was checked at the construction snapshot; missing evidence fails the local knowledge gate. |
| K-002 | Authority hierarchy | Current source, migrations, tests, executable configuration, CI, Docker, and current docs outrank legacy claims. |
| K-003 | Stable claim status | A `stable` Concept cannot present Phase 1 `STALE`, `CONTRADICTED`, `UNVERIFIED`, or `NON_FACTUAL` material as an unqualified current fact. |
| K-004 | Partial claims | `PARTIALLY_VERIFIED` content separates observed repository behavior from unverified production/policy behavior. |
| K-005 | Unknowns | Every affected Concept names relevant NOCs/UNKNOWNs and links to the draft Decision Register; no unresolved proposal is written as a fact. |
| K-006 | Historical scope | Historical Concepts identify their snapshot and cannot be used as current implementation sources. |
| K-007 | Verification freshness | A changed Concept cannot retain an old verification event without a recorded recheck. |
| K-008 | Security correction | Stale/contradicted authentication and logout statements are excluded from current security Concepts and replaced with current source evidence. |
| K-009 | No invented infrastructure | Production provider, ingress, backup, observability, storage, external identity, or execution claims require explicit evidence/owner input. |
| K-010 | No attestation fiction | The Phase 1 evidence manifest is not treated as an Attested Computation or runtime receipt. |

## Migration-reconciliation checks

| ID | Check | Expected result |
| --- | --- | --- |
| M-001 | Document rows | 353 Phase 1 matrix rows; every row has a destination/action. |
| M-002 | Disposition totals | `MIGRATE 168`, `MERGE 117`, `REGENERATE 35`, `PRESERVE_HISTORICAL 19`, `DELETE_LEGACY_ONLY 14`. |
| M-003 | Merge groups | All 13 IDs are present; group row totals sum to 353. |
| M-004 | Review overlay | 75 `Yes` and 278 `No` rows remain distinguishable from primary disposition. |
| M-005 | Claim ledger | 60 claims reconcile to 20 current, 6 historical, 12 partial, 7 stale, 2 contradicted, 8 unverified, and 5 non-factual. |
| M-006 | Diagram map | All 50 paths reconcile to 19 PRESERVE, 19 MERGE, 11 REGENERATE, and 1 DROP. |
| M-007 | Owner register | All 18 NOC records are present; 17 remain open unless new evidence explicitly resolves them, and NOC-16 remains historical/resolved. |
| M-008 | Target catalog | 54 catalog paths match the target tree and build-strategy totals are 19/12/11/7/5. |
| M-009 | Candidate labels | The 24 Phase 1 candidate labels sum to 353 and map to target IDs. |
| M-010 | No silent omission | Every `DELETE_LEGACY_ONLY` row has an absorption reason and cannot be removed before cutover gates. |

## Legacy/application safety checks

These checks are required for the migration phase, not Google conformance.

| ID | Check | Rule |
| --- | --- | --- |
| S-001 | Git scope | Only `docs/okf-migration/**` and `HANDOFF.md` are changed by Phase 2. |
| S-002 | Legacy diff | No file under `okf/**` or `okf-bootstrap/**` is modified, deleted, moved, or renamed. |
| S-003 | Legacy registry | `okf/index.yaml` path, bytes, and SHA-256 remain unchanged. |
| S-004 | Replacement absence | No future target bundle exists at `/okf/` or under a production parallel root during Phase 2. |
| S-005 | Application boundary | No backend/frontend/migration/configuration/CI/dependency behavior changes. |
| S-006 | Git operations | No Phase 2 commit, push, stage, reset, stash, cleanup, or history rewrite occurs. |

The safety checks compare a recorded baseline with the final worktree and use
Git status/diff as source of truth. They do not infer provenance from file
timestamps.

## Recommended lightweight implementation

Phase 3+ should implement one small, repository-local validator using an
already available runtime rather than a new framework. Preferred design:

- a Node.js script under `scripts/` only after an implementation-phase
  decision authorizes it;
- built-in filesystem, URL, and regular-expression support;
- the repository's already-approved YAML parser/dependency if available,
  otherwise a minimal parser dependency decision before use;
- no network access, no application startup, and no database mutation;
- deterministic sorted paths and JSON output;
- explicit `--bundle`, `--catalog`, `--manifest`, `--report`, and `--class`
  inputs;
- CI-friendly exit codes above, with no auto-fix mode.

The validator is not created in Phase 2. Adding it must remain a future
implementation change under the phase boundary and should be accompanied by
tests for reserved filenames, YAML edge cases, links, source URLs, counts,
legacy safety, and all 13 merge groups.

## Validation limitations

Static validation cannot prove deployed production topology, applied schema,
backup/recovery, real owner identity, live authorization behavior, or external
provider guarantees. Those remain content/owner gates. Mermaid syntax/render
validation may be run separately with an approved renderer; its absence is not
silently reported as a passed visual review.
