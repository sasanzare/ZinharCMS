# ZinharCMS Google OKF v0.2 Migration Staging and Cutover Plan

**DESIGN ONLY - NOT YET IMPLEMENTED**

This plan defines how a future Phase 3+ construction can create the Google OKF
bundle without mixing it with the legacy `/okf/` tree. Phase 2 creates no
staging directory, moves no files, and performs no cutover.

## Boundaries

| Boundary | Location | Authority and purpose |
| --- | --- | --- |
| Legacy preservation corpus | `/okf/`, `/okf-bootstrap/`, and `/okf/index.yaml` | Historical/source preservation only. It remains intact until cutover verification is complete. It is not Google OKF v0.2 authority. |
| Migration design/evidence | `docs/okf-migration/` | Phase reports, manifests, mapping, decisions, validation contracts, and handoff. These documents are outside the future bundle. |
| Temporary construction bundle | `docs/okf-migration/staging/google-okf-v0.2/` | Isolated candidate Google OKF tree. It is not canonical and must not be linked as the repository's current OKF until cutover. It is not created in Phase 2. |
| Final canonical bundle | `/okf/` after cutover | The only production/documentation navigation target after the atomic cutover. It follows [TARGET_BUNDLE_TREE.md](TARGET_BUNDLE_TREE.md). |

The staging path is deliberately under the migration boundary and has a
different root from legacy `/okf/`. It prevents a reader from confusing legacy
custom metadata with the target Google frontmatter. A future stage must not use
`okf/` as a build directory or write a second target tree beside legacy files.

## Authority model during transition

| Question | Authoritative source before cutover | Staging role | Authoritative source after cutover |
| --- | --- | --- | --- |
| Current implementation behavior | Source code, migrations, tests, executable configuration, CI, and current docs in the source hierarchy | Staging Concept is a source-backed summary and must cite an immutable snapshot | Same implementation hierarchy; bundle never replaces executable specifications |
| Legacy knowledge preservation | Phase 1 matrix, claim ledger, diagram map, owner register, and intact legacy trees | Mapping/reconciliation input; legacy claims are not promoted without evidence | Phase reports/history and final bundle history preserve the verified lineage; old trees are no longer canonical navigation |
| Target Concept content | Not yet available | Staging bundle, with `draft`/`stable` and `sources` according to policy | Final `/okf/` bundle after validation and review |
| Repository navigation | Existing README and legacy navigation | No production navigation link to staging | Updated README/handoff/docs point to final `/okf/` |
| Open decisions | `OWNER_DECISION_REGISTER.md` | `decisions/owner-decision-debt.md` may mirror them as a draft Concept | Final Decision Concept plus external register; no unresolved item becomes a fact |

The staging bundle can be consumed for review, but it is never allowed to
silently override the implementation or owner evidence. A stable staging
Concept with a stale/contradicted/unknown claim fails the local policy gate.

## Construction sequence

1. **Freeze the source snapshot.** Record branch, commit, clean/dirty state,
   relevant changed paths, and Phase 1 corpus/manifests. Do not use an
   uncommitted security or application change as an unrecorded source basis.
2. **Create the isolated staging root.** Construct only under
   `docs/okf-migration/staging/google-okf-v0.2/`. Add the target root index,
   root log, domain indexes, and the 19 high-confidence Concepts first.
3. **Build from the catalog.** Add Concepts in target-ID order. Use the
   approved type and metadata policy; create no legacy field aliases and no
   standalone `.mmd` assets.
4. **Attach provenance.** Pin source URLs to the construction commit. Use
   `generated` only when a real builder ran and `verified` only after a real
   verification event. Keep source-sensitive or owner-dependent Concepts in
   `draft`.
5. **Embed diagrams.** Preserve the 19 unique visual blocks, create the 8
   merged blocks, and regenerate the 6 planned blocks only after current source
   review. Keep the existing diagram files untouched as evidence.
6. **Reconcile the mapping.** Run the document, claim, NOC, and diagram
   reconciliation checks before treating the staged tree as complete.
7. **Review the staged bundle.** Review index coverage, source links, stale and
   contradicted claims, ownership caveats, and the absence of legacy metadata.
8. **Record a construction checkpoint.** Update the migration log and handoff
   with actual paths, counts, validator output, and unresolved blockers.

No step above is performed by Phase 2.

## Validation gates before cutover

Cutover is prohibited until all of the following are true:

### Gate A - source and worktree safety

- the source commit and staging input are recorded;
- no legacy file was modified, deleted, moved, or renamed during construction;
- `okf/index.yaml` hash and bytes remain unchanged;
- application, dependency, CI, Docker, and deployment files are unchanged by
  the knowledge migration;
- the staging tree is outside `/okf/` and no canonical link points to it.

### Gate B - Google OKF conformance

- every non-reserved staged `.md` has UTF-8, parseable frontmatter, and a
  non-empty `type`;
- root and domain indexes follow reserved semantics;
- root index declares only `okf_version: "0.2"` in frontmatter;
- root log has newest-first ISO date headings;
- optional families, when used, follow Google v0.2 shape.

### Gate C - ZinharCMS policy

- all 54 catalog paths and 13 indexes exist in staging;
- only the 19 approved types and zero extension keys are used;
- local required metadata and sources are present;
- actors, statuses, freshness, source URLs, links, and index entries pass;
- embedded diagram policy is satisfied and no `.mmd`/`index.yaml` is in the
  staged bundle;
- stable current Concepts do not contain unsupported, stale, contradicted, or
  unresolved owner claims as if they were facts.

### Gate D - preservation reconciliation

- 353 document rows map to target IDs/actions;
- 13 merge groups and the exact disposition counts reconcile;
- 50 diagram rows map to 19 preserve, 19 merge, 11 regenerate, and 1 drop;
- 60 claims and 18 NOC records remain accounted for;
- the equivalence review signs off on legacy knowledge that is intentionally
  omitted because it is redundant or navigation-only.

### Gate E - human review and authority approval

- the owner approves the staging bundle as the proposed replacement;
- any `human:<id>` verification actors are real and supplied by the owner;
- production-only gaps are either resolved with evidence or explicitly kept as
  draft/blocked content;
- the cutover commit and rollback/revert plan are approved.

The exact check classes and exit codes are in
[OKF_VALIDATION_CONTRACT.md](OKF_VALIDATION_CONTRACT.md).

## Cutover sequence

The physical replacement is a later, explicitly authorized operation. It must
be reviewable as one coherent repository change:

1. Freeze the approved staging source snapshot and ensure the worktree is in
   the expected state.
2. Run all validation gates and save the actual machine-readable report under
   the migration evidence boundary.
3. Confirm the Phase 1 manifest, legacy `okf/index.yaml` hash, and pre-cutover
   legacy tree inventory still match the recorded preservation evidence.
4. Prepare an atomic cutover change that adds the validated target tree at
   `/okf/`, removes the legacy `/okf/` content from the canonical path, and
   updates root README, applicable navigation, and handoff references in the
   same reviewable change. The old `/okf/` must never be replaced by a partial
   target tree.
5. Keep `docs/okf-migration/` and its mapping/evidence records outside `/okf/`.
   Preserve the full legacy snapshot in Git history and retain the history
   Concepts required by the catalog; do not copy the legacy custom registry
   into the new bundle.
6. Decide separately whether `okf-bootstrap/` is removed in that cutover or
   retained until a later zero-residue review. If removed, its unique content
   must already be represented by the history/decision Concepts and the
   migration evidence must remain outside `/okf/`.
7. Run the post-cutover zero-residue audit: no active navigation claims the
   old tree is canonical, no code/tooling points at removed paths, no legacy
   metadata is in the final bundle, and every final link/index/path validates.
8. Record the resulting commit, file inventory, validation report, and exact
   rollback point in `HANDOFF.md`.

The old tree is logically removed only at step 4, after equivalence and human
review gates pass. Phase 2 does not reach that step.

## Rollback and recovery

Rollback is a normal Git revert of the reviewed cutover commit (or a restore
of the pre-cutover branch in a controlled review), never `git reset --hard`,
`git clean`, or an unreviewed destructive command. The pre-cutover commit
retains the intact legacy tree, and the migration reports retain the
preservation evidence. If post-cutover validation fails:

1. stop further documentation changes;
2. record the failing gate and affected paths;
3. revert the cutover change in a reviewed commit or restore the prior branch;
4. leave the legacy tree available as the preservation authority;
5. correct staging/mapping/policy and rerun all required gates.

Rollback does not answer owner decisions or silently restore stale legacy
claims as current truth; it restores the last known repository state while the
design is corrected.

## Zero-residue audit

After cutover, the audit must search active repository documentation, scripts,
CI, and navigation for exact legacy references (`okf-bootstrap`,
`okf/index.yaml`, and legacy-only metadata). Historical references in
`docs/okf-migration/` and history Concepts are allowed when clearly labeled.
The audit must distinguish those references from application/runtime coupling;
Phase 0 found no executable dependency, but the search must be repeated at
the actual cutover commit.

## Phase boundary confirmation

This plan intentionally leaves the construction root absent. Phase 2 has not:

- created `docs/okf-migration/staging/`;
- changed `/okf/`, `/okf-bootstrap/`, or `okf/index.yaml`;
- updated README or application navigation;
- moved or deleted a legacy file;
- constructed a target Concept.
