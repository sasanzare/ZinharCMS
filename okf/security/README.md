---
okf_document_id: "security-readme"
title: "Authentication, Authorization, and Security Architecture"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware"
  - "backend/src/services/rbac.rs"
  - "backend/src/services/rls.rs"
  - "backend/migrations"
  - "frontend/src/stores/useAppStore.ts"
related_documents:
  - "../api/README.md"
  - "../backend/README.md"
  - "../frontend/README.md"
  - "../database/README.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
  - "diagrams/authentication-flow.mmd"
  - "diagrams/session-token-lifecycle.mmd"
  - "diagrams/authorization-decision-flow.mmd"
  - "diagrams/rbac-model.mmd"
  - "diagrams/tenant-access-control.mmd"
---

# Authentication, Authorization, and Security Architecture

## Scope

This Phase 7 section documents the security behavior observable in the repository at commit `8b8c091bdcbba340287d7d31dbae31544ff21d59`. Source code, migrations, configuration parsing, and tests remain authoritative. This documentation does not certify a deployed environment, prove absence of vulnerabilities, or change a control.

## Reading Order

1. [Overview](overview.md)
2. [Trust Boundaries](trust-boundaries.md)
3. [Authentication Architecture](authentication-architecture.md)
4. [Authentication Flows](authentication-flows.md)
5. [Session and Token Lifecycle](session-and-token-lifecycle.md)
6. [Password and Credential Handling](password-and-credential-handling.md)
7. [Authorization Architecture](authorization-architecture.md)
8. [RBAC Model](rbac-model.md)
9. [Roles and Permissions Catalog](roles-and-permissions-catalog.md)
10. [Tenant Access Control](tenant-access-control.md)
11. [Resource Ownership](resource-ownership.md)
12. [Administrative Access](administrative-access.md)
13. [Frontend Security Boundaries](frontend-security-boundaries.md)
14. [Input Validation](input-validation.md)
15. [Secrets and Configuration](secrets-and-configuration.md)
16. [Browser and HTTP Security](browser-and-http-security.md)
17. [Audit and Security Events](audit-and-security-events.md)
18. [Security Testing](security-testing.md)
19. [Threat Register](threat-register.md)
20. [Security Risks](security-risks.md)

## Role Documents

Global roles: [super admin](roles/global-super-admin.md), [admin](roles/global-admin.md), [editor](roles/global-editor.md), [author](roles/global-author.md), and [viewer](roles/global-viewer.md).

Organization roles: [owner](roles/organization-owner.md), [admin](roles/organization-admin.md), [editor](roles/organization-editor.md), [author](roles/organization-author.md), [viewer](roles/organization-viewer.md), and [billing manager](roles/organization-billing-manager.md).

## Permission-Group Documents

- [Authentication and Session](permissions/authentication-and-session.md)
- [Global Administration and Built-in Plugins](permissions/global-administration-and-plugins.md)
- [Organization Administration](permissions/organization-administration.md)
- [Content, Workflow, and Comments](permissions/content-workflow-and-comments.md)
- [Pages, Media, Components, and Webhooks](permissions/pages-media-components-and-webhooks.md)
- [Billing and SaaS Operations](permissions/billing-and-saas-operations.md)
- [Marketplace Management](permissions/marketplace-management.md)
- [Marketplace Runtime Capabilities](permissions/marketplace-runtime-capabilities.md)

## Diagram Navigation

- [Trust boundaries](diagrams/trust-boundaries.mmd)
- [Authentication flow](diagrams/authentication-flow.mmd)
- [Session and token lifecycle](diagrams/session-token-lifecycle.mmd)
- [Authorization decision flow](diagrams/authorization-decision-flow.mmd)
- [RBAC model](diagrams/rbac-model.mmd)
- [Tenant access control](diagrams/tenant-access-control.mmd)

## Cross-Domain Navigation

Use [API Authentication](../api/authentication.md), [API Authorization](../api/authorization.md), and [API Tenant Context](../api/tenant-context.md) for endpoint contracts. Use [Backend Request Handling](../backend/request-handling.md) for middleware composition, [Frontend Authentication and Access](../frontend/authentication-and-access.md) for client behavior, and [Database Multi-Tenancy](../database/multi-tenancy.md) for schema and RLS evidence.

## Verification Boundary

`VERIFIED` means a claim is directly supported by the inspected repository snapshot. It does not mean the control was penetration-tested or verified in production. Items marked `*_UNCLEAR`, `*_UNVERIFIED`, `FRONTEND_ONLY_SECURITY_CHECK`, or `POTENTIAL_SECRET_EXPOSURE` need further engineering or owner review before being treated as an assurance.
