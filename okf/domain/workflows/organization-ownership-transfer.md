---
okf_document_id: "workflow-organization-ownership-transfer"
title: "Organization Ownership Transfer Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-OWNERSHIP"
workflow_name: "Organization Ownership Transfer"
workflow_domain: "DOM-TENANT"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/organizations.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../membership-and-ownership.md"
  - "../../security/resource-ownership.md"
related_diagrams:
  - "../diagrams/tenant-membership-workflow.mmd"
---

# Organization Ownership Transfer Workflow

## Workflow Identity

- ID/name/domain: `WF-OWNERSHIP`, Organization Ownership Transfer, `DOM-TENANT`.
- Trigger/actor: current organization owner selects another active member.
- Status/confidence: `IMPLEMENTED`; High.

## Preconditions

Authenticated active tenant context; caller role exactly owner; target differs from caller; target has active membership in the same organization.

## Main Flow

1. Enforce owner-only action and reject self-transfer.
2. Load target active membership.
3. Begin a database transaction.
4. Demote current owner membership to `admin`.
5. Promote target membership to `owner`, ensure active status, and set `joined_at` if absent.
6. Update `organizations.owner_id` to target user.
7. Commit.
8. Record ownership-transfer audit after commit.
9. Reload and return the new owner membership.

## Alternative Flows

An owner may assign additional owner roles through member role update without changing `organizations.owner_id`; the exact relationship between multiple owner memberships and root `owner_id` is `OWNERSHIP_RULE_UNCLEAR`.

## Failure Flows

Non-owner, self-target, inactive/missing target, or DB error rejects. Transaction errors roll back all three ownership writes. Audit/reload can fail after commit.

## State Changes

Caller role `owner` to `admin`; target role to `owner`/active; organization owner reference changes.

## Data Changes

Two membership updates, one organization update, and usually one audit row.

## Transaction Boundaries

Ownership data changes are atomic. Audit and response reload are outside the transaction.

## Side Effects

Audit only; no notification email was found.

## Completion Criteria

Transaction commits with aligned target owner role and `owner_id`; returned target membership confirms current state.

## Tests

No database workflow or concurrent transfer test found.

## Unknowns and Risks

Multiple owners versus singular `owner_id`, lack of notification, after-commit response failure, and concurrent owner operations need explicit policy/testing.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

