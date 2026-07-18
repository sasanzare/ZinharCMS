---
okf_document_id: "database-entity-organizations-membership"
title: "Organizations and Membership"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-002"
entity_name: "Organizations and Membership"
entity_domain: "Tenancy"
schema_objects: ["organizations", "organization_members", "organization_invitations"]
owning_module: "Organizations"
tenant_scope: "mixed"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0008_v2_phase_one_organizations.sql", "backend/src/routes/organizations.rs", "backend/src/middleware/tenant.rs"]
related_documents: ["database/multi-tenancy.md", "database/relationships.md", "backend/modules/organizations.md"]
related_diagrams: ["database/diagrams/tenant-isolation.mmd"]
uncertainty_markers: ["TRANSACTION_BOUNDARY_UNCLEAR TBU-02", "UNKNOWN U-08"]
---

# Organizations and Membership

## Entity Identity

`DB-ENT-002` is the tenant-root aggregate: organizations, user memberships, and invitations. Organization route-local/shared row types map it. It is implemented with high repository confidence.

## Purpose

Verified purpose: create and manage tenants, establish member roles/status, and invite users. Public-host/domain selection is outside these three tables and remains unclear. No unimplemented lifecycle is assumed.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| organization identity/name/slug | UUID/text; unique identifiers where defined | Tenant root |
| `status` and timestamps | PostgreSQL organization enum/time | Tenant lifecycle |
| membership organization/user | UUID composite PK/FKs | One membership per user per organization |
| membership role/status | PostgreSQL enums | Tenant authorization context |
| invitation email/token/status/expiry | Sensitive text/time/enums | Invitation lifecycle |

## Identifiers

Organizations use UUID PKs and human-facing slugs. Membership uses `(organization_id, user_id)`. Invitations use their own UUID plus security-sensitive token/identity fields.

## Relationships

Organizations parent nearly every tenant domain. Membership joins organizations to global users. Invitations belong to an organization and may reference user actors.

## Ownership

Organizations owns lifecycle writes. Tenant middleware reads active membership. Billing creates subscription state; many modules reference the organization FK.

## Tenant Isolation

The root/membership tables are global control-plane data rather than forced-RLS tenant content. Correct authorization depends on user identity, active organization, membership status/role, and privileged admin paths.

## Lifecycle

Organizations, members, and invitations use status enums and timestamps. Organization creation writes organization, initial membership, and subscription in one transaction, but audit is written afterward (`TBU-02`).

## Constraints and Indexes

Composite membership identity prevents duplicate membership. Slug and invitation lookup uniqueness/indexes support tenant selection and invitation processing.

## Persistence Mapping

Organization handlers use direct SQLx transactions and local mappings. Tenant middleware consumes the resulting membership contract.

## Security and Privacy

Membership roles and invitation tokens are authorization-sensitive; invitation email is personal data. Do not expose token values.

## Known Risks and Unknowns

Audit failure after organization commit, deletion/cascade effects, and the public tenant-selection contract (`UNKNOWN U-08`) are unresolved.

## Related Documents

See [Multi-Tenancy](../multi-tenancy.md), [Transactions and Consistency](../transactions-and-consistency.md), and [SaaS Operations and Audit](saas-operations-and-audit.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Organizations module](../../backend/modules/organizations.md).
