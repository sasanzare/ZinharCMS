---
okf_document_id: "security-permission-billing-saas"
title: "Billing and SaaS Operations Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "billing-saas-operations"
permission_group_name: "Billing and SaaS Operations"
resource_domain: "subscriptions, usage, quotas, beta operations, and provider billing"
permission_scope: "organization and selected global platform operations"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/billing.rs"
  - "backend/src/routes/beta.rs"
  - "backend/src/services/stripe_billing.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../administrative-access.md"
  - "../roles/organization-billing-manager.md"
  - "../../api/endpoints/billing-subscription-and-usage.md"
related_diagrams:
  - "../diagrams/authorization-decision-flow.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Billing and SaaS Operations Permission Group

## Included Permissions and Operations

Subscription reads/changes, checkout, customer portal, usage read/rebuild, quota/rate checks, beta feedback/blockers, participant administration, and Stripe webhook processing.

## Scope and Roles

Named billing management allows organization owner/admin/billing manager. Tenant beta moderation uses owner/admin/editor in selected handlers. Global product dashboards and participant administration require global admin, with super-admin override. The public Stripe webhook uses provider signature validation rather than user RBAC.

## Backend, Frontend, and API

`billing.rs` and `beta.rs` perform role, plan, quota, provider, and lifecycle checks. Billing/Beta pages use local role cues, which are not authoritative. See billing and beta endpoint families.

## Database Implications

Subscriptions, usage counters, rate limits, beta records, audit logs, and operational records are organization-scoped; provider processing may use explicit RLS bypass to locate cross-organization state.

## Tests and Unclear Semantics

RBAC, quota, Stripe signature/flow, and GA-readiness tests cover selected paths. Provider webhook replay/idempotency and every bypass path require route-specific review. `ADMINISTRATIVE_BYPASS_UNCLEAR ABY-01` remains applicable.
