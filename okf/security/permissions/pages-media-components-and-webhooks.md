---
okf_document_id: "security-permission-pages-media-components-webhooks"
title: "Pages, Media, Components, and Webhooks Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "pages-media-components-webhooks"
permission_group_name: "Pages, Media, Components, and Webhooks"
resource_domain: "pages, versions, component registry, media, and CMS webhooks"
permission_scope: "organization with legacy global helpers"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/media.rs"
  - "backend/src/routes/webhooks.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../input-validation.md"
  - "../../api/endpoints/pages-workflow-versions-and-preview.md"
  - "../../api/endpoints/media-library.md"
  - "../../api/endpoints/cms-webhooks.md"
related_diagrams:
  - "../diagrams/authorization-decision-flow.mmd"
  - "../diagrams/rbac-model.mmd"
---

# Pages, Media, Components, and Webhooks Permission Group

## Included Permissions

Named capabilities are page writer/publisher/manager, component registry manager, media writer, workflow reviewer, and webhook manager.

## Operations, Scope, and Roles

- Page/media write: owner/admin/editor/author.
- Page publish/manage/review: owner/admin/editor.
- Component registry: owner/admin/editor.
- Webhooks: owner/admin.

Read handlers often rely on active tenant membership and RLS without a dedicated reader helper; consult endpoint contracts.

## Backend Enforcement and API

`pages.rs`, `media.rs`, and `webhooks.rs` call named organization helpers. Preview also requires token and tenant context, with query-string compatibility. See the pages, media, component, and webhook endpoint families.

## Frontend Checks

Page, media, workflow, and settings views present controls based on available state and server errors; frontend behavior is not a security boundary.

## Database Implications

Pages, versions, custom components, media metadata/variants, webhooks, and deliveries are tenant-scoped and use forced RLS. Media files live on the filesystem; system components have special RLS read/write semantics.

## Tests and Unclear Semantics

Workflow, media-processing, webhook, page UI, and hardening tests cover selected behavior. Static file serving, upload security, and exhaustive reader authorization remain `SECURITY_TEST_COVERAGE_UNCLEAR STCU-01`.
