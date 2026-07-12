# ZinharCMS V3 Marketplace General Availability Release Notes

ZinharCMS V3 Marketplace General Availability enables controlled installation
of reviewed and approved Marketplace products for target users. This release
builds on the completed creator workflow, validation, review, catalog,
installation lifecycle, permission controls, host-owned adapters, one-time
purchases, customer reviews, abuse reporting, analytics, creator tooling, QA,
performance, and beta readiness phases.

## Highlights

- Approved Component Packs and Design Templates can be discovered, inspected,
  installed, disabled, uninstalled, updated, and rolled back.
- Install and update flows re-check compatibility, permissions, artifact
  integrity, review status, and entitlement gates.
- Paid one-time Marketplace purchases and full-refund entitlement revocation are
  available where Stripe configuration is enabled.
- Creator analytics and the internal monitoring dashboard expose Marketplace
  health, installs, purchases, reports, refunds, and risky products.
- Customer reviews, abuse reports, moderation queues, emergency block, and kill
  switches provide controlled takedown and incident response.
- Creator documentation and local CLI tooling support validate, pack, and submit
  workflows before upload.
- Phase 14 beta readiness evidence is the final pre-GA input.

## Public Docs

Public docs for GA operators and users:

- `docs/MARKETPLACE_CREATOR_GUIDE.md` for creator packaging and submission;
- `docs/V3_MARKETPLACE_POLICY.md` for final policy, review, moderation, and
  takedown rules;
- `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md` for support workflow, rollback,
  and incident checklist;
- `docs/API.md` for route boundaries;
- `docs/V3_PHASE_FIFTEEN.md` for launch readiness and General Availability gates.

## Monitoring Dashboard

Use these signals during the GA launch window:

- `/health`;
- `/ready`;
- `/api/marketplace/analytics/admin`;
- Marketplace report queue;
- Marketplace purchase and entitlement records;
- Phase 13 load-smoke baselines;
- Phase 14 beta readiness report.

## Support Plan

Prioritize these incident classes:

- broken install;
- malicious product;
- wrong payment;
- refund, dispute, or payout issue;
- critical abuse report;
- emergency block or kill-switch event.

Support must capture organization id, listing id, version id, installation id,
purchase id, report id, creator id, request id, and timestamps when available.

## Known Limitations

- Uploaded Marketplace package code is still not executed.
- Automated payout transfer execution is deferred.
- Partial refunds remain unsupported by the current Marketplace runtime.
- External notification delivery remains deferred.
- Runtime error telemetry and warehouse export are not part of this release.
- Marketplace GA does not bypass review policy; only approved products can be
  installed in production.

## Go/No-Go

Go when:

- Launch Readiness gates pass;
- Phase 14 beta evidence has no ownerless P0/P1 blocker;
- `scripts/marketplace-phase15-ga-check.ps1` passes or produces an approved
  `-ReportOnly` exception list;
- the support owner confirms readiness for broken install, malicious product,
  and wrong payment incidents;
- the release owner confirms rollback criteria and communication owners.

No-go when:

- active P0/P1 Marketplace blocker lacks an owner;
- approved products cannot be installed in the target production environment;
- support cannot access the operations runbook or monitoring dashboard;
- payment/refund/entitlement behavior is inconsistent for target users.
