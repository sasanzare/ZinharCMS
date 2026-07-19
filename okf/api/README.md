---
okf_document_id: "api-readme"
title: "API Architecture and Contracts"
project: "ZinharCMS"
category: "api"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes"
  - "backend/src/middleware"
  - "backend/src/error.rs"
  - "backend/src/services/rbac.rs"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "api/overview.md"
  - "api/route-architecture.md"
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/README.md"
  - "frontend/api-client.md"
  - "database/entity-catalog.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
  - "api/diagrams/api-request-lifecycle.mmd"
  - "api/diagrams/authentication-flow.mmd"
  - "api/diagrams/authorization-flow.mmd"
  - "api/diagrams/frontend-api-contract-flow.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
  - "DOCUMENTATION_CODE_CONFLICT DCC-09"
  - "VERSIONING_BEHAVIOR_UNCLEAR VBU-01"
  - "ERROR_CONTRACT_UNCLEAR ECU-01"
---

# API Architecture and Contracts

## Scope

This Phase 6 section documents the implemented HTTP, WebSocket, and static-file surfaces of ZinharCMS. It maps route registration to request and response contracts, authentication, tenant selection, role checks, frontend consumers, backend owners, persistence entities, tests, and generated OpenAPI.

The current route registration and handler code remain authoritative. These documents describe the implementation; they do not define new endpoints, change middleware, or replace generated OpenAPI. The verified snapshot contains 168 registered handler-method endpoints, one public static-file subtree, 17 route groups, 21 endpoint families, and 141 typed frontend request functions.

## Reading Order

1. [Overview](overview.md)
2. [Route Architecture](route-architecture.md)
3. [Route Group Catalog](route-group-catalog.md)
4. [Endpoint Catalog](endpoint-catalog.md)
5. [Request Contracts](request-contracts.md)
6. [Response Contracts](response-contracts.md)
7. [Error Contracts](error-contracts.md)
8. [Authentication](authentication.md)
9. [Authorization](authorization.md)
10. [Tenant Context](tenant-context.md)
11. [Versioning and Compatibility](versioning-and-compatibility.md)
12. [Pagination, Filtering, and Sorting](pagination-filtering-and-sorting.md)
13. [Uploads, Downloads, and Streaming](uploads-downloads-and-streaming.md)
14. [OpenAPI Consistency](openapi-consistency.md)
15. [Frontend Contract Map](frontend-contract-map.md)
16. [Backend Module Map](backend-module-map.md)
17. [API Testing](api-testing.md)
18. [API Risks](api-risks.md)

Use the route-group catalog when the owning Axum module is known. Use the endpoint catalog and its family documents for task-oriented contract work.

## Verified Surface Summary

| Surface | Verified shape |
| --- | --- |
| Public HTTP handlers | 17 endpoints: system, public authentication, public delivery, and Stripe webhook |
| Authenticated non-tenant handlers | 12 endpoints: session, organization bootstrap/invitation acceptance, global beta administration, and built-in plugins |
| Tenant-protected handlers | 139 endpoints under the tenant middleware |
| Public static files | `GET`/`HEAD` behavior delegated by `ServeDir` below `/uploads`; not part of the 168 handler-method count |
| WebSocket | `GET /api/preview/{page_id}` upgrades after preview-specific authentication and tenant selection |
| Public versioned delivery | `/api/v1/*` only |
| Administrative API | Primarily unversioned `/api/*` |
| OpenAPI | Runtime JSON at `/openapi.json`; 149 of 168 registered handlers are included |
| Frontend | 141 typed request functions; every current frontend request path and method matches a registered handler |

## Contract Authority

When sources conflict, use route registration for method and path, handler extractors and DTOs for the wire contract, middleware composition for authentication and tenant requirements, role-check calls for authorization, migrations and queries for persistence behavior, tests for demonstrated behavior, and generated OpenAPI as a secondary projection. Record rather than erase disagreements.

## Known Contract Findings

- `OPENAPI_IMPLEMENTATION_CONFLICT OIC-01`: 19 registered handlers are absent from the Utoipa path list, and no OpenAPI security scheme or tenant-header contract is declared.
- `DOCUMENTATION_CODE_CONFLICT DCC-09`: `docs/API.md` names `/api/billing/webhook`; route registration uses `/api/billing/stripe/webhook`.
- `VERSIONING_BEHAVIOR_UNCLEAR VBU-01`: only delivery routes are under `/api/v1`; no repository-wide versioning or deprecation policy was found.
- `ERROR_CONTRACT_UNCLEAR ECU-01`: `AppError` has a stable JSON shape, but framework rejections, middleware short-circuits, timeout responses, static files, and WebSocket upgrade failures do not all use it.

## Change Rule

For an API change, update route registration, handler contracts, frontend types/calls, OpenAPI annotations, relevant tests, this endpoint catalog, the owning group and family documents, and any affected backend or database cross-reference. Do not treat an OpenAPI annotation alone as proof that a route is reachable.
