---
okf_document_id: "api-versioning-compatibility"
title: "API Versioning and Compatibility"
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
  - "backend/src/routes/mod.rs"
  - "backend/src/routes/delivery.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/overview.md"
  - "api/openapi-consistency.md"
  - "api/frontend-contract-map.md"
uncertainty_markers:
  - "VERSIONING_BEHAVIOR_UNCLEAR VBU-01"
  - "PAGINATION_FILTERING_VERSIONING_BEHAVIOR_UNCLEAR PFVBU-01"
---

# API Versioning and Compatibility

## Observed Versioning

Only the public delivery surface is path-versioned: `/api/v1/content/*`, `/api/v1/pages/*`, `/api/v1/settings/public`, `/api/v1/navigation`, `/api/v1/sitemap.xml`, and `/api/v1/robots.txt`. Administrative, authentication, billing, organization, Marketplace, media, page-builder, comment, webhook, system, and static-file paths are unversioned.

No repository-wide version negotiation, media-type versioning, compatibility header, deprecation header, sunset mechanism, or version-routing abstraction was found. `VERSIONING_BEHAVIOR_UNCLEAR VBU-01` records the absence of an explicit policy; it does not mean current paths are unresolved.

## Compatibility Boundaries

- Rust DTOs define backend serialization at build time.
- TypeScript API types are manually duplicated and are not generated from OpenAPI.
- Utoipa OpenAPI is generated at runtime but does not cover all routes or security requirements.
- Database enums and JSON shapes can constrain accepted API values even when OpenAPI does not enumerate them.
- Marketplace package compatibility is a domain rule distinct from HTTP API versioning.

## Change Classification

| Change | Compatibility expectation |
| --- | --- |
| Add optional request field with a server default | Usually additive; verify older frontend payloads |
| Add response field | Usually additive; verify strict external consumers |
| Remove or rename field | Breaking |
| Change field type, nullability, enum, or semantic default | Breaking or behaviorally breaking |
| Change method/path, auth zone, tenant requirement, or role requirement | Breaking |
| Change pagination defaults or maximums | Behaviorally breaking |
| Change error status/code/shape | Breaking for clients with recovery logic |
| Add endpoint without OpenAPI/client updates | Contract drift even if runtime-compatible |

## Current Policy Gap

There is no evidence defining support windows for `/api/v1`, backward compatibility for unversioned routes, or conditions for introducing `/api/v2`. `PAGINATION_FILTERING_VERSIONING_BEHAVIOR_UNCLEAR PFVBU-01` applies to future compatibility promises that are not backed by source or an accepted decision.

## Recommended Change Protocol

Before a breaking change, identify all frontend request functions, external integration callers, OpenAPI annotations, tests, endpoint and group documentation, and database values involved. Introduce a documented compatibility decision before adding another version. Phase 6 records the current surface but does not propose or implement a versioning strategy.
