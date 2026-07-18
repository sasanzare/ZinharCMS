---
okf_document_id: "backend-testing-map"
title: "Backend Testing Map"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/Cargo.toml"
  - "backend/src"
  - ".github/workflows/backend-ci.yml"
related_documents:
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/error-handling.md"
  - "backend/backend-risks.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
uncertainty_markers:
  - "TEST_COVERAGE_GAP TCG-01"
  - "TEST_ISOLATION_UNCLEAR TIU-01"
  - "UNKNOWN"
---

# Backend Testing Map

## Frameworks and Commands

Backend tests use Rust's built-in `#[test]`, Tokio's `#[tokio::test]`, and Tower request helpers where HTTP routing is exercised. `tower` is a development dependency. The repository-defined backend CI commands are:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

CI runs these from `backend/`. From the repository root, the equivalent targeted test invocation is `cargo test --manifest-path backend/Cargo.toml --all-features`. This equivalent is derived from Cargo behavior; the CI file explicitly defines only the working-directory form.

## Test Organization

Tests are colocated in source files under `#[cfg(test)]` modules. No separate `backend/tests` integration-test directory or backend-owned end-to-end suite was found. Some route tests construct an Axum router and send in-process requests; service tests more often validate pure rules, DTO transformation, signatures, or local failure behavior.

Fixtures are primarily inline constructors, JSON values, sample manifests/packages, and configuration test constructors. No shared fixture crate, fixture directory, snapshot framework, or property-test framework was found. Mocking is generally performed with sample values, local test helpers, deterministic signing payloads, or test-specific provider inputs rather than a general mocking library.

## Database and External-Service Approach

Backend CI provisions PostgreSQL 16 and Redis 7 services and exports application environment values, but many located tests do not require them. No dedicated database reset/fixture harness was found in the backend test sources (`TIU-01`). Stripe tests focus on signing, configuration, and service behavior; email tests cover provider/failure-mode selection; webhook and Marketplace tests use local data and helpers. Tests that call live external services were not identified.

## Module-to-Test Map

| Module | Test paths | Test type | Behaviors covered | Confidence | Known gaps |
|---|---|---|---|---|---|
| [BE-MOD-001 Bootstrap and Runtime](modules/bootstrap-runtime.md) | `backend/src/config.rs` | Unit | Configuration parsing and test config construction | Medium | Startup, migration, seeding, global layers, shutdown not directly covered |
| [BE-MOD-002 Authentication](modules/authentication.md) | `backend/src/services/security.rs`; indirect JWT/password consumers | Unit | Selected security counter/rule behavior | Low | Registration/login/token rotation and middleware contracts lack colocated route tests |
| [BE-MOD-003 Tenant Authorization and RLS](modules/tenant-authorization.md) | `services/quota.rs`, `rate_limit.rs`, `rbac.rs`, `security.rs` | Unit | Limits, roles, selected policy helpers | Medium | Tenant middleware, RLS transaction context, and cross-module enforcement lack boundary tests |
| [BE-MOD-004 Organizations and SaaS Operations](modules/organizations.md) | No colocated organization test module found | Unverified | No direct behaviors located | Low | Membership, invitation, lifecycle, and authorization flows |
| [BE-MOD-005 Billing and Quotas](modules/billing-quotas.md) | `routes/billing.rs`; `services/stripe_billing.rs`; `services/quota.rs` | Unit/in-process helper | Billing validation/signature/config and quota rules | Medium | Provider/database lifecycle and tenant integration |
| [BE-MOD-006 Content Types, Entries, and Workflow](modules/content-workflow.md) | `services/workflow.rs` | Unit | Workflow transition rules | Low | Content schemas, entry validation integration, SQL, authorization, cache/webhook side effects |
| [BE-MOD-007 Editorial Comments](modules/comments.md) | No colocated comments test module found | Unverified | No direct behaviors located | Low | Polymorphic resources, moderation, tenant access, persistence |
| [BE-MOD-008 Media](modules/media.md) | No colocated media/media-processing test module found | Unverified | No direct behaviors located | Low | Multipart limits, file/database consistency, processing, authorization |
| [BE-MOD-009 Pages, Builder, and Preview](modules/pages-builder-preview.md) | No colocated pages test module found | Unverified | No direct behaviors located | Low | Builder state, adapter integration, preview WebSocket, cache invalidation |
| [BE-MOD-010 Public Delivery and Cache](modules/public-delivery-cache.md) | `routes/delivery.rs` | Unit/in-process route | Cache key/delivery response and selected route behavior | Medium | Redis/database integration, invalidation races, failure policy |
| [BE-MOD-011 CMS Webhooks](modules/cms-webhooks.md) | `services/webhooks.rs` | Unit | Signing/delivery helper behavior and error cases | Medium | Registration routes, durable retry, ordering, database/provider integration |
| [BE-MOD-012 Built-In Plugins](modules/built-in-plugins.md) | `plugins/seo.rs` | Unit | SEO plugin output/behavior | Medium | Router/tenant integration and broader plugin lifecycle |
| [BE-MOD-013 Beta and Release Operations](modules/beta-release-operations.md) | `routes/beta.rs`; `services/hardening.rs`; `services/ga_readiness.rs` | Unit | Beta rules, hardening gates, GA readiness evaluation | High for pure rules | Database-backed operations and deployed readiness |
| [BE-MOD-014 Marketplace Creator, Submission, Validation, and Review](modules/marketplace-creator-review.md) | `routes/marketplace.rs`; `services/marketplace_manifest.rs`, `package.rs`, `validation.rs`, `submission.rs`, `review.rs`, `policy.rs`, `domain.rs` | Unit/in-process helper | Manifest/package validation, submission/review transitions, policy/domain rules | High for pure rules | Full persistence, authorization, artifact storage, concurrent review |
| [BE-MOD-015 Marketplace Catalog and Installation](modules/marketplace-catalog-installation.md) | `services/marketplace_catalog.rs`; `marketplace_installation.rs`; shared marketplace route tests | Unit | Catalog filters/rules and installation lifecycle helpers | Medium | Database-backed install/rollback, entitlement concurrency, route contracts |
| [BE-MOD-016 Marketplace Runtime Security and Host Adapters](modules/marketplace-runtime-adapters.md) | `routes/marketplace_adapters.rs`, `marketplace_runtime.rs`; `services/marketplace_adapters.rs`, `marketplace_runtime.rs` | Unit/in-process helper | Adapter contracts, runtime policy/security behavior | Medium | Host/page integration, isolation under real artifacts, resource limits |
| [BE-MOD-017 Marketplace Finance](modules/marketplace-finance.md) | `services/marketplace_finance.rs`; `services/stripe_billing.rs`; shared marketplace tests | Unit | Finance calculations/transitions and Stripe signing/config behavior | Medium | Live provider, ledger concurrency, reconciliation, payout persistence |
| [BE-MOD-018 Marketplace Feedback, Analytics, and Readiness](modules/marketplace-feedback-analytics-readiness.md) | `services/marketplace_feedback.rs`, `analytics.rs`, `performance.rs`, `readiness.rs`, `marketplace_phase_thirteen.rs`, `phase_fourteen.rs`, `phase_fifteen.rs` | Unit | Feedback rules, projections, performance/readiness/release gates | High for pure rules | Production telemetry, database aggregation, route authorization, release operations |

## Architecture-Boundary Tests

Tests that would materially protect the documented architecture but were not found as a coherent suite include:

- public versus authenticated versus tenant router access contracts;
- tenant context and RLS propagation through each tenant module;
- one stable external error payload matrix;
- content/page mutation to delivery-cache invalidation and webhook behavior;
- media metadata/file atomicity and cleanup;
- Marketplace creator-to-catalog-to-install-to-runtime-to-finance lifecycle;
- process restart/multi-replica page preview behavior;
- provider outage and retry/fallback behavior.

## Phase 5 Database Test Findings

No separate `backend/tests` database integration suite, reusable PostgreSQL fixture/reset harness, or broad runtime migration/RLS assertion layer was found. CI provisions PostgreSQL, but most identified colocated tests are pure/static; Marketplace Tokio tests found during Phase 5 focus on filesystem artifacts. The tracked tenant fixture is manual/local-staging evidence. The hardening checker covers 24 of the 32 forced-RLS tables intended by current migrations (`DCC-12`). See [Database Testing](../database/database-testing.md) for the required isolation, migration, constraint, transaction, idempotency, and trigger scenarios.

## Coverage Gaps and Confidence

`TEST_COVERAGE_GAP TCG-01`: core Authentication, Organizations, Content, Comments, Media, and Pages route files have no colocated `#[cfg(test)]` module at the verification commit. This does not prove that behavior is entirely untested; frontend tests, manual tests, or unlocated external suites may exist. Phase 3 found no valid coverage report and therefore reports no percentage.

## Maintenance Guidance

When adding a test, update the owning module document and this map. Prefer tests that cross a documented boundary when a change involves middleware, transactions, cache invalidation, files, provider side effects, or more than one module.
