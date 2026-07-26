# ZinharCMS V3 Source Release Notes

ZinharCMS V3 publishes the completed Marketplace implementation as a GitHub
source-code release. It includes the creator workflow, validation, review,
catalog, installation lifecycle, permission controls, host-owned adapters,
one-time purchases, customer reviews, abuse reporting, analytics, creator
tooling, QA, performance, and beta-readiness work. This release does not deploy
or enable a hosted production environment.

## Release Identity

- Application version: `3.0.0`.
- Git tag: `v3.0.0`.
- Release type: non-prerelease GitHub source-code release.
- Release scope: GitHub source code and GitHub-generated source archives only.
- Production General Availability is not part of this source release and
  requires a separately approved deployment go/no-go process.

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
- Phase 14 beta-readiness evidence remains an input for any future production
  deployment; it is not a source-publication gate.

## Public Docs

Documentation included with the source release:

- `docs/MARKETPLACE_CREATOR_GUIDE.md` for creator packaging and submission;
- `docs/V3_MARKETPLACE_POLICY.md` for final policy, review, moderation, and
  takedown rules;
- `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md` for support workflow, rollback,
  and incident checklist;
- `docs/API.md` for route boundaries;
- `docs/V3_PHASE_FIFTEEN.md` for launch readiness and General Availability gates.

## Future Deployment Monitoring References

Operators of a future deployed environment should use these signals during
their launch window:

- `/health`;
- `/ready`;
- `/api/marketplace/analytics/admin`;
- Marketplace report queue;
- Marketplace purchase and entitlement records;
- Phase 13 load-smoke baselines;
- Phase 14 beta readiness report.

## Future Deployment Support Plan

Operators of a deployed environment should prioritize these incident classes:

- broken install;
- malicious product;
- wrong payment;
- refund, dispute, or payout issue;
- critical abuse report;
- emergency block or kill-switch event.

Support must capture organization id, listing id, version id, installation id,
purchase id, report id, creator id, request id, and timestamps when available.

## Known Limitations

- This release publishes source code only. It does not publish container images,
  binary packages, a hosted service, or a production deployment.
- Production General Availability and target-environment health are not claimed
  by the `v3.0.0` source tag.
- Uploaded Marketplace package code is still not executed.
- Automated payout transfer execution is deferred.
- Partial refunds remain unsupported by the current Marketplace runtime.
- External notification delivery remains deferred.
- Runtime error telemetry and warehouse export are not part of this release.
- A future Marketplace deployment must not bypass review policy; only approved
  products may be installed.

## Pre-Publication Source Release Gate

Before publication, go when:

- all release-version sources report `3.0.0`;
- applicable backend and frontend CI checks pass;
- the GPLv3 license and release-facing documentation are present;
- `main` is pushed and the release commit has a clean working tree;
- `v3.0.0` does not already exist before tag creation;
- the owner explicitly approves the final source release notes and publication.

No-go when:

- version, license, CI, or documentation checks fail;
- uncommitted or unpushed release changes remain;
- the intended tag already exists unexpectedly or points to a different commit;
- the release notes imply a production deployment that was not verified.

## Production Deployment Is Separate

The Phase 15 launch-readiness checks, target-environment API smoke, beta
evidence, monitoring, support, rollback, and communication-owner gates remain
required before any future production General Availability claim. Skipping
those checks for the source release is not approval to skip them for deployment.
