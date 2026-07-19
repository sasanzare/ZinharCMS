---
okf_document_id: "workflow-organization-provisioning"
title: "Organization Provisioning Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-ORG-PROVISION"
workflow_name: "Organization Provisioning"
workflow_domain: "DOM-TENANT"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/services/quota.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/organizations-and-membership.md"
  - "../multi-tenancy-behavior.md"
related_diagrams:
  - "../diagrams/tenant-membership-workflow.mmd"
---

# Organization Provisioning Workflow

## Workflow Identity

- ID/name/domain: `WF-ORG-PROVISION`, Organization Provisioning, `DOM-TENANT`.
- Trigger/actor: authenticated user submits create organization.
- Status/confidence: `IMPLEMENTED`; High.

## Preconditions

Valid authentication; nonempty name; normalized slug; globally unused organization slug; available default plan row. No existing tenant context or global admin role is required.

## Main Flow

1. Normalize name and slug.
2. Begin a database transaction.
3. Insert an active organization with the caller as `owner_id`.
4. Insert active `owner` membership with `joined_at`.
5. Insert the default free/manual subscription through the quota service.
6. Commit the transaction.
7. Record an organization audit event outside the core transaction.
8. Load membership and plan limits and return organization detail.

## Alternative Flows

Existing users can create more than one organization. The migration-created `default` organization is a separate bootstrap path and does not run this handler.

## Failure Flows

Validation or uniqueness failures occur before/within the transaction. A missing default plan or DB error rolls back core rows. Audit or final detail load can fail after commit, causing an error response even though the organization exists.

## State Changes

Organization becomes `active`; caller membership becomes `active`/`owner`; subscription becomes `active` on the default plan.

## Data Changes

Creates one organization, one member, one subscription, then usually one audit row.

## Transaction Boundaries

Organization/member/subscription are atomic. Audit and response reload are not part of that transaction.

## Side Effects

Database writes only; no default content, page, media, navigation, webhook, or email initialization.

## Completion Criteria

Core transaction commits and organization detail is returned. Operators should treat post-commit audit/reload failure as a possible partial response failure.

## Tests

Quota default-subscription code is covered indirectly; no full database workflow/rollback test was found.

## Unknowns and Risks

Default plan absence, audit-after-commit semantics, bootstrap/default-organization interaction, and intended organization count per user are not governed as product policy.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

