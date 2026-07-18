---
okf_document_id: "okf-readme"
title: "ZinharCMS Open Knowledge Format"
project: "ZinharCMS"
category: "knowledge-base"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "README.md"
  - "okf-bootstrap/phase-zero-summary.md"
  - "okf-bootstrap/10-proposed-okf-structure.md"
  - "okf-bootstrap/11-implementation-phases.md"
related_documents:
  - "index.yaml"
  - "project/overview.md"
  - "project/repository-map.md"
  - "project/glossary.md"
  - "project/navigation-guide.md"
  - "references/source-register.md"
  - "architecture/README.md"
  - "architecture/overview.md"
  - "database/README.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
  - "backend/README.md"
  - "backend/overview.md"
  - "backend/module-catalog.md"
  - "backend/backend-risks.md"
  - "frontend/README.md"
  - "frontend/feature-catalog.md"
  - "frontend/frontend-risks.md"
uncertainty_markers:
  - "UNKNOWN"
  - "NEEDS_OWNER_CONFIRMATION"
  - "INFERRED_FROM_CODE"
  - "INFERRED_FROM_STRUCTURE"
  - "DOCUMENTATION_CODE_CONFLICT"
  - "PLANNED_NOT_IMPLEMENTED"
  - "IMPLEMENTATION_STATUS_UNCLEAR"
  - "INFERRED_FROM_CONFIGURATION"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR"
  - "DEPENDENCY_DIRECTION_UNCLEAR"
  - "PROPOSED_NOT_IMPLEMENTED"
  - "FEATURE_BOUNDARY_UNCLEAR"
  - "COMPONENT_OWNERSHIP_UNCLEAR"
  - "ROUTING_BEHAVIOR_UNCLEAR"
  - "STATE_OWNERSHIP_UNCLEAR"
  - "API_CONTRACT_UNCLEAR"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED"
  - "DUPLICATED_CONTRACT"
  - "DEAD_OR_UNUSED_CODE_UNCONFIRMED"
  - "UI_BEHAVIOR_UNVERIFIED"
---

# ZinharCMS Open Knowledge Format

## Purpose

The okf directory is the organized knowledge and navigation layer for ZinharCMS. It gives human contributors and AI coding agents a verified starting point for understanding the product, locating implementation evidence, recognizing uncertainty, and finding the most relevant existing documentation.

> Source code remains the primary source of truth for implemented behavior. OKF documents explain project context, structure, decisions, responsibilities, and verified relationships.

OKF does not replace repository README files, API specifications, tests, source-code comments, database migrations, Architecture Decision Records, or existing diagrams. It connects those sources and explains how their authority differs.

## Audience

Use this knowledge base if you are:

- joining the project as a developer, architect, technical project manager, or contributor;
- maintaining the Rust backend or React administration application;
- operating the repository-provided local or production-like container stack;
- creating or reviewing Marketplace products;
- analyzing or modifying the repository as an AI coding agent.

## Source-of-Truth Priority

When sources disagree, use this priority and record the conflict:

1. current source code;
2. current configuration and manifest files;
3. database migrations and schemas;
4. tests;
5. current deployment configuration;
6. existing technical documentation;
7. Phase Zero analysis;
8. inference from repository structure.

Generated OpenAPI, runtime output, and deployed-environment evidence may provide additional verification, but they must be identified with their generation or observation context.

## OKF and Existing Documentation

The repository already contains extensive documentation under ../docs/, including phase records, API and architecture guides, operator runbooks, Marketplace contracts, and Mermaid diagrams. These files remain in place.

OKF has a different role:

- it provides a stable entry point and machine-readable index;
- it distinguishes current implementation evidence from historical phase records;
- it links to authoritative documents instead of copying them;
- it makes conflicts, unknowns, and owner decisions visible;
- it guides readers toward the smallest relevant source set.

The cumulative ../HANDOFF.md file is operational recovery history, not canonical product architecture. The Phase Zero reports under ../okf-bootstrap/ are supporting analysis, not runtime authority.

## Uncertainty Markers

| Marker | Meaning |
| --- | --- |
| UNKNOWN | The repository and available environment do not establish the fact |
| NEEDS_OWNER_CONFIRMATION | A product, policy, ownership, or operational decision is required |
| INFERRED_FROM_CODE | The conclusion follows from current implementation but is not explicitly governed |
| INFERRED_FROM_STRUCTURE | The conclusion follows from repository organization only |
| INFERRED_FROM_CONFIGURATION | The conclusion follows from tracked executable configuration, not a verified deployed environment |
| INFERRED_FROM_TESTS | The conclusion follows from the exercised test setup and assertions only |
| DOCUMENTATION_CODE_CONFLICT | Documentation and current implementation disagree |
| PLANNED_NOT_IMPLEMENTED | A source identifies future work, but current implementation evidence is absent |
| PROPOSED_NOT_IMPLEMENTED | A target architecture decision is documented but is not implemented |
| IMPLEMENTATION_STATUS_UNCLEAR | Some evidence exists, but the complete capability cannot be verified |
| ARCHITECTURAL_BOUNDARY_UNCLEAR | Component or responsibility ownership is not consistently defined by current structure |
| DEPENDENCY_DIRECTION_UNCLEAR | Source or contract dependencies cross the apparent layer direction |
| FEATURE_BOUNDARY_UNCLEAR | Frontend responsibility does not map to one stable feature owner |
| COMPONENT_OWNERSHIP_UNCLEAR | Shared UI/component ownership or supported interface is not governed |
| ROUTING_BEHAVIOR_UNCLEAR | Route behavior is only partially established across source and actual hosting |
| STATE_OWNERSHIP_UNCLEAR | Reactive, persistent, transport, or server-state ownership is distributed or undefined |
| API_CONTRACT_UNCLEAR | Frontend and backend contract equivalence is not enforced or verified |
| AUTHORIZATION_BEHAVIOR_UNVERIFIED | Browser visibility or guards do not prove backend authorization |
| RESPONSIBILITY_OVERLAP | More than one feature or module owns adjacent behavior |
| DUPLICATED_CONTRACT | The same contract is maintained independently in more than one source area |
| DEAD_OR_UNUSED_CODE_UNCONFIRMED | Usage was not found, but removal safety or dead-code status is not proven |
| UI_BEHAVIOR_UNVERIFIED | Browser, responsive, accessibility, direction, or visual behavior was not executed |

Never remove or weaken a marker without stronger evidence or an explicit owner decision.

## Phase 1 Documents

| Document | Purpose |
| --- | --- |
| [Machine-Readable Index](index.yaml) | Document registry, verification metadata, planned sections, and uncertainty IDs |
| [Project Overview](project/overview.md) | Project identity, purpose, users, capabilities, boundaries, technology, lifecycle, and high-level unknowns |
| [Repository Map](project/repository-map.md) | Human-readable map from common concerns to real repository paths |
| [Glossary](project/glossary.md) | Controlled definitions for project-specific and ambiguous terms |
| [Navigation Guide](project/navigation-guide.md) | Short task-oriented reading and source-navigation sequence |
| [Source Register](references/source-register.md) | Material evidence used to create and verify current OKF documents |

## Phase 2 Architecture Documents

| Document | Purpose |
| --- | --- |
| [System Architecture](architecture/README.md) | Architecture entry point, evidence rules, reading order, and diagram index |
| [Architecture Overview](architecture/overview.md) | Client-server classification, modular-monolith backend, runtime shape, and confidence |
| [System Boundaries](architecture/boundaries.md) | Trust, tenant, frontend, plugin, storage, and external-system boundaries |
| [Components and Responsibilities](architecture/components.md) | Runtime and source component responsibilities |
| [Dependency Model](architecture/dependency-model.md) | Backend and frontend dependency direction and verified exceptions |
| [Runtime Flows](architecture/runtime-flows.md) | Startup, public, authenticated, tenant, delivery, media, webhook, and preview flows |
| [Integration Points](architecture/integration-points.md) | PostgreSQL, Redis, filesystem, browser, provider, plugin, and Marketplace integration contracts |
| [Architecture Risks](architecture/architecture-risks.md) | Evidence-based risk register and priority follow-up |
| [Architecture Decisions](architecture/decisions/decision-register.md) | Observed and explicitly accepted architecture choices |

Recommended architecture reading order: [Overview](architecture/overview.md), [Boundaries](architecture/boundaries.md), [Components](architecture/components.md), [Dependency Model](architecture/dependency-model.md), [Runtime Flows](architecture/runtime-flows.md), [Integration Points](architecture/integration-points.md), [Risks](architecture/architecture-risks.md), and [Decisions](architecture/decisions/decision-register.md).

Architecture diagram navigation: [System Context](architecture/diagrams/system-context.mmd), [Container View](architecture/diagrams/container-view.mmd), [Backend Request Flow](architecture/diagrams/backend-request-flow.mmd), [Frontend-Backend Flow](architecture/diagrams/frontend-backend-flow.mmd), and [Dependency Direction](architecture/diagrams/dependency-direction.mmd).

## Phase 3 Backend Documents

The [Backend Documentation](backend/README.md) entry point records the verified source-level backend structure, 18 significant modules, boundaries, dependencies, request handling, services and domain logic, persistence, application state, shared infrastructure, errors, tests, risks, and four backend-specific diagrams.

Recommended backend reading order: [Backend Overview](backend/overview.md), [Module Catalog](backend/module-catalog.md), [Module Boundaries](backend/module-boundaries.md), [Dependency Map](backend/dependency-map.md), [Request Handling](backend/request-handling.md), [Services and Domain](backend/services-and-domain.md), [Persistence Access](backend/persistence-access.md), [Configuration and State](backend/configuration-and-state.md), [Shared Infrastructure](backend/shared-infrastructure.md), [Error Handling](backend/error-handling.md), [Testing Map](backend/testing-map.md), and [Backend Risks](backend/backend-risks.md).

For a feature change, select the owning document from the [Module Catalog](backend/module-catalog.md) or browse [individual module documents](backend/modules/). The module map, request lifecycle, dependency flow, and state composition diagrams are indexed from the backend entry point.

## Phase 4 Frontend Documents

The [Frontend Architecture](frontend/README.md) entry point records the one verified frontend application, 13 significant features, route and layout composition, shared components, state ownership, API integration, browser access cues, forms, styling, loading and failure behavior, Page Builder, configuration, tests, risks, and five frontend-specific diagrams.

Recommended frontend reading order: [Frontend Overview](frontend/overview.md), [Application Catalog](frontend/application-catalog.md), [Feature Catalog](frontend/feature-catalog.md), [Feature Boundaries](frontend/feature-boundaries.md), [Routing](frontend/routing.md), [Pages and Layouts](frontend/pages-and-layouts.md), [Component Architecture](frontend/component-architecture.md), [State Management](frontend/state-management.md), [API Client](frontend/api-client.md), [Authentication and Access](frontend/authentication-and-access.md), [Forms and Validation](frontend/forms-and-validation.md), [Styling and Design System](frontend/styling-and-design-system.md), [Loading, Errors, and Notifications](frontend/loading-errors-and-notifications.md), [Page Builder](frontend/page-builder.md), [Configuration and Build](frontend/configuration-and-build.md), [Testing Map](frontend/testing-map.md), and [Frontend Risks](frontend/frontend-risks.md).

For a feature change, select the owning document from the [Frontend Feature Catalog](frontend/feature-catalog.md) or browse [individual feature documents](frontend/features/). Use the [Application Map](frontend/diagrams/frontend-application-map.mmd), [Routing Flow](frontend/diagrams/frontend-routing-flow.mmd), [State Flow](frontend/diagrams/frontend-state-flow.mmd), [API Flow](frontend/diagrams/frontend-api-flow.mmd), and [Page Builder Flow](frontend/diagrams/page-builder-flow.mmd) for visual navigation.

## Phase 5 Database Documents

The [Database Architecture](database/README.md) entry point documents PostgreSQL and SQLx configuration, all 26 forward migrations, 51 application tables grouped into 18 significant entities, 108 intended active foreign keys consolidated into 55 significant relationship groups, 109 explicit index names, 118 intended effective RLS policies, transaction and lifecycle behavior, persistence mapping, fixtures, tests, risks, and five database diagrams. Runtime migration state remains unknown and the OKF documents are not an executable schema.

Recommended reading order: [Database Overview](database/overview.md), [Schema Catalog](database/schema-catalog.md), [Entity Catalog](database/entity-catalog.md), [Relationships](database/relationships.md), [Module Data Ownership](database/module-data-ownership.md), [Migrations](database/migrations.md), [Constraints and Indexes](database/constraints-and-indexes.md), [Transactions and Consistency](database/transactions-and-consistency.md), [Multi-Tenancy](database/multi-tenancy.md), [Lifecycle and Auditing](database/lifecycle-and-auditing.md), [Persistence Mapping](database/persistence-mapping.md), [Seeds and Fixtures](database/seeds-and-fixtures.md), [Database Testing](database/database-testing.md), and [Database Risks](database/database-risks.md).

Select one of the 18 entity groups from the [Entity Catalog](database/entity-catalog.md) and use the [Database Domain Map](database/diagrams/database-domain-map.mmd), [Entity Relationship Overview](database/diagrams/entity-relationship-overview.mmd), [Module Data Ownership Diagram](database/diagrams/module-data-ownership.mmd), [Tenant Isolation Flow](database/diagrams/tenant-isolation.mmd), or [Migration Lifecycle](database/diagrams/migration-lifecycle.mmd) for visual navigation.

## Using the Index

Start with [index.yaml](index.yaml). Its documents list records every current Phase 1 through Phase 5 file, verification commit, evidence paths, related documents, diagrams, and relevant marker IDs. Its `current_sections` and `planned_sections` distinguish completed knowledge areas from future work.

Paths in index.yaml are relative to the okf directory unless a field explicitly identifies a repository-relative evidence path.

## For Human Developers

1. Read the [Project Overview](project/overview.md).
2. Use the [Repository Map](project/repository-map.md) to locate implementation and tests.
3. Read the [Architecture Overview](architecture/overview.md) before changing a cross-cutting runtime or dependency boundary.
4. Read the [Backend Overview](backend/overview.md) and owning [module document](backend/module-catalog.md) before changing backend behavior.
5. Read the [Frontend Overview](frontend/overview.md) and owning [feature document](frontend/feature-catalog.md) before changing frontend behavior.
6. Read the [Database Overview](database/overview.md), schema catalog, and owning entity document before changing persistence.
7. Check the [Glossary](project/glossary.md) before introducing or redefining project terminology.
8. Follow the [Navigation Guide](project/navigation-guide.md) for common tasks.
9. Verify behavior against current code, migrations, configuration, and tests.
10. Record conflicts instead of silently treating a historical document as current.

## For AI Coding Agents

1. Start with okf/index.yaml.
2. Read okf/project/overview.md.
3. Use okf/project/repository-map.md to locate code.
4. Read okf/architecture/overview.md and the relevant boundary, component, dependency, flow, integration, risk, or decision document.
5. For backend work, read okf/backend/README.md, the catalog entry, and the owning module document.
6. For frontend work, read okf/frontend/README.md, the feature catalog entry, and the owning feature document.
7. For database work, read okf/database/README.md, schema-catalog.md, relationships.md, and the owning entity document.
8. Read relevant specialized OKF documents before modifying a subsystem.
9. Verify critical claims against source code and migrations.
10. Update related OKF documents when implementation changes invalidate them.
11. Never invent undocumented business rules.

If a specialized OKF document is still planned, use [Navigation Guide - Missing Documentation](project/navigation-guide.md#when-documentation-is-missing), then consult current source, tests, existing documentation, and Phase Zero evidence.

## Planned Knowledge Areas

| Target phase | Planned area |
| ---: | --- |
| 6 | API route and contract inventory, errors, realtime/provider contracts, and OpenAPI coverage |
| 7 | Authentication, authorization, tenant isolation, upload security, and threat model |
| 8 | Business rules, workflows, billing, delivery, and multi-tenancy |
| 9 | Built-in plugins, Marketplace, and extensibility |
| 10 | Development, testing, deployment, operations, observability, recovery, and troubleshooting |
| 11 | Mermaid selection, correction, parser/render validation, and diagram index |
| 12 | Full validation, traceability, and synchronization |

These later areas are registered as planned in index.yaml. Their files do not exist yet.

## Update Policy

Changes to architecture, module responsibilities, APIs, database structures, security rules, deployment processes, or major workflows must trigger a review of the related OKF documents.

During a review:

1. verify affected claims against the highest-priority current sources;
2. update last_verified_commit and last_verified_date only after verification;
3. update index.yaml when document status, evidence, relationships, or markers change;
4. preserve unresolved markers until evidence or an owner decision resolves them;
5. link existing authoritative material instead of creating competing copies;
6. keep historical phase records distinct from current implementation summaries.

## Phase Status

Phase 1 established the OKF entry point and project navigation layer. Phase 2 established verified system architecture, boundaries, components, dependency direction, runtime flows, integrations, risks, decisions, and five diagrams. Phase 3 established the verified backend module catalog, 18 module documents, structural guides, risk/test maps, and four backend-specific diagrams. Phase 4 established the verified frontend application and feature catalogs, 13 feature documents, shared architecture guides, risk/test maps, and five frontend-specific diagrams. Phase 5 is complete: it adds 16 database guides, 18 entity documents, and five database diagrams based on all 26 migrations and current persistence code. API, security, business, extension, operations, diagram-hardening, and final synchronization work remains planned for Phases 6 through 12.
