---
okf_document_id: "api-response-contracts"
title: "API Response Contracts"
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
  - "backend/src/error.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/endpoint-catalog.md"
  - "api/error-contracts.md"
  - "api/frontend-contract-map.md"
uncertainty_markers:
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-01"
---

# API Response Contracts

## JSON Success Model

Most handlers return `Json<T>` or `Result<Json<T>, AppError>`. There is no global success envelope. A response may therefore be an object, a raw array, or a domain-specific list wrapper.

| Shape | Examples |
| --- | --- |
| Single resource/object | Content type, entry, page, organization, installation, usage |
| Raw array | Content types, comments in several operations, organization members, plugins |
| Page wrapper | `{ data, page, per_page }` for content entries, pages, media, and public delivery lists |
| Action result | Logout `{ revoked }`, workflow resource after transition, checkout/session object |
| Headers plus JSON | Authentication sets or clears the refresh cookie while returning JSON |

The frontend `request<T>` helper always parses a successful response with `response.json()`. Current frontend-covered endpoints return JSON. New `204 No Content`, text, XML, binary, or streaming calls require a different frontend parsing path.

## Status Codes

The dominant success status is 200. Explicit 201 behavior exists for selected Marketplace creation operations, including product review creation, abuse report creation, installation, template import, and payout requests. Marketplace checkout can return 200 for a free purchase path or 201 for a created paid checkout path. Many other create handlers return 200 rather than 201.

No uniform `Location` header or resource-creation status policy was found. Status expectations in endpoint-family documents should be verified against explicit `StatusCode` tuples and not inferred from HTTP method names.

## Non-JSON Responses

- `/api/v1/sitemap.xml` returns XML.
- `/api/v1/robots.txt` returns text.
- `/api/preview/{page_id}` upgrades to WebSocket and then emits text messages containing page JSON.
- `/uploads/*` delegates file responses and metadata headers to `ServeDir`.
- Middleware and framework-generated failures can return non-`ErrorBody` responses.

## Field and Time Conventions

DTO fields use `snake_case`. UUIDs serialize as strings. Chrono timestamps serialize in RFC 3339-compatible form through Serde. JSON-valued domain fields remain structurally open where the Rust type is `serde_json::Value`.

## Stability

Response DTOs are Rust structs, while frontend contracts are handwritten TypeScript interfaces. There is no generated frontend client or schema-parity test. `RESPONSE_CONTRACT_UNCLEAR RSCU-01` applies where a framework response, dynamic `Value`, provider result, or branch-specific status prevents one closed response schema.
