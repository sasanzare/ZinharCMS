---
okf_document_id: "backend-module-content-workflow"
title: "Content Types, Entries, and Workflow"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-006"
module_name: "Content Types, Entries, and Workflow"
module_paths:
  - "backend/src/routes/content.rs"
  - "backend/src/models/content.rs"
  - "backend/src/services/entry_validation.rs"
  - "backend/src/services/workflow.rs"
module_type: "Domain and HTTP module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/models/content.rs"
  - "backend/src/services/entry_validation.rs"
  - "backend/src/services/workflow.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "architecture/components.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "RESPONSIBILITY_OVERLAP RO-04"
  - "BUSINESS_RULE_UNVERIFIED BRU-03"
  - "MODULE_BOUNDARY_UNCLEAR MBU-02"
---

# Content Types, Entries, and Workflow

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-006` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/content.rs`; `backend/src/models/content.rs`; `backend/src/services/entry_validation.rs`; `backend/src/services/workflow.rs` |

## Responsibility

Verified responsibility: Manages content-type definitions and entries, validates dynamic data, applies editorial transitions, invokes plugin hooks and webhooks, and invalidates delivery cache after mutations.

Shared or inferred responsibility: Entry lifecycle orchestration depends directly on Delivery, Webhooks, Built-in Plugins, audit, quota, security, RBAC, and RLS.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/content.rs`
- `backend/src/models/content.rs`
- `backend/src/services/entry_validation.rs`
- `backend/src/services/workflow.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`content::router`, public route handlers, validation helpers, and workflow transition functions.

## Internal Structure

One route module contains content-type and entry DTOs, SQL, transactions, transitions, and cross-module side effects.

## Public and Internal Interfaces

Tenant HTTP router/DTOs, field-schema parsing and entry-validation functions, workflow transition helpers, plugin/webhook calls.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization, PostgreSQL, audit/quota/security, Delivery cache, Webhooks, and Built-in Plugins.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Content types, field-schema JSON, content entries, statuses, and audit/delivery/webhook side-effect data.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

SPA content administration, public Delivery reads, Comments references, plugin hooks, and Marketplace hook adapters.

## Data Concepts

Content types, field-schema JSON, content entries, statuses, and audit/delivery/webhook side-effect data.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Schema CRUD; entry validate/save; workflow transition; publish side effects and cache invalidation.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Validation uses `AppError::Validation`; workflow and persistence failures propagate through `AppError`.

## Configuration

Shared application, tenant, quota, and webhook configuration; no module-specific feature flag was found.

Secret values and local environment contents are intentionally excluded.

## Tests

Entry-validation and workflow modules contain unit tests; no colocated route test block was found in `content.rs`.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Content-type and entry ownership share one route file. Transaction and compensation semantics for post-mutation side effects require confirmation.

Relevant markers: `RESPONSIBILITY_OVERLAP RO-04`, `BUSINESS_RULE_UNVERIFIED BRU-03`, `MODULE_BOUNDARY_UNCLEAR MBU-02`.

## Related Documents

- [Backend Module Catalog](../module-catalog.md)
- [Module Boundaries](../module-boundaries.md)
- [Dependency Map](../dependency-map.md)
- [Testing Map](../testing-map.md)
- [Backend Risks](../backend-risks.md)
- [Architecture Components](../../architecture/components.md)
- [Architecture Boundaries](../../architecture/boundaries.md)
- [Architecture Dependency Model](../../architecture/dependency-model.md)
- [Backend Module Map](../diagrams/backend-module-map.mmd)
- [Backend Dependency Flow](../diagrams/backend-dependency-flow.mmd)
