---
okf_document_id: "okf-readme"
title: "ZinharCMS Open Knowledge Format"
project: "ZinharCMS"
category: "knowledge-base"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "17e69e266c558c8568ec65524560d52d7cb89d4c"
last_verified_date: "2026-07-17"
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
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
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
| DOCUMENTATION_CODE_CONFLICT | Documentation and current implementation disagree |
| PLANNED_NOT_IMPLEMENTED | A source identifies future work, but current implementation evidence is absent |
| PROPOSED_NOT_IMPLEMENTED | A target architecture decision is documented but is not implemented |
| IMPLEMENTATION_STATUS_UNCLEAR | Some evidence exists, but the complete capability cannot be verified |
| ARCHITECTURAL_BOUNDARY_UNCLEAR | Component or responsibility ownership is not consistently defined by current structure |
| DEPENDENCY_DIRECTION_UNCLEAR | Source or contract dependencies cross the apparent layer direction |

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

## Using the Index

Start with [index.yaml](index.yaml). Its documents list records every current Phase 1 and Phase 2 file, verification commit, evidence paths, related documents, diagrams, and relevant marker IDs. Its `current_sections` and `planned_sections` distinguish completed knowledge areas from future work.

Paths in index.yaml are relative to the okf directory unless a field explicitly identifies a repository-relative evidence path.

## For Human Developers

1. Read the [Project Overview](project/overview.md).
2. Use the [Repository Map](project/repository-map.md) to locate implementation and tests.
3. Read the [Architecture Overview](architecture/overview.md) before changing a cross-cutting runtime or dependency boundary.
4. Check the [Glossary](project/glossary.md) before introducing or redefining project terminology.
5. Follow the [Navigation Guide](project/navigation-guide.md) for common tasks.
6. Verify behavior against current code, migrations, configuration, and tests.
7. Record conflicts instead of silently treating a historical document as current.

## For AI Coding Agents

1. Start with okf/index.yaml.
2. Read okf/project/overview.md.
3. Use okf/project/repository-map.md to locate code.
4. Read okf/architecture/overview.md and the relevant boundary, component, dependency, flow, integration, risk, or decision document.
5. Read relevant specialized OKF documents before modifying a subsystem.
6. Verify critical claims against source code.
7. Update related OKF documents when implementation changes invalidate them.
8. Never invent undocumented business rules.

If a specialized OKF document is still planned, use [Navigation Guide - Missing Documentation](project/navigation-guide.md#when-documentation-is-missing), then consult current source, tests, existing documentation, and Phase Zero evidence.

## Planned Knowledge Areas

| Target phase | Planned area |
| ---: | --- |
| 3 | Backend capability modules and dependency map |
| 4 | Frontend routing, state, API client, builder, localization, and accessibility |
| 5 | Database schema, relationships, RLS, constraints, migrations, and retention |
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

Phase 1 established the OKF entry point, project overview, repository map, glossary, navigation guide, source register, and machine-readable index. Phase 2 establishes the verified system architecture, boundaries, components, dependency direction, runtime flows, integration points, risks, decisions, and five focused Mermaid diagrams. Detailed module, frontend, API, database, security, business, extension, and operations documentation remains planned for Phases 3 through 12.
