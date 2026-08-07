# Phase 4 — Architecture & Application Knowledge Migration

**Status:** COMPLETE  
**Date:** 2026-08-07  
**Source HEAD:** `e37e94e2e6960a2547f33bf1ebb4225f818b3a4b`  
**Staging root:** `docs/okf-migration/staging/google-okf-v0.2/`

## Executive result

Phase 4 semantically consolidated the current Architecture, Backend, and
Frontend application knowledge that was safe to construct from repository
evidence. Three deferred merge targets were built:

* `TARGET-008` — Architecture / Integrations and Side Effects
* `TARGET-012` — Backend / Persistence, Services, and Configuration
* `TARGET-016` — Frontend / Feature Boundaries

The isolated staging bundle now contains 22 Concepts, 13 indexes, one root
log, 13 existing embedded Mermaid blocks, and zero standalone Mermaid files.
The legacy `/okf/` tree remains canonical and untouched. No cutover,
redirect, application change, database change, dependency change, CI change,
or runtime change was performed.

The bundle shape and Concept metadata were checked against the [Google Open
Knowledge Format v0.2 specification](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
and the repository's Phase 2 local policy.

## Scope and eligibility

The Phase 3 baseline contained 54 targets: 19 built and 35 deferred. Phase 4
selected only `DEFERRED_MERGE` targets whose primary domain was Architecture,
Backend, or Frontend. The eligible set was exactly `TARGET-008`, `TARGET-012`,
and `TARGET-016`. No API, Database, Security, Domain, Operations,
Development, Decision, or History target was pulled forward merely because it
was referenced by the selected concepts.

The corresponding legacy groups were G002, G003, and G008. The full input
outcome record is in [the Phase 4 merge ledger](PHASE_04_MERGE_LEDGER.md).

## Bundle delta

| Measure | Phase 3 baseline | Phase 4 result | Delta |
| --- | ---: | ---: | ---: |
| Concepts | 19 | 22 | +3 |
| Indexes | 13 | 13 | 0 |
| Root log files | 1 | 1 | 0 |
| Embedded Mermaid blocks | 13 | 13 | 0 |
| Standalone `.mmd` files | 0 | 0 | 0 |
| Staging files | 33 | 36 | +3 |
| Legacy source records considered | — | 72 | G002/G003/G008 |

The three index files for the selected domains and the root staging log were
updated. No Phase 3 Concept file was modified.

## Current implementation knowledge constructed

### TARGET-008 — Integrations and Side Effects

This Architecture Concept records the current cache, public-delivery
invalidation, webhook dispatch and delivery recording, email delivery modes,
local file-cleanup jobs, outbound HTTP safety boundary, and route-level
transaction/audit boundaries. It explicitly distinguishes local behavior from
global atomicity and external-provider guarantees.

The source packet includes route composition, content/pages/delivery routes,
cache, webhook, email, outbound HTTP, file-cleanup, and configuration sources
at the Phase 4 source head. It links to the existing system architecture,
runtime boundary, backend persistence/configuration, module boundary, API, and
tenant-isolation Concepts.

NOC-02 remains open for storage architecture and asset authorization. NOC-09
remains open for retry, compensation, idempotency, and user-visible failure
guarantees. No unsupported delivery guarantee was promoted.

### TARGET-012 — Persistence, Services, and Configuration

This Backend Concept records the single-process startup sequence, `AppState`,
SQLx/PostgreSQL pool and migration boundary, Redis role, environment-backed
configuration validation surface, service registry, and configured local
upload directory. It separates repository configuration from deployed
infrastructure and does not promote provider values, scaling, backup, or
durability claims.

The source packet includes `main.rs`, `lib.rs`, `state.rs`, `db/mod.rs`,
`config.rs`, `services/mod.rs`, and `Cargo.toml`. It links to the existing
backend runtime/module Concepts, the new side-effect Concept, API contract,
and tenant-isolation Concept.

NOC-02 remains the explicit boundary for object/shared storage, CDN behavior,
asset authorization, backup, and deployment-level durability.

### TARGET-016 — Frontend Feature Boundaries

This Frontend Concept records the React/Vite entry composition, authenticated
shell, page-level feature areas, centralized API boundary, Zustand session and
organization projections, shared components, and internationalization/
direction behavior. It presents content/editorial, page-builder/media,
Marketplace/billing, organization/workspace, beta, settings, and dashboard
areas as features within one authenticated SPA; it does not claim separate
frontend deployments or future team/package ownership.

The source packet includes `main.tsx`, `router.tsx`, `AppShell.tsx`,
`api.ts`, `useAppStore.ts`, `I18nProvider.tsx`, current package metadata, and
the current page/component surface. It links to the existing admin,
routing/state, API, backend persistence, editorial, and page-builder Concepts.

NOC-12 remains open for compatibility, accessibility, schema/workflow,
browser/session, and frontend decomposition policy. NOC-18 remains open for
preferred terminology and abbreviations.

## Legacy consolidation and exclusion policy

The ledger considered all 72 legacy source records in G002, G003, and G008.
It records 10 Architecture, 31 Backend, and 31 Frontend inputs with no
duplicate or silently omitted input path. Repeated README wrappers, legacy
catalog duplicates, historical decision material, and detailed domain/API
views were routed to existing Concepts, left deferred, preserved as
historical-only material, or omitted when they added no current knowledge.

The new Concepts exclude stale, contradicted, unverified, or owner-blocked
claims about production topology, external providers, deployment/scaling,
backup/recovery, object storage/CDN, universal transaction atomicity, retry or
compensation guarantees, durable ownership, browser/accessibility policy, and
future terminology. No content from `okf-bootstrap/` or legacy metadata was
copied into the new Concepts.

## Diagram disposition

No Phase 4 diagram was added. The Phase 1/2 diagram map assigns the relevant
Architecture, Backend, and Frontend visuals to existing runtime, module, and
routing Concepts or to later regenerate targets. A new diagram would either
duplicate those views or imply unsupported topology. All 13 existing diagrams
remain embedded in their Phase 3 Concepts, and there are no standalone `.mmd`
files.

## Validation results

The following checks were executed against the final Phase 4 working tree:

| Check | Result |
| --- | --- |
| UTF-8 Markdown and required Concept frontmatter | PASS — 22/22 Concepts |
| Approved 19-type registry and no custom frontmatter extensions | PASS |
| Non-empty source provenance with `resource` entries | PASS |
| Internal Markdown links, parent traversal, and broken targets | PASS — zero errors |
| Root index, per-domain index coverage, and Concept reachability | PASS — zero orphans |
| Embedded Mermaid fence structure | PASS — 13 open / 13 close blocks |
| Standalone Mermaid files | PASS — 0 |
| Phase 4 build-manifest SHA-256 snapshot | PASS — 36 entries / 0 mismatches |
| Construction status matrix | PASS — 54 unique IDs / no missing IDs |
| Merge ledger | PASS — 72 unique inputs: 10/31/31 |
| New Concept leakage scan | PASS — no legacy path, bootstrap, or Phase 0–3 term |
| `git diff --check` | PASS |
| Legacy and application safety diff | PASS — no changes outside migration docs and HANDOFF |

Three carried Phase 3 Concepts still contain historical provenance wording
(`system-architecture.md`, `migration-and-architecture-decisions.md`, and
`project-overview.md`). Those references were pre-existing, are not legacy
source leakage in the three new Concepts, and were intentionally not rewritten
because Phase 3 artifacts were protected from modification.

No application or runtime test suite was run because Phase 4 changes only
documentation and staging artifacts; no executable source changed.

## Artifacts

* [Phase 4 construction status](PHASE_04_CONSTRUCTION_STATUS.md)
* [Phase 4 merge ledger](PHASE_04_MERGE_LEDGER.md)
* [Phase 4 build manifest](PHASE_04_BUILD_MANIFEST.json)
* [Architecture Concept](staging/google-okf-v0.2/architecture/integrations-and-side-effects.md)
* [Backend Concept](staging/google-okf-v0.2/backend/persistence-services-and-configuration.md)
* [Frontend Concept](staging/google-okf-v0.2/frontend/feature-boundaries.md)

## Remaining work and boundary

After Phase 4, 32 targets remain deferred: 9 merge targets, 11 regenerate
targets, 7 owner-blocked targets, and 5 historical targets. No deferred target
has a placeholder Concept link. Data, API, Security, and Domain knowledge
remain outside this phase, as do operational deployment, recovery,
observability, and owner-decision concepts.

## Recommended next phase

Phase 5 — Data, API, Security & Domain Knowledge Migration
