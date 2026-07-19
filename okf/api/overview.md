---
okf_document_id: "api-overview"
title: "API Overview"
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
  - "backend/src/lib.rs"
  - "backend/src/main.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/README.md"
  - "api/route-architecture.md"
  - "api/endpoint-catalog.md"
  - "architecture/overview.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
  - "api/diagrams/api-request-lifecycle.mmd"
uncertainty_markers:
  - "VERSIONING_BEHAVIOR_UNCLEAR VBU-01"
---

# API Overview

## Classification

ZinharCMS exposes a Rust/Axum REST-like JSON API, a page-preview WebSocket, XML and text delivery resources, and a static upload service. The route tree is assembled in `backend/src/routes/mod.rs`, instantiated with `AppState`, and served from the backend listener configured at runtime.

The API is not one uniform surface. It has three access zones:

| Zone | Middleware context | Endpoint count | Main consumers |
| --- | --- | ---: | --- |
| Public | No authentication or tenant middleware | 17 | Login/register clients, public delivery, Stripe, health checks |
| Authenticated | `auth_middleware` | 12 | Current-user operations, organization bootstrap, global administration, plugin registry |
| Tenant protected | `tenant_middleware` | 139 | Administration UI and organization-scoped integrations |

`tenant_middleware` performs bearer-token validation as part of tenant resolution; tenant-protected routes therefore do not pass through a separate outer authentication layer.

## Protocol and Representation Inventory

- JSON is the default request and response representation.
- Authentication refresh/logout handlers accept raw bytes because they support cookie and body token input.
- The Stripe webhook accepts raw bytes plus headers for signature verification.
- Media and Marketplace version upload handlers accept `multipart/form-data`.
- `/api/v1/sitemap.xml` returns XML and `/api/v1/robots.txt` returns plain text.
- `/api/preview/{page_id}` starts as HTTP and upgrades to WebSocket.
- `/uploads/*` is served from a configured directory by `tower_http::services::ServeDir`.
- No SSE, GraphQL, gRPC, bulk export, resumable upload, or signed download surface was found.

## Path Conventions

Administrative and integration routes use `/api/*`. Public delivery uses `/api/v1/*`. System discovery and probes use `/`, `/health`, `/ready`, and `/openapi.json`. Static media is mounted under `/uploads`.

Path parameters use Axum brace syntax such as `{id}`, `{slug}`, `{type_slug}`, and `{organization_id}`. UUID parsing is generally enforced by typed `Path<Uuid>` extractors. Some public identifiers deliberately accept a UUID-or-slug string.

## API Consumers

The React frontend centralizes network calls in `frontend/src/services/api.ts`. Its 141 request functions all map to registered backend method/path pairs at the verified snapshot. Twenty-seven backend handlers have no direct frontend API wrapper; these include public delivery, webhook receivers, system OpenAPI, WebSocket preview, and several detail or administrative operations.

External consumers include Stripe, CMS webhook destinations, browsers loading public content or uploads, readiness probes, and API clients using generated OpenAPI. Runtime contracts for external systems remain provider- and deployment-dependent.

## Boundaries

This section documents transport contracts and route-level policy. Backend domain behavior is owned by [Backend Documentation](../backend/README.md); persistence behavior by [Database Architecture](../database/README.md); frontend orchestration by [Frontend Architecture](../frontend/README.md); and detailed security analysis is reserved for the recommended Phase 7.
