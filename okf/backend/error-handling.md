---
okf_document_id: "backend-error-handling"
title: "Backend Error Handling"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/error.rs"
  - "backend/src/main.rs"
  - "backend/src/routes"
  - "backend/src/services"
related_documents:
  - "backend/request-handling.md"
  - "backend/services-and-domain.md"
  - "backend/persistence-access.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
related_diagrams:
  - "backend/diagrams/backend-request-lifecycle.mmd"
uncertainty_markers:
  - "ERROR_DISCLOSURE_RISK EDR-01"
  - "ERROR_MODEL_INCONSISTENT EMI-01"
  - "UNKNOWN"
---

# Backend Error Handling

## Main Error Type

`backend/src/error.rs` defines the shared `AppError` enum and `ErrorBody`. The enum is the most common application-to-HTTP error contract. It carries a `String` message in every variant.

| Variant | HTTP status | Stable `error` field |
|---|---:|---|
| `Unauthorized` | 401 | `unauthorized` |
| `Forbidden` | 403 | `forbidden` |
| `NotFound` | 404 | `not_found` |
| `BadRequest` | 400 | `bad_request` |
| `Validation` | 422 | `validation_error` |
| `Conflict` | 409 | `conflict` |
| `TooManyRequests` | 429 | `too_many_requests` |
| `ServiceUnavailable` | 503 | `service_unavailable` |
| `Internal` | 500 | `internal_error` |

`IntoResponse` serializes `{ "error": <category>, "message": <display string> }`. The display string includes the variant prefix and its stored message.

## Conversion and Propagation

Routes commonly return `Result<_, AppError>` and use `?` after converting library or service failures. `From<sqlx::Error>` supplies the only backend-wide automatic conversion observed:

- `RowNotFound` becomes a generic `NotFound` message.
- PostgreSQL error code `23505` becomes `Conflict` with the database error message.
- All other SQLx errors become `Internal` with `sqlx::Error::to_string()`.

Handlers and services also use explicit `map_err` branches to select domain-appropriate variants. `main` uses `anyhow::Context` for startup errors because failures occur before an HTTP response exists.

## Module-Specific Errors

Configuration has `ConfigError`. Marketplace service modules, provider helpers, password/JWT libraries, Redis operations, JSON parsing, file I/O, and validation code can produce their own or third-party errors before callers translate them. There is no single trait requiring consistent conversion or stable module error codes (`EMI-01`).

## User-Visible and Internal Messages

The shared payload distinguishes a stable category from a human-readable message, but `AppError::Internal` still exposes its full display string. The default SQLx conversion and some explicit provider/serialization/file conversions may therefore place technical details in the response. `ERROR_DISCLOSURE_RISK EDR-01` records this verified mechanism; Phase 3 does not assert that sensitive data is protected or that an exploit is present.

## Validation Errors

Application validation generally uses `Validation` (422) or `BadRequest` (400), depending on the module and failure. Axum extractor rejections can use framework-generated payloads/statuses before entering a handler. Marketplace and content validators may return structured/local findings that routes summarize into `AppError`. Error shape is consequently not guaranteed to be identical for every invalid request.

## Persistence Errors

The global SQLx mapping recognizes row absence and unique conflicts. Foreign-key, check-constraint, serialization, timeout, connectivity, and other database errors fall through unless a caller handles them. Some domain routes pre-check state and construct clearer errors; these patterns are operation-specific.

## External-Service and Infrastructure Errors

Redis, Stripe, email webhook, outbound CMS webhook, filesystem, and media-processing failures are mapped at individual call sites. Depending on the operation and configured failure mode, a failure can become `BadRequest`, `ServiceUnavailable`, `Internal`, a logged/fallback result, or a local result returned to the caller. No universal retry or circuit-breaker policy was found.

## Logging Behavior

Global `TraceLayer` records HTTP tracing, and source modules use `tracing` selectively for startup and operation events. `AppError::into_response` itself does not log. Therefore a returned error is not guaranteed by this implementation alone to have a dedicated structured error log with module context. Deployment subscriber filters come from `RUST_LOG`/environment filtering.

## Panic, Timeout, and Fallback Behavior

Normal handlers favor `Result`, but `expect` remains in shutdown signal installation and other source-specific invariants may panic. Rust panics are not converted by `AppError` unless separately caught; no application-wide panic catcher was found. The Tower timeout layer returns a 408 status for requests exceeding 30 seconds, outside `AppError`. Provider/log modes and optional configurations supply feature-specific fallbacks rather than one backend-wide fallback mechanism.

## Test Coverage

Tests cover selected error branches in configuration, billing/Stripe, webhooks, workflow, quota/rate/security, email, delivery, and Marketplace services. No dedicated tests for the complete `AppError` status/payload matrix were found in `backend/src/error.rs`, and no cross-module contract test asserts one external error format. See [Testing Map](testing-map.md).

## Inconsistencies and Risks

- `BadRequest` versus `Validation` selection is module-specific.
- Stable category codes are broad and do not identify module/domain causes.
- Some errors preserve technical source text, while others replace it with a generic message.
- Framework rejections, timeouts, static-file responses, and WebSocket failures do not necessarily use `ErrorBody`.
- Logging and retry behavior vary by call site.

## Related Documentation

See [Request Handling](request-handling.md), [Persistence Access](persistence-access.md), [Shared Infrastructure](shared-infrastructure.md), and risks `BE-RISK-008` and `BE-RISK-010` in [Backend Risks](backend-risks.md).
