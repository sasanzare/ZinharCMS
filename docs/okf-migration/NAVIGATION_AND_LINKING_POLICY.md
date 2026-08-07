# ZinharCMS Google OKF v0.2 Navigation and Linking Policy

**DESIGN ONLY - NOT YET IMPLEMENTED**

This policy defines how a future consumer discovers the 54 Concepts in the
target tree. It follows Google's optional `index.md`, `log.md`, and Markdown
link semantics while adding only the local rules needed for predictable human
and agent navigation.

## Navigation goals

The target bundle must allow an agent or developer to:

1. identify the bundle and its authority boundary from the root;
2. discover a domain before opening every Concept;
3. move between related API, security, database, domain, and operations Concepts
   through ordinary Markdown links;
4. distinguish current, historical, draft, and owner-blocked knowledge;
5. find a source-backed Concept without relying on the legacy YAML registry.

The hierarchy supplies broad parent/child context. Links add meaningful
relationships; they do not recreate a central typed graph.

## `index.md` policy

`index.md` is optional in Google OKF. ZinharCMS chooses a root index and one
index in each of the 12 major domain directories because the target is large
enough to benefit from progressive disclosure but shallow enough that deeper
indexes would add maintenance without improving retrieval.

### Planned index set

| Scope | Index | Policy |
| --- | --- | --- |
| Bundle root | `/okf/index.md` | Required by local policy; the only index allowed to declare `okf_version: "0.2"` |
| Each major domain | `project/index.md`, `architecture/index.md`, `backend/index.md`, `frontend/index.md`, `api/index.md`, `database/index.md`, `security/index.md`, `domain/index.md`, `operations/index.md`, `development/index.md`, `decisions/index.md`, `history/index.md` | Required by local policy; no frontmatter; lists only direct child Concepts |
| Any deeper directory | None planned | Add only after a decision record proves a new navigation boundary is useful |

The root index body contains a short purpose statement and links to the 12
domain directories in the order in
[TARGET_BUNDLE_TREE.md](TARGET_BUNDLE_TREE.md). It has only this frontmatter:

```yaml
---
okf_version: "0.2"
---
```

Domain indexes have no frontmatter. They group Concepts by retrieval role
where useful (for example, `Current implementation`, `Generated surfaces`,
and `Risks and decisions`) and list each direct child exactly once. A Concept
must not be duplicated under two headings merely to improve search.

### Curated versus generated content

Indexes are generated from the approved target catalog and each Concept's
`title` and `description`, then reviewed in the same change as the Concept
set. The generation input is the staged tree; the index is not generated from
legacy `index.yaml`. Human review may adjust section headings and wording, but
must not hide a Concept or introduce a path absent from the tree.

Ordering rules are deterministic:

1. domain directories use the tree order in the target architecture;
2. within a domain, sections are `Current implementation`, `Generated or
   policy surfaces`, `Risks and decisions`, and `History` only when applicable;
3. within a section, Concepts are ordered by catalog ID/path, not by file
   creation time;
4. the root index lists domains in the tree order and does not list individual
   Concepts.

Google consumers may synthesize indexes when absent and must tolerate broken
links. The local policy is stricter for a constructed release: every index
entry intended to be present must resolve, and every direct child Concept must
be listed once. The distinction is recorded in the validation contract.

## `log.md` policy

ZinharCMS uses exactly one log: the root `/okf/log.md`. Per-domain logs are
omitted because they would duplicate Git history and create multiple semantic
maintenance obligations. The root log records only knowledge changes that
matter to a consumer:

- creation or retirement of a Concept;
- a material source/provenance change;
- a resolved owner decision that changes current knowledge;
- a significant diagram or domain-boundary change;
- a deliberate status/freshness transition.

Routine wording, index regeneration, formatting, and ordinary source commits
are not separate log entries. Generated changes are logged only when they
change the knowledge surface or a generated contract, not on every run.

Entries are newest first, use `## YYYY-MM-DD` headings, and contain concise
prose with optional links to affected Concepts. Git remains the authoritative
line-by-line history and the migration reports remain the detailed audit
record. The log never replaces either one.

## Concept link forms

Use these forms in future Concept bodies:

- **Bundle-root-relative links** are preferred for Concept-to-Concept links,
  for example `/security/tenant-isolation.md`. They remain stable if a Concept
  is edited in its domain directory.
- **Relative links** are used in indexes and may be used for a local neighbor,
  for example `./tenant-isolation.md`.
- **External URLs** are used for immutable repository evidence, official
  specifications, and owner-supplied external references. They do not become
  Concept relationships unless the body says what they support.

Do not use `../../` links from a final Concept to files outside `/okf/`.
Project evidence outside the bundle belongs in `sources[].resource` as an
immutable URL, which avoids ambiguity about the authority root.

## Relationship wording

OKF defines links as directed Markdown edges and leaves the relationship kind
to surrounding prose. Use a short sentence before or after a link to make the
edge understandable:

| Relationship | Example wording |
| --- | --- |
| Parent/domain | `This Concept belongs to the security model at /security/security-posture-and-risks.md.` |
| Implements/realizes | `The route surface at /api/route-surface.md realizes the API families described here.` |
| Enforced by | `Tenant access is enforced by /security/tenant-isolation.md.` |
| Persists in | `The workflow is represented by /database/entities-and-relationships.md.` |
| Depends on | `Preview depends on the /security/preview-security.md boundary.` |
| Decision dependency | `The intended routing policy remains in /decisions/owner-decision-debt.md#noc-01.` |
| Historical replacement | `This snapshot is superseded by the current /architecture/system-architecture.md.` |

The exact phrases are not a schema. The requirement is that a reader can tell
why the link exists without a separate graph registry.

## Cross-domain link rules

Use links when the relationship is material to understanding or retrieval:

- `project` links to architecture and governance, not every Concept;
- `architecture` links to backend, frontend, API, database, and security
  boundaries;
- `api` links to security, domain, and generated database/API surfaces;
- `database` links to tenant/security and domain Concepts;
- `security` links to API and domain boundaries that it enforces;
- `domain` links to the API, database, security, and extension Concepts it
  actually depends on;
- `operations` links to implementation/configuration Concepts and decision
  debt when an operational claim is unverified;
- `decisions` links outward to affected Concepts; affected Concepts link back
  only to the specific decision anchor, not to the entire register;
- `history` links to current replacements only when the replacement is useful
  for understanding the historical record.

Avoid a complete mesh. A Concept should normally have a small number of
meaningful outgoing links and no generated backlink section. Parent indexes
provide discovery without requiring every Concept to link to its index.

## Orphan prevention

The future local validator enforces:

1. every Concept has exactly one direct parent index entry;
2. every root domain is listed by the root index;
3. every non-history Concept is reachable from the root index through index
   links;
4. every history/decision Concept is also reachable from the root index, even
   if no current Concept links to it;
5. every internal link that is intended as a current target resolves within the
   staged bundle;
6. external URLs use an allowed URL scheme and are not mistaken for local
   paths;
7. the validator reports unused Concepts but does not delete them.

The Google conformance result remains independent: broken links, missing
indexes, unknown types, and missing optional fields do not make a bundle
non-conformant under the official permissive rules. They are local navigation
quality failures when ZinharCMS deliberately chooses the stricter policy.

## Claim attribution and source navigation

When a Concept has multiple evidence sources, `sources[].id` values are stable
lower-kebab-case keys and body footnotes reference those IDs. Do not use a
positional `sources[0]` convention. The source list is not a replacement for
the relationship graph: links express in-bundle lineage, while source entries
express derivation from external or internal materials.

Every external repository source is pinned to an immutable construction commit.
An unpinned `main` link may appear in a migration report as a specification
reference, but it is not accepted as authoritative Concept provenance.

## Navigation change workflow

When adding, splitting, retiring, or renaming a future Concept:

1. update the target catalog and decision/mapping record;
2. update the direct domain index and root index if the domain surface changes;
3. update links from affected Concepts and the root log if the change is
   semantically meaningful;
4. run the official, local-policy, and migration-reconciliation checks;
5. review unresolved NOC and history links for accidental promotion of old
   claims.

No navigation change is made to the legacy `okf/` tree during Phase 2.

See [OKF_VALIDATION_CONTRACT.md](OKF_VALIDATION_CONTRACT.md) for the
deterministic checks and [MIGRATION_STAGING_AND_CUTOVER_PLAN.md](MIGRATION_STAGING_AND_CUTOVER_PLAN.md)
for when repository-wide navigation may change.
