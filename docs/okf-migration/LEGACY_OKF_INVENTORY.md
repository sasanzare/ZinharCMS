# Legacy OKF Inventory

This inventory describes the existing ZinharCMS knowledge system as found in
the Phase 0 worktree. It does not convert, delete, or assign final migration
disposition to any legacy content. The companion report is
[`PHASE_00_BASELINE_AUDIT.md`](PHASE_00_BASELINE_AUDIT.md).

## Inventory totals

| Item | Count/evidence | Observation |
| --- | ---: | --- |
| Tracked `okf/` files | 390 | 339 Markdown, 50 Mermaid, 1 YAML index |
| Concept-like Markdown | 339 | Every file starts with a custom YAML frontmatter block |
| `index.md` files | 0 | No Google reserved directory listings |
| `log.md` files | 0 | No Google reserved update logs |
| `okf/index.yaml` | 1 | 411,888 bytes, 8,527 lines, root `okf_version: "0.1"` |
| Top-level knowledge areas | 14 | API, architecture, backend, database, delivery, development, domain, extensibility, frontend, maintenance, operations, project, references, security |
| Tracked `okf-bootstrap/` reports | 14 | Plain Markdown analysis/planning reports with no frontmatter |

## `okf/` path inventory

| Legacy path | Files | Role/purpose | Unique knowledge appears? | Migration concern |
| --- | ---: | --- | --- | --- |
| `okf/README.md` | 1 | Human entry point, source-priority rules, uncertainty vocabulary, navigation, and maintenance expectations | Yes: authority model and navigation | Custom frontmatter and prose describe a project-specific OKF convention; README is not a Google `index.md` |
| `okf/index.yaml` | 1 | Machine-readable registry of documents, phases, categories, evidence paths, related documents, markers, current/planned sections, and maintenance status | Yes: central registry and cross-document map | Nonstandard central dependency; declares `0.1`, uses custom fields, and must be preserved before replacement |
| `okf/api/` | 62 | API overview, route groups, endpoint families, request/response/error contracts, auth, tenant context, tests, risks, and diagrams | Yes | Document IDs, source paths, endpoint groupings, and OpenAPI caveats have no direct Google field mapping |
| `okf/architecture/` | 15 | System boundaries, components, dependency direction, runtime flows, integrations, decisions, risks, and diagrams | Yes | Architecture observations mix implementation evidence with inferred boundaries and owner questions |
| `okf/backend/` | 35 | Backend overview, module catalog, module records, services/domain, persistence, configuration, errors, tests, risks, and diagrams | Yes | Module documents are aggregates rather than a Google-defined type; source freshness is historical |
| `okf/database/` | 39 | Schema/entity catalogs, migrations, relationships, constraints/indexes, RLS/tenancy, lifecycle, persistence, fixtures, tests, risks, and diagrams | Yes | Migration-defined schema and runtime-state unknowns must remain distinct during preservation |
| `okf/delivery/` | 14 | CI, artifacts, containers, deployment, environment promotion, database deployment, release, rollback, and risks | Yes | Reference Docker/CI behavior must not be converted into production claims |
| `okf/development/` | 11 | Prerequisites, local environment, commands, testing, debugging, contribution, database development, build/quality, and risks | Yes | Commands and test claims need verification against current manifests and dirty worktree |
| `okf/domain/` | 50 | Domain catalog, rules, invariants, state transitions, lifecycle, tenancy, business workflows, cross-module flows, risks, and diagrams | Yes | Rule meaning and product intent are often separated from executable enforcement; preserve uncertainty markers |
| `okf/extensibility/` | 46 | Built-in plugins, Marketplace, extension points, manifests, permissions, adapters, lifecycle, compatibility, testing, risks, and diagrams | Yes | Built-in plugins and Marketplace host-owned adapters must not be collapsed into one taxonomy |
| `okf/frontend/` | 36 | Frontend overview/catalogs, routes, features, state, API client, builder, forms, localization, styling, tests, risks, and diagrams | Yes | Frontend documents manually duplicate backend/API contracts and may drift from current security work |
| `okf/maintenance/` | 10 | Documentation ownership, update policy, review checklist, staleness detection, validation report, conflict/owner registers, and completion report | Yes: legacy governance and validator assumptions | These policies govern the old schema; deleting them without a replacement loses process knowledge |
| `okf/operations/` | 19 | Runtime topology, configuration, health, logging, metrics, alerts, backup/recovery, external dependencies, troubleshooting, runbooks, and risks | Yes | Local/reference operational evidence is not deployed-environment evidence |
| `okf/project/` | 4 | Project overview, repository map, glossary, and navigation guide | Yes | Glossary and source authority are valuable but use custom fields and legacy links |
| `okf/references/` | 1 | Source register for evidence used by the legacy documents | Yes | Critical provenance candidate; no direct `sources` list equivalent exists today |
| `okf/security/` | 46 | Authentication, sessions, roles, permissions, authorization, tenant access, secrets, browser/HTTP controls, threats, risks, tests, and diagrams | Yes | Security claims have historical verification commits and two files were already dirty before Phase 0 |

The directory counts are from `git ls-files okf/**` and include nested Markdown
and Mermaid files. They are not a proposed future bundle structure.

## Legacy Markdown/frontmatter convention

Every one of the 339 Markdown files begins with `---`, has a closing `---`, and
contains YAML-like metadata. The common fields are:

| Field | Observed coverage | Legacy meaning |
| --- | ---: | --- |
| `okf_document_id` | 339/339 | Arbitrary stable document identifier |
| `title` | 339/339 | Display title |
| `project` | 339/339 | Project name, normally `ZinharCMS` |
| `category` | 339/339 | Project-defined document category/taxonomy |
| `phase` | 339/339 | Legacy documentation phase 1 through 10 |
| `status` | 339/339 | Usually `current` |
| `source_of_truth` | 339/339 | Boolean authority indicator; all observed values are `false` |
| `last_verified_commit` | 339/339 | Git commit used for the legacy evidence snapshot |
| `last_verified_date` | 339/339 | Legacy verification date |
| `primary_sources` | 339/339 | Repository-relative or legacy-relative evidence paths |
| `related_documents` | 339/339 | Cross-document navigation/relationship paths |
| `implementation_view` | 323/339 | Values such as `observed`, `prescriptive`, or `mixed` |
| `related_diagrams` | 298/339 | Links to Mermaid artifacts |
| `uncertainty_markers` | 169/339 | Explicit unknown, conflict, inferred, planned, or unclear IDs |
| Phase-specific fields | 18 key families | Entity, module, feature, domain, role, permission, workflow, security, and extensibility identifiers/statuses |

The field scan found no top-level `type`, `description`, `resource`, `tags`,
`sources`, `generated`, `verified`, `stale_after`, `runtime`, `parameters`,
`computation`, `executor`, or `attester` fields in these Markdown documents.
This is the central structural gap against Google OKF v0.2. It is not evidence
that the corresponding facts are absent from the bodies or evidence paths.

### Identity and naming

Legacy identity is two-layered:

1. `okf_document_id` in each Markdown file;
2. an index entry ID/path in `okf/index.yaml`.

Google OKF instead defines a concept ID as the file path with `.md` removed.
The relationship among filename, `okf_document_id`, index ID, and body title must
be checked document by document. It must not be assumed to be one-to-one merely
because the legacy validator reported unique IDs.

Filenames are mostly lower-kebab-case, with `README.md` used as an entry point
in 14 directories. The 50 Mermaid files use lower-kebab-case names and are
listed in legacy metadata/index records.

### Evidence, provenance, and uncertainty

Legacy documents record evidence using `primary_sources`, `related_documents`,
`related_diagrams`, a central `references/source-register.md`, and explicit
marker IDs such as `UNKNOWN`, `NEEDS_OWNER_CONFIRMATION`,
`DOCUMENTATION_CODE_CONFLICT`, `PLANNED_NOT_IMPLEMENTED`, and
`IMPLEMENTATION_STATUS_UNCLEAR`. The body often separates observed behavior,
inference, implementation status, risks, and unresolved owner questions.

This is valuable provenance and uncertainty knowledge. It is not the Google
`sources` model: there are no observed `sources[].resource` entries, source IDs
used for Markdown footnote attribution, `generated` actors, or `verified` event
lists in the legacy frontmatter scan.

### Lifecycle and freshness

The common lifecycle value is `status: "current"`; additional implementation
and review fields distinguish `observed`, `prescriptive`, `mixed`, `verified`,
`implemented`, `partially implemented`, and unresolved states. The freshness
model is a verification commit/date, not the Google separation of
`generated.at`, `verified`, `status`, and optional `stale_after`.

Many legacy verification commits predate the current branch HEAD. For example,
`okf/README.md` records `131c4f30583affc7a07dbcabaaa45b42c490dc27`, while the
current branch HEAD is `518f74a1b0da5c4ee37c14e2a37a716707468410`. The dirty
worktree also contains current security changes. These facts create a freshness
and drift risk, not an automatic invalidation of every claim.

## `okf/index.yaml` inventory

The root YAML file is the legacy system's navigation and validation anchor. Its
observed structure includes:

| Section/field family | Observed purpose |
| --- | --- |
| `okf_version: "0.1"` | Legacy format version declaration |
| `project` | Repository URL/name, knowledge status, current phase, verification snapshot |
| `authority` | Source-of-truth priority and path conventions |
| `entry_points` | Human and AI navigation lists |
| `current_sections` / `planned_sections` | Phase/coverage state |
| `documents` | IDs, paths, titles, categories, phases, status/review, evidence paths, related documents, and marker IDs |
| marker registers | Unknown, owner-confirmation, conflict, planned, inferred, unclear, overlap, and implementation-status categories |

The legacy validation report at
`okf/maintenance/validation-report.md` records 390 index entries including
`index.yaml` itself and reports custom checks for parseability, ID uniqueness,
file/index parity, metadata consistency, links, evidence paths, Mermaid static
structure, terminology, secrets, and Git scope. The report also explicitly says
that the validation logic is not a tracked repository script or CI job.

## `okf-bootstrap/` inventory

| Path | Role/purpose | Unique knowledge appears? | Migration concern |
| --- | --- | --- | --- |
| `00-repository-inventory.md` | Baseline tracked tree, generated boundaries, root files, and repository observations | Yes | Contains historical counts and an old branch/HEAD; use as historical evidence only |
| `01-technology-inventory.md` | Technology and runtime profile | Yes | Verify against current manifests/configuration |
| `02-architecture-observations.md` | Runtime architecture and system-boundary observations | Yes | Separates observed/inferred/unknown claims; preserve markers |
| `03-module-inventory.md` | 22-module capability inventory | Yes | Module grouping is project-specific, not Google taxonomy |
| `04-documentation-audit.md` | Existing docs, freshness, conflicts, duplication, and preservation notes | Yes | Valuable source-selection reasoning; contains historical status claims |
| `05-mermaid-audit.md` | Diagram inventory and parser/render limitations | Yes | Do not treat static checks as parser/render conformance |
| `06-database-inventory.md` | Migration/schema/RLS/entity/index/constraint inventory | Yes | Runtime schema and production state remain unknown |
| `07-api-inventory.md` | Route, OpenAPI, client, error, and contract inventory | Yes | Route and annotation counts are historical snapshots |
| `08-conventions-inventory.md` | Naming, code, documentation, testing, branch, and validation conventions | Yes | Distinguishes inferred conventions from enforced rules |
| `09-knowledge-gaps.md` | UNKNOWN, owner-question, conflict, and domain gap registers | Yes | Must be carried forward rather than flattened into status fields |
| `10-proposed-okf-structure.md` | Proposed legacy directory and index design | Yes, as design history | It is not the Google v0.2 specification and must not be treated as normative |
| `11-implementation-phases.md` | Legacy OKF phase sequence and definitions of done | Yes | Historical plan conflicts with the new Google migration boundary |
| `12-owner-questions.md` | Questions requiring owner/policy/platform evidence | Yes | Critical unresolved decisions for later knowledge curation |
| `phase-zero-summary.md` | Summary, counts, conflicts, gaps, and legacy next-action | Yes | Its statement that no final `okf/` existed is now historical |

No `okf-bootstrap` file is referenced by executable source, manifests, scripts,
Docker, or CI. It is documentation/history coupling only.

## Validation assumptions embedded in the legacy system

The legacy maintenance documents define a process that assumes:

- frontmatter schemas vary by legacy phase and category;
- `okf_document_id` and index IDs are globally unique;
- every file has exactly one index entry;
- evidence paths and custom relation paths resolve under bespoke rules;
- statuses and uncertainty markers are validated against project vocabularies;
- Mermaid files are statically inspected and linked;
- secret scans and Git scope checks are part of review;
- `last_verified_commit` and `last_verified_date` are the staleness signal.

These assumptions are project governance, not Google OKF requirements. In
particular, Google consumers must tolerate missing optional fields, unknown
extensions, unknown types, broken links, and missing indexes.

## Important references/dependencies

| Reference | Evidence | Role |
| --- | --- | --- |
| `README.md` | Repository layout and documentation links | Human navigation into legacy OKF |
| `HANDOFF.md` | Historical OKF Phase Zero/One overrides and path references | Operational continuation/history; contains stale-looking superseded instructions |
| `okf/README.md` | Source priority, navigation, maintenance rules | Legacy entry point and interpretation layer |
| `okf/index.yaml` | Registry and marker graph | Legacy machine-readable index |
| `okf/references/source-register.md` | Source mapping | Legacy provenance register |
| `okf/maintenance/*.md` | Update, staleness, checklist, validation, conflicts, owner questions | Legacy workflow and validator assumptions |
| `okf-bootstrap/*.md` | Historical audit/planning reports | Bootstrap reasoning and preservation candidates |
| `docs/security/*.md` | Selected phase documents cite legacy OKF paths | Documentation-only cross-reference |

## Preliminary observation

The legacy tree contains substantial project knowledge and no evidence of
runtime execution. It should remain intact until Phase 1 produces a per-document
preservation map. This is a preliminary structural observation, not a final
deletion or conversion disposition.
