---
okf_document_id: "api-testing"
title: "API Testing"
project: "ZinharCMS"
category: "api-testing"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/src/middleware"
  - "backend/src/services"
  - "frontend/src"
related_documents:
  - "backend/testing-map.md"
  - "frontend/testing-map.md"
  - "api/openapi-consistency.md"
  - "api/api-risks.md"
uncertainty_markers:
  - "TEST_COVERAGE_UNCLEAR TCU-API-01"
---

# API Testing

## Observed Coverage

API evidence is dominated by colocated Rust unit tests and service tests. Route-local tests were found in beta, billing, delivery, Marketplace core, Marketplace adapters, and Marketplace runtime, plus middleware/security tests. Marketplace services have comparatively broad domain-rule coverage.

No `backend/tests` integration harness or comprehensive real-router HTTP contract suite was found. No frontend test directly invokes the shared API client. Therefore route registration, middleware stacking, serialization, status codes, headers, and database effects are not demonstrated end to end for all families.

## Current Strengths

- Delivery has focused parsing/escaping tests.
- Security and selected validation helpers have unit tests.
- Marketplace compatibility, permissions, artifacts, lifecycle, finance, and readiness logic has substantial service-level coverage.
- Frontend components and state utilities have tests in selected features, though not transport contract tests.

## Critical Gaps

| Gap | Contract risk |
| --- | --- |
| No registered-route inventory test | Accidental route removal, move, or access-zone change |
| No OpenAPI parity test | Current 19-operation gap can grow unnoticed |
| No authentication/tenant matrix through the router | Missing headers, invalid membership, rate/quota order, and preview exceptions can drift |
| No complete `AppError` and framework rejection matrix | Clients cannot rely on one error envelope |
| No frontend/backend schema parity test | Manual TypeScript DTO drift |
| Sparse multipart/static/WebSocket tests | Upload limits, content types, ranges, upgrade failures, and message behavior remain weakly specified |
| Limited provider callback tests | Stripe signature/idempotency behavior can regress at the transport boundary |

## Recommended Contract Test Matrix

For each significant family, exercise at least: public request when protected, missing/invalid bearer token, missing/invalid tenant ID, inactive membership, insufficient role, valid request, invalid path/query/body, not found, conflict, quota/rate failure, and provider/service unavailability where relevant. Assert method/path reachability, status, headers, JSON shape, tenant isolation, persistence effect, audit effect, and webhook/cache side effects as applicable.

Add a generated set comparison between Axum registration and Utoipa operations, plus representative Rust-JSON/TypeScript fixture checks. `TEST_COVERAGE_UNCLEAR TCU-API-01` means absence of a test is not evidence of incorrect behavior; it records unverified contract risk.

## Phase 7 Security Matrix

Add register/login/refresh/logout cookie integration tests, malformed/expired/stale token cases, complete global and organization role matrices, owner/IDOR cases, live cross-tenant RLS CRUD, bypass preconditions, CSRF/CORS/header behavior, audit effects, and Marketplace permission expansion/runtime kill-switch cases. [Security Testing](../security/security-testing.md) records current evidence and `SECURITY_TEST_COVERAGE_UNCLEAR STCU-01`.

## Phase 8 Domain Workflow Matrix

For every workflow in [Cross-Module Workflows](../domain/cross-module-workflows.md), test precondition denial, valid transition, transaction rollback, retry/idempotency behavior, and observable side effects. Priority API scenarios include concurrent owner removal, entry/page transition permissions, page restore validation, media quota and file failure, unsafe webhook URL and timeout, out-of-order Stripe events, Marketplace permission reapproval, refund entitlement revocation, and tenant-scoped access. See [Business Rule Testing](../domain/business-rule-testing.md).

## Extensibility API Test Priorities

Add real-router and database-backed tests for plugin registry synchronization/enablement, Marketplace install/update/rollback/uninstall, permission reapproval, kill-switch races, component/template adapters, and cross-tenant denial. See [Extensibility Testing](../extensibility/extensibility-testing.md).
