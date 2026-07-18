---
okf_document_id: "database-entity-saas-operations-audit"
title: "SaaS Operations and Audit"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-011"
entity_name: "SaaS Operations and Audit"
entity_domain: "SaaS Operations"
schema_objects: ["organization_domains", "organization_rate_limits", "audit_logs", "email_deliveries", "saas_alert_rules"]
owning_module: "Organizations and SaaS Operations"
tenant_scope: "tenant"
implementation_status: "PARTIALLY_IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0012_v2_phase_seven_saas_ops.sql", "backend/src/routes/organizations.rs", "backend/src/services/audit.rs", "backend/src/services"]
related_documents: ["database/lifecycle-and-auditing.md", "database/multi-tenancy.md", "backend/modules/organizations.md", "backend/modules/billing-quotas.md"]
related_diagrams: ["database/diagrams/database-domain-map.mmd"]
uncertainty_markers: ["IMPLEMENTATION_STATUS_UNCLEAR ISU-01", "IMPLEMENTATION_STATUS_UNCLEAR ISU-02", "RELATIONSHIP_UNCLEAR RLU-02", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# SaaS Operations and Audit

## Entity Identity

`DB-ENT-011` groups tenant domain configuration, rate limits, general audit history, email delivery state, and alert definitions. Storage is implemented; end-to-end domain routing and alert evaluation are partial/unverified.

## Purpose

Verified purpose: persist operational configuration and history. Domain verification-to-routing is `ISU-01`; automatic alert evaluation/delivery is `ISU-02`. No additional operations platform is inferred.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| domain org/name/status/verification | UUID/text/time | Tenant custom-domain lifecycle |
| rate-limit organization/limits | Organization PK/numeric fields | Tenant throttling configuration |
| audit org/user/action/entity/metadata | UUID/text/JSONB/time | Actor and logical subject history |
| email destination/template/status/provider | Sensitive text/JSONB/time | Delivery operation record |
| alert org/key/threshold/enabled | UUID/text/numeric/boolean | Alert definition |

## Identifiers

Records use UUIDs except organization-keyed rate limits. Domains and alert keys have scoped uniqueness. Audit subject identity is logical `(entity_type, entity_id)`.

## Relationships

All five concepts belong to organizations. Audit actors optionally reference users; audit subjects are polymorphic (`RLU-02`).

## Ownership

Organizations and SaaS services own configuration; nearly every domain writes audit through shared services (`EOU-01`).

## Tenant Isolation

All five tables are forced RLS. Administrative/bypass paths require explicit review.

## Lifecycle

Domains, email, and alerts use statuses/timestamps; audit is historical. No common retention/purge policy exists.

## Constraints and Indexes

Domain/key uniqueness, status checks, organization FKs, and organization/time/status indexes support operations.

## Persistence Mapping

Organization routes and shared operational services use direct SQLx. Audit writes can be in-domain transactions or post-commit depending on the caller.

## Security and Privacy

Domains, email addresses/content, audit metadata, actor IDs, and operational thresholds are sensitive.

## Known Risks and Unknowns

Routing/evaluation completion, audit completeness, retention, redaction, and recovery are unresolved.

## Related Documents

See [Lifecycle and Auditing](../lifecycle-and-auditing.md), [Multi-Tenancy](../multi-tenancy.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with [Organizations](../../backend/modules/organizations.md) and [Billing and Quotas](../../backend/modules/billing-quotas.md).
