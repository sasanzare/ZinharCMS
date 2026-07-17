# Mermaid Audit

## Scope and Validation Method

The repository contains 43 standalone Mermaid source files under docs/diagrams. No Mermaid blocks are embedded in Markdown. Every Mermaid file is linked from docs/diagrams/TRACEABILITY.md.

The audit checked diagram declarations, obvious delimiter balance, repository links, subject overlap, and alignment with current code and migrations. Every file contains a recognized declaration after any leading evidence comments: flowchart, sequenceDiagram, stateDiagram-v2, or erDiagram. A Mermaid parser or mmdc executable is not installed in the repository or local dependency set, so parser-level rendering validity is UNKNOWN.

No final OKF diagram was created in Phase Zero.

## Inventory

| File | Type | Subject | Current-code alignment | Standalone context | Recommended action |
| --- | --- | --- | --- | --- | --- |
| 00-implementation-status-map.mmd | Flowchart | Documentation and implementation status | PARTIALLY_CURRENT; see DCC-05 | Filename, labels, and traceability entry provide context; no explicit Mermaid title | Update the API/OpenAPI status distinction before linking from OKF |
| 01-project-scope.mmd | Flowchart | Project scope, implemented areas, and conflicts | CURRENT as a scope and conflict map | Sufficient context; no explicit title | Preserve and link as audit evidence |
| 02-system-context.mmd | Flowchart | System context and external dependencies | PARTIALLY_CURRENT; see DCC-06 | Sufficient context; no explicit title | Update paid Marketplace capability before OKF reuse |
| 03-identity-and-authorization-boundaries.mmd | Flowchart | Authentication, authorization, tenant, and public boundaries | CURRENT | Sufficient context; no explicit title | Preserve and link from security architecture |
| 04-container-architecture.mmd | Flowchart | Runtime containers and dependencies | CURRENT | Sufficient context; no explicit title | Preserve and link from architecture |
| 05-local-development-runtime.mmd | Flowchart | Local development runtime | CURRENT for the repository-provided local environment | Sufficient context; no explicit title | Preserve and link from development setup |
| 06-production-deployment.mmd | Flowchart | Proposed/known production deployment boundary | PARTIALLY_CURRENT because actual production topology is UNKNOWN | Sufficient context; no explicit title | Preserve repository-known facts and retain production unknowns |
| 07-backend-component-architecture.mmd | Flowchart | Backend components | CURRENT | Sufficient context; no explicit title | Preserve; refresh if route modules are split |
| 08-route-boundaries.mmd | Flowchart | Public, authenticated, and tenant route stacks | CURRENT | Sufficient context; no explicit title | Preserve and link from API architecture |
| 09-request-middleware-pipeline.mmd | Sequence diagram | Request and middleware lifecycle | CURRENT | Sufficient context; no explicit title | Preserve |
| 10-frontend-route-component-architecture.mmd | Flowchart | Browser routes and page components | CURRENT | Sufficient context; no explicit title | Preserve; verify during Phase 4 |
| 11-frontend-state-and-api-flow.mmd | Flowchart | State stores and API client | CURRENT | Sufficient context; no explicit title | Preserve; document missing automatic token refresh separately |
| 12-i18n-and-rtl-flow.mmd | Flowchart | Language and direction flow | PARTIALLY_CURRENT because I18N coverage prose is stale | Sufficient context; no explicit title | Verify coverage and update with frontend documentation |
| 13-identity-auth-data-model.mmd | ER diagram | Users, roles, tokens, and login attempts | CURRENT | Sufficient context; no explicit title | Preserve and reuse as a source for the final database set |
| 14-core-content-data-model.mmd | ER diagram | Content types and entries | CURRENT | Sufficient context; no explicit title | Preserve |
| 15-page-builder-data-model.mmd | ER diagram | Pages, versions, components, and comments | CURRENT | Sufficient context; no explicit title | Preserve |
| 16-media-delivery-webhook-data-model.mmd | ER diagram | Media, delivery configuration, and webhooks | CURRENT | Sufficient context; no explicit title | Preserve |
| 17-organization-tenancy-data-model.mmd | ER diagram | Organizations, members, invitations, domains, and tenant operations | CURRENT | Sufficient context; no explicit title | Preserve |
| 18-billing-operations-beta-data-model.mmd | ER diagram | Plans, subscriptions, usage, audit, and beta operations | CURRENT | Sufficient context; no explicit title | Preserve |
| 19-marketplace-data-model.mmd | ER diagram | Marketplace base entities | CURRENT for the base model | Sufficient context; no explicit title | Extend or supplement for Phase 9 and Phase 10 tables |
| 20-marketplace-package-review-pipeline.mmd | Flowchart | Upload, validation, review, catalog | PARTIALLY_CURRENT; see DCC-10 | Sufficient context; no explicit title | Correct the purchase-runtime class before reuse |
| 21-tenant-ownership-and-rls.mmd | Flowchart | Organization ownership, tenant context, and RLS | CURRENT in concept | Sufficient context; no explicit title | Extend scope to all 32 current RLS tables |
| 22-security-trust-boundaries.mmd | Flowchart | Security and trust boundaries | CURRENT | Sufficient context; no explicit title | Preserve and link from security architecture |
| 23-core-cms-state-machines.mmd | State diagram | Entry, page, comment, and related CMS states | CURRENT | Sufficient context; no explicit title | Preserve |
| 24-auth-organization-billing-state-machines.mmd | State diagram | Authentication, organization, invitation, and billing states | CURRENT | Sufficient context; no explicit title | Preserve |
| 25-marketplace-state-machines.mmd | State diagram | Marketplace creator, submission, review, and listing states | CURRENT | Sufficient context; no explicit title | Preserve |
| 26-auth-and-tenant-sequences.mmd | Sequence diagram | Authentication and tenant request sequences | CURRENT | Sufficient context; no explicit title | Preserve; note browser refresh limitation in prose |
| 27-content-and-page-sequences.mmd | Sequence diagram | Content and page mutation/publish side effects | CURRENT | Sufficient context; no explicit title | Preserve; document compensation gaps |
| 28-media-delivery-webhook-sequences.mmd | Sequence diagram | Media, delivery, and webhook sequences | CURRENT | Sufficient context; no explicit title | Preserve; document storage and authorization decisions |
| 29-billing-operations-sequences.mmd | Sequence diagram | Billing and SaaS operations | CURRENT | Sufficient context; no explicit title | Preserve |
| 30-marketplace-sequences.mmd | Sequence diagram | Early Marketplace capabilities and deferrals | OUTDATED; see DCC-07 | Sufficient context; no explicit title | Rewrite as a historical baseline or replace with current lifecycle diagrams |
| 31-observability-and-failure-recovery.mmd | Flowchart | Observability signals and recovery gaps | CURRENT for repository-known behavior | Sufficient context; no explicit title | Preserve; retain production observability and recovery as UNKNOWN |
| 32-end-to-end-traceability.mmd | Flowchart | Cross-domain source and documentation traceability | CURRENT | Sufficient context; no explicit title | Preserve and link from the source register |
| 33-marketplace-installation-lifecycle.mmd | Flowchart | Installation and entitlement gates | OUTDATED; see DCC-08 | Sufficient context; no explicit title | Update for paid entitlements and refund revocation |
| 34-marketplace-security-runtime.mmd | Flowchart | Runtime authorization and kill-switch policy | CURRENT | Sufficient context; no explicit title | Preserve; retain the no-external-execution boundary |
| 35-marketplace-runtime-adapters.mmd | Flowchart | Host-owned components, templates, and hooks | CURRENT | Sufficient context; no explicit title | Preserve |
| 36-marketplace-finance-lifecycle.mmd | Sequence diagram | Checkout, entitlement, refund, and payout | CURRENT | Sufficient context; no explicit title | Preserve and link from Marketplace finance |
| 37-marketplace-feedback-abuse.mmd | Sequence diagram | Reviews and abuse reporting | CURRENT | Sufficient context; no explicit title | Preserve |
| 38-marketplace-analytics.mmd | Sequence diagram | Creator and admin analytics | CURRENT | Sufficient context; no explicit title | Preserve |
| 39-marketplace-creator-tooling.mmd | Flowchart | CLI, SDK, samples, and creator docs | CURRENT | Sufficient context; no explicit title | Preserve |
| 40-marketplace-qa-performance.mmd | Flowchart | Security QA and performance gates | CURRENT | Sufficient context; no explicit title | Preserve |
| 41-marketplace-beta.mmd | Flowchart | Private and customer beta gates | CURRENT | Sufficient context; no explicit title | Preserve |
| 42-marketplace-launch-ga.mmd | Flowchart | Launch readiness and GA evidence | CURRENT | Sufficient context; no explicit title | Preserve |

## Structural Findings

- All 43 files are reachable from the traceability index.
- Static checks found no obvious unmatched square brackets or parentheses.
- Every diagram has a recognizable type declaration.
- Explicit Mermaid accessibility titles were not found. Filenames and in-diagram labels make the subject understandable, but later OKF work should add consistent title and description metadata when supported by the chosen renderer.
- The ER diagrams partition the schema by domain and intentionally overlap at boundary entities. This is useful, not exact duplication.
- The Marketplace flow and sequence diagrams represent different milestones. Diagrams 30 and 33 are materially stale rather than redundant.

## Syntax and Rendering Status

| Check | Result |
| --- | --- |
| Recognized diagram declaration | 43 of 43 passed static inspection |
| Traceability link | 43 of 43 present |
| Obvious delimiter balance | 43 of 43 passed heuristic inspection |
| Mermaid parser validation | UNKNOWN; no parser is installed |
| Rendered visual inspection | UNKNOWN; no renderer is installed and Phase Zero does not install dependencies |

## Recommendations for Later OKF Phases

1. Phase 2 should select and update diagrams 02 through 09, 21, 22, and 31 for the final architecture set.
2. Phase 4 should validate diagrams 10 through 12 against current route, state, and localization code.
3. Phase 5 should generate a current schema index from migrations, then update or supplement ER diagrams 13 through 19 rather than merging every table into one unreadable diagram.
4. Phase 9 should correct diagrams 20, 30, 33, and 34 through 42 as a coherent Marketplace set.
5. Phase 11 should add consistent metadata, run an actual Mermaid parser, render every selected diagram, and record the renderer version.
6. Existing diagrams should be linked or updated in place. Duplicating them under okf should be avoided unless the final structure requires a generated, source-controlled derivative.

## Audit Markers

- DOCUMENTATION_CODE_CONFLICT DCC-05: diagram 00 does not distinguish the current manual API document from partial generated OpenAPI coverage.
- DOCUMENTATION_CODE_CONFLICT DCC-06: diagram 02 still describes paid Marketplace purchases as deferred.
- DOCUMENTATION_CODE_CONFLICT DCC-07: diagram 30 describes installation, purchase, payout, and rating capabilities as unimplemented.
- DOCUMENTATION_CODE_CONFLICT DCC-08: diagram 33 treats paid entitlements as a future rejection path.
- DOCUMENTATION_CODE_CONFLICT DCC-10: diagram 20 assigns the implemented or partial purchase_runtime node to the planned class.
- UNKNOWN: parser-level and rendered Mermaid validity remain unverified.
