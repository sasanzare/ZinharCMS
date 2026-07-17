---
okf_document_id: "backend-risks"
title: "Backend Risk Register"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src"
  - "backend/Cargo.toml"
  - ".github/workflows/backend-ci.yml"
  - "okf/backend/module-boundaries.md"
  - "okf/backend/dependency-map.md"
  - "okf/backend/testing-map.md"
related_documents:
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/persistence-access.md"
  - "backend/error-handling.md"
  - "backend/testing-map.md"
  - "architecture/architecture-risks.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR"
  - "TEST_COVERAGE_GAP TCG-01"
  - "ERROR_DISCLOSURE_RISK EDR-01"
  - "NEEDS_OWNER_CONFIRMATION"
---

# Backend Risk Register

## Scope and Rating

This register contains architecture risks supported by repository evidence. It does not classify style preferences as risks and does not claim production incidents. Likelihood and impact are qualitative Phase 3 assessments; owner confirmation is required where operational evidence or accepted risk posture is absent.

## BE-RISK-001 — Unenforced Module Boundaries

- **Description:** One crate permits routes, services, middleware, and domain areas to import each other without an automated dependency rule; a few reverse-layer imports already exist.
- **Evidence:** [Module Boundaries](module-boundaries.md#observed-boundary-violations-and-exceptions), [Dependency Map](dependency-map.md#circular-or-reverse-layer-exceptions), `backend/src/services/health.rs`, `backend/src/services/marketplace_adapters.rs`.
- **Affected modules:** All; especially Pages, Marketplace Runtime/Adapters, Delivery, Content, and health/readiness code.
- **Likelihood:** High.
- **Impact:** High change impact and unclear ownership.
- **Severity:** `HIGH`.
- **Existing mitigation:** Rust module structure, review, module-level documentation, focused tests.
- **Recommended follow-up phase:** Phase 6 architecture reconciliation after Phase 4 API/data evidence.
- **Owner confirmation required:** Yes.
- **Status:** `OPEN`.

## BE-RISK-002 — Business Logic and Direct SQL Concentrated in Handlers

- **Description:** Many handlers combine HTTP extraction, policy, SQL, transactions, side effects, and response mapping.
- **Evidence:** [Services and Domain](services-and-domain.md#business-rule-placement), [Persistence Access](persistence-access.md#access-pattern), route sources under `backend/src/routes`.
- **Affected modules:** Authentication, Organizations, Content, Comments, Media, Pages, Billing, Webhooks, and Marketplace routes.
- **Likelihood:** High.
- **Impact:** High regression risk, limited reuse, and difficult isolated testing.
- **Severity:** `HIGH`.
- **Existing mitigation:** Reusable services exist for selected policies and complex Marketplace behavior; SQL transactions protect selected operations.
- **Recommended follow-up phase:** Phase 4 API and data mapping, then targeted refactoring decisions.
- **Owner confirmation required:** No for existence; yes for accepted design.
- **Status:** `OPEN`.

## BE-RISK-003 — Dense Marketplace Coupling

- **Description:** Creator, catalog, installation, runtime, finance, feedback, analytics, and release services form a dense graph with shared domain/policy/package concepts and phase-named modules.
- **Evidence:** [Dependency Map](dependency-map.md#high-coupling-areas), [Module Catalog](module-catalog.md), Marketplace route/service sources.
- **Affected modules:** BE-MOD-014 through BE-MOD-018, plus Pages and Billing integration points.
- **Likelihood:** High.
- **Impact:** High cross-feature change and regression surface.
- **Severity:** `HIGH`.
- **Existing mitigation:** Separate service files, domain/policy helpers, and comparatively strong pure-rule tests.
- **Recommended follow-up phase:** Phase 4 lifecycle/API/data mapping and Phase 6 boundary review.
- **Owner confirmation required:** Yes for intended long-term phase-module structure.
- **Status:** `OPEN`.

## BE-RISK-004 — Process-Local Preview State

- **Description:** Page preview broadcast channels live in an in-memory map inside one process and disappear on restart.
- **Evidence:** [Configuration and State](configuration-and-state.md#application-state-composition), [state diagram](diagrams/application-state-composition.mmd), `backend/src/state.rs`.
- **Affected modules:** Pages, Builder, and Preview; Bootstrap and Runtime.
- **Likelihood:** Medium in multi-replica or restart scenarios.
- **Impact:** High for missed preview updates or uneven replica behavior.
- **Severity:** `HIGH`.
- **Existing mitigation:** Tokio broadcast channels work within one process; no distributed mitigation was found.
- **Recommended follow-up phase:** Phase 5 runtime/operations documentation.
- **Owner confirmation required:** Yes regarding deployment topology.
- **Status:** `NEEDS_OWNER_CONFIRMATION`.

## BE-RISK-005 — Tenant Enforcement Is Distributed

- **Description:** Tenant middleware, RBAC/RLS/quota/rate helpers, handler authorization, and direct SQL jointly enforce tenant boundaries; uniform query coverage is not proven by structure alone.
- **Evidence:** [Tenant module](modules/tenant-authorization.md), [Persistence Access](persistence-access.md#tenant-isolation), [Module Boundaries](module-boundaries.md#communication-rules-observed-in-code).
- **Affected modules:** Every tenant-protected module.
- **Likelihood:** Medium.
- **Impact:** High data-isolation or authorization consequences if a path omits the required control.
- **Severity:** `HIGH`.
- **Existing mitigation:** Tenant router group, `TenantContext`, RBAC/RLS services, database policies/migrations, resource checks.
- **Recommended follow-up phase:** Phase 4 data/API mapping and the dedicated security phase.
- **Owner confirmation required:** Yes for intended enforcement invariant.
- **Status:** `OPEN`.

## BE-RISK-006 — Authentication Context Ownership Is Broad

- **Description:** Middleware-owned `Claims` is a domain-wide request contract, while resource-specific authorization remains distributed.
- **Evidence:** [Module Boundaries](module-boundaries.md#observed-boundary-violations-and-exceptions), [Authentication module](modules/authentication.md), protected route imports.
- **Affected modules:** Authentication, Tenant Authorization, Organizations, Plugins, Beta, and all tenant modules.
- **Likelihood:** Medium.
- **Impact:** Medium change impact and potential policy inconsistency.
- **Severity:** `MEDIUM`.
- **Existing mitigation:** Central authentication middleware and JWT service.
- **Recommended follow-up phase:** Dedicated security phase.
- **Owner confirmation required:** Yes for contract ownership.
- **Status:** `OPEN`.

## BE-RISK-007 — Cross-System State Is Not Atomically Coordinated

- **Description:** Operations can combine PostgreSQL with filesystem, Redis, provider, webhook, email, or in-memory side effects that cannot share one database transaction.
- **Evidence:** [Persistence Access](persistence-access.md#risks-and-unknowns), [Request Handling](request-handling.md#handler-processing), Media/Delivery/Webhook/Finance module documents.
- **Affected modules:** Media, Content, Pages, Delivery, CMS Webhooks, Billing, Marketplace Finance and Installation.
- **Likelihood:** High under partial failure.
- **Impact:** High inconsistency, stale cache, orphan files, or duplicated/missed side effects.
- **Severity:** `HIGH`.
- **Existing mitigation:** SQL transactions for selected writes, explicit invalidation/helpers, provider-specific verification.
- **Recommended follow-up phase:** Phase 4 operation/data mapping and Phase 5 runtime reliability.
- **Owner confirmation required:** Yes for retry/idempotency guarantees.
- **Status:** `OPEN`.

## BE-RISK-008 — Error Detail Disclosure and Inconsistent Error Contracts

- **Description:** `AppError` maps broad categories consistently, but internal, database, provider, extractor, timeout, and WebSocket paths do not share one sanitization or payload contract.
- **Evidence:** [Error Handling](error-handling.md#user-visible-and-internal-messages), `backend/src/error.rs`.
- **Affected modules:** All HTTP modules; persistence and external integrations.
- **Likelihood:** High because technical strings are passed into shared messages by verified conversions.
- **Impact:** High if sensitive operational detail is exposed; medium client-contract inconsistency otherwise.
- **Severity:** `HIGH`.
- **Existing mitigation:** Stable high-level categories and explicit mapping at selected call sites.
- **Recommended follow-up phase:** Phase 4 API error contract and dedicated security review.
- **Owner confirmation required:** No for mechanism; yes for exposure policy.
- **Status:** `OPEN`.

## BE-RISK-009 — Startup Seeding and Configuration Need Production Controls

- **Description:** Startup runs migrations and conditionally seeds a default administrator with fixed source-defined initial credential material; local-oriented defaults and optional providers coexist with production use.
- **Evidence:** [Configuration and State](configuration-and-state.md#startup-state-transitions), `backend/src/main.rs`, `backend/src/config.rs`.
- **Affected modules:** Bootstrap and Runtime, Authentication, Organizations, Billing, Email.
- **Likelihood:** Medium; deployment practice is unknown.
- **Impact:** High if bootstrap credentials or defaults are not operationally controlled.
- **Severity:** `HIGH`.
- **Existing mitigation:** Seeding occurs only when no users exist; JWT length validation; environment overrides.
- **Recommended follow-up phase:** Phase 5 deployment/operations and dedicated security review.
- **Owner confirmation required:** Yes.
- **Status:** `NEEDS_OWNER_CONFIRMATION`.

## BE-RISK-010 — External Side-Effect Reliability Is Feature-Specific

- **Description:** Stripe, email, outbound webhooks, Redis, and filesystem operations use different fallback, retry, and error-mapping behavior; no shared durable queue or circuit breaker was found.
- **Evidence:** [Shared Infrastructure](shared-infrastructure.md#absent-or-unverified-shared-facilities), [Error Handling](error-handling.md#external-service-and-infrastructure-errors), provider/service sources.
- **Affected modules:** Billing, Marketplace Finance, CMS Webhooks, Media, Delivery, organization/auth notification flows.
- **Likelihood:** Medium.
- **Impact:** High for lost, duplicated, delayed, or user-blocking operations.
- **Severity:** `HIGH`.
- **Existing mitigation:** Provider signatures, selected error mapping, configurable email failure mode, operation-specific helpers.
- **Recommended follow-up phase:** Phase 5 runtime/operations reliability.
- **Owner confirmation required:** Yes for delivery guarantees.
- **Status:** `OPEN`.

## BE-RISK-011 — Core Boundary and Integration Test Gaps

- **Description:** Tests are mostly colocated unit tests; several core route modules and cross-module boundaries lack located tests or a dedicated integration suite.
- **Evidence:** [Testing Map](testing-map.md#coverage-gaps-and-confidence), `backend/src`, absence of `backend/tests` at the verification commit.
- **Affected modules:** Authentication, Organizations, Content, Comments, Media, Pages, tenant/RLS boundaries, multi-system side effects.
- **Likelihood:** High.
- **Impact:** High regression and policy-boundary risk.
- **Severity:** `HIGH`.
- **Existing mitigation:** CI format/clippy/test gates and strong pure-rule tests in Marketplace and selected services.
- **Recommended follow-up phase:** Phase 4 contract mapping followed by test-plan implementation.
- **Owner confirmation required:** No for located gap; yes for external/manual coverage.
- **Status:** `OPEN`.

## BE-RISK-012 — Provider and Plugin Ownership Overlap

- **Description:** Stripe behavior spans CMS Billing and Marketplace Finance, while built-in plugin behavior crosses route, page/content, and tenant concerns without a separate plugin host boundary.
- **Evidence:** [Module Boundaries](module-boundaries.md#ownership-matrix), [Shared Infrastructure](shared-infrastructure.md#shared-area-catalog), Billing/Finance/Plugin module documents.
- **Affected modules:** Billing and Quotas, Marketplace Finance, Built-In Plugins, Pages/Content.
- **Likelihood:** Medium.
- **Impact:** Medium duplicated behavior and change coupling; runtime trust impact requires later analysis.
- **Severity:** `MEDIUM`.
- **Existing mitigation:** Distinct route/service modules and a narrow built-in SEO implementation.
- **Recommended follow-up phase:** Phase 4 API/integration mapping and later plugin/security phase.
- **Owner confirmation required:** Yes for intended shared adapter ownership.
- **Status:** `OPEN`.

## BE-RISK-013 — Documentation and Source Can Drift

- **Description:** Direct SQL, route-local DTOs, dense module graphs, and phase-named Marketplace evolution make manually maintained maps vulnerable to drift.
- **Evidence:** [Module Catalog](module-catalog.md#catalog-maintenance), [Testing Map](testing-map.md#maintenance-guidance), `okf/index.yaml` verification metadata.
- **Affected modules:** All backend modules and OKF consumers.
- **Likelihood:** High as code evolves.
- **Impact:** Medium incorrect navigation or change planning.
- **Severity:** `MEDIUM`.
- **Existing mitigation:** Commit/date metadata, evidence paths, uncertainty markers, index registration, and cross-links.
- **Recommended follow-up phase:** Every subsequent phase and each backend change.
- **Owner confirmation required:** No.
- **Status:** `MONITOR`.

## Review Guidance

Reassess this register after Phase 4 maps endpoint and data contracts, after deployment evidence becomes available, and whenever an affected source boundary changes. A risk is not resolved merely because documentation exists; close it only when implementation evidence or an explicit accepted-risk decision supports closure.
