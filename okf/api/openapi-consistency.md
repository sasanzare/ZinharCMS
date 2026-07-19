---
okf_document_id: "api-openapi-consistency"
title: "OpenAPI Consistency"
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
  - "backend/src/routes/mod.rs"
  - "backend/src/routes"
related_documents:
  - "api/endpoint-catalog.md"
  - "api/authentication.md"
  - "api/tenant-context.md"
  - "api/versioning-and-compatibility.md"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# OpenAPI Consistency

## Generation Model

`GET /openapi.json` serializes the Utoipa `ApiDoc` assembled in `backend/src/routes/mod.rs`. Handler annotations describe operations and component schemas. Route registration remains authoritative because the Utoipa `paths(...)` list is separate from Axum router composition.

## Verified Coverage

The snapshot contains 168 registered handler-method endpoints. Utoipa includes 149 handler operations. There are no Utoipa-only handler operations that lack a registered runtime method/path. The following 19 registered handlers are missing from the OpenAPI path list:

| Method | Path |
| --- | --- |
| GET | `/api/v1/sitemap.xml` |
| GET | `/api/v1/robots.txt` |
| GET | `/api/marketplace/catalog` |
| GET | `/api/marketplace/catalog/{listing_slug}` |
| GET | `/api/marketplace/creator` |
| POST | `/api/marketplace/creator` |
| PATCH | `/api/marketplace/creators/{creator_id}/verification` |
| GET | `/api/marketplace/listings` |
| POST | `/api/marketplace/listings` |
| PUT | `/api/marketplace/listings/{listing_id}` |
| POST | `/api/marketplace/listings/{listing_id}/submit` |
| POST | `/api/marketplace/listings/{listing_id}/versions/upload` |
| GET | `/api/marketplace/listings/{listing_id}/submissions` |
| GET | `/api/marketplace/review/queue` |
| GET | `/api/marketplace/review/events` |
| GET | `/api/marketplace/review/reports` |
| PATCH | `/api/marketplace/review/submissions/{submission_id}` |
| POST | `/api/marketplace/review/listings/{listing_id}/moderation` |
| GET | `/openapi.json` |

The static `/uploads` service is also intentionally outside the handler count and OpenAPI.

## Security Conflict

No OpenAPI security scheme was found, and operations do not declare bearer authentication or the `X-Organization-Id` tenant header. As a result, generated clients and interactive documentation cannot infer the verified public/authenticated/tenant zones. This and the 19 missing operations are `OPENAPI_IMPLEMENTATION_CONFLICT OIC-01`.

## Contract Quality Limits

Annotations often document primary success responses but do not establish a complete error matrix. Framework rejections, middleware responses, WebSocket behavior, static files, provider signature requirements, multipart details, and branch-dependent statuses are not represented uniformly.

## Consistency Procedure

For each API change, compare the Axum method/path set with `ApiDoc::paths`, confirm DTOs appear in `components`, add security and tenant requirements where applicable, verify success and error statuses, and exercise `/openapi.json` in a router-level test. A future automated route/OpenAPI parity test is recommended; none exists now.
