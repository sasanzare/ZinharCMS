---
okf_document_id: "architecture-risks"
title: "Architecture Risks"
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
  - "backend/src/state.rs"
  - "backend/src/routes"
  - "backend/src/middleware"
  - "backend/src/services"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
  - "docker-compose.prod.yml"
  - "docs/diagrams/ARCHITECTURE_AUDIT.md"
  - "okf-bootstrap/04-documentation-audit.md"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/decisions/decision-register.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/dependency-direction.mmd"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "UNKNOWN U-08"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-02"
  - "NEEDS_OWNER_CONFIRMATION NOC-06"
  - "NEEDS_OWNER_CONFIRMATION NOC-09"
  - "NEEDS_OWNER_CONFIRMATION NOC-14"
  - "DOCUMENTATION_CODE_CONFLICT DCC-01"
  - "DOCUMENTATION_CODE_CONFLICT DCC-10"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-02"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-03"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-02"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-03"
  - "PLANNED_NOT_IMPLEMENTED PNI-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# Architecture Risks

## Rating Method

Risk is rated from the verified repository architecture, not from observed production incidents. `HIGH` means a plausible failure has broad security, tenant, durability, or delivery impact. `MEDIUM` means the issue increases change cost, scaling risk, or operational ambiguity. `LOW` means localized impact. Ratings should be revised only with implementation or operational evidence.

## Risk Register

| ID and title | Description and evidence | Affected components | Likelihood | Impact | Severity | Current mitigation | Follow-up phase | Owner confirmation | Status |
|---|---|---|---|---|---|---|---|---|---|
| AR-001 — Blurred route/service/persistence boundaries | Route modules combine HTTP, direct SQL, policy, files, cache, and providers; `backend/src/routes` and ABU-01 | Backend routes, services, data access | High | Cross-cutting rule bypass, duplication, and change regression | `HIGH` | Tenant middleware, RLS helpers, reusable services | Phase 3 module ownership | Required for target boundary policy | `OPEN` |
| AR-002 — Bidirectional and reverse dependencies | Middleware/services share context/function coupling and health imports a route type; DDU-01/DDU-02 | Middleware, JWT/quota/rate/RLS/health services, route types | High | Increased change radius and difficult layer extraction | `MEDIUM` | Coupling is registered and source-level, not a package cycle | Phase 3 dependency map | Required before redefining ownership | `OPEN` |
| AR-003 — Frontend/backend contract drift | TypeScript types manually mirror Rust route behavior without generated/shared schema; DDU-03 | SPA API client/types and backend routes | Medium | Independently compiling incompatible requests/responses | `HIGH` | One central API client; coordinated validation expectation | Phases 4 and 6 | Required for future contract authority | `OPEN` |
| AR-004 — Process-local preview state | Broadcast channels live in `AppState` and are not shared through Redis | Backend replicas, page editor, preview clients | Medium if scaled beyond one process | Lost or partitioned realtime updates | `MEDIUM` | Works within one process | Phase 10 runtime topology | Required for scaling/session-affinity policy | `OPEN` |
| AR-005 — Non-durable external side effects | CMS webhooks use one spawned in-process attempt and no queue/retry worker; `backend/src/services/webhooks.rs`, ABU-03 | Backend process, webhook subscribers, delivery records | Medium | Accepted mutation can outlive or lose notification work | `HIGH` | Timeout, HMAC signature, delivered/failed record | Phases 8 and 10 | Required under NOC-09 | `OPEN` |
| AR-006 — Non-atomic file and database state | Media/package file I/O occurs beside PostgreSQL updates; media and Marketplace routes | Upload filesystem, media/package rows, backend instances | Medium | Orphaned/missing files or inconsistent replicas | `HIGH` | Organization paths, validation, tracked volume | Phases 5, 7, and 10 | Required under NOC-02 | `OPEN` |
| AR-007 — Distributed tenant enforcement | Middleware, ownership checks, direct SQL, and RLS helpers jointly enforce isolation | All tenant routes, services, PostgreSQL | Medium | Cross-tenant data exposure if a path omits required context | `HIGH` | Membership middleware, RBAC checks, RLS policies/helpers | Phases 5 and 7 | Required for isolation assurance | `OPEN` |
| AR-008 — Configuration authority ambiguity | Two environment templates and incomplete production-like provider wiring; config and Compose files | Backend config, integrations, deployment operators | Medium | Environment drift or silently disabled/miswired provider | `MEDIUM` | Central `Config` parser and tracked templates | Phase 10 configuration/operations | Required under NOC-14/NOC-06 | `OPEN` |
| AR-009 — Redis criticality and broad key scan | Cache fails open, rate/readiness fail closed, prefix invalidation uses `KEYS`; cache/rate services | Delivery, tenant requests, readiness, Redis | Medium | Availability coupling or latency under large keyspace | `MEDIUM` | Explicit per-use failure handling and DB fallback for delivery | Phase 10 capacity/operations | Required for SLO and capacity assumptions | `OPEN` |
| AR-010 — Documentation and diagram drift | DCC-01 through DCC-10 conflict with builder, Marketplace finance, API, and lifecycle code | Contributors, AI agents, architecture decisions | High | Incorrect changes or operational assumptions | `MEDIUM` | Source-first policy, source register, verified Phase 2 diagrams | Phase 11 and Phase 12 | Not required to preserve known conflicts; required to resolve intent | `OPEN` |
| AR-011 — Fixed public delivery organization | Delivery selects active slug `default`; `routes/delivery.rs`, U-08/NOC-01/ISU-01 | Public delivery, organizations, domains | High for multi-tenant public use | Incorrect tenant content selection or incomplete public routing | `HIGH` | Limitation is explicitly marked; no unsupported completion claim | Phase 8 business/tenancy | Required under NOC-01 | `OPEN` |
| AR-012 — Known startup administrator credential | An empty users table creates a hard-coded default administrator identity and password; `backend/src/main.rs` | Authentication, new environments, default organization | Environment-dependent | Unauthorized administrator access if exposed unchanged | `HIGH` | The password is hashed; deployment exposure remains unknown | Phase 7 security | Required for bootstrap policy | `OPEN` |
| AR-013 — Unknown production/release topology | Compose/Dockerfiles are reference evidence only; U-01/NOC-06/ISU-03 | All runtime components and operations | High uncertainty, not an incident probability | Invalid availability, security, or scaling assumptions | `MEDIUM` | All production claims remain marked unknown | Phase 10 operations | Required | `OPEN` |
| AR-014 — Concentrated modules and selective boundary tests | Broad route modules and frontend pages carry many paths; backend tests are colocated and frontend has three page tests | Backend feature routes, frontend pages, CI | Medium | Regressions across high-coupling boundaries | `MEDIUM` | Existing CI and selected tests | Phases 3 and 4 | Not required for observed risk; required for coverage target | `OPEN` |
| AR-015 — In-process plugin isolation | Built-in Rust plugins execute inside backend trust/resource boundary; plugin registry and SEO plugin | Backend process, content routes, built-in plugins | Low with current trusted built-ins | Plugin defect can affect request/process behavior | `MEDIUM` | Only built-in trusted plugins run; Marketplace package code is not executed | Phase 9 extensions and Phase 7 security | Required before accepting third-party in-process code | `OPEN` |

## Cross-Cutting Themes

### Isolation

The highest-impact architecture concern is not the absence of tenancy controls; several controls exist. The concern is that enforcement responsibility is distributed across multiple layers and direct query paths. Any new query, cache key, filesystem path, WebSocket, or integration must carry organization context deliberately.

### Durability

PostgreSQL is durable, but preview state, spawned webhook tasks, and filesystem/database coordination do not inherit database durability. A successful mutation must not be assumed to imply successful delivery of every side effect.

### Contract Ownership

Backend route types, service types, middleware context types, frontend API types, historical docs, and diagrams can all describe overlapping contracts. Current behavior remains in code; a future contract-authority decision should reduce duplication without being invented in Phase 2.

### Operational Evidence

Repository configuration demonstrates how the system can run. It does not demonstrate how it does run in production. Risk ratings for availability and scale remain provisional until topology, traffic, incidents, backups, and ownership are supplied.

## Priority Follow-Up

1. Treat `AR-012`, `AR-007`, `AR-005`, `AR-006`, `AR-011`, and `AR-003` as review gates for related changes.
2. In Phase 3, map backend capability ownership and direct infrastructure access without changing the architecture.
3. In Phases 5 and 7, validate tenant isolation, file exposure, token transport, and RLS coverage.
4. In Phase 10, obtain production, recovery, observability, and delivery evidence before revising operational risks.

## Evidence Documents

- [Architecture Overview](overview.md)
- [System Boundaries](boundaries.md)
- [Components and Responsibilities](components.md)
- [Dependency Model](dependency-model.md)
- [Runtime Flows](runtime-flows.md)
- [Integration Points](integration-points.md)
- [Architecture Decision Register](decisions/decision-register.md)
