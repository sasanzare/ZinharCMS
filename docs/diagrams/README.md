# ZinharCMS Architecture Diagrams

This directory contains the final evidence-based architecture set produced by the
20-step Mermaid documentation process. Runtime claims were checked against current
migrations, backend route composition, handlers/services, frontend code, tests,
runtime configuration, and only then supporting documentation.

## Evidence And Conventions

| File | Purpose |
| --- | --- |
| `REPOSITORY_INVENTORY.md` | Architecture-relevant repository inventory and file responsibilities. |
| `ARCHITECTURE_AUDIT.md` | Final implementation matrix, conflicts, assumptions, and RLS matrix. |
| `AMBIGUITIES.md` | Resolved interpretations and owner decisions still required. |
| `FILE_EVIDENCE_INDEX.md` | Searchable domain-to-source evidence matrix. |
| `TRACEABILITY.md` | Diagram-to-migration/route/service/frontend/test/documentation matrix. |
| `DIAGRAM_CONVENTIONS.md` | Mermaid syntax, evidence, status, and naming rules. |

## Diagram Catalog

| Range | Coverage |
| --- | --- |
| `00` | Whole-product implementation status. |
| `01`-`04` | Scope, system context, identity/authorization, and container architecture. |
| `05`-`06` | Local and production-like deployment runtime. |
| `07`-`09` | Backend components, route boundaries, and middleware pipeline. |
| `10`-`12` | Frontend routes, state/API flow, and i18n/RTL. |
| `13`-`19` | Identity, CMS, page, media, organization, billing, beta, and Marketplace data models. |
| `20`-`22` | Marketplace review pipeline, RLS ownership, and security boundaries. |
| `23`-`25` | Core CMS, auth/organization/billing, and Marketplace state machines. |
| `26`-`30` | End-to-end auth, CMS, media, billing, operations, and Marketplace sequences. |
| `31` | Observability and failure recovery. |
| `32` | End-to-end source traceability by major domain. |
| `33` | Phase-6 Marketplace installation lifecycle, gates, updates, and rollback. |
| `34` | Phase-7 Marketplace permission, sandbox policy, and kill-switch flow. |

## Implementation Status Language

- `[IMPLEMENTED]` means current runtime/schema evidence supports the behavior.
- `[PARTIAL]` means meaningful behavior exists but an important lifecycle,
  reliability, authorization, or UI path is absent.
- `[PLANNED]` means documentation plans the capability but runtime evidence is absent.
- `[DOCUMENTED ONLY]` marks non-runtime infrastructure or claims.
- `[DECISION REQUIRED]` marks a product/operations decision code cannot answer.
- `[EXTERNAL]` marks actors or integrations outside the ZinharCMS process.

## Final Validation

- Every `.mmd` file contains exactly one standalone Mermaid declaration.
- Markdown fences are not used inside `.mmd` files.
- Evidence comments reference existing repository paths.
- Paid purchase, payout, customer-rating, and executable sandbox/runtime capabilities
  remain visually separate from the implemented Phase-6 free installation lifecycle.
- No Mermaid parser or project-local Mermaid dependency was installed in the
  repository, so parser validation was unavailable.
- All Mermaid files were independently checked with repository-local static
  validation for declaration type, block closure, participant IDs, ER/state syntax,
  class references, quotes/brackets, and mixed syntax.

## Evidence Priority

1. Current SQL migrations and constraints.
2. Backend route composition and middleware.
3. Backend handlers and services.
4. Frontend router, API client, stores, hooks, and pages.
5. Automated tests.
6. Docker, environment, and CI configuration.
7. Current architecture and API documentation.
8. README.
9. Historical phase documentation.

Production behavior and SQL migrations were not changed by this documentation set.
