# V3 Marketplace Initial Policy

This is the initial review, approval, rejection, moderation, and takedown policy for ZinharCMS V3 Marketplace.

## Policy Principles

- Registry first: no uploaded artifact is executed directly.
- Review first: no product appears in the public catalog before approval.
- Tenant safe: installed products must never access another organization's data.
- Permission explicit: every requested permission must be declared and approved.
- Auditable: submission, review, install, purchase, update, rollback, and takedown decisions must be logged.
- Reversible: every installed product must support disable, uninstall, and rollback where applicable.
- Conservative by default: unclear risk is quarantined, not approved.

## Submission Requirements

Every submission must include:

- creator identity
- product type
- listing title and description
- category
- manifest
- semantic version
- package artifact
- checksum
- compatibility range
- permission request list
- changelog
- support contact
- license or usage terms

Missing required fields send the submission back for changes.

## Review Decisions

Reviewers can use these decisions:

- Approve: the product is safe, complete, compatible, and eligible for the catalog.
- Request changes: the product is close but missing metadata, assets, docs, compatibility detail, or minor safety clarification.
- Reject: the product violates policy or does not fit the Marketplace scope.
- Quarantine: the product may be valid but requires internal security, legal, or runtime evaluation.
- Suspend: the published product is temporarily hidden or blocked from new installs.
- Takedown: the product is removed from discovery and blocked because of policy, security, legal, or abuse concerns.

## Approval Rules

A product can be approved only when:

- it matches a supported product type
- it has a valid manifest
- its checksum matches the reviewed package
- it has a compatible target ZinharCMS version range
- requested permissions are supported
- the creator is allowed to publish
- the package passes static validation
- no high-risk security issue remains open
- installation can be audited
- disable and rollback behavior is known

## Rejection Rules

A product must be rejected when it:

- is classified as Unsupported
- bypasses RBAC, tenant context, or audit logging
- attempts cross-organization data access
- requires direct database credentials
- hides executable behavior
- requests undeclared permissions
- cannot be reviewed from package and manifest data
- impersonates another creator or product
- includes malicious, deceptive, copied, or prohibited content
- cannot be safely disabled or blocked

## Quarantine Rules

Quarantine is used when:

- a security risk is plausible but unconfirmed
- runtime behavior cannot be evaluated automatically
- a dependency is unknown, obfuscated, or unverifiable
- legal ownership is disputed
- a permission request does not map cleanly to the existing permission model

Quarantined products do not appear in the catalog and cannot be installed.

## Moderation And Takedown

Published products can be suspended or taken down when:

- a security issue is reported
- abuse is confirmed
- tenant isolation is at risk
- package checksum no longer matches the reviewed artifact
- the creator loses publishing eligibility
- the product breaks compatibility promises
- payment, refund, or licensing rules are violated

Emergency takedown must block new installations immediately. Existing installations may be disabled through a kill switch in later V3 phases.

## Phase 15 Final Launch Policy

This is the final policy baseline for Launch Readiness and General Availability.

- Only reviewed and approved products can be installed in production.
- Launch support must treat broken install, malicious product, wrong payment, refund/dispute, payout issue, and emergency block as first-class incident classes.
- Support workflow must use existing Marketplace records first: installation, purchase, entitlement, revenue ledger, report, moderation queue, internal notification, beta feedback, and admin analytics.
- Product rollback must use the installed-app rollback API when a safe rollback version exists; direct database edits are not a support workflow.
- A malicious product must be suspended, unpublished, emergency blocked, or kill-switched using existing moderation/runtime controls.
- Wrong payment handling must preserve purchase, entitlement, and ledger invariants; partial refunds and automated payout transfer execution remain outside the current runtime contract.
- Public docs, release notes, and the operations runbook must be available before GA.
- Marketplace GA does not enable uploaded package execution, approval bypass, anonymous install, or automated payout transfer execution.

## Phase 0.1 Decision Matrix

| Submission | Decision |
| --- | --- |
| Component Pack with valid manifest and no risky permissions | Approve or request changes |
| Design Template with organization-local assets and rollback path | Approve or request changes |
| Integration Plugin with undeclared network behavior | Reject or quarantine |
| Backend Extension before sandbox support | Reject or defer |
| Product requiring direct database access | Reject |
| Product that can read another tenant's data | Reject and escalate |
| Product with unclear ownership or copied assets | Quarantine |
| Product outside accepted taxonomy | Unsupported |
