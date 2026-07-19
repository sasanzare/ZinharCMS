---
okf_document_id: "security-authorization-architecture"
title: "Authorization Architecture"
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
  - "backend/src/routes/mod.rs"
  - "backend/src/services/rbac.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/routes"
related_documents:
  - "rbac-model.md"
  - "tenant-access-control.md"
  - "resource-ownership.md"
  - "../api/authorization.md"
related_diagrams:
  - "diagrams/authorization-decision-flow.mmd"
  - "diagrams/rbac-model.mmd"
---

# Authorization Architecture

## Decision Layers

Authorization is a distributed decision, not one policy-engine call:

1. Router placement determines whether identity and tenant context are required.
2. Global RBAC checks use `Claims.role` from the access token.
3. Organization RBAC checks use `TenantContext.role` loaded from active membership.
4. Resource checks enforce creator, author, membership, entitlement, lifecycle, or subject relationships in selected handlers/services.
5. Tenant-aware SQL and forced RLS constrain database rows.
6. Marketplace runtime checks installation status, approved permissions, operation mapping, payload/entry-point policy, and kill switches.

All applicable layers must permit an operation. Frontend visibility is not part of the authoritative decision.

## Global and Organization Separation

Global roles govern authentication-only platform paths such as built-in plugins, product beta administration, Marketplace review/moderation/analytics, finance verification, and global kill switches. Organization roles govern tenant-scoped content, pages, media, comments, webhooks, organization administration, billing, and Marketplace installation/runtime controls.

The string values `admin`, `editor`, `author`, and `viewer` exist in both namespaces but are assigned, loaded, and checked differently. A global role does not automatically become an organization role after initial default-organization mapping.

## Override Semantics

- Global `super_admin` passes every `require_any` global check even when not listed.
- Organization `owner` passes every `require_org_any` organization check even when not listed.
- Neither override skips authentication, tenant membership, resource checks, lifecycle checks, entitlements, or RLS.
- Explicit RLS bypass is a separate backend mechanism and is not derived automatically from either override.

## Enforcement Gaps and Ambiguity

`AUTHORIZATION_ENFORCEMENT_UNCLEAR AEU-01`: role checks are distributed across many handlers, and no complete endpoint-by-role integration matrix was found. The route catalog is therefore evidence, not a proof that every intended action has a check.

`RBAC_MAPPING_UNCLEAR RMU-01`: database `roles.permissions` arrays are seeded and updated, but runtime RBAC reads role names, not those arrays.

`PERMISSION_SEMANTICS_UNCLEAR PSU-01`: legacy colon-delimited database permissions, named RBAC capabilities, and Marketplace dot-delimited runtime permissions are distinct vocabularies with no unified resolver.
