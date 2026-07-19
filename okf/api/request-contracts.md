---
okf_document_id: "api-request-contracts"
title: "API Request Contracts"
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
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/endpoint-catalog.md"
  - "api/pagination-filtering-and-sorting.md"
  - "api/uploads-downloads-and-streaming.md"
  - "api/authentication.md"
uncertainty_markers:
  - "REQUEST_CONTRACT_UNCLEAR RCU-01"
---

# API Request Contracts

## Contract Authority

For a reachable endpoint, combine the registered method/path with its handler extractors and request DTO. Utoipa annotations are supporting documentation, not routing authority. The endpoint catalog identifies every handler; endpoint-family documents summarize their DTOs and special rules.

## Request Channels

| Channel | Implementation pattern | Contract behavior |
| --- | --- | --- |
| Path | `Path<T>` | UUIDs are rejected by extraction when typed as `Uuid`; string slugs receive handler validation where implemented |
| Query | `Query<T>` | Serde field names are used; defaults and bounds are handler-specific |
| JSON body | `Json<T>` | Deserialization occurs before handler logic; domain validation follows in the handler or service |
| Authentication | `Authorization: Bearer <token>` | Required by authenticated and tenant-protected routers |
| Tenant selection | `X-Organization-Id: <uuid>` | Required by normal tenant-protected HTTP requests |
| Cookie | `zinhar_refresh_token` | Accepted by refresh/logout; scoped to `/api/auth` |
| Raw body | `Bytes` | Used by refresh/logout token parsing and Stripe signature verification |
| Multipart | `Multipart` | Used by media upload and Marketplace package-version upload |
| WebSocket query fallback | `access_token` or `token`, plus `organization_id` | Preview-only alternative to headers |

## JSON Naming and Optionality

Rust DTO field names are serialized and deserialized as written unless a local Serde attribute changes behavior. The dominant wire convention is `snake_case`. `Option<T>` fields may be omitted or `null` unless validation imposes another rule. `#[serde(default)]` and default functions create endpoint-specific behavior and must be checked on the DTO rather than inferred globally.

## Validation Layers

Requests can fail at several boundaries:

1. HTTP parsing and Axum extraction;
2. authentication and tenant middleware;
3. route-level role or ownership checks;
4. normalization and validation functions;
5. quota, compatibility, workflow, and business services;
6. PostgreSQL constraints or provider validation.

There is no repository-wide request-validation framework or uniform field-error object. Most semantic validation returns `AppError::Validation` with a human-readable message.

## Bodyless Mutations

Several workflow actions are modeled as `POST` with no JSON payload, for example publish, submit, approve, reject, archive, restore, rollback, enable, disable, and installation transitions. Confirmation-sensitive deletes commonly use `?confirm=true`; this is a query contract, not a request body.

## Contract Gaps

- Framework rejection bodies for malformed JSON, invalid UUID paths, missing required query fields, and body-limit failures are not normalized through `AppError` (`REQUEST_CONTRACT_UNCLEAR RCU-01`).
- OpenAPI does not declare the bearer or tenant-header requirements.
- No shared schema registry enforces parity between Rust DTOs and the manually maintained TypeScript types.

## Review Checklist

When changing a request, verify the Axum extractor, Serde attributes, handler validation, frontend call and TypeScript type, Utoipa request body and parameters, tests, endpoint catalog entry, and any provider signature rules.
