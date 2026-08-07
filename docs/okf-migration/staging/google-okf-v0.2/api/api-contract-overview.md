---
type: API Contract
title: API Contract Overview
description: Observed backend API route families and the boundary between generated route documentation and a complete compatibility contract.
status: draft
sources:
  - id: source-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/mod.rs
    title: backend/src/routes/mod.rs at construction commit
  - id: source-frontend-api
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/services/api.ts
    title: frontend/src/services/api.ts at construction commit
  - id: source-error
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/error.rs
    title: backend/src/error.rs at construction commit
---

# Observed API surface

The route tree exposes public health/readiness, authentication, delivery,
preview, media, protected organization, content, pages, comments, billing,
plugins, webhooks, and Marketplace families. The frontend API service
centralizes calls for the browser feature surface and coordinates access-token
attachment, organization context, credential handling, refresh retry, and blob
responses.

The backend also exposes an OpenAPI JSON route and uses route annotations for
generated documentation. The repository evidence supports an observed route
inventory and selected annotations; it does not establish that the generated
document is a complete, versioned, or compatibility-governed public contract.

Error responses use a JSON error/message shape at the application boundary,
with generic client-facing text for internal failures. Exact compatibility,
versioning, deprecation, and ownership rules remain an open migration decision.

Authentication details are in [authentication and session contract](/api/authentication-and-session-contract.md), and client context handling is in [routing and state](/frontend/routing-and-state.md).
