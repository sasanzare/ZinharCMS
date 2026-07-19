---
okf_document_id: "workflow-tenant-invitation-membership"
title: "Tenant Invitation and Membership Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-MEMBERSHIP"
workflow_name: "Tenant Invitation and Membership"
workflow_domain: "DOM-TENANT"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/services/email.rs"
  - "backend/src/services/quota.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../membership-and-ownership.md"
  - "../../security/tenant-access-control.md"
related_diagrams:
  - "../diagrams/tenant-membership-workflow.mmd"
---

# Tenant Invitation and Membership Workflow

## Workflow Identity

- ID/name/domain: `WF-MEMBERSHIP`, Tenant Invitation and Membership, `DOM-TENANT`.
- Trigger/actors: owner/admin invites; authenticated recipient accepts.
- Status/confidence: `IMPLEMENTED`; High predicates, Medium end-to-end guarantee.

## Preconditions

Inviter has active tenant context and admin capability; only owner can invite an owner. Email/role are valid and member capacity allows the pending invite. Acceptance requires authenticated user email match, token hash match, pending status, future expiry, and member capacity.

## Main Flow

1. Validate inviter role, target email, requested member role, and capacity.
2. Generate an opaque token and persist only its hash with `pending` invitation and expiry.
3. Upsert the pending invitation for organization/email.
4. Send invitation email and record invitation-create audit.
5. Recipient submits token while authenticated.
6. Query token, pending status, expiry, and matching user email in a transaction.
7. Recheck member capacity.
8. Insert or reactivate membership with invitation role and active status.
9. Mark invitation `accepted` with timestamp and commit.
10. Record acceptance audit and return active membership.

## Alternative Flows

Admin can revoke a pending invitation. Listing invitations first expires overdue pending rows. Existing membership is reactivated/upserted, retaining `joined_at` when already present.

## Failure Flows

Invalid/expired/wrong-recipient token is reported as not found. Role/email/quota errors reject creation or acceptance. Email/audit failure can occur outside the membership transaction. Concurrent capacity consumption is not protected by one verified lock.

## State Changes

Invitation: `pending` to `accepted`, `revoked`, or `expired`. Membership: absent/suspended/invited to `active` on acceptance; complete prior graph is unclear.

## Data Changes

Invitation row, possible email-delivery row, membership upsert, and audit records.

## Transaction Boundaries

Membership activation and invitation acceptance are atomic. Capacity reads, email, and audit are not all inside the same transaction.

## Side Effects

Invitation email/provider attempt and audit logs.

## Completion Criteria

Acceptance transaction commits and active membership can be loaded. Tenant access begins on subsequent requests using that organization ID.

## Tests

No end-to-end token/email/expiry/quota/transaction test was found.

## Unknowns and Risks

Capacity races, email delivery guarantee, invitation token rotation after upsert, and suspended-member reactivation policy need owner confirmation.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

