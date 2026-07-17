---
okf_document_id: "architecture-readme"
title: "System Architecture"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "mixed"
last_verified_commit: "17e69e266c558c8568ec65524560d52d7cb89d4c"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/state.rs"
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "frontend/src/services/api.ts"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "README.md"
  - "project/overview.md"
  - "project/repository-map.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/README.md"
  - "architecture/decisions/decision-register.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/backend-request-flow.mmd"
  - "architecture/diagrams/frontend-backend-flow.mmd"
  - "architecture/diagrams/dependency-direction.mmd"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-06"
  - "DOCUMENTATION_CODE_CONFLICT DCC-04"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# System Architecture

## Purpose

This directory describes the architecture implemented by the verified repository state. It classifies the system, identifies runtime and ownership boundaries, records dependency direction, traces representative flows, registers external integrations, and preserves architectural risks and inferred decisions.

The architecture is a client-server application composed of a separately built React single-page application and one Rust/Axum backend process. The backend is a modular monolith: route, middleware, service, plugin, and infrastructure modules share one crate, process, deployment unit, and application state. PostgreSQL is the system of record, Redis provides cache and rate-limit support, and the tracked production-like Compose file also uses local filesystem storage.

## Evidence Rules

- Current code, migrations, manifests, and executable configuration are stronger evidence than narrative documents or historical diagrams.
- `observed` means directly established by the repository.
- `inferred` means the conclusion follows from implementation or configuration but is not explicitly governed.
- `mixed` means the document contains both observed and clearly labeled inferred material.
- `proposed` is reserved for an explicitly proposed architecture that is not implemented.
- `unclear` means the available evidence does not establish the architecture.

No document in this directory changes application behavior or replaces implementation sources.

## Architecture Status

| Classification | Phase 2 status |
|---|---|
| `OBSERVED` | Client-server split, one backend process, major runtime dependencies, router groups, shared state, and tracked container configuration are directly verified |
| `PARTIALLY_DOCUMENTED` | Detailed module, schema, API, security, business, plugin, and operations references remain assigned to later phases |
| `INFERRED` | Modular-monolith classification, layer descriptions, and reconstructed decisions are inferred from repeated implementation structure |
| `PROPOSED` | No target architecture is proposed by Phase 2 |
| `NEEDS_OWNER_CONFIRMATION` | Production topology, public tenant routing, storage operations, durability guarantees, and decision rationale remain unresolved |

Current implementation behavior remains authoritative. A diagram, inferred decision, or narrative layer name must never override current code, migrations, manifests, configuration, or verified runtime evidence.

## Using and Verifying This Section

Developers should start with the overview, select the relevant boundary or flow, follow its exact evidence paths, inspect current imports and runtime calls, and review the linked risk and decision before changing a cross-cutting concern. Re-run link, metadata, path, and Mermaid checks whenever an architecture file changes.

AI coding agents should use `okf/index.yaml` as the registry, preserve uncertainty markers, verify every important claim against primary sources, avoid inventing clean layers that the code does not enforce, and update the affected architecture files when a major dependency, integration, runtime component, or boundary changes.

## Reading Order

1. [Architecture Overview](overview.md)
2. [System Boundaries](boundaries.md)
3. [Components and Responsibilities](components.md)
4. [Dependency Model](dependency-model.md)
5. [Runtime Flows](runtime-flows.md)
6. [Integration Points](integration-points.md)
7. [Architecture Risks](architecture-risks.md)
8. [Architecture Decisions Guide](decisions/README.md)
9. [Architecture Decision Register](decisions/decision-register.md)

## Diagram Index

| Diagram | Scope |
|---|---|
| [System Context](diagrams/system-context.mmd) | People, client, backend, data stores, storage, and external providers |
| [Container View](diagrams/container-view.mmd) | Repository-defined runtime containers and backend internal composition |
| [Backend Request Flow](diagrams/backend-request-flow.mmd) | Public, authenticated, and tenant-protected HTTP handling |
| [Frontend-Backend Flow](diagrams/frontend-backend-flow.mmd) | SPA routing, state, central API client, backend, and data dependencies |
| [Dependency Direction](diagrams/dependency-direction.mmd) | Intended source direction and verified reverse or bidirectional coupling |

## Current Architecture Classification

| Dimension | Classification | Confidence |
|---|---|---|
| System style | Client-server web application | High |
| Backend style | Modular monolith in one Rust crate and process | High |
| Frontend style | React single-page application | High |
| Primary persistence | PostgreSQL | High |
| Supporting state | Redis and process-local memory | High |
| Binary asset storage | Local filesystem in repository-defined configuration | High |
| External integration style | Synchronous HTTP plus spawned in-process webhook tasks | High |
| Actual production topology | `UNKNOWN` | Low |

## Scope Limits

This phase does not define a target architecture, create new services, change dependency direction, change APIs, add a queue, alter storage, modify deployment configuration, or resolve owner decisions. Detailed module, database, API, security, workflow, extension, and operations documentation remains assigned to later OKF phases.
