---
okf_document_id: "domain-saas-operations-beta"
title: "SaaS Operations and Beta Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-SAAS"
domain_name: "SaaS Operations and Beta"
domain_status: "PARTIALLY_IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/routes/beta.rs"
  - "backend/src/services/audit.rs"
  - "backend/src/services/email.rs"
  - "backend/migrations/0012_v2_phase_seven_saas_ops.sql"
  - "backend/migrations/0014_v2_phase_nine_beta_release.sql"
related_documents:
  - "../domain-catalog.md"
  - "../workflows/beta-feedback-and-readiness.md"
  - "../../backend/modules/beta-release-operations.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# SaaS Operations and Beta Domain

## Domain Identity

- Domain ID: `DOM-SAAS`
- Terminology: organization domain, rate limit, audit log, email delivery, alert rule, beta participant, feedback, GA blocker, readiness dashboard.
- Implementation: `PARTIALLY_IMPLEMENTED`; boundary `OVERLAPPING`; confidence Medium.

## Responsibility

- Verified: manage domains/rate limits, read audit/email/alert records, submit/triage beta feedback, manage blockers/participants, and aggregate readiness indicators.
- Inferred: these records support SaaS operations and launch decisions.
- Shared: Organizations, Billing/quotas, Webhooks, email/audit, Marketplace readiness services.
- Unclear: domain verification routing, alert evaluation, production launch decision, and retry/escalation operations.

## Core Entities

Organization domains/rate limits, audit logs, email deliveries, SaaS alert rules, beta participants, beta feedback, and beta GA blockers.

## Core Services

Organization/beta routes, audit/email/quota/RLS/RBAC services, and Marketplace readiness contract services.

## API Surface

Current organization operational endpoints, tenant beta dashboard/feedback/blockers, global product dashboard, and participant upsert. See [SaaS Operations](../../api/endpoints/saas-operations.md) and [Beta Operations](../../api/endpoints/beta-release-operations.md).

## Frontend Surface

`OrganizationPage`, `BetaPage`, `DashboardPage`, and Marketplace readiness/analytics surfaces.

## Actors

Organization admin/editor/member, organization owner, global admin, and authenticated feedback submitter.

## Business Rules

`BR-SAAS-001` through `BR-SAAS-003`, plus tenant/billing rules.

## Invariants

Allowed status/value sets, nonempty bounded text, metadata objects, one beta participant per organization, and tenant RLS.

## State and Lifecycle

Participant, feedback, blocker, domain, email, and alert states are stored. Destination values are constrained, but most allowed-from/to graphs are `STATE_TRANSITION_UNCLEAR`.

## Access Rules

Tenant administration uses active membership and organization roles. Global beta dashboard/participant management uses global admin and bypass. Audit/email data is sensitive and tenant-scoped where applicable.

## Validation Rules

Domain syntax/status, positive rate limits, audit/email/alert shape, beta category/severity/status/priority/text, and metadata object validation.

## Workflows

[Beta Feedback and Readiness](../workflows/beta-feedback-and-readiness.md), organization provisioning/administration, and billing/webhook operational flows.

## Side Effects

Transactional audit on several operations, email delivery records/provider attempts, beta writes, and read-only aggregation across billing/webhook/quota state.

## Tests

Beta choice/text/metadata and dashboard SQL composition have unit/static tests. Domain verification, rate-limit runtime integration, audit completeness, email provider/retry, beta state transitions, and release decisions lack end-to-end tests.

## Risks and Unknowns

Automated alerting is unclear, operational records have no retention policy, dashboard thresholds do not prove GA readiness, and global bypass caller policies remain distributed.

Return to the [Domain Catalog](../domain-catalog.md).

