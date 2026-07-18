---
okf_document_id: "database-entity-beta-release-records"
title: "Beta Release Records"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-012"
entity_name: "Beta Release Records"
entity_domain: "Release Readiness"
schema_objects: ["beta_participants", "beta_feedback", "beta_ga_blockers"]
owning_module: "Beta and Release Readiness"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED_WITH_RUNTIME_STATUS_UNCLEAR"
confidence: "high"
primary_sources: ["backend/migrations/0014_v2_phase_nine_beta_release.sql", "backend/src/routes/beta.rs", "backend/src/services"]
related_documents: ["database/multi-tenancy.md", "database/lifecycle-and-auditing.md", "backend/modules/beta-release-operations.md"]
related_diagrams: ["database/diagrams/database-domain-map.mmd"]
uncertainty_markers: ["IMPLEMENTATION_STATUS_UNCLEAR ISU-03", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# Beta Release Records

## Entity Identity

`DB-ENT-012` covers organization beta participation, feedback, and GA blocker records. Schema and code are implemented; actual launch/deployment state is `ISU-03`.

## Purpose

Verified purpose: record tenant participation, feedback severity/status, and release blockers. It does not prove that a production beta or GA gate is active.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| participant organization/status | Organization PK/text/time | Tenant participation state |
| feedback ID/org/category/severity/status | UUID/FK/constrained text | Feedback workflow |
| blocker ID/org/severity/status/owner | UUID/FK/text/UUID | Readiness blocker workflow |
| notes/metadata/timestamps | Text/JSONB/time | Operational context |

## Identifiers

Participant identity is `organization_id`; feedback and blockers use UUID PKs. Human-facing references are not documented as stable public IDs.

## Relationships

All records belong to an organization; actor/owner references connect to users where defined.

## Ownership

Beta routes and release-readiness services own writes; admin dashboards consume aggregate state.

## Tenant Isolation

All three tables are forced RLS with policies introduced by a migration template.

## Lifecycle

Participation, feedback, and blocker statuses plus resolution/closure times express lifecycle. No deletion or retention policy is established.

## Constraints and Indexes

Organization PK/FKs, status/severity checks, and organization/status/time indexes support dashboards and queues.

## Persistence Mapping

Beta routes/services use tenant and bypass transactions with route/service-local rows.

## Security and Privacy

Feedback and blocker notes may expose incidents, customer context, or personal information.

## Known Risks and Unknowns

Actual launch status, retention, privileged dashboard scope, and operational ownership are unresolved.

## Related Documents

See [Lifecycle and Auditing](../lifecycle-and-auditing.md), [Database Testing](../database-testing.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Beta Release Operations module](../../backend/modules/beta-release-operations.md).
