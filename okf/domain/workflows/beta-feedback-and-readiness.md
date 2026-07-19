---
okf_document_id: "workflow-beta-feedback-readiness"
title: "Beta Feedback and Readiness Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-BETA"
workflow_name: "Beta Feedback and Readiness"
workflow_domain: "DOM-SAAS"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/beta.rs"
  - "backend/migrations/0014_v2_phase_nine_beta_release.sql"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/saas-operations-and-beta.md"
  - "../../api/endpoints/beta-release-operations.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Beta Feedback and Readiness Workflow

## Workflow Identity

- ID/name/domain: `WF-BETA`, Beta Feedback and Readiness, `DOM-SAAS`.
- Trigger/actors: tenant member submits feedback; editor/admin triages; global admin manages participants/views product dashboard.
- Status/confidence: `IMPLEMENTED`; Medium.

## Preconditions

Active tenant context for tenant records; validated category/severity/text/metadata; editor/admin for feedback or blocker updates; global admin for participant/product operations.

## Main Flow

1. Tenant user submits bounded beta feedback.
2. Handler inserts feedback and audit in tenant transaction.
3. Editor/admin changes feedback status/severity through allowed-value validation.
4. Editor/admin creates/updates GA blockers with priority/status.
5. Organization dashboard aggregates participant state, unresolved feedback/blockers, failed billing/webhook records, and exceeded quota metrics.
6. Global admin uses bypass product dashboard and upserts participant cohort/status/metadata.

## Alternative Flows

Tenant reads feedback and blockers without mutation where authorized. Status can be changed directly to any allowed destination; no required sequence is enforced.

## Failure Flows

Invalid choice/text/metadata, wrong tenant entity, role denial, or DB/audit transaction error rejects. Dashboard dependency query failure rejects aggregation.

## State Changes

Participant, feedback, and blocker status values change, but transition graph is `STATE_TRANSITION_UNCLEAR`.

## Data Changes

Beta participant/feedback/blocker and audit rows. Dashboard reads billing events, webhook deliveries, and usage.

## Transaction Boundaries

Tenant feedback/blocker writes and audit are transactional. Global participant upsert uses bypass transaction. Dashboard is read-only aggregation.

## Side Effects

Audit records; no automatic notification, ticket, or release deployment.

## Completion Criteria

Mutation returns persisted row, or dashboard returns current aggregate. A “ready” display is evidence, not an automatic GA decision.

## Tests

Choice/text/metadata and query-composition tests. No auth/tenant/transaction/state-transition/dashboard integration tests.

## Unknowns and Risks

Product readiness thresholds, actual deployment state, status sequences, alert/escalation, retention, and relation to Marketplace readiness phases.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

