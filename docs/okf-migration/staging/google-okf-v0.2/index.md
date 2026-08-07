---
okf_version: "0.2"
---

# ZinharCMS Google OKF v0.2 staging bundle

This isolated, non-canonical bundle contains the Phase 3 foundation Concepts
that can be constructed directly from current repository evidence. It is not
the active `/okf/` bundle and does not authorize cutover or legacy changes.

## Domains

* [Project](project/index.md) - product scope and repository boundary.
* [Architecture](architecture/index.md) - runtime composition and request boundaries.
* [Backend](backend/index.md) - backend runtime and module boundaries.
* [Frontend](frontend/index.md) - the administrative React application and state flow.
* [API](api/index.md) - observed API and authentication contract boundaries.
* [Database](database/index.md) - deferred schema and database-policy targets.
* [Security](security/index.md) - authentication, authorization, tenancy, and preview controls.
* [Domain](domain/index.md) - editorial, page-builder, Marketplace, and plugin capabilities.
* [Operations](operations/index.md) - deferred operational targets.
* [Development](development/index.md) - development and testing evidence.
* [Decisions](decisions/index.md) - Phase 2 migration and architecture decisions.
* [History](history/index.md) - historical snapshots deferred from the current bundle.

## Staging boundary

The constructed slice is limited to 19 source-backed Concepts and the
navigation indexes required by the Phase 2 design. Deferred targets are
recorded in the Phase 3 construction reports; they are not represented by
placeholder Concept links here. The legacy `okf/` and `okf-bootstrap/` trees
remain untouched.
