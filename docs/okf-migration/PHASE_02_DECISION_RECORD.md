# ZinharCMS Google OKF Migration - Phase 2 Decision Record

**DESIGN ONLY - NOT YET IMPLEMENTED**

These records capture non-trivial Phase 2 architecture decisions. They are
planning decisions for the future bundle, not changes to the current `okf/`
tree. Each decision is intentionally reversible until the target bundle is
constructed and reviewed.

## OKF-ADR-001 - Keep the final canonical bundle at `/okf/` after cutover

**Decision:** The future Google OKF v0.2 bundle will occupy the canonical
repository `/okf/` path only after a validated cutover. The current `/okf/`
remains legacy preservation material until then.

**Rationale:** Existing human navigation expects `/okf/`, but that path is
currently occupied by an incompatible custom format. A final root change is
useful only after the entire corpus has a target mapping and the replacement
can be validated as one coherent bundle.

**Alternatives considered:** Keep the target permanently under another path;
convert legacy files in place; maintain two canonical bundles. The first would
leave the repository with ambiguous long-term navigation; the others would
mix schemas or create competing authority.

**Consequences:** Phase 3+ must build outside `/okf/`, and cutover must update
repository references atomically. Rollback remains possible through the
pre-cutover Git commit.

**Basis:** Phase 0 risk OKF-MIG-014 and the Phase 2 bundle-boundary design;
Google OKF permits a bundle as a subdirectory but does not choose a repository
path.

**Reversibility:** High until cutover; after cutover, a reviewed revert can
restore the previous canonical path.

## OKF-ADR-002 - Use an isolated staging root

**Decision:** Future construction occurs only under
`docs/okf-migration/staging/google-okf-v0.2/`. Phase 2 does not create it.

**Rationale:** A distinct root makes legacy custom truth and Google target
truth visually and structurally unambiguous, while keeping staging within the
controlled migration boundary.

**Alternatives considered:** Build under `/okf/`, build under `okf-next/`, or
build outside the repository with no tracked review surface. In-place build is
unsafe; a permanent parallel root creates long-term ambiguity; an untracked
workspace makes review and handoff incomplete.

**Consequences:** Staging must not be linked from canonical navigation and must
be removed or archived after cutover according to the staging plan.

**Basis:** Phase 0 risk OKF-MIG-014; Phase 1 preservation boundary; official
OKF directory-tree semantics.

**Reversibility:** High; the staging root can be discarded without touching
legacy or application files, subject to normal Git review.

## OKF-ADR-003 - Consolidate by capability, not legacy file

**Decision:** Construct 54 Concepts from coherent capability boundaries rather
than translating 353 Markdown rows one-to-one.

**Rationale:** API, backend, database, security, domain, frontend, and
Marketplace documents repeatedly describe the same systems from different
views. One responsibility per Concept improves retrieval, provenance, review,
and freshness without losing the row-level preservation map.

**Alternatives considered:** One Concept per legacy file; a very small set of
giant domain documents. The first reproduces duplication and stale snapshots;
the second makes independent source/review boundaries unreadable.

**Consequences:** Every merge group needs an explicit target crosswalk and
future generation must keep exhaustive catalogs separate where they are
mechanically derived.

**Basis:** Phase 1 `CONCEPT_CANDIDATE_MAP.md`, 13 merge groups, and Google
Concept ID semantics.

**Reversibility:** Medium; target Concepts can be split later if independent
source, security, lifecycle, or retrieval boundaries emerge.

## OKF-ADR-004 - Use a 19-value descriptive type vocabulary

**Decision:** Use the 19 values in
[OKF_TYPE_AND_METADATA_POLICY.md](OKF_TYPE_AND_METADATA_POLICY.md). Types are
descriptive frontmatter values, not a directory taxonomy or registry.

**Rationale:** Google does not define a fixed type registry. A small local
vocabulary helps agents route Concepts while avoiding one type per legacy
category, directory, role, or historical report.

**Alternatives considered:** Copy legacy `category` values; use one generic
`Document` type; create a type for every target path. Each loses useful routing
or creates taxonomy proliferation.

**Consequences:** A new type requires an explicit decision and catalog impact;
consumers outside ZinharCMS must still tolerate the descriptive strings.

**Basis:** Google `SPEC.md` sections 4.1 and 11 explicitly leave type values
producer-defined and require consumers to tolerate unknown types.

**Reversibility:** High; a type can be renamed during staging with a catalog
and link/index update before cutover.

## OKF-ADR-005 - Require curated root and domain indexes

**Decision:** Use 13 `index.md` files: one root index and one in each major
domain. Use no deeper indexes initially.

**Rationale:** 54 Concepts need progressive disclosure, but a shallow
capability tree makes one index per domain sufficient. Generated entries
derived from titles/descriptions reduce drift, with review for wording and
section grouping.

**Alternatives considered:** No indexes; a single root index; an index in every
subdirectory; a legacy YAML registry. No indexes reduce discoverability, a
single index is too broad, deep indexes add maintenance, and YAML reproduces
the legacy schema.

**Consequences:** Index coverage and ordering become local policy checks;
indexes must be updated with Concept changes.

**Basis:** Google `SPEC.md` sections 3.1 and 8; Phase 0 finding that the legacy
YAML registry is not a Google v0.2 index.

**Reversibility:** High; indexes are optional under Google OKF and can be
reduced if maintenance evidence shows they are not useful.

## OKF-ADR-006 - Use one root semantic log

**Decision:** Use exactly one root `log.md`, with newest-first ISO date groups.
Do not create per-domain logs.

**Rationale:** Agents benefit from a concise semantic history, while Git
already provides detailed file history. Multiple logs would duplicate
maintenance and make update scope ambiguous.

**Alternatives considered:** No logs; per-domain logs; a generated copy of Git
history. No log loses semantic milestones, per-domain logs multiply burden, and
Git duplication is noisy.

**Consequences:** Only meaningful knowledge changes are logged; routine index
regeneration is not.

**Basis:** Google `SPEC.md` section 9 and repository Git workflow.

**Reversibility:** High; `log.md` is optional and can be removed with a
navigation decision if the maintenance signal proves redundant.

## OKF-ADR-007 - Adopt official provenance fields and zero custom extensions

**Decision:** Use official `sources`, `generated`, `verified`, `status`, and
`stale_after` only when their facts are known. Require local `title`,
`description`, `status`, and `sources`, but define zero custom frontmatter keys.

**Rationale:** The 53 legacy keys mix identity, navigation, review, phase, and
uncertainty. Standard fields, body prose, ordinary links, immutable source
URLs, and Git history preserve the meaning without creating a second schema.

**Alternatives considered:** Preserve all legacy keys; retain a large
translation extension; omit all provenance. The first two recreate legacy
complexity; the last loses source traceability.

**Consequences:** Some legacy labels will be expressed as body/history text,
and the local validator is intentionally stricter than Google about unknown
keys. Sources must be pinned and checked at generation time.

**Basis:** Google `SPEC.md` sections 4-7; Phase 1 metadata translation;
OKF-MIG-002, OKF-MIG-005, and OKF-MIG-009.

**Reversibility:** Medium; local required fields can be relaxed, but a future
extension would require a new compatibility review.

## OKF-ADR-008 - Treat verification as a separate conservative event

**Decision:** Never convert legacy review labels, commit hashes, or dates into
Google `verified` without an actual identified actor and current-content check.
Use real `process:<id>` or owner-supplied `human:<id>` actors only.

**Rationale:** Authorship, verification, source snapshot, and freshness are
different facts. Conflating them would promote stale or contradicted claims.

**Alternatives considered:** Map `last_verified_commit` directly; use a
synthetic human reviewer; mark every generated Concept human-reviewed. All
would create false trust signals.

**Consequences:** Many initial Concepts will be unverified or machine-
confirmed until an actual review occurs; this is intentional.

**Basis:** Google `SPEC.md` sections 5.2-5.3 and 7; Phase 1 claim ledger.

**Reversibility:** High; a real verification event can be added later without
changing Concept identity.

## OKF-ADR-009 - Represent open knowledge as Decision debt

**Decision:** Carry the 17 open NOCs and 15 UNKNOWN labels in one draft
`Decision` Concept with per-NOC headings and explicit links from affected
Concepts. Do not invent production facts.

**Rationale:** Open questions need to be discoverable and linked, but separate
one-file-per-question Concepts would be unnecessarily granular and a current
Concept must not silently absorb them.

**Alternatives considered:** Drop unresolved questions; resolve by assumption;
create 17 tiny files. Dropping loses risk, assumptions create false truth, and
the tiny files multiply maintenance.

**Consequences:** Affected Concepts may be partially buildable but remain
`draft` or contain explicit policy limits until owner evidence arrives.

**Basis:** Phase 1 owner register and Google `status: draft`; no Google
unknown-specific field is required.

**Reversibility:** High; the register can split into separate decisions later
if ownership or lifecycle requires it.

## OKF-ADR-010 - Keep history in a dedicated target area and migration docs

**Decision:** Include five `history/` Concepts for unique historical context,
while retaining full Phase 0/1 reports and manifests outside the final bundle
under `docs/okf-migration/`.

**Rationale:** Historical reasoning and owner questions must survive, but a
full copy of 19 historical rows and 14 bootstrap reports would recreate the
legacy corpus and confuse current authority.

**Alternatives considered:** Delete all history after conversion; copy every
legacy report into the bundle; keep every historical item only outside the
bundle. The first loses context, the second creates duplication, and the third
makes the final bundle harder to understand without migration records.

**Consequences:** History Concepts need explicit snapshot labels and source
links; migration reports remain the detailed audit authority.

**Basis:** Phase 1 historical dispositions, claim ledger, and migration risks
OKF-MIG-001/004/007.

**Reversibility:** Medium; a history Concept can be split or externalized after
review, but only after confirming no unique reasoning is lost.

## OKF-ADR-011 - Embed diagrams in Concept Markdown bodies

**Decision:** Do not place standalone `.mmd` files in the target bundle. Embed
19 preserved visual blocks, 8 merged blocks, and 6 regenerated blocks in the
owning Concept bodies.

**Rationale:** Google OKF defines Markdown Concepts but no Mermaid artifact
semantics. Embedded diagrams keep the bundle self-contained without treating
non-Concept files as a second registry, while the existing diagram sources
remain outside as evidence.

**Alternatives considered:** Copy all 50 `.mmd` files; retain a `diagrams/`
asset directory; drop all diagrams. Copying would preserve duplication and
ambiguity, an asset directory needs an unstandardized reference contract, and
dropping loses valuable boundaries.

**Consequences:** Diagram syntax/render review is part of Concept review; the
final bundle has an estimated 33 visual blocks rather than 50 files.

**Basis:** Phase 1 diagram map and Google OKF bundle/Markdown semantics.

**Reversibility:** Medium; a later policy could place reviewed assets in an
explicit external references area without changing Concept IDs.

## OKF-ADR-012 - Use a lightweight read-only validator and no attestation

**Decision:** Future automation should be one small dependency-light validator
with separate Google, local-policy, integrity, reconciliation, and safety
exit classes. Do not introduce Attested Computation concepts.

**Rationale:** The repository needs deterministic preservation checks, not a
large OKF runtime. The Phase 1 manifest is an audit artifact, not a sanctioned
computation with an executor, receipt, and attester.

**Alternatives considered:** Copy Google's reference tooling; add a schema
framework; create Attested Computations for every audit. These add unsupported
runtime or maintenance obligations.

**Consequences:** Static checks cannot prove production behavior or live owner
policy; those remain explicit content gates.

**Basis:** Google `SPEC.md` non-goals and computation sections; Phase 1 risk
OKF-MIG-017.

**Reversibility:** High; a future separate decision can add an attested
computation only with a real executor/attester contract.

## OKF-ADR-013 - Make cutover atomic and rollback by revert

**Decision:** After staging gates pass, add the validated target at `/okf/`,
remove the legacy canonical contents, and update active navigation in one
reviewed cutover change. Roll back with a reviewed Git revert or restore of
the pre-cutover branch; never use destructive reset/clean operations.

**Rationale:** A partial replacement would create a mixed authority tree. The
pre-cutover commit retains the full legacy corpus and provides a recoverable
rollback point.

**Alternatives considered:** Incremental in-place replacement; leave both
trees permanently; delete legacy before adding the target. Each creates a
period of ambiguity or an irreversible knowledge gap.

**Consequences:** Cutover requires a full validation/review gate and a
repository-reference audit. It remains outside Phase 2.

**Basis:** Phase 0/1 preservation constraints and
[MIGRATION_STAGING_AND_CUTOVER_PLAN.md](MIGRATION_STAGING_AND_CUTOVER_PLAN.md).

**Reversibility:** High before cutover and recoverable after cutover through a
reviewed revert.

## Decision summary

The decisions collectively produce a minimal Google-native design:

- one isolated future bundle boundary;
- 54 capability Concepts instead of 353 copied files;
- 19 descriptive types and zero extensions;
- official provenance/trust/lifecycle semantics used conservatively;
- visible owner-decision and historical boundaries;
- 13 indexes, one semantic log, 33 embedded visuals;
- deterministic validation without a runtime dependency;
- atomic cutover with a recoverable legacy commit.

These decisions are complete for Phase 2. Construction decisions that require
current source snapshots, owner answers, or actual validation events remain
future work and are recorded as blockers rather than guessed here.
