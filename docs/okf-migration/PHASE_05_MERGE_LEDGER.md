# Phase 5 Merge Ledger

**Date:** 2026-08-07
**Source HEAD:** `6f51612c5716c7d9c3365553811053fd24a03947`
**Scope:** the eight constructed Phase 5 Concepts and the nine evaluated candidates

This ledger records semantic routing, not file deletion. The complete Phase 1
path matrix remains the authoritative 353-row source inventory. Phase 5
reconciles every primary legacy category mapped to the selected targets and
records how shared API, database, security, domain, Marketplace, and
maintenance rows remain routed to carried, regenerated, owner-blocked,
historical, or Phase 6 targets.

## Outcome vocabulary

* `KNOWLEDGE_MERGED` — current source-backed knowledge was retained in a new
  Phase 5 Concept.
* `DUPLICATE_OMITTED` — a repeated wrapper or overlapping summary added no
  unique knowledge after merge.
* `STALE_OMITTED` — the current source no longer supports the legacy claim.
* `CONTRADICTED_OMITTED` — current source directly contradicts the legacy
  statement.
* `UNVERIFIED_OMITTED` — the topic is relevant but the legacy assertion was
  not promoted without current evidence.
* `HISTORICAL_DEFERRED` — history remains outside current Concepts.
* `OWNER_DECISION_DEFERRED` — implementation mechanics may be present, but
  policy or ownership requires a decision.
* `OUT_OF_TARGET_SCOPE` — the input belongs to a later or different target.
* `NO_UNIQUE_KNOWLEDGE` — no distinct source-backed claim remained after
  consolidation.

## Constructed-target crosswalk

| Target | Current source packet | Legacy inputs retained | Inputs omitted or deferred | Conflict/owner boundary |
| --- | --- | --- | --- | --- |
| TARGET-021 | `routes/mod.rs`, `delivery.rs`, `webhooks.rs`, cache, webhook, tenant/RLS services | Public delivery and webhook family claims; organization selection; published filters; cache keys/invalidation; HMAC delivery recording | Repeated endpoint wrappers and exhaustive route catalogs remain with API regeneration; legacy retry/compatibility assertions not promoted | NOC-01, NOC-08, NOC-09, NOC-11 |
| TARGET-022 | Marketplace, runtime, adapter, plugin routes and RBAC | Current route families, role gates, creator ownership checks, adapter/runtime boundary, built-in plugin surface | Repeated Marketplace summaries; external execution, settlement, dispute, and compatibility promises | NOC-08, NOC-10, NOC-17 |
| TARGET-025 | RLS service, tenant middleware, migrations, RLS test source | Session context, forced RLS, policy shapes, explicit predicate defense, table inventory | Exhaustive schema/entity catalog and deployed-state claims remain regeneration/deployment work; privacy/retention policy deferred | NOC-01, NOC-03, NOC-05, NOC-11 |
| TARGET-031 | File security, media, cleanup, processing, migration 0030, tests | Upload purpose limits, content/key checks, lifecycle, public/private media, quarantine, cleanup | Legacy malware/durability/retention guarantees without current evidence; deployment storage policy | NOC-02, NOC-05 |
| TARGET-032 | Auth/security/Step-Up middleware, RBAC, audit/cleanup services, migration tests | Source hardening, security headers, session/MFA/Step-Up, audit metadata controls, evidence limits | Stale negatives, unverified deployment/threat claims, and owner-only operational risk statements | NOC-04, NOC-05, NOC-15 |
| TARGET-036 | Delivery routes, cache, RLS service, route composition | Published-state/public-flag behavior, default organization selection, cache/invalidation boundary | Intended custom-domain routing and universal freshness/atomicity promises | NOC-01, NOC-09, NOC-11 |
| TARGET-037 | Billing routes, quota service, RBAC, migrations 0010–0012 | Plan/subscription/usage model, request/content/media limits, organization-row lock | Provider settlement/failure, refund/tax, compensation, and retry policy | NOC-09 |
| TARGET-040 | Marketplace validation/manifest/package/runtime services, routes, migrations, tests | Artifact checks, permission snapshots, runtime allowlists, kill switches, non-executing result | External execution, archive retention, settlement, and full lifecycle regeneration visual | NOC-10, NOC-17 |

## Primary legacy category decisions

The following path-level records are the Phase 5 primary inputs. Shared rows
from the large API/database/security/domain groups are also accounted for in
the group-routing table below.

### PublicDelivery — 2 primary rows

* `okf/domain/domains/delivery-settings-and-webhooks.md` —
  `KNOWLEDGE_MERGED` into TARGET-021 and TARGET-036; current delivery and
  webhook behavior was rebuilt from source.
* `okf/domain/workflows/publication-webhook-delivery.md` —
  `KNOWLEDGE_MERGED` into TARGET-021 and TARGET-036; asynchronous dispatch was
  retained without retry or durability claims.

### BillingAndQuotas — 2 primary rows

* `okf/domain/domains/billing-and-quotas.md` — `KNOWLEDGE_MERGED` into
  TARGET-037; current plan, subscription, and usage mechanics were retained.
* `okf/domain/workflows/billing-subscription.md` — `KNOWLEDGE_MERGED` into
  TARGET-037; provider and compensation policy was omitted as unresolved.

### Media — 2 primary rows

* `okf/domain/domains/media-library.md` — `KNOWLEDGE_MERGED` into TARGET-031;
  current media visibility and lifecycle controls were retained.
* `okf/domain/workflows/media-upload-and-processing.md` —
  `KNOWLEDGE_MERGED` into TARGET-031; current upload validation and processing
  evidence was retained.

### TenantIsolation — 5 primary rows

* `okf/domain/membership-and-ownership.md` — `KNOWLEDGE_MERGED` into
  TARGET-025; only source-backed membership/context mechanics were retained.
* `okf/domain/domains/organizations-and-membership.md` —
  `KNOWLEDGE_MERGED` into TARGET-025; active membership lookup was retained.
* `okf/domain/workflows/organization-ownership-transfer.md` —
  `KNOWLEDGE_MERGED` for observed route/context mechanics and
  `OWNER_DECISION_DEFERRED` for durable ownership policy.
* `okf/domain/workflows/organization-provisioning.md` —
  `KNOWLEDGE_MERGED` for organization context and `OWNER_DECISION_DEFERRED`
  for deployment/provisioning authority.
* `okf/domain/workflows/tenant-invitation-and-membership.md` —
  `KNOWLEDGE_MERGED` for active membership checks; policy ownership was not
  promoted.

### SecurityPosture — 12 primary rows

* `okf/security/README.md` — `NO_UNIQUE_KNOWLEDGE`.
* `okf/security/administrative-access.md` — `KNOWLEDGE_MERGED` for current
  role and sensitive-operation gates.
* `okf/security/audit-and-security-events.md` — `KNOWLEDGE_MERGED` for current
  audit helpers and secret-shaped metadata rejection.
* `okf/security/browser-and-http-security.md` — `KNOWLEDGE_MERGED` for
  current API security headers; deployment edge policy omitted.
* `okf/security/frontend-security-boundaries.md` — `UNVERIFIED_OMITTED` from
  this source-only security posture merge; frontend policy remains outside the
  selected evidence packet.
* `okf/security/input-validation.md` — `KNOWLEDGE_MERGED` for current source
  validation boundaries.
* `okf/security/overview.md` — `KNOWLEDGE_MERGED` after replacing snapshot
  certainty with current source evidence.
* `okf/security/secrets-and-configuration.md` — `UNVERIFIED_OMITTED` for
  deployment secret-management claims.
* `okf/security/security-risks.md` — `UNVERIFIED_OMITTED` for legacy risk
  assertions not re-established by the current source packet.
* `okf/security/security-testing.md` — `UNVERIFIED_OMITTED`; test-source
  presence was recorded, but no application tests ran in Phase 5.
* `okf/security/threat-register.md` — `OWNER_DECISION_DEFERRED`; risk
  acceptance and ownership are not source facts.
* `okf/security/trust-boundaries.md` — `KNOWLEDGE_MERGED` into the layered
  security Concept and its constructed visualization.

### Extensibility — 28 primary rows

* `okf/extensibility/README.md` — `NO_UNIQUE_KNOWLEDGE`.
* `okf/extensibility/plugin-data-and-migrations.md` — `KNOWLEDGE_MERGED` into
  TARGET-022 and TARGET-040 for current persistence/installation boundaries.
* `okf/extensibility/plugin-discovery-and-registration.md` —
  `KNOWLEDGE_MERGED` into TARGET-022 for current plugin and adapter surfaces.
* `okf/extensibility/plugin-lifecycle.md` — `KNOWLEDGE_MERGED` into TARGET-022
  and TARGET-040 for current lifecycle gates.
* `okf/extensibility/plugin-manifest.md` — `KNOWLEDGE_MERGED` into TARGET-040
  for current manifest validation.
* `okf/extensibility/plugin-permissions.md` — `KNOWLEDGE_MERGED` into
  TARGET-022 and TARGET-040 for allowlisted permissions and snapshots.
* `okf/extensibility/tenant-and-global-scope.md` — `KNOWLEDGE_MERGED` for
  observed organization/global boundaries; ownership policy omitted.
* `okf/extensibility/terminology.md` — `DUPLICATE_OMITTED`; no owner-approved
  glossary was established.
* `okf/extensibility/extension-points/cms-entry-after-publish.md` —
  `OUT_OF_TARGET_SCOPE`; current built-in hook behavior remains in the carried
  extensibility Concept.
* `okf/extensibility/extension-points/cms-entry-before-save.md` —
  `OUT_OF_TARGET_SCOPE`; current built-in hook behavior remains carried.
* `okf/extensibility/extension-points/page-builder-component-registry.md` —
  `OUT_OF_TARGET_SCOPE`; the current component registry remains with the
  carried page-builder/extensibility Concepts.
* `okf/extensibility/plugins/seo-auto.md` — `OUT_OF_TARGET_SCOPE`; built-in
  plugin behavior remains with the carried Concept.
* `okf/extensibility/activation-and-deactivation.md` — `KNOWLEDGE_MERGED` into
  TARGET-022 and TARGET-040 for observed controls.
* `okf/extensibility/backend-extension-points.md` — `OUT_OF_TARGET_SCOPE`;
  internal extension mechanics remain with the carried Concept.
* `okf/extensibility/compatibility-and-versioning.md` —
  `OWNER_DECISION_DEFERRED` under NOC-08.
* `okf/extensibility/component-and-block-registration.md` —
  `KNOWLEDGE_MERGED` for current adapter registration mechanics.
* `okf/extensibility/development-workflow.md` — `OUT_OF_TARGET_SCOPE`; the
  development maintenance boundary is Phase 6.
* `okf/extensibility/extensibility-catalog.md` — `DUPLICATE_OMITTED` as a
  catalog wrapper; current route/service facts were retained elsewhere.
* `okf/extensibility/extensibility-risks.md` — `UNVERIFIED_OMITTED` for legacy
  risk certainty not re-established in the current packet.
* `okf/extensibility/extensibility-testing.md` — `UNVERIFIED_OMITTED`; test
  source is available but this phase did not execute it.
* `okf/extensibility/extension-points.md` — `DUPLICATE_OMITTED` as an
  overlapping index-like summary.
* `okf/extensibility/frontend-extension-points.md` — `OUT_OF_TARGET_SCOPE`;
  frontend extension boundaries remain carried Phase 3/4 knowledge.
* `okf/extensibility/hooks-and-events.md` — `KNOWLEDGE_MERGED` for current
  host-owned hook/adapter boundaries.
* `okf/extensibility/installation-and-removal.md` — `KNOWLEDGE_MERGED` into
  TARGET-022 and TARGET-040.
* `okf/extensibility/isolation-and-trust.md` — `KNOWLEDGE_MERGED` for current
  non-execution and permission-gate behavior.
* `okf/extensibility/overview.md` — `DUPLICATE_OMITTED` as an overlapping
  summary.
* `okf/extensibility/plugin-architecture.md` — `KNOWLEDGE_MERGED` for the
  built-in versus Marketplace boundary.
* `okf/extensibility/plugin-configuration.md` — `KNOWLEDGE_MERGED` for current
  configuration/permission evidence; deployment policy omitted.

### Marketplace — 15 primary rows

* `okf/extensibility/extension-points/marketplace-public-hook-adapter.md` —
  `KNOWLEDGE_MERGED` into TARGET-022 and TARGET-040.
* `okf/extensibility/extension-points/marketplace-runtime-authorization.md` —
  `KNOWLEDGE_MERGED` into TARGET-022 and TARGET-040.
* `okf/extensibility/extension-points/marketplace-template-adapter.md` —
  `KNOWLEDGE_MERGED` into TARGET-022.
* `okf/extensibility/marketplace/commerce-entitlements.md` —
  `KNOWLEDGE_MERGED` for current purchase/entitlement mechanisms; settlement
  policy omitted.
* `okf/extensibility/marketplace/creator-listing-version.md` —
  `KNOWLEDGE_MERGED` for creator/listing/version mechanics.
* `okf/extensibility/marketplace/feedback-moderation-analytics.md` —
  `KNOWLEDGE_MERGED` for route and role boundaries.
* `okf/extensibility/marketplace/installation-update-rollback.md` —
  `KNOWLEDGE_MERGED` for lifecycle gates.
* `okf/extensibility/marketplace/package-validation-review.md` —
  `KNOWLEDGE_MERGED` for artifact validation; policy decisions omitted.
* `okf/extensibility/marketplace/runtime-permissions-adapters.md` —
  `KNOWLEDGE_MERGED` for allowlists, snapshots, and non-execution.
* `okf/domain/domains/marketplace.md` — `KNOWLEDGE_MERGED` into carried
  Marketplace knowledge and the new runtime boundary.
* `okf/domain/workflows/marketplace-installation-lifecycle.md` —
  `KNOWLEDGE_MERGED` into TARGET-022 and TARGET-040.
* `okf/domain/workflows/marketplace-product-publication.md` —
  `KNOWLEDGE_MERGED` for review/state mechanics; owner moderation policy
  omitted.
* `okf/domain/workflows/marketplace-purchase-and-entitlement.md` —
  `KNOWLEDGE_MERGED` for current tables/routes; settlement and refund policy
  omitted.
* `okf/extensibility/marketplace-architecture.md` — `DUPLICATE_OMITTED` as an
  architecture wrapper; source mechanics were retained in target Concepts.
* `okf/extensibility/marketplace-workflows.md` — `DUPLICATE_OMITTED` as an
  overlapping workflow wrapper.

### MaintenanceGovernance — 8 primary rows

All eight records were evaluated and assigned `OUT_OF_TARGET_SCOPE` for Phase
5 because their target is the Phase 6 development/documentation boundary:

* `okf/maintenance/README.md`
* `okf/maintenance/change-impact-matrix.md`
* `okf/maintenance/documentation-ownership.md`
* `okf/maintenance/final-completion-report.md`
* `okf/maintenance/okf-update-policy.md`
* `okf/maintenance/review-checklist.md`
* `okf/maintenance/staleness-detection.md`
* `okf/maintenance/validation-report.md`

No maintenance policy was copied into a current Phase 5 Concept.

## Shared merge-group routing

The large shared groups are not silently dropped when a primary category is
constructed. The Phase 1 path matrix continues to own the exact row-level
mapping; this table records the Phase 5 outcome for each group and its
non-Phase-5 destinations.

| Merge group | Total Phase 1 rows | Phase 5 destinations | Other routing | Phase 5 outcome |
| --- | ---: | --- | --- | --- |
| MERGE-GROUP-004 API | 57 | TARGET-021, TARGET-022, TARGET-037 | Carried TARGET-018/020; regenerated TARGET-019; other API families remain on their catalog targets | `KNOWLEDGE_MERGED`, `DUPLICATE_OMITTED`, or `DEFERRED_REGENERATE` by mapped row |
| MERGE-GROUP-005 Database | 34 | TARGET-025, TARGET-037 | Regenerated TARGET-023/024; owner-blocked TARGET-026; carried tenant security TARGET-029 | `KNOWLEDGE_MERGED` for RLS/quota facts; `DEFERRED_REGENERATE` or `OWNER_DECISION_DEFERRED` for broader policy |
| MERGE-GROUP-006 Security | 40 | TARGET-025, TARGET-031, TARGET-032, TARGET-040 | Carried TARGET-027/028/029/030; owner/operational risk inputs remain deferred | `KNOWLEDGE_MERGED`, `UNVERIFIED_OMITTED`, or `OWNER_DECISION_DEFERRED` by mapped row |
| MERGE-GROUP-007 Domain | 44 | TARGET-021, TARGET-025, TARGET-036, TARGET-037, TARGET-040 | Carried TARGET-033/034/038; regenerated TARGET-035; owner policy remains deferred | `KNOWLEDGE_MERGED` for current mechanics; unresolved policy remains deferred |
| MERGE-GROUP-009 Extensibility | 39 | TARGET-022, TARGET-040 | Carried TARGET-038/039; development and compatibility items remain out of scope or owner-blocked | `KNOWLEDGE_MERGED`, `DUPLICATE_OMITTED`, `OUT_OF_TARGET_SCOPE`, or `OWNER_DECISION_DEFERRED` by mapped row |
| MERGE-GROUP-011 Maintenance | 21 | TARGET-047 evaluated only | Carried/regenerated/deferred development and decision targets; historical rows remain historical | `OUT_OF_TARGET_SCOPE` for the eight primary maintenance rows; other rows retain Phase 4 matrix status |

This preserves the no-silent-disappearance rule while keeping Phase 5 focused
on the approved domains and avoiding a second full repository regeneration.

## Conflict and omission review

* Current source wins over stale legacy negatives, including old statements
  that omitted MFA, Step-Up, file-hardening, or Marketplace safety controls.
* Unverified legacy deployment, monitoring, storage, privacy, retention,
  settlement, and owner statements are not promoted as current facts.
* Repeated README, overview, catalog, and wrapper material is omitted only
  after its current knowledge is retained in a selected or carried Concept.
* Historical and owner-decision records remain reachable through the migration
  artifacts and status matrix; they are not silently turned into current
  Concepts.
