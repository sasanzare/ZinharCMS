---
okf_document_id: "backend-extension-points"
title: "Backend Extension Points"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/marketplace_runtime.rs"
related_documents:
  - "extension-points.md"
  - "hooks-and-events.md"
  - "isolation-and-trust.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
---

# Backend Extension Points

## Public or Intended Extension Contracts

- CmsPlugin entry.before_save and entry.after_publish callbacks.
- Marketplace manifest contract and static permission vocabulary.
- Marketplace component, template, and public-hook declarations interpreted by host adapters.
- Runtime authorization operation contract.

## Internal Integration Points

Axum router merges, SQLx helpers, audit functions, cache invalidation, webhook delivery, provider calls, and service modules are internal integrations. They are not registered by third-party code and are not public plugin APIs.

## Unsupported or Deferred

No arbitrary backend extension execution, shared-library loader, scripting engine, WASM host, worker protocol, plugin-owned router merge, plugin dependency injection, or isolation supervisor was found. The backend_extension manifest product type is accepted by schema validation but rejected by the current installation gate. It is PLANNED_NOT_IMPLEMENTED as an executable extension model.

Contracts are source-level and do not carry stability guarantees. EXTENSION_POINT_UNVERIFIED applies to any internal function proposed for third-party consumption.

## Backend Surface Matrix

| Surface | Contract | Registration | Runtime execution | State access | Database access | Errors | Concurrency | Security boundary | Tests | Stability/status |
|---|---|---|---|---|---|---|---|---|---|---|
| Entry before-save | CmsPlugin mutable callback | Static vector | Synchronous request path | EntryData and PluginContext | Indirect through host persistence after callback | Propagates | Sequential vector order | Trusted in-process | SEO unit tests | Public-to-repository contract; unversioned |
| Entry after-publish | CmsPlugin immutable callback | Static vector | Synchronous publish path | EntryData and PluginContext | No direct contract | Propagates | Sequential | Trusted in-process | No subscriber test | Implemented contract; behavior partial |
| Component registry | Record/API contract | Seed, tenant CRUD, host upsert | Host query/validation | AppState/tenant context | Direct SQLx under RLS | HTTP/DB errors | DB constraints; collision semantics partial | RBAC/RLS | Selected tests | Implemented data contract |
| Template adapter | Manifest/request contract | Active ready installation | Host preview/import | AppState/tenant | Transactional page/version/import writes | Rollback/error | One transaction; filesystem already external | RBAC/RLS/quota/validation | Helper/frontend tests | Version 2026-07 adapter |
| Public hook/runtime policy | Definition and authorization | Static types plus manifest | Decision only, not package execution | Installation/runtime state | Tenant reads and decision audit paths | Denial reason | Request-local | Capability/kill-switch boundary | Policy tests | Partially implemented |
| Routes/handlers/services/middleware/repositories | No plugin contract | Static application code | Host only | Full application | Host-defined | Host-defined | Host-defined | Internal | Normal module tests | Internal implementation detail |
| Content-type validator | No third-party registration | Static route/service code | Host only | Request data | Host persistence | Validation error | Request-local | Tenant/RBAC | Validation tests | Internal |
| Background jobs/events | No plugin worker/subscriber registry | Static/spawned host code | In-process host | Host state | Host-defined | Mixed | Async behavior internal | Internal | Selected tests | EXTENSION_POINT_UNVERIFIED |
| Storage/search/media/auth/authorization/notifications/import-export providers | No public provider registry found | None | None | None | None | Not applicable | Not applicable | Not available to packages | None | PLANNED_NOT_IMPLEMENTED or internal-only |
