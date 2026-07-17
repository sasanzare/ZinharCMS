# Proposed OKF Structure

## Design Principles

The final OKF should be a concise navigation and knowledge layer over authoritative code, migrations, generated OpenAPI, and existing operational documents. It should not copy every phase record or duplicate every Mermaid source.

The structure should:

- separate observed current state from historical plans;
- identify code, migration, test, and owner-policy sources for every material claim;
- link existing documents when they remain authoritative;
- isolate UNKNOWN, NEEDS_OWNER_CONFIRMATION, and DOCUMENTATION_CODE_CONFLICT markers;
- treat built-in plugins and Marketplace extensions as separate systems;
- preserve domain-specific database diagrams instead of one unreadable all-table ERD;
- support incremental phases without presenting incomplete sections as final.

Phase Zero proposes this structure only. The okf directory must not be created until the next phase is authorized.

## Proposed Directory Tree

~~~text
okf/
  README.md
  index.yaml
  project/
    overview.md
    repository-map.md
    glossary.md
    documentation-map.md
  architecture/
    overview.md
    system-context.md
    runtime-containers.md
    backend-components.md
    frontend-components.md
    request-and-data-flows.md
    trust-boundaries.md
    external-dependencies.md
    decisions.md
  modules/
    index.md
    backend.md
    frontend.md
    cms.md
    saas-operations.md
    marketplace.md
  frontend/
    overview.md
    routing-and-shell.md
    state-and-api-client.md
    forms-and-validation.md
    page-builder.md
    localization-and-accessibility.md
  database/
    overview.md
    schema-index.md
    relationships.md
    tenancy-and-rls.md
    states-and-constraints.md
    migrations-and-seeding.md
    retention-and-deletion.md
  api/
    overview.md
    route-index.md
    authentication-and-tenancy.md
    authorization-matrix.md
    errors-and-validation.md
    pagination-and-idempotency.md
    webhooks-and-websockets.md
    openapi-coverage.md
    versioning.md
  security/
    overview.md
    authentication.md
    authorization.md
    tenant-isolation.md
    upload-and-artifact-security.md
    secrets-and-provider-signatures.md
    threat-model.md
  business/
    content-and-page-workflows.md
    organizations-and-plans.md
    quotas-and-billing.md
    delivery-and-cache.md
    failure-and-compensation.md
  extensions/
    overview.md
    built-in-plugins.md
    marketplace-domain.md
    package-validation-and-review.md
    installation-and-runtime-policy.md
    host-owned-adapters.md
    finance.md
    feedback-and-analytics.md
  development/
    local-setup.md
    code-conventions.md
    testing-and-quality.md
    database-change-workflow.md
    contribution-workflow.md
  operations/
    deployment.md
    configuration.md
    observability.md
    backup-and-restore.md
    incident-response.md
    troubleshooting.md
  diagrams/
    index.md
    validation-report.md
  references/
    source-register.md
    documentation-conflicts.md
    knowledge-gaps.md
    owner-decisions.md
~~~

## Top-Level Files

| Proposed path | Purpose | Primary sources | Existing artifact disposition | Phase | Priority |
| --- | --- | --- | --- | ---: | --- |
| okf/README.md | Audience, scope, authority rules, navigation, freshness model | README, AGENTS, Phase Zero reports | New index; link existing docs | 1 | High |
| okf/index.yaml | Machine-readable document registry, status, owners, sources, markers, last verification | All OKF files and source register | New metadata; do not contain secrets | 1 and maintained | High |

## Project and Architecture

| Proposed area | Purpose | Primary sources | Preserve, update, or link | Phase | Priority |
| --- | --- | --- | --- | ---: | --- |
| project/overview.md | Product scope, audiences, implemented capability summary, non-goals | README, current phase records, code inventory | Consolidate current state; link historical phases | 1 | High |
| project/repository-map.md | Directory responsibilities, generated/ignored assets, entry points | Git inventory, Docker/CI files | New observed map | 1 | High |
| project/glossary.md | Stable domain terms and abbreviations | Models, routes, UI labels, existing docs | New; owner terminology markers remain explicit | 1 | High |
| project/documentation-map.md | Canonical, historical, generated, and conflicting document map | Documentation audit | New navigation; do not move existing docs | 1 | High |
| architecture/* | Context, containers, components, flows, boundaries, dependencies, decisions | Runtime code, routes, state, Docker, diagrams 02-09, 21-22, 31 | Link/update selected Mermaid sources; record production UNKNOWN values | 2 | High |

## Modules and Frontend

| Proposed area | Purpose | Primary sources | Preserve, update, or link | Phase | Priority |
| --- | --- | --- | --- | ---: | --- |
| modules/index.md | Ownership, responsibility, dependency, API, data, and test map for 22 observed modules | Module inventory and source | New authoritative navigation | 3 | High |
| modules/backend.md | Runtime and cross-cutting backend structure | backend/src | Consolidate observed boundaries | 3 | High |
| modules/cms.md | Content, media, pages, delivery, comments, webhooks, built-in plugins | Relevant routes/services/migrations | Link Phase One-Six records as history | 3 | High |
| modules/saas-operations.md | Organizations, tenancy, billing, beta, and GA operations | V2 source and docs | Link V2 guides and current routes | 3 | High |
| modules/marketplace.md | Map Marketplace modules without duplicating extension detail | V3 source and docs | Link extensions section | 3 | High |
| frontend/* | Router, shell, stores, API client, forms, builder, localization, accessibility | frontend/src, package.json, diagrams 10-12, I18N.md | Update DCC-02 and link UI tests | 4 | High |

## Database and API

| Proposed area | Purpose | Primary sources | Preserve, update, or link | Phase | Priority |
| --- | --- | --- | --- | ---: | --- |
| database/overview.md | Platform, ownership model, counts, and authority | All migrations, SQLx configuration | New concise overview | 5 | High |
| database/schema-index.md | All 51 tables with purpose, owner, keys, RLS, and migration | Migrations | Generate and verify from SQL | 5 | High |
| database/relationships.md | Domain relationships and ERD navigation | Migrations, diagrams 13-19 | Update/link domain diagrams | 5 | High |
| database/tenancy-and-rls.md | RLS table/policy matrix and runtime settings | Migrations, rls.rs, tenant middleware | Include all 32 RLS tables | 5 | High |
| database/states-and-constraints.md | Enums, text states, checks, uniqueness, service authority | Migrations and services | New state authority matrix | 5 | High |
| database/migrations-and-seeding.md | Startup migration, numbering, seed, rollback limitations | main.rs, migrations, seeds | New current workflow | 5 | High |
| database/retention-and-deletion.md | Hard delete, archive, history, purge, backup implications | Migrations/routes plus NOC-03/NOC-05 | Remain draft until owner decisions | 5 and 10 | High |
| api/* | Route inventory, boundaries, contracts, errors, realtime/provider endpoints, coverage, versioning | Router, handlers, AppError, OpenAPI, API.md | Correct DCC-09; measure OpenAPI gaps | 6 | High |

## Security, Business Rules, and Extensions

| Proposed area | Purpose | Primary sources | Preserve, update, or link | Phase | Priority |
| --- | --- | --- | --- | ---: | --- |
| security/* | Auth, roles, tenant isolation, uploads, secrets, signatures, threats | Middleware, services, migrations, security tests, diagrams 03/22/24/26/34/40 | Consolidate controls; leave operational requirements marked | 7 | High |
| business/* | Workflows, organizations, plans, billing, delivery, cache, compensation | Services, routes, migrations, tests, phase docs | Extract executable rules and label owner policy gaps | 8 | High |
| extensions/built-in-plugins.md | In-process host plugin registry and hooks | plugins directory and cms_plugins | Keep separate from Marketplace | 9 | High |
| extensions/marketplace-domain.md | Current creator, listing, package, install, finance, feedback model | Migrations 0014-0025, Marketplace services | Correct DCC-03 | 9 | High |
| extensions/package-validation-and-review.md | Upload, static checks, moderation, state transitions | Validation/review source and diagrams 20/25/30 | Correct DCC-07/DCC-10 | 9 | High |
| extensions/installation-and-runtime-policy.md | Install lifecycle, permissions, kill switches, non-execution boundary | Marketplace installation/runtime code and diagrams 33-34 | Correct DCC-08 | 9 | High |
| extensions/host-owned-adapters.md | Component, template, and hook adapters | Adapter code and diagram 35 | Link current evidence | 9 | Medium |
| extensions/finance.md | Checkout, entitlement, ledger, refund, payout | Finance source and diagram 36 | Current behavior plus roadmap markers | 9 | High |
| extensions/feedback-and-analytics.md | Reviews, reports, notifications, aggregates, readiness | Feedback/analytics source and diagrams 37-42 | Current behavior and operational limits | 9 | Medium |

## Development, Operations, Diagrams, and References

| Proposed area | Purpose | Primary sources | Preserve, update, or link | Phase | Priority |
| --- | --- | --- | --- | ---: | --- |
| development/* | Setup, conventions, tests, database changes, contribution | CI, manifests, Docker, source patterns, owner decisions | Link README setup; do not invent policy | 10 | High |
| operations/* | Deployment, configuration, observability, recovery, incidents, troubleshooting | Docker/CI, existing V2/V3 runbooks, owner/platform evidence | Draft repository-known parts; keep UNKNOWN markers | 10 | High |
| diagrams/index.md | Purpose, source, owner, status, and consumer of selected diagrams | TRACEABILITY.md and Mermaid audit | Link existing files; avoid unnecessary copies | 11 | Medium |
| diagrams/validation-report.md | Parser/renderer version and validation outcomes | Selected Mermaid set and CI | New after actual rendering | 11 | Medium |
| references/source-register.md | Claim-to-code/migration/test/document traceability | All OKF files | New and machine-checkable where practical | 12 | High |
| references/documentation-conflicts.md | DCC register and resolution status | Phase Zero conflicts | Migrate and maintain conflict IDs | 1 and maintained | High |
| references/knowledge-gaps.md | UNKNOWN register and verification state | Phase Zero gaps | Migrate and maintain unknown IDs | 1 and maintained | High |
| references/owner-decisions.md | NOC questions, decisions, dates, rationale, affected docs | Owner responses | New decision log; never fabricate answers | 1 and maintained | High |

## Proposed Metadata Model

Each index.yaml entry should include:

- id and path;
- title and audience;
- status: draft, current, historical, blocked, or superseded;
- authority: code, migration, generated, repository-doc, or owner-policy;
- source paths;
- applicable modules;
- marker IDs;
- last_verified_date and verification method;
- owner when known;
- related diagrams and documents.

## Migration Rules for Existing Documentation

1. Do not move or rename existing phase records during OKF creation.
2. Link current guides and diagrams from OKF when duplication would create two authorities.
3. Correct concrete conflicts in their original files during the relevant later phase, with separate authorization for those edits.
4. Summarize historical plans only when needed to explain present architecture.
5. Keep HANDOFF.md operational and outside the OKF knowledge hierarchy.
6. Generate schema and route inventories from source where practical, then record the generation command and version.

## Completion Boundary

The structure is complete as a Phase Zero proposal. No final folder, metadata registry, copied diagram, or implementation document has been created.
