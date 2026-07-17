---
okf_document_id: "frontend-api-client"
title: "Frontend API Client"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
primary_sources:
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/pages"
  - "frontend/src/vite-env.d.ts"
related_documents:
  - "frontend/state-management.md"
  - "frontend/authentication-and-access.md"
  - "frontend/loading-errors-and-notifications.md"
  - "frontend/configuration-and-build.md"
  - "backend/request-handling.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "DUPLICATED_CONTRACT DC-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
---

# Frontend API Client

## Client Boundary

`VERIFIED`: All direct frontend `fetch` behavior is centralized in `frontend/src/services/api.ts`. Pages call methods on one exported `api` object. The client is handwritten and uses types from `frontend/src/types/api.ts`; no generated client, OpenAPI runtime, GraphQL client, query cache, or feature-specific transport module was found.

This document describes frontend integration structure. Detailed method/path contracts belong to Phase 6.

## Base URL and Request Construction

- `VITE_API_URL` supplies the backend base URL at build time.
- A local development URL is the source fallback when the variable is absent.
- JSON requests set `Content-Type: application/json`.
- Multipart operations pass `FormData` and allow the browser to set its content boundary.
- Requests use `credentials: "include"`.
- Authenticated calls attach `Authorization: Bearer ...` when a token exists.
- Tenant calls additionally attach `X-Organization-Id` when an active ID exists.

The `auth` request option controls both bearer and organization behavior; individual API methods determine whether it is enabled.

## Response and Error Model

Successful responses are parsed with `response.json()` and cast to the generic expected type. There is no runtime schema check. Non-success responses attempt to parse a JSON `message` or `error`, fall back to `statusText`, and throw `ApiError` with status and message.

No verified handling exists for an empty successful body, a non-JSON successful body, structured field errors, standardized problem details, or response contract versioning. Pages generally catch `ApiError` and render its message through `StatusBadge`.

## Client Capability Groups

| Group | Frontend responsibility | Primary consumers |
|---|---|---|
| System | API info, health, readiness | Dashboard, Shell, Settings |
| Authentication | Register, login, refresh, logout, current user | Auth, Shell, Settings |
| Billing | Plans, usage, plan change, checkout, portal, rebuild | Billing, Dashboard |
| Beta | Dashboard, feedback, blockers, participants, product dashboard | Beta |
| Organizations | List/detail, members, invites, domains, limits, audit, email, alerts, workspaces | Organization, Shell, Workspace Redirect |
| Content types | List and model CRUD | Content Types, Entries, Workflow, Dashboard |
| Entries | List, CRUD, workflow transitions | Entries, Workflow, Dashboard |
| Marketplace | Catalog, creator, submissions, install lifecycle, runtime, finance, feedback, analytics | Marketplace, Pages |
| Media | List, multipart upload, update, delete | Media, Dashboard |
| Pages | List, CRUD, workflow, versions, restore | Pages, Workflow, Dashboard |
| Components | Component registry | Pages |
| Marketplace adapters | Component/template/hook host adapters | Pages, Marketplace |
| Comments | List, create, update | Workflow |
| Plugins | List and update | Workflow |
| Webhooks | List, CRUD, test | Settings |

## Uploads and Browser Exits

Media and Marketplace package-version uploads use `FormData`. Billing checkout and portal methods return URLs that the page assigns to `window.location`. Pages constructs a WebSocket preview URL from the API base URL and copies it to the clipboard; the frontend does not itself open that socket in `PagesPage`.

## Refresh and Retry

The auth group exposes a refresh method, but no automatic response interceptor, `401` refresh-and-retry sequence, retry/backoff policy, or global session-expiry handler was found. Requests do not use `AbortController`, timeouts, request IDs, or client-side tracing.

## Contract Ownership

### ACU-01 and DC-01

The Rust backend owns reachable runtime contracts. `types/api.ts` manually duplicates the frontend view of many request/response shapes. TypeScript compilation proves only internal frontend consistency; it cannot prove the server returns the asserted shape. No shared schema package, code generation, contract fixture suite, or runtime validation closes this boundary.

This is both `API_CONTRACT_UNCLEAR ACU-01` and `DUPLICATED_CONTRACT DC-01`, corresponding to architecture marker `DEPENDENCY_DIRECTION_UNCLEAR DDU-03`.

## Authentication and Tenant Context

The API module keeps token and organization variables synchronized through setters called by the Zustand store. Because header construction uses these module variables, bypassing store actions can create UI/transport disagreement. Missing headers do not produce a frontend preflight error; backend responses determine the failure.

Frontend header attachment does not establish authorization. See [Authentication and Access](authentication-and-access.md) and [Backend Request Handling](../backend/request-handling.md).

## Change Guidance

- Add methods to the central client unless a deliberate transport boundary is documented.
- Update browser types and backend contracts together; add contract tests when practical.
- Preserve multipart behavior by not setting a manual content type for `FormData`.
- Define empty-body, cancellation, retry, session-expiry, and field-error semantics before centralizing them.
- Never infer endpoint reachability or authorization from a client method alone; verify router composition and backend enforcement.

## Related Documents

- [State Management](state-management.md)
- [Authentication and Access](authentication-and-access.md)
- [Frontend API Flow](diagrams/frontend-api-flow.mmd)
- [Backend Request Handling](../backend/request-handling.md)
- [Architecture Dependency Model](../architecture/dependency-model.md)

