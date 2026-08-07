---
type: Component
title: Module Boundaries
description: Current Rust route, middleware, service, plugin, and state boundaries within the modular backend.
status: draft
sources:
  - id: source-lib
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/lib.rs
    title: backend/src/lib.rs at construction commit
  - id: source-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/mod.rs
    title: backend/src/routes/mod.rs at construction commit
  - id: source-services
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/mod.rs
    title: backend/src/services/mod.rs at construction commit
  - id: source-plugins
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/plugins/mod.rs
    title: backend/src/plugins/mod.rs at construction commit
---

# Module layout

The crate exposes configuration, database, error, middleware, models, plugin,
route, service, and shared-state modules. The route module composes feature
families including authentication, organizations, content, pages, media,
delivery, billing, comments, webhooks, plugins, and Marketplace subdomains.

Middleware carries cross-cutting request context. Services hold reusable
authentication, session, MFA, preview-ticket, RLS, workflow, Marketplace, and
other domain operations. The plugin module defines in-process hooks and the
built-in plugin registry; it is not an external worker boundary.

This Concept records implementation boundaries, not durable ownership. The
module, operations, security, and documentation owner question remains open
in the migration decision register. The built-in plugin behavior is expanded
in [extensibility and built-in plugins](/domain/extensibility-and-built-in-plugins.md).
