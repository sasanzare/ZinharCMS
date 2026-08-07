---
type: Architecture
title: Runtime and Request Boundaries
description: Observed public, authenticated, tenant-scoped, and preview request boundaries in the backend route tree.
status: draft
sources:
  - id: source-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/mod.rs
    title: backend/src/routes/mod.rs at construction commit
  - id: source-pages
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/pages.rs
    title: backend/src/routes/pages.rs at construction commit
  - id: source-delivery
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/delivery.rs
    title: backend/src/routes/delivery.rs at construction commit
---

# Observed request layers

The backend route tree exposes public health/readiness and delivery routes,
public authentication routes, authenticated routes, tenant-protected routes,
and a separate public preview router. The protected and tenant-protected
groups apply middleware in the router composition rather than representing
independent services.

Tenant-protected requests require organization context and pass through the
tenant middleware before reaching content, pages, media, Marketplace, plugin,
billing, analytics, finance, runtime, comments, and webhook route families.
Public delivery selects published content through an observed default
organization path; custom-domain or host-based organization routing remains an
owner decision and is not asserted here.

The preview path is intentionally separate from ordinary public delivery. It
uses a short-lived ticket and performs revalidation while a preview connection
is active; the detailed control is documented in [preview security](/security/preview-security.md).

## Construction boundary

This Concept describes route and middleware composition observed in source. It
does not establish a public API compatibility policy, deployment topology, or
the intended final organization-routing policy.
