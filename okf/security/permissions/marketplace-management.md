---
okf_document_id: "security-permission-marketplace-management"
title: "Marketplace Management Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "marketplace-management"
permission_group_name: "Marketplace Management"
resource_domain: "Marketplace creator, catalog, review, installation, feedback, finance, and kill switches"
permission_scope: "creator ownership, global platform, and organization"
implementation_status: "verified"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/routes/marketplace_analytics.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/services/rbac.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../resource-ownership.md"
  - "../administrative-access.md"
  - "../../api/endpoints/marketplace-installation-lifecycle.md"
related_diagrams:
  - "../diagrams/authorization-decision-flow.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Marketplace Management Permission Group

## Included Permissions and Operations

Creator profile/listing/version/submission ownership; platform review/moderation/abuse/analytics; organization install/purchase/update/rollback/enable/disable/uninstall and permission approval; creator payout ownership; global finance verification; organization/global kill switches.

## Scope and Roles

- Creator operations require the authenticated creator owner and lifecycle eligibility.
- Platform review/moderation/analytics and selected finance operations require global admin, with super-admin override.
- Installation, purchase, and organization runtime control require organization owner/admin.
- Creator payouts combine creator ownership, tenant/product gates, and selected organization helper checks.

## Backend Enforcement and API

Marketplace route/service modules combine RBAC, owner checks, entitlements, product/version state, permission snapshots, artifact checks, RLS or bypass, and audit. Consult all Marketplace endpoint-family documents, especially installation, runtime security, finance, feedback, analytics, and adapters.

## Frontend Checks

`MarketplacePage` hides/restricts controls using local global and organization roles, creator state, entitlement, confirmations, and permissions. These are `FRONTEND_ONLY_SECURITY_CHECK FOSC-01`.

## Database and Tests

Marketplace tables span global catalog/review entities and tenant installations/purchases/runtime state. Tests include unit rules and static security contracts, but not a complete runtime attack suite.

## Unclear Semantics

The overlap of creator ownership, tenant role, global role, entitlement, and RLS bypass is `AUTHORIZATION_ENFORCEMENT_UNCLEAR AEU-01` unless verified at the exact handler.
