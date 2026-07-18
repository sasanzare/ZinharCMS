---
okf_document_id: "database-entity-public-settings-navigation"
title: "Public Settings and Navigation"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-008"
entity_name: "Public Settings and Navigation"
entity_domain: "Public Delivery Configuration"
schema_objects: ["public_settings", "navigation_items"]
owning_module: "Delivery and Settings"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0005_phase_five_delivery_api.sql", "backend/migrations/0008_v2_phase_one_organizations.sql", "backend/src/routes/delivery.rs"]
related_documents: ["database/multi-tenancy.md", "database/lifecycle-and-auditing.md", "backend/modules/public-delivery-cache.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["UNKNOWN U-08", "DATA_LIFECYCLE_UNCLEAR DLU-02"]
---

# Public Settings and Navigation

## Entity Identity

`DB-ENT-008` groups tenant public key/value configuration and ordered navigation trees used by delivery endpoints.

## Purpose

Verified purpose: persist public presentation settings and navigation items. The mechanism selecting the public organization for a host is not established (`UNKNOWN U-08`).

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| setting organization/key/value | UUID/text/JSONB; composite PK | Tenant configuration entry |
| navigation ID/organization | UUID/FKs | Tenant navigation identity |
| label, URL/type, order | Text/numeric/checks | Rendered navigation data |
| `parent_id` | Nullable self-FK | Navigation hierarchy |
| visibility/time fields | Boolean/time | Delivery state |

## Identifiers

Settings use `(organization_id, key)` after the tenancy migration's PK replacement. Navigation rows use UUID PKs and ordering within tenant/tree context.

## Relationships

Both belong to organizations; navigation is self-referential. Navigation destinations may logically target pages or external URLs without one universal FK.

## Ownership

Delivery/settings writes and reads these records; frontend/public consumers receive projections.

## Tenant Isolation

Both tables are forced RLS and explicitly tenant-scoped. Public access still needs a trusted organization selection path.

## Lifecycle

Rows are mutable and may be replaced/deleted. No revision or retention history is defined (`DLU-02`).

## Constraints and Indexes

Composite setting identity, organization FKs, navigation parent FK, ordering/visibility checks, and tenant lookup indexes support delivery.

## Persistence Mapping

Delivery/settings code uses direct SQLx projections and dynamic query building in selected listing paths.

## Security and Privacy

Values are publicly oriented but must not be assumed safe for arbitrary secrets. Administrative writes require tenant authorization.

## Known Risks and Unknowns

Public tenant routing, navigation target integrity, cache invalidation, and history behavior are unresolved.

## Related Documents

See [Multi-Tenancy](../multi-tenancy.md), [Relationships](../relationships.md), and [CMS Webhooks and Deliveries](cms-webhooks-and-deliveries.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Public Delivery and Cache module](../../backend/modules/public-delivery-cache.md).
