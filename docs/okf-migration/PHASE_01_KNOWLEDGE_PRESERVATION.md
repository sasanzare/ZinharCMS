# ZinharCMS Google OKF Migration

## Phase 1 — Legacy Knowledge Preservation Map and Verified Migration Inventory

Status: COMPLETE for the Phase 1 preservation boundary. No legacy content was
deleted, renamed, rewritten, converted, or replaced.

## Objective

Build a complete, evidence-backed map of the legacy knowledge corpus before any
future Google OKF conversion or deletion. The result identifies current
knowledge, historical evidence, stale or contradicted claims, consolidation
candidates, diagram treatment, metadata/provenance translation, concept
candidates, and owner decision debt.

The target remains the official Google Open Knowledge Format v0.2. The Phase 1
artifacts are planning and evidence records only; they are not the replacement
bundle.

## Scope and boundary

Audited:

- all 339 Markdown files under legacy okf/;
- all 14 Markdown files under okf-bootstrap/;
- all 50 Mermaid source files under okf/;
- okf/index.yaml and other material files found in the two legacy trees;
- relevant current source, migrations, tests, configuration, CI, Docker, README,
  Phase 0 reports, and handoff evidence.

The phase changed only migration documentation under docs/okf-migration/ and
the Phase 1 checkpoint in HANDOFF.md. The two pre-existing modified legacy
files, okf/backend/configuration-and-state.md and okf/security/overview.md,
remain untouched by Phase 1.

## Methodology

The work used staged evidence reduction:

1. Re-established the Git baseline instead of assuming the Phase 0 state was
   unchanged.
2. Enumerated paths deterministically and calculated a path-plus-byte SHA-256
   corpus hash.
3. Parsed frontmatter keys, source lists, headings, links, sizes, statuses,
   uncertainty markers, and diagram references.
4. Counted domains and categories, identified structural/index documents, and
   reviewed every bootstrap document as its own row.
5. Compared full-file hashes for exact duplicates. There are no exact duplicate
   Markdown files.
6. Used token/trigram similarity as candidate evidence only, then formed
   semantic consolidation groups from the repeated API, architecture,
   backend, database, security, domain, frontend, extensibility, operations,
   delivery, maintenance, provenance, and bootstrap views.
7. Deep-verified high-value claims against current source, migrations, tests,
   configuration, CI, and Docker. Legacy documents were never used as the sole
   authority for current implementation claims.
8. Recorded material claims in the [Claim Verification
   Ledger](CLAIM_VERIFICATION_LEDGER.md), per-document outcomes in the
   [Disposition Matrix](LEGACY_DOCUMENT_DISPOSITION_MATRIX.md), and every
   diagram in the [Diagram Preservation Map](DIAGRAM_PRESERVATION_MAP.md).
9. Preserved unresolved questions and unknowns in the [Owner Decision
   Register](OWNER_DECISION_REGISTER.md).

Temporary inspection code was run locally and was not added to the repository.

## Corpus statistics

| Corpus item | Count | Notes |
| --- | ---: | --- |
| Legacy Markdown under okf/ | 339 | All have custom frontmatter; none has required Google type |
| Bootstrap Markdown | 14 | Plain Markdown with no frontmatter |
| Legacy Mermaid files | 50 | All under okf; separately classified |
| Legacy index files | 1 | okf/index.yaml, 411,888 bytes, 8,527 lines |
| Other material files in the two trees | 0 | No additional extensions discovered |
| Total material files audited | 404 | 353 Markdown, 50 Mermaid, 1 YAML |
| Total audited bytes | 2,652,948 | Deterministic path-plus-byte corpus hash recorded in the manifest |

The legacy frontmatter inventory contains 53 distinct keys. The most common
fields are category, project, title, phase, status, okf_document_id,
last_verified_commit, last_verified_date, primary_sources, related_documents,
and the uncertainty/implementation fields. The detailed semantics are in
[Legacy Metadata Translation](LEGACY_METADATA_TRANSLATION.md).

## Domain breakdown

| Domain/root | Markdown documents |
| --- | ---: |
| API | 57 |
| domain | 44 |
| security | 40 |
| extensibility | 39 |
| database | 34 |
| backend | 31 |
| frontend | 31 |
| operations | 15 |
| delivery | 11 |
| development | 11 |
| architecture | 10 |
| maintenance | 10 |
| project | 4 |
| references | 1 |
| root okf README | 1 |
| okf-bootstrap | 14 |
| **Total Markdown** | **353** |

The category values are a legacy taxonomy, not a Google type registry. The
future candidate map therefore uses current capability boundaries rather than
preserving the directory tree one-for-one.

## Verification strategy and major findings

### Architecture and runtime

Current evidence supports a React/Vite administration SPA, one Rust/Axum/Tokio
backend process, PostgreSQL, Redis, local configured files, built-in in-process
plugins, and configured external integrations. The repository does not prove
independently deployed domain services, a durable worker, a message broker,
object storage adapter, search service, external identity provider, API
gateway, production metrics collector, or production ingress.

The route source separates public, authenticated, and tenant-protected
subtrees. Preview is a separate public WebSocket upgrade path with ticket and
current authorization checks. This is a material security boundary and should
survive as a dedicated future concept.

### Database, tenancy, and RLS

The migration set and current services show a shared PostgreSQL database with
tenant session context, RLS functions/policies, explicit organization
relationships, tenant-aware connections, and tests for important isolation
contracts. Source and repository tests do not prove the applied schema,
privileges, or cross-tenant behavior in every deployed environment. The
future Database and TenantIsolation concepts must retain that distinction.

### Authentication, authorization, and security

Current source supports bearer access tokens, cookie-bound opaque refresh
families, MFA, Step-Up, organization membership, RBAC, ownership checks, audit
paths, preview tickets, outbound restrictions, and the current file-storage
hardening worktree. Two legacy negative statements are not current truth:
the old permissions text says MFA/session flows were not found, and an HTTP
security document says logout requires a bearer token. Current auth routes show
cookie/origin logout outside the bearer middleware.

Security material is high value but review-sensitive because the worktree
contains pre-existing Phase 7 modifications and because deployment-level
controls remain unverified.

### API and frontend

The route tree is the authority for reachability. The legacy endpoint, group,
backend-module, and frontend-contract views are useful but overlap heavily and
contain source-sensitive counts/coverage claims. Future API concepts should be
regenerated from routes, handlers, DTOs, OpenAPI annotations, tests, and the
central frontend client. The frontend store and API client provide current
authentication/organization integration evidence, but do not establish a
browser support or accessibility policy.

### Marketplace and domain

Marketplace current knowledge is substantial: creators, listings, versions,
submissions, review/moderation, abuse reports, installations, runtime
permissions and kill switches, finance, payouts, entitlements, and analytics
are visible in routes, services, migrations, and tests. The repository does not
settle future external package execution, appeals, cleanup automation, tax,
disputes, partial refunds, or provider settlement guarantees. Those remain
decision debt.

### Operations and delivery

Compose and Dockerfiles are useful reference topology and CI shows quality
gates. Neither is proof of production deployment, ingress, recovery,
observability, backup, storage durability, release authority, or rollback
ownership. Operations and delivery documents are therefore mostly regeneration
candidates with historical risk knowledge preserved.

## Duplication and consolidation findings

Full-file SHA-256 comparison found zero exact duplicate Markdown documents.
Similarity scanning identified candidate overlaps, but similarity was not
treated as proof of duplicate semantics. The stable semantic groups below
prevent a blind 339-doc-to-339-concept conversion:

| Group | Scope | Documents | Treatment |
| --- | --- | ---: | --- |
| MERGE-GROUP-001 | Project overview, README, glossary, navigation, repository map | 5 | Merge identity and navigation; regenerate repository map |
| MERGE-GROUP-002 | Architecture overview, boundaries, components, decisions, diagrams | 10 | Consolidate architecture narrative and retain decision history |
| MERGE-GROUP-003 | Backend overview, catalogs, modules, persistence, services, tests | 31 | Merge module views around current backend boundaries |
| MERGE-GROUP-004 | API overview, groups, endpoint families, route catalogs, contracts | 57 | Regenerate exhaustive surfaces; merge family-level concepts |
| MERGE-GROUP-005 | Database guides, entities, schema, relationships, RLS | 34 | Regenerate schema/entity inventories; merge ownership views |
| MERGE-GROUP-006 | Security, roles, permissions, authentication, RBAC, tenancy | 40 | Consolidate around Authentication, Authorization, and Security |
| MERGE-GROUP-007 | Domain, workflows, rules, entities, tenancy, billing, delivery | 44 | Merge overlapping domain views by verified capability |
| MERGE-GROUP-008 | Frontend architecture, features, state, API, testing | 31 | Merge feature summaries with current frontend architecture |
| MERGE-GROUP-009 | Plugins, extension points, Marketplace extensibility | 39 | Separate built-in plugins from Marketplace runtime boundaries |
| MERGE-GROUP-010 | Operations, Docker/deployment, CI, release, recovery | 26 | Regenerate from current configuration and owner policy |
| MERGE-GROUP-011 | Development, maintenance, governance, testing workflows | 21 | Merge procedures and preserve governance debt |
| MERGE-GROUP-012 | Bootstrap audit/planning series | 14 | Preserve as historical evidence and decision lineage |
| MERGE-GROUP-013 | Source register and provenance lineage | 1 | Translate useful source semantics to Google sources |

The largest consolidation pressure is the API group: endpoint-family,
route-group, group summary, backend module, frontend wrapper, and diagram views
often describe the same boundary from different perspectives. The same
pattern appears in database entities versus schema catalogs and in security
roles/permissions versus RBAC/tenant documents.

## Preservation and disposition findings

The complete row-level results are in the [Disposition
Matrix](LEGACY_DOCUMENT_DISPOSITION_MATRIX.md). Primary outcomes are:

| Primary disposition | Count |
| --- | ---: |
| MIGRATE | 168 |
| MERGE | 117 |
| PRESERVE_HISTORICAL | 19 |
| REGENERATE | 35 |
| DELETE_LEGACY_ONLY | 14 |
| **Document rows** | **353** |

DELETE_LEGACY_ONLY is limited to navigation/directory summaries whose useful
links or claims must be absorbed first. It is a future candidate only. The
review overlay marks 75 documents for final source/owner review; it does not
change the primary disposition and is not permission to delete anything.

All 14 bootstrap documents are represented individually. They are preserved
historically because they contain snapshot provenance, gap registers, Mermaid
and API audits, proposed structure, implementation sequencing, and owner
questions that should not be lost merely because they are not current
implementation facts.

## Metadata and provenance findings

The legacy model combines identifiers, taxonomy, review labels, source paths,
commit/date snapshots, diagrams, uncertainty codes, implementation stance,
role/module/entity catalogs, and phase history. The future treatment is:

- map title, descriptive category/type input, source paths, and applicable
  lifecycle semantics to Google OKF fields only after review;
- use sources with stable IDs and path/commit context for lineage;
- use generated for deterministic catalogs and verified only with an identified
  actor and timestamp;
- derive status/freshness from current evidence and policy, not from legacy
  current labels or guessed intervals;
- keep uncertainty, confidence, local role/module IDs, and implementation
  stance as extension/body candidates;
- move project/phase/navigation relationships into body content and normal
  Markdown links;
- drop the legacy document ID as a schema field and derive the final concept ID
  from the approved Google bundle path;
- do not invent Attested Computation concepts merely because Phase 1 has a
  deterministic evidence manifest.

Details and field counts are in
[LEGACY_METADATA_TRANSLATION.md](LEGACY_METADATA_TRANSLATION.md).

## Diagram findings

Every Mermaid file is individually mapped in
[DIAGRAM_PRESERVATION_MAP.md](DIAGRAM_PRESERVATION_MAP.md):

| Future diagram disposition | Count |
| --- | ---: |
| PRESERVE | 19 |
| MERGE | 19 |
| REGENERATE | 11 |
| DROP | 1 |
| **Total** | **50** |

The unique preservation candidates are mostly tenant/RLS, security trust and
authorization, entity relationship, lifecycle, Marketplace installation,
page-builder, and system-boundary views. The largest diagram risks are
source-sensitive request flows, unverified deployment/recovery/observability
claims, and diagrams that present the same relationship at API/backend/domain
layers. No diagram was modified or rendered during Phase 1.

## Owner questions and decision debt

The [Owner Decision Register](OWNER_DECISION_REGISTER.md) preserves 18 NOC
records and 15 UNKNOWN labels from the bootstrap gap report. Seventeen NOC
decisions remain open; NOC-16 is resolved by the root GPL-3.0-only LICENSE and
is retained as historical evidence.

Highest-priority open decisions are public tenant/custom-domain routing
(NOC-01), production storage and asset authorization (NOC-02), backup/recovery
(NOC-03), observability and on-call (NOC-04), legal/privacy retention
(NOC-05), deployment/promotion/rollback (NOC-06), side-effect guarantees
(NOC-09), Marketplace scope/finance/runtime boundaries (NOC-10), and owner
assignment (NOC-15).

## Migration blockers and evidence gaps

Phase 2 can design a target concept hierarchy, but final stable concepts remain
blocked by:

1. absent production topology, ingress, storage, recovery, observability, and
   release evidence;
2. unknown public tenant/custom-domain routing intent;
3. unknown retention/privacy/legal requirements;
4. unresolved API compatibility/versioning policy;
5. unresolved Marketplace settlement, appeal, cleanup, and external execution
   scope;
6. missing module/document/support ownership;
7. legacy claims whose dates and negative statements are stale or contradicted;
8. the need to keep the dirty security Phase 7 worktree distinct from
   migration-generated changes.

These are documented blockers, not reasons to lower the preservation
acceptance criteria.

## Phase 1 conclusion

The complete Markdown corpus and diagram corpus are enumerated and classified.
Current high-value claims were verified against current repository evidence,
historical and stale claims were separated, duplicates were turned into stable
consolidation groups, provenance semantics were translated conceptually, and
all owner questions were preserved. The legacy tree is safe to leave in place
while the replacement design is prepared.

## Phase 2 readiness

The repository is ready for:

- designing a final Google OKF v0.2 concept hierarchy and bundle boundary;
- selecting a small set of current concepts from the candidate map;
- defining naming, type, source, verification, lifecycle, freshness, extension,
  and navigation conventions;
- planning generated API/database/provenance inventories from current evidence;
- mapping historical and decision-debt concepts without promoting them to
  current truth.

Phase 2 must use the verified preservation map, resolve or explicitly carry the
owner decisions, and continue to avoid deleting or converting legacy content
until the target design is reviewed.

## Quantitative completion summary

| Metric | Count |
| --- | ---: |
| Legacy Markdown documents | 339 |
| Legacy bootstrap documents | 14 |
| Legacy diagrams | 50 |
| Legacy index files | 1 |
| MIGRATE | 168 |
| MERGE | 117 |
| PRESERVE_HISTORICAL | 19 |
| REGENERATE | 35 |
| DELETE_LEGACY_ONLY | 14 |
| REVIEW_REQUIRED overlay | 75 |
| Merge groups | 13 |
| Claims VERIFIED_CURRENT | 20 |
| Claims VERIFIED_HISTORICAL | 6 |
| Claims PARTIALLY_VERIFIED | 12 |
| Claims STALE | 7 |
| Claims CONTRADICTED | 2 |
| Claims UNVERIFIED | 8 |
| Claims NON_FACTUAL | 5 |
| Diagrams PRESERVE | 19 |
| Diagrams REGENERATE | 11 |
| Diagrams MERGE | 19 |
| Diagrams DROP | 1 |
| Open owner decisions | 17 |
