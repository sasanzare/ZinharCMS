---
okf_document_id: "backend-services-and-domain"
title: "Backend Services and Domain Logic"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "backend/src/plugins"
  - "backend/src/models"
related_documents:
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/request-handling.md"
  - "backend/persistence-access.md"
related_diagrams:
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "BUSINESS_RULE_LOCATION_UNCLEAR BRU-01"
  - "BUSINESS_RULE_LOCATION_UNCLEAR BRU-02"
  - "BUSINESS_RULE_LOCATION_UNCLEAR BRU-03"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR"
---

# Backend Services and Domain Logic

## Current Organization

ZinharCMS does not implement a single formal domain layer. Business behavior is distributed across Axum route handlers, `services`, middleware, built-in plugins, model methods/structures, SQL constraints and migrations, and provider-specific code. The `services` directory is the strongest reusable-logic layer, but service modules do not share one interface or lifecycle pattern.

## Service Families

| Family | Examples | Observed role |
|---|---|---|
| Identity and policy | `jwt`, `password`, `security`, `rbac`, `rls` | Authentication primitives and authorization helpers |
| Capacity and protection | `quota`, `rate_limit`, `hardening` | Tenant/request constraints and operational checks |
| CMS domain | `entry_validation`, `workflow`, `media_processing`, `cache`, `webhooks`, `audit` | Content validation, transitions, file processing, side effects |
| Providers | `stripe_billing`, `email` | External provider interaction and behavior selection |
| Operations | `health`, `ga_readiness` | Dependency/readiness evaluation; health service use is unconfirmed (`DU-01`) |
| Marketplace domain | `marketplace_domain`, `policy`, `manifest`, `package`, `validation`, `submission`, `review`, `catalog`, `installation`, `runtime`, `adapters`, `finance`, `feedback`, `analytics`, `performance`, `readiness`, phase services | Marketplace types, rules, workflows, persistence, integrations, and release gates |

## Domain Modules

The clearest explicit domain vocabulary appears in `services/marketplace_domain.rs` and related Marketplace modules. Core CMS domain concepts are more often represented by route-local DTOs, `models` records, service functions, SQL enums/constraints, and handler branches. This is an observed organizational difference, not a judgment that one area lacks business rules.

## Business Rule Placement

| Location | Examples of responsibility | Consequence |
|---|---|---|
| Route handlers | Request validation, ownership checks, lifecycle branching, SQL, response mapping | HTTP and business concerns are frequently coupled (`BRU-01`) |
| Services | Reusable policies, validation, transitions, integrations, multi-step operations | Best current reuse boundary, but interfaces vary |
| Middleware | Bearer/tenant context, broad access/rate/quota enforcement | Cross-cutting policy is applied before many handlers |
| Models/DTOs | Persisted and transported shapes | Coverage is partial; many DTOs remain route-local (`BRU-02`) |
| Migrations/database | Types, constraints, defaults, triggers/RLS policy | Some invariants are enforced below application code |
| Plugins | Built-in SEO plugin behavior | Plugin contract and implementation are colocated |

## Service Patterns

Most services expose functions or data types directly from their Rust modules. Some functions are pure transformations or validators; some accept state fragments such as a pool, Redis client, or config; some create HTTP clients or use provider credentials; some return `AppError`, while others return library-specific or local error types that callers translate.

No universal service trait, dependency-injection container, command bus, query bus, event bus, aggregate framework, or repository interface was found. Marketplace adapters and built-in plugins use trait-like contracts in narrower feature contexts, but they do not define a backend-wide application pattern.

## Commands, Queries, and Transactions

Read and write behavior is separated by handler/service function names and SQL operations, not by formal Command Query Responsibility Segregation. Explicit SQL transactions appear where several writes or tenant/RLS-scoped operations must be coordinated. Other operations issue individual statements. Transaction ownership is therefore operation-specific and must be checked at the call site.

## Validation

Validation is layered and non-uniform:

- Axum extractors validate request shape at the framework boundary.
- Handler code validates required values and state transitions.
- `entry_validation` validates dynamic content against content type definitions.
- Marketplace manifest, package, submission, policy, and validation modules enforce Marketplace rules.
- Database constraints and enums provide final persistence invariants.

A single validation library or error vocabulary is not used across every module.

## Orchestration and Side Effects

Handlers commonly orchestrate persistence plus audit, cache invalidation, email, webhook, provider, file, or preview behavior. The Marketplace service family also contains service-to-service orchestration. There is no general durable event bus; outbound webhooks and other side effects use feature-specific helpers. Whether every side effect is transactionally coordinated with its originating write is `UNKNOWN` until each operation is reviewed.

## Interfaces and Ownership Concerns

- Broad use of route-owned DTOs creates reverse-layer dependencies in a few services.
- Claims and tenant context are middleware-owned types with domain-wide consumers.
- Delivery cache behavior is invoked by Content and Pages, but its ownership is tied to a route module (`BRU-03`).
- Stripe behavior exists in both CMS Billing and Marketplace Finance; the business lifecycles remain separate.
- Marketplace phase-named services encode release/readiness evolution; their long-term consolidation status is `NEEDS_OWNER_CONFIRMATION`.

## Testing of Rules

Service-level unit tests are strongest in Marketplace policy/validation/readiness, workflow, webhook, Stripe, quota/rate/RBAC/security, and operational readiness helpers. Many core CMS routes have no colocated test module, so behavior may rely on compilation, database constraints, frontend use, or unlocated external testing. See [Testing Map](testing-map.md).

## Maintenance Guidance

When changing a rule, begin at the owning [module document](modules/), then search both its route and service files plus related migrations and tests. Do not assume the service module is the only rule owner. If a new shared contract would be imported by routes and services, place it outside a route module and update the boundary/dependency maps.

## Phase 8 Rule Ownership Cross-Reference

The [Business Rule Catalog](../domain/business-rule-catalog.md) records rule IDs, triggers, conditions, outcomes, enforcement layers, and evidence. Rules remain distributed: route handlers coordinate transactions and side effects, services own reusable policy and provider logic, migrations enforce durable constraints, and the frontend repeats selected validation and action availability. Use the owning [domain document](../domain/domain-catalog.md) and [workflow](../domain/cross-module-workflows.md) before moving a rule between layers.
