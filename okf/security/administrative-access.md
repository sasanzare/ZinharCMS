---
okf_document_id: "security-administrative-access"
title: "Administrative Access"
project: "ZinharCMS"
category: "security-authorization"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/routes/beta.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/organizations.rs"
related_documents:
  - "rbac-model.md"
  - "tenant-access-control.md"
  - "roles/global-super-admin.md"
  - "roles/organization-owner.md"
related_diagrams:
  - "diagrams/authorization-decision-flow.mmd"
---

# Administrative Access

## Administrative Planes

| Plane | Roles | Examples |
| --- | --- | --- |
| Global platform | `super_admin`, `admin` | Product beta administration, Marketplace review/moderation/analytics, finance verification, global kill switch, built-in plugins |
| Organization | `owner`, `admin` | Members, invitations, domains, rate limits, audit visibility, webhooks, Marketplace lifecycle/kill switch |
| Organization billing | `owner`, `admin`, `billing_manager` | Subscription, checkout, portal, usage rebuild |
| Editorial | `owner`, `admin`, `editor` | Publish/review, content types, pages, components, comment moderation |

Global `super_admin` is a global RBAC override. Organization `owner` is an organization RBAC override. These are independent privileges.

## RLS Bypass Operations

Explicit bypass transactions appear in global beta, Marketplace catalog/review/analytics/runtime/finance, Stripe billing, and related cross-organization paths. Callers are expected to perform global role, ownership, provider-signature, or other checks before or around bypass use.

The helper itself does not accept claims or enforce a role. `ADMINISTRATIVE_BYPASS_UNCLEAR ABY-01` therefore requires every new bypass caller to document its authorization precondition, scope, audit behavior, and test.

## Bootstrap Access

An empty user database causes startup to create a deterministic development administrator, and the first public registration becomes `super_admin`. This is a high-impact bootstrap boundary. `POTENTIAL_SECRET_EXPOSURE PSE-01` and `AUTHENTICATION_FLOW_UNCLEAR AFU-01` apply because no production-only disable switch or installation ceremony was found.

## Audit Expectations

Many organization and Marketplace administrative mutations write audit events, but authentication, authorization denials, global-role assignment, and every bypass entry are not uniformly audited. See `AUDIT_COVERAGE_UNCLEAR ACU-01`.
