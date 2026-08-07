# Google OKF v0.2 Baseline

This document records the official migration target for ZinharCMS. It is a
normative reading guide for later phases, not a target bundle and not a reason
to rewrite the existing legacy tree during Phase 0.

## Authority and version check

Primary authority: [GoogleCloudPlatform/knowledge-catalog `okf/SPEC.md`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md).

Supporting official tree: [GoogleCloudPlatform/knowledge-catalog `okf/`](https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf).

The specification read on 2026-08-07 identifies itself as **Open Knowledge
Format (OKF) Version 0.2**. The current upstream `main` source has not changed
to a version other than the expected v0.2. The specification itself, not the
reference agent, visualizer, examples, or third-party implementations, is the
normative source.

## Minimal model

| Term | Official meaning |
| --- | --- |
| Bundle | A self-contained hierarchical collection of knowledge documents; the unit of distribution |
| Concept | One unit of knowledge represented by one Markdown document |
| Concept ID | The concept file path within the bundle with `.md` removed |
| Frontmatter | YAML metadata delimited by `---` at the start of a Markdown file |
| Body | Everything after frontmatter; standard Markdown with no required sections |
| Link | A standard Markdown link between concepts, expressing a directed relationship in context |
| Source/provenance | Materials a concept derives from, recorded in `sources` |
| Trust/lifecycle | Optional frontmatter signals for authorship, verification, status, and freshness |
| Attested Computation | A concept that describes a sanctioned computation and how a consumer can verify a run |

OKF is intentionally minimal. It does not define a schema registry, central
authority, runtime, validator, storage system, serving system, query system, or
fixed domain taxonomy.

## MUST — conformance requirements

These are the requirements that later migration work must satisfy when it calls
a directory a conformant OKF v0.2 bundle.

| Requirement | Official baseline |
| --- | --- |
| Bundle/document form | A bundle is a directory tree of Markdown files. A bundle may be distributed as a Git repository, tar/zip archive, or subdirectory. Git is recommended, not required. |
| Concept file encoding | Every concept is a UTF-8 Markdown file with an opening `---` on its own line at the start and a closing `---` on its own line. |
| Required field | Every non-reserved `.md` file has parseable YAML frontmatter containing a non-empty `type` field. `type` is the only always-required frontmatter key. |
| Type handling | Producers should choose descriptive type strings; consumers must tolerate unknown type values as generic concepts. There is no central type registry. |
| Reserved filenames | `index.md` and `log.md` have defined meanings at any directory level and must not be used as concept documents. When present, they follow their respective structures. |
| Index structure | An `index.md` contains directory listings grouped under headings. It has no frontmatter except that a bundle-root `index.md` may carry `okf_version`. |
| Log structure | A `log.md` is a flat, newest-first history with ISO `YYYY-MM-DD` date headings. Entry wording is prose; bold action labels are only conventional. |
| Standard links | Concept relationships use ordinary Markdown links, either bundle-root-relative paths beginning with `/` or ordinary relative paths. The surrounding prose conveys relationship meaning. |
| Path-valued fields | `resource`, `sources[].resource`, `computation`, `executor.resource`, and `attester.resource` accept absolute URLs, bundle-root-relative paths, or relative paths. A source resource may also be a non-path scope descriptor. |
| Actor convention | Identity fields use `<producer>/<version>`, `human:<id>`, or `process:<id>`. Human-authored or human-confirmed actors must use the `human:` prefix so trust tiers can be derived correctly. |
| Consumer permissiveness | Consumers must not reject a bundle for missing optional fields, unknown additional keys, unknown types, broken cross-links, or missing `index.md`. A bare `verified` mapping must be treated as a one-element list. |

The conformance section is deliberately small. It does not require every
recommended field, every conventional heading, or every optional trust,
provenance, lifecycle, or computation family.

## SHOULD — recommended conventions

The following conventions improve consumption but are not universal mandatory
fields:

| Convention | Official baseline |
| --- | --- |
| Descriptive concept metadata | Use `type`, `title`, `description`, `resource`, and `tags` where applicable. A title may be derived from the filename when absent. |
| Type values | Choose short, descriptive, self-explanatory values such as `API Endpoint`, `Metric`, `Playbook`, or `Reference`; do not invent or depend on a central registry. |
| Body structure | Prefer structural Markdown such as headings, lists, tables, and fenced code blocks. There are no required body sections. `# Schema`, `# Examples`, and `# Computation` have conventional meanings when applicable. |
| Provenance attribution | Use `sources` for source materials. Include a stable `sources[].id` when body claims need attribution, and cite that ID through a Markdown footnote. |
| Index entries | Include a linked concept's description when generating `index.md`; an index may be generated or synthesized by a consumer. |
| Git distribution | Prefer Git when history, attribution, and diffs matter. |
| Trust/lifecycle interpretation | Derive trust tiers and staleness only from the defined fields, and surface a failing attestation rather than silently dropping it. |

## MAY — optional fields and features

All of the following are optional unless a concept uses the relevant feature:

### Provenance: `sources`

```yaml
sources:
  - id: implementation
    resource: backend/src/routes/mod.rs
    title: Composed backend router
    author: process:repository-audit
    last_modified: 2026-08-07
usage_window: { from: 2026-08-01, to: 2026-08-07 }
```

Each source entry must have `resource` if the `sources` family is used. `id`,
`title`, `author`, `usage_count`, and `last_modified` are optional. A shared
`usage_window` frames usage counts, and an individual entry may override it.
These credibility signals are facts, not a stored credibility score. Lineage is
expressed by concept links; deeper explicit external lineage is out of scope for
v0.2.

### Trust and freshness

```yaml
generated: { by: process:repository-audit, at: 2026-08-07T12:00:00Z }
verified:
  - { by: human:reviewer-id, at: 2026-08-07T13:00:00Z }
status: stable
stale_after: 2026-09-07
```

`generated` records how the current content was produced; `generated.by` is
required within that mapping and `generated.at` records the last meaningful
change. `verified` records who or what confirmed the content and may be either a
list or one bare `{ by, at }` mapping. Trust tiers are derived as:

- no `verified`: unverified;
- only non-`human:` verifiers: machine-confirmed;
- at least one `human:` verifier: human-reviewed.

Trust tiers are advisory signals, not access control. `status` is optional and
uses `draft`, `stable`, or `deprecated` as the defined lifecycle values; absent
status means stable. `stale_after` is an optional absolute `YYYY-MM-DD` date and
is stale when the current date is on or after it.

### Attested Computations

`type: Attested Computation` is an optional concept type. For that type,
`runtime` is required and says how parameters, executor, and attester are
interpreted. Optional contract fields include:

- `parameters`: typed named bindings with `name`, `type`, and `required`;
- `computation`: a path to a separate computation file instead of an inline
  body fence;
- `executor`: a resource plus a receipt shape;
- `attester`: deterministic code that checks the receipt.

The computation may be a single fenced block under `# Computation` or a file
named by `computation`. OKF records the contract and checking interface; it does
not execute code, define an executor package format, define an attester ABI, or
store runtime receipts in the bundle. `verified` and attestation are distinct:
the former confirms a definition, while the latter checks one execution.

### Extensions and references

Producers may include additional frontmatter keys. Consumers should preserve
unknown keys during round-tripping and must not reject them. A `references/`
subdirectory is a convention for mirrored external material, run instructions,
or code; it is not required. Producers may use other directories and may omit
all index files.

## Directory and navigation semantics

Directory nesting organizes concepts into groups and is independent of domain.
The hierarchy itself supplies an implicit parent/child navigation signal, while
ordinary Markdown links add directed relationships. A link's exact relationship
kind is conveyed by its surrounding prose; OKF does not define a typed edge
syntax. Consumers must tolerate a broken link because the target may be written
later.

`index.md` is for progressive disclosure at any directory level. `log.md` is a
chronological history for the corresponding scope. Neither is required for a
bundle to conform.

## Explicit non-goals

The official specification does **not**:

- define a fixed taxonomy of concept types;
- prescribe storage, serving, search, or query infrastructure;
- replace OpenAPI, Protobuf, Avro, database schemas, or other domain schemas;
- define how an executor or attester is packaged or invoked;
- define a full attestation wire protocol, ABI, cache, or runtime lifecycle;
- require a schema registry, central authority, or mandatory toolchain.

Google's reference agent, sample bundles, and visualizer may explain usage, but
they are not additional ZinharCMS conformance requirements.

## ZinharCMS-specific decisions still open

The following must be decided from repository evidence and, where necessary,
owner input in later phases:

1. What is the future bundle boundary: all of `okf/`, selected concepts, or a
   curated subset linked to existing `docs/` and source files?
2. How should each legacy document map to a descriptive `type` without creating
   a premature fixed taxonomy?
3. Which `okf_document_id` values remain useful as extension metadata, and how
   are they reconciled with file-path concept identity?
4. How should `primary_sources`, `related_documents`, source-register entries,
   uncertainty markers, verification commits, and historical phase dates map to
   `sources`, links, `generated`, `verified`, `status`, and `stale_after`?
5. Which `.mmd` diagrams remain project artifacts, which are linked through
   `references/`, and whether any need a separate concept describing them?
6. Should the future bundle include generated `index.md` files, `log.md`, or
   neither? If an index is added, what progressive-disclosure groups are useful?
7. Is any attested computation useful for ZinharCMS, or would adding one create
   unsupported runtime/attester obligations?
8. Which production, ownership, retention, deployment, Marketplace, API, and
   terminology questions in `okf-bootstrap/12-owner-questions.md` need owner
   decisions rather than repository-derived documentation?

These are migration decisions, not v0.2 requirements. Phase 1 must answer them
only to the extent supported by evidence and must preserve `UNKNOWN` and
`NEEDS_OWNER_CONFIRMATION` markers otherwise.

## Versioning and conformance checklist for later phases

- [ ] Every retained non-reserved `.md` concept has parseable YAML frontmatter.
- [ ] Every retained non-reserved `.md` concept has a non-empty `type`.
- [ ] Any `index.md` and `log.md` files are reserved and structurally valid.
- [ ] Root `index.md`, if present, is the only place an `okf_version` declaration
      is used in index frontmatter.
- [ ] Unknown extension keys and broken links do not cause a consumer-rejection
      policy to be introduced.
- [ ] Source attribution, verification, lifecycle, freshness, and computation
      semantics are not conflated during conversion.
- [ ] No project taxonomy or validator rule is described as official unless it
      is stated in the Google specification.
