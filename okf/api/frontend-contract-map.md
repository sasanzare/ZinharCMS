---
okf_document_id: "api-frontend-contract-map"
title: "Frontend API Contract Map"
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
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
  - "backend/src/routes"
related_documents:
  - "frontend/api-client.md"
  - "frontend/authentication-and-access.md"
  - "api/endpoint-catalog.md"
  - "api/versioning-and-compatibility.md"
related_diagrams:
  - "api/diagrams/frontend-api-contract-flow.mmd"
uncertainty_markers:
  - "FRONTEND_BACKEND_CONTRACT_CONFLICT FBCC-01"
  - "DUPLICATED_CONTRACT DC-01"
---

# Frontend API Contract Map

## Shared Client

`frontend/src/services/api.ts` owns the base URL, request construction, authentication and tenant headers, JSON parsing, and domain-grouped API methods. `frontend/src/types/api.ts` owns most response and request interfaces. The types are manually maintained rather than generated from Rust DTOs or OpenAPI (`DUPLICATED_CONTRACT DC-01`).

The verified client defines 141 request functions. Every function's current HTTP method and normalized path matches a registered backend handler. This means `FRONTEND_BACKEND_CONTRACT_CONFLICT FBCC-01` is not active for method/path reachability in the snapshot; it remains the marker for future DTO, auth, status, or route drift.

## Transport Behavior

- The base URL comes from frontend configuration.
- Requests include credentials.
- Calls marked `auth: true` attach the local access token and, when selected, `X-Organization-Id`.
- Successful responses are always parsed as JSON.
- Failed responses become `ApiError(status, message)`.
- There is no automatic refresh, replay, retry, backoff, cancellation policy, or generated schema validation.

## Frontend Coverage by Domain

The client covers authentication, billing, beta operations, organizations, content types and entries, pages and components, media, comments, CMS webhooks, built-in plugins, and all major Marketplace domains. Individual endpoint-family documents name the owning client namespaces.

The following 27 registered handlers have no direct shared-client request function:

| Domain | Backend-only or indirectly consumed handlers |
| --- | --- |
| System/auth | `GET /api/auth`, `GET /openapi.json` |
| Provider callback | `POST /api/billing/stripe/webhook` |
| Public delivery | All eight `/api/v1/*` handlers |
| Detail helpers | `GET /api/comments/{id}`, `GET /api/content-types/{id}`, `GET /api/entries/{type_slug}/{id}`, `GET /api/media/{id}`, `GET /api/pages/{id}`, `GET /api/pages/slug/{slug}`, `GET /api/webhooks/{id}`, `GET /api/plugins/{plugin_key}` |
| Components/preview | Component create/get/update/delete and `GET /api/preview/{page_id}` |
| Marketplace administration | Creator verification, revenue ledger, payout-account verification |

Public pages may use browser navigation or other rendering paths rather than the administration client. WebSocket preview is constructed outside the normal JSON request helper.

## Contract Review Map

| Change | Frontend evidence to review |
| --- | --- |
| Method/path | Domain method in `services/api.ts` and all call sites |
| Request field | API method parameter/payload and related form/state type |
| Response field | Generic `request<T>` argument and `types/api.ts` interface |
| Authentication/tenant zone | `auth` option, local token/organization selection, protected UI |
| Non-JSON or 204 response | Shared parser or a specialized transport implementation |
| Error recovery | `ApiError` consumers and page-level state handling |

## High-Risk Drift Areas

Dynamic `serde_json::Value` payloads, Marketplace compatibility/permission JSON, branch-specific status codes, manual enums, multipart bodies, and non-JSON surfaces have the weakest static parity. Use a real router contract test plus a frontend type/call review before declaring compatibility.
