---
type: Domain Workflow
title: Page Builder and Preview
description: Page JSON validation, component registration, version snapshots, protected preview issuance, and public preview handling.
status: draft
sources:
  - id: source-pages
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/pages.rs
    title: backend/src/routes/pages.rs at construction commit
  - id: source-preview-tickets
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/preview_tickets.rs
    title: backend/src/services/preview_tickets.rs at construction commit
  - id: source-frontend-router
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/router.tsx
    title: frontend/src/router.tsx at construction commit
---

# Page lifecycle and preview

Protected page routes provide page CRUD, status handling, component registry
access, version snapshots, validation, and sanitization of page JSON. The
builder is part of the administrative route surface and uses the tenant and
authorization boundaries established by the backend.

Preview is issued from the protected page flow and consumed through a separate
public preview router. The ticket is short-lived and single-use; origin,
protocol, audience, user, organization, and page access are checked before
content is returned. Active preview sessions are periodically revalidated.

The component registry and JSON validation are repository observations. This
Concept does not define a durable component compatibility policy or promise a
production preview topology. The security control is detailed in [preview security](/security/preview-security.md).

## Preserved visualizations

### page-builder-workflow

```mermaid
flowchart LR
    Editor["Authenticated editor"] --> Validate["Validate and sanitize page JSON"]
    Validate --> Save["Save page and version snapshot"]
    Save --> Issue["Issue short-lived preview ticket"]
    Issue --> Preview["Public preview router"]
    Preview --> Revalidate["Periodic access revalidation"]
```

### page-builder-flow

```mermaid
sequenceDiagram
    participant Browser as Admin browser
    participant PageAPI as Page API
    participant Preview as Preview router
    Browser->>PageAPI: Edit and save page
    PageAPI-->>Browser: Page/version result
    Browser->>PageAPI: Request preview ticket
    PageAPI-->>Browser: Short-lived ticket
    Browser->>Preview: Open preview with ticket protocol
    Preview-->>Browser: Preview content or close on failed revalidation
```
