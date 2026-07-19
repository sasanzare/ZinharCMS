---
okf_document_id: "domain-readme"
title: "Business Rules and Domain Workflows"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "backend/migrations"
  - "frontend/src"
related_documents:
  - "../backend/services-and-domain.md"
  - "../database/entity-catalog.md"
  - "../api/endpoint-catalog.md"
  - "../security/README.md"
related_diagrams:
  - "diagrams/domain-map.mmd"
  - "diagrams/cross-module-orchestration.mmd"
---

# Business Rules and Domain Workflows

## Purpose and Scope

This section records the business behavior observed in the current ZinharCMS implementation. Phase 8 catalogs significant domains, rules, invariants, state transitions, workflows, validation, lifecycle behavior, side effects, tests, and risks. It does not define future product policy and does not modify application behavior.

Current source code, service orchestration, database constraints and transactions, API handlers, authorization enforcement, and tests outrank this documentation. Product intent is not automatically established by a field name, UI label, comment, or historical document.

## Interpretation Rules

- An `EXPLICIT_BUSINESS_RULE` is directly expressed by an authoritative branch, validator, transition function, transaction, or constraint.
- An `INFERRED_BUSINESS_RULE` is a repeated implementation pattern without an explicit governing contract.
- Frontend validation improves usability but is never an authorization or persistence boundary.
- `APPLICATION_ENFORCED_RULE` identifies handler, middleware, plugin, or service enforcement.
- `DATABASE_ENFORCED_RULE` identifies constraints, keys, triggers, RLS, or transaction-backed persistence behavior.
- A rule can be partially enforced when equivalent entry points or layers do not provide the same guarantee.
- Code behavior describes what the repository does; it does not by itself prove why the product should do it.

## Recommended Reading Order

1. [Domain Overview](overview.md)
2. [Domain Catalog](domain-catalog.md)
3. The relevant [domain document](domains/)
4. [Business Rule Catalog](business-rule-catalog.md)
5. [Cross-Module Workflows](cross-module-workflows.md) and the relevant workflow
6. [Invariants](invariants.md)
7. [State Transitions](state-transitions.md)
8. The relevant lifecycle, validation, testing, and risk documents

## Primary Documents

- [Domain Overview](overview.md)
- [Domain Catalog](domain-catalog.md)
- [Business Rule Catalog](business-rule-catalog.md)
- [Invariants](invariants.md)
- [State Transitions](state-transitions.md)
- [Cross-Module Workflows](cross-module-workflows.md)
- [Content Lifecycle](content-lifecycle.md)
- [Publication Workflow](publication-workflow.md)
- [Revisions and Versioning](revisions-and-versioning.md)
- [Deletion and Restoration](deletion-and-restoration.md)
- [Multi-Tenancy Behavior](multi-tenancy-behavior.md)
- [Membership and Ownership](membership-and-ownership.md)
- [Page Builder Rules](page-builder-rules.md)
- [Validation Rules](validation-rules.md)
- [Settings and Configuration Rules](settings-and-configuration-rules.md)
- [Background Processes](background-processes.md)
- [Domain Events](domain-events.md)
- [Business Rule Testing](business-rule-testing.md)
- [Domain Risks](domain-risks.md)

## Domain Documents

- [Identity and Access](domains/identity-and-access.md)
- [Organizations and Membership](domains/organizations-and-membership.md)
- [Content and Editorial Workflow](domains/content-and-editorial.md)
- [Pages and Page Builder](domains/pages-and-page-builder.md)
- [Media Library](domains/media-library.md)
- [Delivery, Settings, and Webhooks](domains/delivery-settings-and-webhooks.md)
- [Billing and Quotas](domains/billing-and-quotas.md)
- [SaaS Operations and Beta](domains/saas-operations-and-beta.md)
- [Plugins and Components](domains/plugins-and-components.md)
- [Marketplace](domains/marketplace.md)

## Workflow Documents

- [Organization Provisioning](workflows/organization-provisioning.md)
- [Tenant Invitation and Membership](workflows/tenant-invitation-and-membership.md)
- [Organization Ownership Transfer](workflows/organization-ownership-transfer.md)
- [Content Entry Save](workflows/content-entry-save.md)
- [Editorial Publication](workflows/editorial-publication.md)
- [Page Builder Save and Version](workflows/page-builder-save-and-version.md)
- [Page Version Restoration](workflows/page-version-restoration.md)
- [Media Upload and Processing](workflows/media-upload-and-processing.md)
- [Publication Webhook Delivery](workflows/publication-webhook-delivery.md)
- [Billing Subscription](workflows/billing-subscription.md)
- [Beta Feedback and Readiness](workflows/beta-feedback-and-readiness.md)
- [Marketplace Product Publication](workflows/marketplace-product-publication.md)
- [Marketplace Installation Lifecycle](workflows/marketplace-installation-lifecycle.md)
- [Marketplace Purchase and Entitlement](workflows/marketplace-purchase-and-entitlement.md)

## Diagrams

- [Domain Map](diagrams/domain-map.mmd)
- [Content Lifecycle](diagrams/content-lifecycle.mmd)
- [Publication Workflow](diagrams/publication-workflow.mmd)
- [Tenant Membership Workflow](diagrams/tenant-membership-workflow.mmd)
- [Page Builder Workflow](diagrams/page-builder-workflow.mmd)
- [Cross-Module Orchestration](diagrams/cross-module-orchestration.mmd)

## Cross-Phase References

- [Backend Services and Domain](../backend/services-and-domain.md)
- [Frontend Feature Catalog](../frontend/feature-catalog.md)
- [Database Entity Catalog](../database/entity-catalog.md)
- [API Endpoint Catalog](../api/endpoint-catalog.md)
- [Security RBAC Model](../security/rbac-model.md)
- [Tenant Access Control](../security/tenant-access-control.md)
- [Resource Ownership](../security/resource-ownership.md)

## For AI Coding Agents

1. Start with `domain-catalog.md`.
2. Read the relevant domain document.
3. Read `business-rule-catalog.md`.
4. Review the relevant workflow document.
5. Review `invariants.md`.
6. Review state transitions before changing status behavior.
7. Verify rules against current source code and tests.
8. Verify access rules against Phase 7 security documents.
9. Verify persistence rules against Phase 5 database documents.
10. Never invent undocumented business behavior.
11. Never treat frontend-only validation as authoritative enforcement.
12. Update related OKF documents when business behavior changes.

