# ZinharCMS Google OKF v0.2 Type and Metadata Policy

**DESIGN ONLY - NOT YET IMPLEMENTED**

This policy defines the local metadata contract for the future bundle described
in [TARGET_BUNDLE_TREE.md](TARGET_BUNDLE_TREE.md). It distinguishes official
Google OKF semantics from stricter ZinharCMS choices. It does not modify the
legacy frontmatter in `okf/`.

## Authority split

The official source is Google [`okf/SPEC.md`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md).
Google v0.2 requires only a parseable frontmatter block and a non-empty
`type` on each non-reserved Concept. It does not define this vocabulary or
require the optional fields below.

ZinharCMS imposes a small local policy for source-backed navigation and safe
maintenance. A local failure is a ZinharCMS policy failure, not evidence that
Google OKF itself rejects the bundle. The future validator reports these
classes separately.

## Approved type vocabulary

There are 19 approved type values. Values are descriptive metadata, not
directory names, database table names, or a central taxonomy registry.

| Type | Purpose | Use for | Do not use for | Origin |
| --- | --- | --- | --- | --- |
| `Project` | Product identity and scope | Product overview | Every document that mentions ZinharCMS | ZinharCMS-defined from project candidate evidence |
| `Reference` | Curated orientation or evidence map | Glossary, repository map, source lineage | Current runtime behavior or decisions | Google-example-derived descriptive type |
| `Architecture` | System structure and boundary reasoning | Runtime boundary, integrations, architecture decisions | Endpoint or entity catalogs | ZinharCMS-defined |
| `Component` | An independently understandable implementation component | Backend runtime, frontend app, plugin boundary | A business workflow or one function | Google-example-derived descriptive type |
| `API Contract` | A public or client-facing interface contract | Route-family contracts and compatibility caveats | Internal module descriptions | ZinharCMS-defined |
| `API Endpoint` | A generated/reconciled endpoint surface | Route/OpenAPI inventory | Narrative API architecture | Google-example-derived from the official example vocabulary |
| `Data Model` | Schema/entity relationship knowledge | Migrations and entity graph | Legal retention or runtime authorization policy | Google-example-derived descriptive type |
| `Database Policy` | Database behavior or governance policy | RLS, tenant data, retention boundary | A complete schema inventory | ZinharCMS-defined |
| `Authentication Flow` | Identity/session/MFA behavior | Authentication and session Concepts | General security risk register | Candidate vocabulary aligned to Google examples |
| `Authorization Policy` | Permission and access decision rules | RBAC and authorization | Authentication token lifecycle | ZinharCMS-defined |
| `Security Control` | A security boundary, control, or risk posture | Tenant, preview, storage, posture Concepts | Product roadmap or owner-only decisions | ZinharCMS-defined |
| `Domain Model` | A business capability model | Billing and quota capability | A single state transition procedure | ZinharCMS-defined |
| `Domain Workflow` | A stateful business process | Content, page, and editorial workflows | Raw schema or route list | Google-example-derived descriptive type |
| `Resource Boundary` | Ownership, storage, or delivery boundary | Media and public delivery | General domain overviews | ZinharCMS-defined |
| `Marketplace Domain` | Marketplace behavior and lifecycle | Catalog, review, installation, finance, runtime boundaries | Generic extension hooks | ZinharCMS-defined |
| `Operational Guide` | Runtime operation, environment, release, or support procedure | Compose/CI/deployment/observability Concepts | Current source architecture | Google-example-derived from `Playbook` semantics |
| `Development Guide` | Contributor, test, or maintenance workflow | Development and documentation procedures | Product runtime behavior | ZinharCMS-defined |
| `Decision` | An accepted or unresolved policy decision | NOC register and migration ADRs | Unverified implementation facts | Google-example-derived descriptive type |
| `Historical Record` | A dated or superseded knowledge record | Phase and bootstrap history | Current implementation truth | ZinharCMS-defined |

No new type may be added for a single legacy directory or document category.
Adding a type requires a new decision record, catalog impact analysis, and
evidence that an existing type cannot describe the concept.

## Field classes

### Google-required fields

| Field | Semantics | Phase 3 rule | Validation |
| --- | --- | --- | --- |
| `type` | Short string identifying the Concept kind | Required on every non-reserved `.md` | Non-empty string and one of the 19 local values |

### Google-standard optional fields adopted by project policy

| Field | Semantics | Required when | Optional/prohibited cases | Value and validation rule |
| --- | --- | --- | --- | --- |
| `title` | Human-readable display name | Every future Concept | Never omitted from the target bundle | Non-empty scalar; should match catalog title |
| `description` | One-sentence retrieval/index summary | Every future Concept | Do not use as a multi-paragraph abstract | Non-empty scalar; no unverified guarantee language |
| `resource` | Canonical URI for the underlying asset | The Concept directly describes a unique asset with a stable URI | Omit for abstract workflows, decisions, and history | Absolute URL or accepted bundle-relative path; no invented URI |
| `tags` | Short cross-cutting labels | Recommended when a Concept spans domains or retrieval facets | Omit when tags add no useful distinction | YAML list of short strings; no tag registry is required |
| `sources` | Materials from which the Concept derives | Every current, historical, decision, or reference Concept in this source-backed bundle | No omission in the final target bundle; a source must still be meaningful | Non-empty list; each entry has `resource`; stable `id` when body footnotes use it |
| `generated` | How the current content was produced | Only when a real generator/process/agent wrote the content | Omit for manually authored content when no producer event is recorded | Mapping with `by` and ISO 8601 `at`; no placeholder actor |
| `verified` | Actual confirmation against sources/resource | Only after an identified human or process performs the check | Omit rather than infer from legacy labels or dates | Mapping or list of `{by, at}`; must use actor convention |
| `status` | Lifecycle state | Every future Concept under local policy | None; use body text for historical scope, not custom lifecycle values | Exactly `draft`, `stable`, or `deprecated` |
| `stale_after` | Absolute date after which the Concept is stale | Only when an owner-defined review interval exists | Omit for history, decisions, and stable knowledge without a policy | ISO `YYYY-MM-DD`; no Phase 2 dates |

`title`, `description`, `status`, and `sources` are local requirements, not
Google requirements. `generated`, `verified`, `stale_after`, `resource`, and
`tags` remain conditional because fabricating them would create a false trust
signal.

### Root index and log metadata

`index.md` and `log.md` are reserved filenames and are not Concepts:

- The root `index.md` may contain frontmatter with exactly
  `okf_version: "0.2"`; it has no `type`.
- Domain `index.md` files contain no frontmatter.
- `log.md` files contain no frontmatter. The only planned log is the root log,
  with newest-first `YYYY-MM-DD` headings.
- No `okf_version` key occurs in a Concept or a non-root index.

## Provenance policy

### Source entries

Every final Concept has at least one `sources` entry. Each entry has:

```yaml
sources:
  - id: route-composition
    resource: https://github.com/sasanzare/ZinharCMS/blob/<immutable-commit>/backend/src/routes/mod.rs
    title: Backend route composition
```

The construction process replaces `<immutable-commit>` with an actual commit
SHA before writing the bundle. The design does not authorize an unpinned
branch URL, a guessed file path, or a source path that was not checked at the
construction snapshot. Source `author`, `usage_count`, `last_modified`, and
`usage_window` are included only when the fact is known; no credibility score
is stored.

Sources may point to current source, migrations, tests, configuration, CI,
current project documentation, Phase 1 evidence, or an external authoritative
reference. A legacy OKF file alone cannot promote a claim to current truth.
When a source is a final bundle Concept, the body uses a normal bundle-root
link such as `/database/schema-and-migrations.md`; external repository
evidence uses an immutable URL.

When a body sentence needs per-claim attribution, its footnote label is the
matching `sources[].id`. A central source register or custom relation array is
not part of the target schema.

### Legacy provenance translation

| Legacy information | Target treatment |
| --- | --- |
| `primary_sources` | `sources` entries after path/existence/snapshot verification |
| `related_documents` | Contextual Markdown links with relationship wording |
| `related_diagrams` | Body links/diagram sections; no custom key |
| `last_verified_commit` | Immutable source URL context or history prose; never a verifier actor |
| `last_verified_date` | Used only if the actual verification event and scope are known |
| `okf_document_id` | Dropped; Concept identity is the target file path |
| `category`, `phase`, and project labels | Type input, body context, or history; never copied blindly |
| `confidence`, uncertainty, and implementation fields | Body caveat, Decision link, or omission; zero custom extensions |

This follows the detailed [legacy metadata translation](LEGACY_METADATA_TRANSLATION.md).

## Generated and verified policy

`generated` and `verified` answer different questions:

- `generated` says who/what wrote the current file and when it last changed.
- `verified` says who/what checked the current content against its sources or
  resource and when.

The construction process may use an actor such as
`process:zinharcms-okf-builder` only if that process actually exists and ran.
The validator may use a real process actor such as
`process:zinharcms-okf-validator` only for checks it actually performed. A
human reviewer is recorded only as an owner-supplied `human:<id>`; neither
`human:codex` nor an invented role is allowed.

The trust interpretation remains the official one:

- absent `verified`: unverified;
- non-human verifiers only: machine-confirmed;
- at least one `human:` verifier: human-reviewed.

Trust is advisory and never an authorization or deletion gate. A meaningful
edit to a verified Concept removes or invalidates its previous verification in
the next construction change. Verification of an index or validator report
does not verify every linked Concept automatically.

## Lifecycle and freshness policy

| State | Use in ZinharCMS | Required body treatment |
| --- | --- | --- |
| `draft` | Incomplete, owner-blocked, source-sensitive, or awaiting review | State the missing evidence/decision and link the relevant Decision Concept |
| `stable` | Current, evidence-backed content ready for normal consumption | Distinguish repository facts from deployment/policy limits |
| `deprecated` | Superseded current-state material retained for links/history | State the replacement or historical scope; never present it as current |

`Historical Record` is a type, not a fourth lifecycle value. A historical
Concept may be `stable` as a stable record of history, but its body must label
the snapshot and it must not be used as current implementation evidence.

Use `stale_after` only for Concepts with a documented review trigger or
owner-defined interval. The initial candidates are route/API surfaces,
deployment/operations, security posture, and generated schema catalogs. Do not
invent a universal TTL and do not populate dates in Phase 2. History and
decision records normally omit `stale_after`; they change when their underlying
record changes rather than because a date elapsed.

## Unknowns and decisions

Open knowledge is represented with standard fields and body structure:

1. `decisions/owner-decision-debt.md` is a `Decision` with `status: draft`.
2. Each NOC has a stable Markdown heading/anchor, current status, affected
   Concepts, evidence gap, and required resolution evidence.
3. Affected Concepts contain an explicit `Open decision dependencies` section
   and never place the unresolved proposal in a factual implementation table.
4. When a NOC resolves, the register, affected Concept sections, indexes, and
   any `verified` events are updated together.

The 15 UNKNOWN labels from Phase 1 remain visible through the register and
affected Concept caveats. No `unknown`, `needs_owner_confirmation`, or
`confidence` extension is introduced.

## Extensions and prohibited fields

The target defines **zero custom frontmatter extension keys**. The Google
specification permits unknown keys, but the local zero-extension policy makes
the bundle easier to exchange and prevents the 53-key legacy model from being
recreated under new names.

The following are prohibited in target Concept frontmatter by ZinharCMS
policy: `okf_document_id`, `project`, `category`, `phase`,
`source_of_truth`, `review_status`, `last_verified_commit`,
`last_verified_date`, `primary_sources`, `related_documents`,
`related_diagrams`, `uncertainty_markers`, `implementation_view`, and
topic-specific legacy status keys. Their meaning must be represented by
official fields, body prose, Markdown links, immutable source URLs, Git
history, or an explicit history/decision Concept.

Attested Computation is not used. ZinharCMS has no sanctioned computation
contract, executor, receipt, or deterministic attester in this migration. A
future request to add one requires a separate decision record and contract;
the Phase 1 evidence manifest is not an attested computation.

## Policy validation summary

The future validator must report separately:

- Google conformance: `type`, frontmatter, encoding, and reserved files;
- local metadata policy: allowed types, required local fields, sources, actors,
  status, freshness, zero extensions, and prohibited legacy keys;
- knowledge integrity: source URLs, claim status, owner-debt separation, and
  no unsupported stable claims.

See [OKF_VALIDATION_CONTRACT.md](OKF_VALIDATION_CONTRACT.md) for executable
check boundaries and [PHASE_02_DECISION_RECORD.md](PHASE_02_DECISION_RECORD.md)
for the decisions that make this policy reversible and reviewable.
