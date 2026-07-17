---
okf_document_id: "backend-module-cms-webhooks"
title: "CMS Webhooks"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-011"
module_name: "CMS Webhooks"
module_paths:
  - "backend/src/routes/webhooks.rs"
  - "backend/src/services/webhooks.rs"
module_type: "Integration and HTTP module"
boundary_status: "OBSERVED"
primary_sources:
  - "backend/src/routes/webhooks.rs"
  - "backend/src/services/webhooks.rs"
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
  - "MODULE_BOUNDARY_UNCLEAR MBU-05"
  - "NEEDS_OWNER_CONFIRMATION NOC-09"
  - "PLANNED_NOT_IMPLEMENTED PNI-04"
---

# CMS Webhooks

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-011` |
| Module type | Integration and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OBSERVED` |
| Confidence | High |
| Source paths | `backend/src/routes/webhooks.rs`; `backend/src/services/webhooks.rs` |

## Responsibility

Verified responsibility: Manages tenant webhook subscriptions and delivery history, signs outbound payloads, validates target URLs, dispatches HTTP requests, and records one delivery attempt.

Shared or inferred responsibility: Content/Pages call the webhook service directly. JWT signing helper and RLS utilities are reused internally.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/webhooks.rs`
- `backend/src/services/webhooks.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`webhooks::router`, webhook CRUD/test handlers, and service event/dispatch functions.

## Internal Structure

Routes own subscription SQL and DTOs; service owns event types, target checks, signature generation, spawned dispatch, and delivery recording.

## Public and Internal Interfaces

Tenant webhook HTTP routes and internal dispatch/service functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization/RBAC/RLS, PostgreSQL, reqwest, JWT/HMAC helpers, `AppState`, and external receivers.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Webhook subscriptions, event filters/secrets, delivery status, response/error records.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Content and Pages lifecycle handlers, tenant administrators, and external webhook endpoints.

## Data Concepts

Webhook subscriptions, event filters/secrets, delivery status, response/error records.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Mutation to event/payload, subscription load, spawned signed request, and delivery-row update.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Invalid targets/signing/network failures are recorded or propagated depending on call; spawned delivery is non-durable.

## Configuration

No queue/retry configuration; general network and application state only.

Secret values and local environment contents are intentionally excluded.

## Tests

Webhook service contains URL, signature, and event tests. No durable retry or process-loss integration test exists.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Delivery guarantee, retry lifecycle, shutdown draining, and target-network policy need owner/security review.

Relevant markers: `MODULE_BOUNDARY_UNCLEAR MBU-05`, `NEEDS_OWNER_CONFIRMATION NOC-09`, `PLANNED_NOT_IMPLEMENTED PNI-04`.

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
