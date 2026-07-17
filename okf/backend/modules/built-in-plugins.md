---
okf_document_id: "backend-module-built-in-plugins"
title: "Built-In Plugins"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-012"
module_name: "Built-In Plugins"
module_paths:
  - "backend/src/plugins/mod.rs"
  - "backend/src/plugins/seo.rs"
  - "backend/src/routes/plugins.rs"
module_type: "Plugin module"
boundary_status: "EXPLICIT"
primary_sources:
  - "backend/src/plugins/mod.rs"
  - "backend/src/plugins/seo.rs"
  - "backend/src/routes/plugins.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "architecture/components.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "VERIFIED"
  - "NEEDS_OWNER_CONFIRMATION"
---

# Built-In Plugins

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-012` |
| Module type | Plugin module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `EXPLICIT` |
| Confidence | High |
| Source paths | `backend/src/plugins/mod.rs`; `backend/src/plugins/seo.rs`; `backend/src/routes/plugins.rs` |

## Responsibility

Verified responsibility: Defines the trusted `CmsPlugin` interface, registers the built-in SEO plugin, synchronizes registry/configuration records, and runs entry hooks before save and after publish.

Shared or inferred responsibility: Content invokes plugin hooks; route management and trait implementation share the `cms_plugins` registry.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/plugins/mod.rs`
- `backend/src/plugins/seo.rs`
- `backend/src/routes/plugins.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`CmsPlugin`, `builtin_plugins`, hook runners, and `plugins::router`.

## Internal Structure

The trait/registry is in `plugins/mod.rs`; SEO implementation is private; routes expose list/update/enable/disable operations.

## Public and Internal Interfaces

In-process plugin trait and hook functions plus authenticated plugin-registry routes.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Content data, `AppState`, PostgreSQL, `AppError`, Claims/RBAC, serde JSON.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Enabled plugin keys/settings and transient entry/plugin context.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Content entry handlers, plugin administration clients, and built-in plugin implementations.

## Data Concepts

Enabled plugin keys/settings and transient entry/plugin context.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Registry state to enabled keys; content mutation to hook invocation; management route to registry update.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Plugin or SQL failures use `AppError` and share request/process trust boundaries.

## Configuration

Database registry settings; no dynamic package loading or execution configuration.

Secret values and local environment contents are intentionally excluded.

## Tests

SEO plugin has two unit tests; route synchronization behavior has no dedicated route test block.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Built-in trusted plugins must remain distinct from Marketplace artifacts; third-party in-process isolation is not defined.

Relevant markers: `VERIFIED`, `NEEDS_OWNER_CONFIRMATION`.

## Related Documents

- [Backend Module Catalog](../module-catalog.md)
- [Module Boundaries](../module-boundaries.md)
- [Dependency Map](../dependency-map.md)
- [Testing Map](../testing-map.md)
- [Backend Risks](../backend-risks.md)
- [Architecture Components](../../architecture/components.md)
- [Architecture Boundaries](../../architecture/boundaries.md)
- [Architecture Dependency Model](../../architecture/dependency-model.md)
- [Backend Module Map](../diagrams/backend-module-map.mmd)
- [Backend Dependency Flow](../diagrams/backend-dependency-flow.mmd)
