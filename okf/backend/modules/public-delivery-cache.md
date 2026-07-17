---
okf_document_id: "backend-module-public-delivery-cache"
title: "Public Delivery and Cache"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-010"
module_name: "Public Delivery and Cache"
module_paths:
  - "backend/src/routes/delivery.rs"
  - "backend/src/services/cache.rs"
module_type: "HTTP and infrastructure module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/delivery.rs"
  - "backend/src/services/cache.rs"
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
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-05"
---

# Public Delivery and Cache

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-010` |
| Module type | HTTP and infrastructure module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/delivery.rs`; `backend/src/services/cache.rs` |

## Responsibility

Verified responsibility: Provides unauthenticated published content/page/settings/navigation delivery, sitemap/robots generation, Redis cache reads/fills, and cache invalidation functions used by mutations.

Shared or inferred responsibility: Content and Pages call route-owned invalidation functions, creating reverse feature-to-route coupling. Delivery reads several domains directly.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/delivery.rs`
- `backend/src/services/cache.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`delivery::router`, public handlers, `invalidate_content_cache`, and `invalidate_page_cache`.

## Internal Structure

The route module resolves the public organization and performs RLS-aware reads; cache service wraps JSON get/set/invalidation.

## Public and Internal Interfaces

Public `/api/v1` and text delivery routes plus route-owned cache invalidation functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: PostgreSQL/RLS, Redis/cache service, slug validation, Content/Pages data, Config/AppState.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Published entries/pages, public settings/navigation, site URL, cache keys/values.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Public site consumers; Content and Pages mutation handlers; health/readiness indirectly depend on Redis.

## Data Concepts

Published entries/pages, public settings/navigation, site URL, cache keys/values.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Public request to fixed organization lookup, cache lookup, database fallback, serialization, and cache fill.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Cache read can fall back to DB; delivery SQL failures use `AppError`; invalidation is best effort.

## Configuration

Redis URL and application base/site data; cache TTL is a service constant.

Secret values and local environment contents are intentionally excluded.

## Tests

`routes/delivery.rs` contains three static/unit tests. No deployed multi-domain delivery test evidence was found.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Fixed `default` organization routing and route-owned invalidation interface remain unresolved.

Relevant markers: `NEEDS_OWNER_CONFIRMATION NOC-01`, `IMPLEMENTATION_STATUS_UNCLEAR ISU-01`, `DEPENDENCY_DIRECTION_UNCLEAR DDU-05`.

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
