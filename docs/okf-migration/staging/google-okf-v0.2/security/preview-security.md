---
type: Security Control
title: Preview Security
description: Short-lived single-use preview tickets, origin/protocol checks, and active preview revalidation.
status: stable
sources:
  - id: source-preview-tickets
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/preview_tickets.rs
    title: backend/src/services/preview_tickets.rs at construction commit
  - id: source-pages
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/pages.rs
    title: backend/src/routes/pages.rs at construction commit
  - id: source-config
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/config.rs
    title: backend/src/config.rs at construction commit
---

# Preview handoff

The protected page flow issues a random preview ticket and stores only its
hash in Redis. The public preview router consumes the ticket with a single-use
read/delete operation, validates the audience, page, user, organization,
authentication version, expiry, origin, and protocol constraints, and applies
the configured short lifetime and rate limits.

While a preview connection is active, the page route periodically revalidates
the current preview context. Changes to authentication, membership,
permission, or page access can therefore close the preview instead of leaving
an old authorization decision active.

This Concept records the implemented ticket and revalidation controls. It does
not claim a broader public delivery policy or a production Redis topology.

The route separation is documented in [runtime and request boundaries](/architecture/runtime-and-request-boundaries.md), and page validation and lifecycle are described in [page builder and preview](/domain/page-builder-and-preview.md).
