---
okf_document_id: "api-error-contracts"
title: "API Error Contracts"
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
  - "backend/src/error.rs"
  - "backend/src/middleware"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/response-contracts.md"
  - "backend/error-handling.md"
  - "api/api-risks.md"
uncertainty_markers:
  - "ERROR_CONTRACT_UNCLEAR ECU-01"
---

# API Error Contracts

## Application Error Envelope

Handlers using `AppError` return:

```json
{
  "error": "validation_error",
  "message": "validation error: example"
}
```

| Variant | HTTP status | `error` value |
| --- | ---: | --- |
| `Unauthorized` | 401 | `unauthorized` |
| `Forbidden` | 403 | `forbidden` |
| `NotFound` | 404 | `not_found` |
| `BadRequest` | 400 | `bad_request` |
| `Validation` | 422 | `validation_error` |
| `Conflict` | 409 | `conflict` |
| `TooManyRequests` | 429 | `too_many_requests` |
| `ServiceUnavailable` | 503 | `service_unavailable` |
| `Internal` | 500 | `internal_error` |

`sqlx::Error::RowNotFound` maps to 404. PostgreSQL unique-constraint code `23505` maps to 409. Other SQLx failures map to 500.

## Non-Uniform Error Sources

`ERROR_CONTRACT_UNCLEAR ECU-01` covers failures that do not necessarily use `ErrorBody`:

- Axum path, query, JSON, multipart, and body-limit rejections;
- bare middleware short-circuits in authentication or tenant processing;
- the global timeout fallback, which returns 408;
- static-file 404 and method handling from `ServeDir`;
- WebSocket upgrade and post-upgrade connection failures;
- provider- or library-generated transport errors before handler conversion.

Consumers must use HTTP status as the primary failure signal and parse `error`/`message` defensively.

## Frontend Behavior

The frontend attempts to parse a failed response as JSON, chooses `message` and then `error`, falls back to a status-based message, and throws `ApiError(status, message)`. It does not preserve the backend `error` code as a distinct typed field. There is no automatic token refresh, retry, backoff, or per-error recovery policy in the shared client.

## Exposure Risk

The current SQLx fallback includes `other.to_string()` in `AppError::Internal`, and unique violations include the database message. Depending on the underlying error, this can disclose schema or database detail. Phase 7 should evaluate redaction and correlation IDs before changing the public error contract.

## Testing Gap

No exhaustive test matrix demonstrates every `AppError` status/body pair together with framework and middleware failures. Add contract tests around the real router before promising uniformity.
