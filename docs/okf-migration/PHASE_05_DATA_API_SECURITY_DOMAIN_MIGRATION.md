# Phase 5 — Data, API, Security & Domain Knowledge Migration

**Status:** COMPLETE for the isolated Phase 5 staging boundary
**Date:** 2026-08-07
**Source HEAD:** `6f51612c5716c7d9c3365553811053fd24a03947`
**Staging root:** `docs/okf-migration/staging/google-okf-v0.2/`
**Phase 4 baseline:** `docs/okf-migration/PHASE_04_BUILD_MANIFEST.json`

## Executive result

Phase 5 constructed eight evidence-backed Concepts across Data, API, Security,
and Domain boundaries:

* `TARGET-021` — Public Delivery and Webhooks Contract
* `TARGET-022` — Marketplace and Extension Contracts
* `TARGET-025` — Tenant Data Policy
* `TARGET-031` — Storage and File Security
* `TARGET-032` — Security Posture and Risks
* `TARGET-036` — Public Delivery
* `TARGET-037` — Billing and Quotas
* `TARGET-040` — Marketplace Runtime and Safety Boundary

`TARGET-047` was evaluated but remains `OUT_OF_SCOPE` for Phase 5 because
documentation maintenance belongs to the Phase 6 development/governance
boundary. All other non-selected targets retain their prior Phase 3/4,
regeneration, owner-decision, or historical status. No unresolved owner policy
was promoted to an implementation fact.

The staging bundle now contains 30 Concepts, 13 indexes, one root log, 17
embedded Mermaid blocks, zero standalone `.mmd` files, and 44 files. The
legacy `/okf/` tree, `/okf-bootstrap/`, and `okf/index.yaml` remain untouched.
No canonical cutover, application change, migration change, dependency change,
CI change, runtime change, deletion, commit, or push was performed.

## Scope and eligibility

The Phase 5 boundary covered only source-derived data/database, API,
authentication/authorization, security, tenancy, public delivery, billing and
quota, Marketplace, and extension knowledge. The nine candidates were checked
against the Phase 4 matrix, Phase 1 mapping, current source at the immutable
Phase 5 HEAD, and the owner-decision register. `PHASE5_ELIGIBLE` was applied
only where current implementation mechanics could be stated without deciding
deployment, legal, compatibility, settlement, or ownership policy.

The official Google OKF v0.2 reference remains the [official OKF specification](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md).
The repository's stricter local metadata, type, source, link, and navigation
rules remain documented separately and were applied to each new staging
Concept.

## Git baseline

| Item | Phase 5 starting state |
| --- | --- |
| Repository | `D:\All projects\ZinharCMS` |
| Branch | `security/security-audit-fixes` |
| Starting HEAD | `6f51612c5716c7d9c3365553811053fd24a03947` |
| Origin comparison | `origin/security/security-audit-fixes` matched starting HEAD |
| Working tree | Clean before Phase 5 construction |
| Staged paths | None |
| Recent Phase 4 commit | `6f51612c docs(okf): complete Google OKF v0.2 Phase 4 migration` |
| Phase 5 Git policy | Documentation/staging changes only; no stage, commit, push, branch, reset, clean, stash, or history rewrite |

After construction, the only expected dirty paths are the Phase 5 reports,
status/ledger/verification/manifest files, the eight new staging Concepts, four
domain-index updates, the staging log update, and `HANDOFF.md`.

## Phase 4 baseline verification

The Phase 4 manifest was rechecked as the immutable pre-Phase-5 staging
baseline. It records 22 Concepts, 13 indexes, one log, 13 embedded Mermaid
blocks, zero standalone Mermaid files, and 36 post-Phase-4 staging files. Its
full file-hash snapshot was the comparison source for the Phase 5 delta. The
Phase 4 target matrix had 54 unique target rows, with nine `DEFERRED_MERGE`
targets corresponding to the nine Phase 5 candidates.

The carried Phase 4 source HEAD was not reused as provenance for new Concepts.
Every new Concept source entry is pinned to the Phase 5 source HEAD above.
Existing Phase 3/4 Concepts were not rewritten merely to refresh their older
provenance; their carried history is recorded as pre-existing bundle state.

## Phase 5 target selection

| ID | Staging path / type | Domain | Merge group(s) | Legacy count | Claims | Evidence packet | Diagram result | NOCs | UNKNOWNs | Eligibility |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TARGET-021 | `api/public-delivery-and-webhooks-contract.md` / API Contract | API | MERGE-GROUP-004, 007 | PublicDelivery 2 primary rows; shared API/webhook rows | CLAIM-0019, CLAIM-0033 | `routes/mod.rs`, `delivery.rs`, `webhooks.rs`, cache, webhook, tenant/RLS services | No new visual; public-delivery flow is owned by TARGET-036 | NOC-01, 08, 09, 11 | host/custom-domain selection; compatibility/deprecation; retries/idempotency | PHASE5_ELIGIBLE |
| TARGET-022 | `api/marketplace-and-extension-contracts.md` / API Contract | API | MERGE-GROUP-004, 009 | Marketplace 15; Extensibility 28; shared API rows | CLAIM-0015, CLAIM-0017 | Marketplace, runtime, adapter, plugin routes and RBAC | No new visual; installation visual remains with Marketplace and runtime visual is in TARGET-040 | NOC-08, 10 | compatibility; external execution; settlement and disputes | PHASE5_ELIGIBLE |
| TARGET-025 | `database/tenant-data-policy.md` / Database Policy | Database | MERGE-GROUP-005, 006, 007 | TenantIsolation 5; shared Database 34 and security rows | CLAIM-0008 | RLS service, migrations 0009/0010/0015/0020/0021/0022/0024/0030, tenant middleware, RLS test source | Added tenant-data policy flow | NOC-01, 03, 05, 11 | live applied state; privacy/retention; public routing | PHASE5_ELIGIBLE |
| TARGET-031 | `security/storage-and-file-security.md` / Security Control | Security | MERGE-GROUP-006, 007, 009 | Media 2; shared SecurityPosture 12 | CLAIM-0018, CLAIM-0037 | file security, media, cleanup, processing, migration 0030, Phase 7 tests | No new visual; storage boundary is textual and linked | NOC-02, 05 | production storage/CDN; malware service; retention | PHASE5_ELIGIBLE |
| TARGET-032 | `security/security-posture-and-risks.md` / Security Control | Security | MERGE-GROUP-006; G010 risk inputs remain deferred | SecurityPosture 12 | CLAIM-0018, CLAIM-0029 | auth/security/step-up middleware, RBAC, security audit/cleanup, migration tests | Added layered-security boundary flow | NOC-04, 05, 15 | monitoring/on-call; security ownership; policy/retention | PHASE5_ELIGIBLE |
| TARGET-036 | `domain/public-delivery.md` / Resource Boundary | Domain | MERGE-GROUP-004, 007 | PublicDelivery 2; shared TenantIsolation 5 | CLAIM-0019 | delivery routes, cache, RLS, route composition | Added published-delivery workflow | NOC-01, 09, 11 | custom domains; routing verification; delivery guarantees | PHASE5_ELIGIBLE |
| TARGET-037 | `domain/billing-and-quotas.md` / Domain Model | Domain | MERGE-GROUP-004, 005, 007 | BillingAndQuotas 2; shared billing/API/database rows | None promoted beyond current source | billing routes, quota service, RBAC, migrations 0010–0012 | No new visual | NOC-09 | provider failure; compensation; refunds/tax; retry policy | PHASE5_ELIGIBLE |
| TARGET-040 | `domain/marketplace-runtime-and-safety-boundary.md` / Marketplace Domain | Domain | MERGE-GROUP-006, 009 | Extensibility 28; Marketplace 15 | CLAIM-0017 | Marketplace manifest/package/validation/runtime services, runtime routes, migrations 0020/0021, safety tests | Added runtime-safety gate; DG-REGEN-04 remains deferred | NOC-10, 17 | external execution; archive retention; settlement/cleanup policy | PHASE5_ELIGIBLE |
| TARGET-047 | `development/documentation-maintenance.md` / Development Guide | Development | MERGE-GROUP-011 | MaintenanceGovernance 8 | CLAIM-0038 | HANDOFF, migration reports, Git state, validation policy | None | NOC-13, 14 | canonical owner; review triggers; enforced maintenance | OUT_OF_SCOPE |

The legacy counts are Phase 1 category counts. Shared group rows are not
double-counted as unique Phase 5 inputs; their target routing is recorded in
the merge ledger.

## Database & RLS migration

Phase 5 documents source-defined database behavior, not a schema migration.
The repository contains SQLx migration files through `0030`; this is a source
chronology fact and not proof of an applied production version. The new tenant
data Concept records the base 0009 helpers and forced policies, the later
billing, Marketplace, and file-cleanup RLS additions, and the application
context mechanism. It deliberately does not replace the future exhaustive
schema/entity targets.

The RLS evidence is layered:

1. `tenant.rs` requires an organization header and active membership.
2. `rls.rs` sets organization/user session settings and disables bypass for
   normal tenant connections and transactions.
3. migrations use forced RLS and `USING`/`WITH CHECK` organization predicates
   on the enumerated tenant tables, with a special system-row policy for the
   component registry.
4. source queries repeat organization predicates in the inspected delivery,
   webhook, media, Marketplace, and quota paths.
5. the repository contains live-test code for cross-tenant visibility and
   mutation behavior, but Phase 5 did not execute a live database test.

No claim that every table, every bypass caller, or every deployed database is
covered identically is made.

## API migration

The two API Concepts are family-level contracts. They record route composition,
middleware placement, request context, response/state boundaries, and the
explicit limits of the served OpenAPI document. Public delivery is separate
from tenant-protected webhook administration. Marketplace routes are separated
by catalog/creator/review, installation, runtime/adapters, finance, analytics,
and built-in plugin families, with their observed global/organization/creator
authorization decisions.

`TARGET-019` remains the exhaustive route/OpenAPI regeneration target. The
Phase 5 Concepts do not invent version numbers, compatibility windows,
deprecation schedules, or an OpenAPI completeness guarantee.

## Authentication & Authorization migration

No existing authentication or authorization Concept was modified. Phase 5
cross-links the current bearer/session validation, MFA/Step-Up, RBAC, and tenant
membership controls from the new API, database, security, and Marketplace
Concepts. Marketplace routes use global admin checks for moderation and
administrator surfaces, organization admin/installer/approver checks for
tenant operations, and creator-owner checks for creator-owned operations.

These are observed mechanisms, not a claim that the role catalog is complete,
that ownership is assigned, or that a deployment has the same configuration.

## Security migration

The storage Concept consolidates content detection, MIME agreement, PDF
structure checks, filename/key normalization, traversal and link-component
rejection, checksums, media lifecycle, restricted/public delivery, quarantine,
and cleanup jobs. The posture Concept consolidates security headers, session
validation, Step-Up, RBAC, audit metadata controls, cleanup, and the known
absence of deployment/monitoring/ownership evidence.

Legacy stale negative statements were omitted where current source evidence
contradicts them. Unverified threat, retention, incident, and operations
claims were not promoted to stable controls.

## Tenancy migration

Tenancy is recorded separately at request, application-query, database-policy,
and public-delivery levels. The current public route selects the repository's
active `default` organization; tenant routes require `X-Organization-Id` and
membership. The source does not establish custom-domain or host-derived public
tenant routing, so NOC-01 and NOC-11 remain visible in every affected Concept.

## Domain & Marketplace migration

Public delivery records published resource selection, public flags, cache
keys/invalidation, and rich-content/component handling. Billing records plans,
subscriptions, usage, request/content/media limits, and the organization-row
lock in media quota checks. Marketplace runtime records artifact validation,
permission snapshots, installation/runtime states, kill switches, allowlists,
bounded payloads, and the explicit `not_executed` result.

Marketplace finance and package infrastructure are described as implemented
mechanisms only. Settlement, disputes, refunds, tax, archive retention,
external execution, and operational cleanup policies remain owner decisions.

## Semantic consolidation

The merge process used current source and tests as the authority when legacy
material conflicted or used unsupported certainty. Repeated README/catalog
wrappers were omitted, detailed current route/domain/security claims were
merged into the eight selected Concepts, and exhaustive route/schema/quality
catalogs were left to regeneration targets. Historical audit and decision
material was not copied into current Concepts.

The full source-input decisions are in [the Phase 5 merge ledger](PHASE_05_MERGE_LEDGER.md).

## Staging bundle status

| Measure | Phase 4 baseline | Phase 5 result | Delta |
| --- | ---: | ---: | ---: |
| Concepts | 22 | 30 | +8 |
| Indexes | 13 | 13 | 0 |
| Root log files | 1 | 1 | 0 |
| Embedded Mermaid blocks | 13 | 17 | +4 |
| Standalone `.mmd` files | 0 | 0 | 0 |
| Staging files | 36 | 44 | +8 |

The file count excludes the Phase 5 reports and manifest outside the staging
root. Four domain indexes and the root staging log were modified. No existing
Concept file was modified.

## Google OKF conformance

The staging Concepts use UTF-8 Markdown with YAML frontmatter bounded by
`---`, a non-empty official `type`, and local `title`, `description`, `status`,
and `sources` fields. Reserved `index.md` and `log.md` files remain navigation
artifacts rather than Concepts. No custom frontmatter extension keys,
`generated`, or `verified` claims were fabricated. All new source URLs point to
the immutable Phase 5 source HEAD.

The official v0.2 specification defines the permissive baseline; local
ZinharCMS policy adds the stricter approved type vocabulary, source, index,
link, and zero-extension requirements.

## ZinharCMS policy validation

The eight new Concepts use only approved local types and contain no legacy
metadata keys. Their internal links stay within the staging bundle and use the
project's root-relative Concept-link convention. New visuals are embedded in
the Concepts that own the current evidence; no standalone Mermaid file was
created. Index entries were added exactly once for each new direct child.

## Security & data integrity

The separate [Phase 5 security and data verification ledger](PHASE_05_SECURITY_DATA_VERIFICATION.md)
records stable verification IDs, source evidence, result labels, and the
distinction between source verification and live deployment proof. The ledger
does not claim live PostgreSQL, Redis, ingress, object storage, external
malware, payment, webhook receiver, or monitoring validation.

## Diagram results

Four current, source-derived visuals were added:

* tenant data policy flow;
* layered security boundaries;
* published delivery workflow; and
* Marketplace runtime safety gate.

The Phase 1 preserve mapping for tenant isolation, trust boundaries, and
publication workflow was reconciled into current-source visuals. The extension
lifecycle regeneration visual remains deferred with its owner target. No
diagram was added for an API endpoint catalog, billing, storage deployment, or
documentation maintenance because those views would either duplicate an
existing Concept or require deferred evidence.

## Target construction status

The complete 54-row target matrix is in [PHASE_05_CONSTRUCTION_STATUS.md](PHASE_05_CONSTRUCTION_STATUS.md).
Final counts are 19 `BUILT_PHASE_3`, 3 `BUILT_PHASE_4`, 8 `BUILT_PHASE_5`, 11
`DEFERRED_REGENERATE`, 7 `BLOCKED_OWNER_DECISION`, 1 `OUT_OF_SCOPE`, and 5
`HISTORICAL_DEFERRED`.

## Legacy safety

The starting snapshots of `/okf/`, `/okf-bootstrap/`, and `okf/index.yaml` were
compared with the Phase 5 ending tree. No legacy file was edited, deleted,
renamed, converted, or replaced. Application source, database migration
files, dependency manifests/lockfiles, CI workflows, and runtime/configuration
files have no Phase 5 diff. No canonical `/okf/` cutover or redirect was
performed.

## Validation performed

Final validation results are recorded after the artifact manifest was written:

| Check | Result |
| --- | --- |
| Staging count and tree shape | PASS — 30 Concepts, 13 indexes, one log, 17 Mermaid blocks, zero standalone `.mmd`, 44 files |
| New Concept frontmatter and approved types | PASS — 8/8 |
| Internal links, root reachability, and index coverage | PASS |
| Embedded Mermaid fence structure | PASS — 17 open / 17 close blocks |
| Phase 5 manifest hash reconciliation | PASS — 44 entries / 0 mismatches |
| 54-target construction status | PASS — 54 unique IDs / no missing rows |
| Source-input merge ledger | PASS — all Phase 5 primary categories and shared routing recorded |
| Security/data verification ledger | PASS — stable verification IDs with explicit limits |
| Legacy/application safety comparison | PASS — no out-of-scope source or legacy changes |
| `git diff --check` | PASS — exit 0; Git emitted only expected LF/CRLF normalization warnings |
| Executable test suites | NOT RUN — Phase 5 changed only documentation and staging artifacts |

The final row is updated only from the actual command result; no application
test is represented as run or passed by this documentation phase.

## Prohibited actions confirmation

No legacy deletion, canonical replacement, cutover, redirect, application
runtime modification, database migration modification, dependency change, CI
change, branch switch, stage, commit, push, reset, clean, stash, or history
rewrite was performed.

## Open issues

* Public host/custom-domain and domain-verification routing remain owner and
  implementation decisions.
* Compatibility/versioning/deprecation and complete OpenAPI policy remain
  unresolved.
* Applied migration state, production storage/CDN, backup/recovery,
  retention/privacy/legal hold, malware/monitoring services, alert ownership,
  provider settlement, compensation, refunds/tax, external Marketplace
  execution, and archive retention remain unverified or owner-blocked.
* The Phase 5 source evidence is pinned to the current branch HEAD; future
  source changes require a new construction or verification pass.

## Recommended next phase

Phase 6 should address the remaining `DEFERRED_REGENERATE`,
`BLOCKED_OWNER_DECISION`, `OUT_OF_SCOPE`, and historical targets according to
the status matrix. Begin with an explicit selection for development,
documentation maintenance, operations/regeneration, and owner-decision
resolution. Keep the staging bundle non-canonical and do not modify the
legacy trees or cut over without a separate authorization decision.
