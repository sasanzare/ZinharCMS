# ZinharCMS Google OKF Migration - Phase 2 Target Architecture

**Phase:** 2 - Google OKF v0.2 Target Architecture and Bundle Design
**Status:** COMPLETE for architecture and design only
**Design date:** 2026-08-07 (Europe/London)
**Implementation boundary:** No target bundle was created. No legacy content was changed.

## Objective

Define the smallest coherent, evidence-backed architecture that a later phase
can use to construct a Google Open Knowledge Format (OKF) v0.2 bundle for
ZinharCMS. This report decides the bundle boundary, concept granularity,
directory hierarchy, type vocabulary, metadata and provenance policy,
navigation, diagrams, staging, validation, and legacy mapping. It is a design
record, not a converted knowledge bundle.

The detailed artifacts are:

- [Target bundle tree](TARGET_BUNDLE_TREE.md)
- [Target concept catalog](TARGET_CONCEPT_CATALOG.md)
- [Type and metadata policy](OKF_TYPE_AND_METADATA_POLICY.md)
- [Navigation and linking policy](NAVIGATION_AND_LINKING_POLICY.md)
- [Legacy-to-target mapping](LEGACY_TO_TARGET_MAPPING.md)
- [Staging and cutover plan](MIGRATION_STAGING_AND_CUTOVER_PLAN.md)
- [Validation contract](OKF_VALIDATION_CONTRACT.md)
- [Phase 2 decision record](PHASE_02_DECISION_RECORD.md)

## Current baseline and constraints

Phase 2 started from the actual repository state rather than the older Phase 1
working-tree description:

| Item | Phase 2 baseline |
| --- | --- |
| Repository | `D:\All projects\ZinharCMS` |
| Branch | `security/security-audit-fixes` |
| Starting HEAD | `1cb6f43be6d46bee0bf8ac8b06c833967aae6786` |
| Starting worktree | Clean; no staged, modified, or untracked paths |
| Phase 1 checkpoint | Committed at the starting HEAD; no repository changes occurred after it before Phase 2 |
| Runtime changes in Phase 2 | None permitted and none made |
| Legacy trees | `okf/`, `okf-bootstrap/`, and `okf/index.yaml` remain legacy and untouched |

The Phase 1 evidence remains the preservation boundary. It covers 404 material
legacy files (353 Markdown, 50 Mermaid, and one YAML file), 353 document rows,
13 semantic merge groups, 60 material claims, 17 open NOC decisions, and 15
UNKNOWN labels. The complete inputs are the [Phase 1 preservation report](PHASE_01_KNOWLEDGE_PRESERVATION.md),
[disposition matrix](LEGACY_DOCUMENT_DISPOSITION_MATRIX.md),
[claim ledger](CLAIM_VERIFICATION_LEDGER.md),
[diagram map](DIAGRAM_PRESERVATION_MAP.md),
[owner decision register](OWNER_DECISION_REGISTER.md), and
[Phase 1 evidence manifest](PHASE_01_EVIDENCE_MANIFEST.json).

Phase 2 changes only `docs/okf-migration/**` and `HANDOFF.md`. It does not
delete, move, rename, rewrite, add frontmatter to, or convert any legacy file.
It does not create a parallel production bundle, change application behavior,
add dependencies, alter CI, or create a commit or push.

## Specification baseline

The normative source is the official Google [`okf/SPEC.md`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md),
read directly on 2026-08-07. It still identifies itself as **Open Knowledge
Format Version 0.2**. No Phase 0 or Phase 1 assumption needed reconsideration.

The official requirements are deliberately separated from this project's
policy:

| Class | Decision |
| --- | --- |
| Google MUST | A bundle is a directory tree of Markdown files; every non-reserved `.md` Concept has parseable YAML frontmatter with a non-empty `type`; present `index.md` and `log.md` files follow their reserved structures. |
| Google SHOULD | Use descriptive type values, structural Markdown, standard links, and the optional provenance/trust/lifecycle families when their facts are known. |
| Google MAY | Use `title`, `description`, `resource`, `tags`, `sources`, `generated`, `verified`, `status`, `stale_after`, `references/`, `index.md`, `log.md`, and Attested Computation fields as applicable. |
| ZinharCMS policy | Use a curated hierarchy, a small allowed type vocabulary, local title/description/status/source rules, root-relative links, curated indexes, one root log, embedded diagrams, and zero custom frontmatter extensions. |

Google does not define a fixed taxonomy, central registry, storage/runtime,
query system, domain-schema replacement, or mandatory validator. The future
bundle therefore references OpenAPI, SQL migrations, source, tests, and
operational material rather than trying to replace them.

## Design principles

1. **Source-backed current truth.** Source code, migrations, tests, executable
   configuration, CI, and current documentation outrank legacy OKF claims.
2. **One responsibility per Concept.** A Concept is a useful retrieval and
   review unit, not a historical file container.
3. **Consolidate by capability.** Repeated API, backend, database, security,
   domain, frontend, and Marketplace views become a small set of linked
   Concepts.
4. **Separate observed, historical, and intended knowledge.** Code behavior,
   owner policy, historical audits, and migration decisions never share an
   unlabeled current-state paragraph.
5. **Use official semantics before extensions.** Standard fields, body
   structure, links, Git history, and immutable source URLs are preferred over
   project-specific metadata.
6. **Make uncertainty visible.** An unresolved NOC remains a draft Decision
   Register entry and a blocker/caveat on affected Concepts; it is not a fact.
7. **Build in isolation.** The legacy `/okf/` tree remains the preservation
   source until a validated staging bundle passes cutover gates.
8. **Keep maintenance realistic.** Indexes are generated or reviewed from
   Concept descriptions, while the root log records semantic changes rather
   than duplicating Git history.

## Bundle boundary

The final canonical bundle is the future repository `/okf/` directory after a
controlled cutover. Its boundary is exactly:

- root `index.md` and root `log.md`;
- the 12 domain directories and 54 Concept files in
  [TARGET_BUNDLE_TREE.md](TARGET_BUNDLE_TREE.md);
- Mermaid diagrams embedded in the owning Concept bodies;
- Concept frontmatter and Markdown body content only.

The final bundle does **not** contain source code, database migrations, tests,
CI files, product documentation, the migration reports, `okf/index.yaml`,
legacy custom metadata, or a copied `okf-bootstrap/` tree. Those materials stay
outside the bundle as evidence or migration history and are referenced through
immutable source URLs when a future Concept needs them.

The current `/okf/` is not this bundle. It is the legacy preservation corpus.
The isolated construction boundary is
`docs/okf-migration/staging/google-okf-v0.2/`, which is deliberately not
created in Phase 2. The exact lifecycle and authority rules are in
[MIGRATION_STAGING_AND_CUTOVER_PLAN.md](MIGRATION_STAGING_AND_CUTOVER_PLAN.md).

## Concept granularity

The target uses 54 Concepts, a material reduction from 353 legacy Markdown
documents. The following rules govern future splitting and merging:

- Keep one Concept when claims share one capability, evidence set, lifecycle,
  and review boundary.
- Split when a topic has a separate security boundary, runtime subsystem,
  source family, owner decision, freshness interval, or independent retrieval
  need.
- Merge legacy files when they are alternate views of the same capability,
  such as route catalogs plus endpoint-family summaries, entity lists plus
  schema catalogs, or role lists plus authorization flows.
- Regenerate exhaustive catalogs when they are mechanically derivable from
  current routes, migrations, tests, configuration, or CI.
- Preserve historical audits and decision lineage as explicitly typed history;
  never merge their snapshot claims into current implementation Concepts.
- Omit only navigation-only material after all useful links and claims have
  been absorbed and the mapping validator proves the row is covered.

The full Concept-level plan, including every path and Phase 1 source group, is
in [TARGET_CONCEPT_CATALOG.md](TARGET_CONCEPT_CATALOG.md). It assigns 19
Concepts to direct verified construction, 12 to semantic merge, 11 to
regeneration, 7 to owner-blocked construction, and 5 to historical
preservation.

## Target hierarchy and domain boundaries

The hierarchy is capability-oriented rather than a copy of the legacy tree:

`project`, `architecture`, `backend`, `frontend`, `api`, `database`,
`security`, `domain`, `operations`, `development`, `decisions`, and `history`.

Each major directory receives one curated `index.md`; no deeper directory is
needed for the current design. The tree, purpose, exclusions, merge groups,
and expected counts are defined in [TARGET_BUNDLE_TREE.md](TARGET_BUNDLE_TREE.md).

The separation is intentional:

- `api` describes externally reachable contracts and generated route surfaces.
- `security` describes authentication, authorization, tenant, preview, and
  storage controls; it is not folded into generic API or database summaries.
- `database` describes migration-defined schema and data policies; it does not
  assert deployed schema, backup, or retention facts without evidence.
- `domain` describes business capabilities and workflows; it does not replace
  route or schema specifications.
- `operations` describes repository-proven local/reference topology and later
  owner-supplied production operations; it never upgrades Compose to proof of
  production.
- `decisions` and `history` make unresolved and historical knowledge visible
  without allowing it to masquerade as current truth.

## Type and metadata strategy

The approved vocabulary contains 19 descriptive values. Types are metadata
values, not directory names or separate taxonomy documents. The vocabulary,
use/non-use rules, field requirements, and actor rules are normative for the
future ZinharCMS bundle in
[OKF_TYPE_AND_METADATA_POLICY.md](OKF_TYPE_AND_METADATA_POLICY.md).

The local policy requires `title`, `description`, `status`, and `sources` for
future Concepts because this bundle is intended to be navigable and
source-backed. It does not require `generated`, `verified`, `stale_after`, or
`resource` when the corresponding fact does not exist. `type` remains the only
Google-required field. The project defines **zero custom frontmatter
extensions**; legacy IDs, categories, phases, confidence labels, and relation
arrays are translated to body prose, links, standard fields, or history.

## Provenance, trust, and freshness

Future Concepts use `sources` entries with required `resource` values. For
repository evidence, the builder uses immutable GitHub blob URLs pinned to the
construction commit; it does not use an unpinned `main` URL or a bundle-relative
path that escapes `/okf/`. In-bundle lineage is expressed with ordinary
Markdown links and surrounding prose. Per-claim footnotes use stable source
IDs when multiple sources need attribution.

`generated` describes how content was written, while `verified` records an
actual confirmation. A legacy `last_verified_commit`, date, or `review_status`
does not become `verified` automatically. Human actors require a real
`human:<id>` supplied by the owner/reviewer; automated checks use a real
`process:<id>`; no synthetic human identity is permitted. A meaningful edit
invalidates the prior verification for the edited Concept until rechecked.

`status` is limited to official `draft`, `stable`, or `deprecated` values.
Open decisions, incomplete content, and blocked sections remain `draft`.
Historical/superseded current-state material is `deprecated` or explicitly
typed `Historical Record` according to the content. `stale_after` is used only
where a real owner-defined freshness rule exists; Phase 2 creates no dates.

## Unknown and owner-decision strategy

The future `decisions/owner-decision-debt.md` Concept contains the 17 open NOC
records and 15 UNKNOWN labels as a draft Decision Register. Each affected
current Concept has an explicit body section naming its blocker and separating
observed repository behavior from the unresolved policy. NOC-16 remains a
resolved historical question backed by `LICENSE` and is not re-opened.

Resolution requires an owner-approved decision or new authoritative evidence,
an update to the register, propagation to affected Concepts, and a fresh
validation/verification event. No owner question is resolved by Phase 2.

## Historical knowledge strategy

Historical knowledge remains useful but is not current truth. Five
`history/` Concepts consolidate the 19 historical document dispositions and
the 14 bootstrap reports into phase, bootstrap, conflict/completion, and
legacy planning records. Full Phase 0/1 migration reports and manifests remain
outside the final bundle under `docs/okf-migration/`; they are the detailed
audit record, not duplicate current Concepts. Historical Concepts use explicit
scope and source language and are never cited as implementation authority.

## Diagram strategy

The future bundle contains no standalone `.mmd` files. Mermaid is embedded in
the body of the owning Concept so diagrams remain ordinary Markdown content;
the existing `docs/diagrams/` and legacy `.mmd` files remain external evidence.
The design preserves 19 unique visual relationships, consolidates 19 legacy
diagrams into 8 domain-level visuals, and regenerates 11 source-sensitive
diagrams as 6 future visuals. The expected final count is therefore **33
embedded Mermaid visual blocks** (19 preserved + 8 merged + 6 regenerated),
with the one DROP candidate producing no replacement visual. Each block is
named in the construction manifest and referenced by surrounding prose, not a
custom frontmatter key.

The complete path-level diagram reconciliation is in
[LEGACY_TO_TARGET_MAPPING.md](LEGACY_TO_TARGET_MAPPING.md).

## Navigation and relationship strategy

The root index and each major-domain index provide progressive disclosure.
Indexes are generated from the approved catalog and then reviewed in the same
change as Concept edits; they contain no frontmatter except the root
`okf_version: "0.2"` declaration. Only the root `log.md` is used, with
newest-first ISO date sections for meaningful semantic changes. Git remains the
detailed history.

Concept links use bundle-root-relative Markdown paths such as
`/security/tenant-isolation.md`; prose names the relationship. Index links are
ordinary relative paths. The project validator requires intended internal
links and index entries to resolve, while remaining compatible with Google's
consumer rule that broken links are not a conformance failure. There is no
central typed-edge registry and no generated backlink boilerplate.

See [NAVIGATION_AND_LINKING_POLICY.md](NAVIGATION_AND_LINKING_POLICY.md) for
the exact rules.

## Validation, staging, and cutover

Future validation is split into three independent claims:

1. **Google conformance:** encoding, frontmatter, type, reserved files, and
   optional-family shape.
2. **ZinharCMS policy:** allowed paths/types/fields, source and actor rules,
   indexes/log, link resolution, freshness, unknowns, and embedded diagram
   policy.
3. **Migration reconciliation:** all 353 document rows, 13 merge groups, 50
   diagram rows, 60 claim statuses, 17 NOCs, and all target catalog rows are
   accounted for.

The proposed lightweight validator and exit-code contract are in
[OKF_VALIDATION_CONTRACT.md](OKF_VALIDATION_CONTRACT.md). Construction begins
only in the isolated staging path, with the legacy tree remaining the
preservation authority. Cutover is a reviewed, atomic repository change that
updates `/okf/` and repository navigation together; rollback is a revert to
the pre-cutover commit, not a destructive reset. The full sequence is in
[MIGRATION_STAGING_AND_CUTOVER_PLAN.md](MIGRATION_STAGING_AND_CUTOVER_PLAN.md).

## Quantitative Phase 2 design summary

| Metric | Design result |
| --- | ---: |
| Proposed bundle directories, including root | 13 |
| Proposed Concept files | 54 |
| Proposed descriptive type values | 19 |
| Proposed `index.md` files | 13 |
| Proposed `log.md` files | 1 |
| Proposed embedded diagram visual blocks | 33 |
| Proposed custom extension keys | 0 |
| Legacy document rows mapped | 353 / 353 |
| Legacy semantic merge groups mapped | 13 / 13 |
| Concepts buildable from verified knowledge | 19 |
| Concepts requiring regeneration | 11 |
| Concepts blocked by owner decisions | 7 |
| Historical-only target Concepts | 5 |
| Open architectural blockers | 0 |
| Open content/owner blockers | 17 NOC records; 15 UNKNOWN labels |

The counts are derived from the target tree and catalog, not an estimate of
future prose length. The target remains substantially smaller than the legacy
corpus because duplicate views become capability Concepts and exhaustive
surfaces are regenerated rather than copied.

## Phase 3 readiness

Phase 3 may begin construction only in
`docs/okf-migration/staging/google-okf-v0.2/` and only after reviewing the
catalog, mapping, policy, and validation contract. The first construction
slice should be the high-confidence 19-Concept foundation, plus the required
indexes, while preserving explicit draft/blocker treatment for owner-dependent
content. It must not write to canonical `/okf/`, delete legacy files, or answer
the 17 open NOCs by assumption.

The next phase is exactly:

**Phase 3 - Core Google OKF v0.2 Bundle Construction**
