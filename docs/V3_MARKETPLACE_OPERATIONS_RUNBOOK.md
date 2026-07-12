# V3 Marketplace Operations Runbook

This runbook covers Phase 15 Launch Readiness, General Availability operations,
support workflow, rollback, and incident checklist for the V3 Marketplace.

## Launch Readiness

Freeze these areas during the launch window:

- Marketplace schema and route boundaries;
- install, update, rollback, permission approval, and kill-switch logic;
- purchase, refund, entitlement, and revenue ledger behavior;
- customer review, abuse report, and moderation queues;
- package validation, review policy, and creator publishing controls;
- public Marketplace documentation and support messaging.

Pre-launch checklist:

1. Confirm Phase 14 beta readiness evidence has been reviewed.
2. Confirm no P0/P1 beta blocker or Marketplace incident remains ownerless.
3. Run `cargo test --manifest-path backend/Cargo.toml marketplace`.
4. Run `scripts/marketplace-phase15-ga-check.ps1`.
5. Confirm `/health` and `/ready` pass in the launch environment.
6. Confirm the monitoring dashboard at `/api/marketplace/analytics/admin` is
   available to global admins.
7. Confirm support has access to this runbook, release notes, API docs, creator
   guide, and Marketplace policy.
8. Confirm rollback owners and communication owners are assigned.

## Support Workflow

1. Triage the ticket as creator, customer, billing, abuse, or platform incident.
2. Capture organization id, creator id, listing id, version id, installation id,
   purchase id, report id, request id, and timestamps when available.
3. Confirm whether the issue is isolated or cross-organization.
4. Use existing UI/API records before considering direct data inspection:
   installations, purchases, reports, beta feedback, analytics, audit logs, and
   internal notifications.
5. Assign severity and owner.
6. For P0/P1, freeze related Marketplace changes until an owner accepts the
   incident.
7. Record the resolution, follow-up action, and whether docs/runbook updates are
   required.

## Incident Checklist

### Broken Install

1. Confirm the installation status, installed version, rollback version, and
   artifact checksum.
2. Disable the installation if it causes active customer harm.
3. Use the rollback API when the rollback target is safe and approved.
4. Check compatibility, permissions, and package validation report.
5. Escalate to emergency block if multiple organizations are affected.
6. Record affected organizations and whether support intervention was required.

### Malicious Product

1. Treat the report as high severity until disproved.
2. Preserve report evidence, package checksum, listing id, version id, creator
   id, install count, and affected organizations.
3. Use moderation to suspend listing or unpublish the unsafe version.
4. Activate emergency block or kill switch if active installations may be unsafe.
5. Keep uploaded package code unexecuted; inspect only reviewed artifact metadata
   and static validation evidence.
6. Notify affected customers through the approved support channel.

### Wrong Payment

1. Confirm the purchase, checkout, entitlement, and revenue ledger state.
2. Check Stripe webhook ordering/idempotency when provider data is involved.
3. Do not manually edit ledger entries.
4. Use supported full-refund behavior when the payment must be reversed.
5. Confirm entitlement revocation for refunded purchases.
6. Escalate partial refund, dispute, or payout transfer questions to the release
   owner because those remain outside the current supported runtime contract.

### Refund, Dispute, Or Payout Issue

1. Capture purchase id, listing id, creator id, organization id, provider event
   id, and ledger entries.
2. Confirm whether the issue is purchase, entitlement, refund, payout onboarding,
   payout verification, or automated transfer execution.
3. Use existing payout verification records for eligibility questions.
4. Treat automated payout transfer execution as deferred; do not promise transfer
   automation from this release.
5. Record customer and creator communication in the support system.

### Abuse Report Or Review Attack

1. Check the Marketplace reports queue.
2. Confirm severity, evidence object, reporter organization, listing, and review
   ownership.
3. For critical reports, confirm an unread internal notification exists.
4. Investigate, resolve, dismiss, publish, or reject through existing moderation
   states.
5. Update the report outcome and record any customer-facing action.

### Emergency Block

1. Confirm scope: listing, version, organization, or global runtime risk.
2. Prefer the narrowest control that stops harm.
3. Use listing moderation for catalog/review risk.
4. Use global or organization kill switches for active runtime risk.
5. Confirm new installs and re-enable attempts are blocked.
6. Document the reason, actor, affected products, and lift criteria.

## Rollback Plan

Rollback is required if any of these occur:

- tenant isolation or RLS behavior fails;
- approved products cannot be installed in production;
- broken install affects multiple customer organizations and cannot be rolled
  back quickly;
- malicious product cannot be blocked quickly;
- wrong payment, refund, or entitlement behavior blocks valid customers;
- admin monitoring dashboard or readiness checks are unavailable during launch;
- frontend release blocks catalog, install, report, or support workflows.

Rollback steps:

1. Announce rollback to the release channel and support owner.
2. Stop new Marketplace launch changes.
3. Disable affected products or use emergency block when customer harm is active.
4. Restore the last known good backend/frontend deployment.
5. Restore database backup only when migration or data integrity is unsafe.
6. Confirm `/health`, `/ready`, catalog, installation list, purchases, reports,
   and admin analytics.
7. Publish customer/creator communication if the incident was user-visible.
8. Record root cause and update this runbook before the next launch attempt.

## Monitoring Dashboard

During GA, watch:

- `/health` and `/ready`;
- `/api/marketplace/analytics/admin`;
- open P0/P1 beta blockers;
- install and rollback failures;
- paid checkout failures and refund events;
- critical abuse reports and unread internal notifications;
- blocked packages and high-risk validation reports;
- catalog/search latency from Phase 13 load-smoke baselines.

## Support Plan

For the first GA monitoring window:

- assign one release owner, one support owner, and one engineering owner;
- review Marketplace analytics and report queues at least daily;
- treat broken install, malicious product, and wrong payment as launch-blocking
  incident classes until triaged;
- keep creator and customer communication in the approved support channel;
- do not enable arbitrary package execution or automated payout transfer
  execution as part of support mitigation.
