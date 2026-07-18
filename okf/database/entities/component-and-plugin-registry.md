---
okf_document_id: "database-entity-component-plugin-registry"
title: "Component and Plugin Registry"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-005"
entity_name: "Component and Plugin Registry"
entity_domain: "CMS Extensibility"
schema_objects: ["component_registry", "cms_plugins"]
owning_module: "Pages and Plugins"
tenant_scope: "mixed"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0004_phase_two_page_builder.sql", "backend/migrations/0008_v2_phase_one_organizations.sql", "backend/migrations/0009_v2_phase_three_rls.sql", "backend/src/routes/plugins.rs", "backend/src/routes/pages.rs"]
related_documents: ["database/multi-tenancy.md", "database/module-data-ownership.md", "backend/modules/built-in-plugins.md"]
related_diagrams: ["database/diagrams/database-domain-map.mmd"]
uncertainty_markers: ["MIGRATION_MODEL_CONFLICT MMC-02", "MODULE_OWNERSHIP_UNCLEAR MOU-05"]
---

# Component and Plugin Registry

## Entity Identity

`DB-ENT-005` groups builder component definitions and built-in CMS plugin metadata. Ownership spans Pages and Plugins; Marketplace runtime records are documented separately.

## Purpose

Verified purpose: register component keys/schemas/rendering metadata and CMS plugin definitions used by builder/plugin flows. Arbitrary Marketplace package execution is not inferred.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| component ID/key/name | UUID/text, key uniqueness | Builder component identity |
| optional `organization_id` | UUID nullable FK | Null system component or tenant custom component |
| schema/config fields | JSONB/text | Component properties and behavior metadata |
| plugin key/name/version/config | Text/JSONB/flags | Built-in plugin registry |
| lifecycle timestamps/enabled state | Boolean/time | Registry state |

## Identifiers

Components use UUID PKs and stable keys. Plugins use migration-defined identifiers/unique keys. Component keys were backfilled before stronger constraints in 0004.

## Relationships

Tenant components optionally belong to organizations. Page documents reference component keys logically through JSON. CMS plugins are global registry rows.

## Ownership

Pages/builder reads component definitions; Plugins owns CMS plugin operations. Shared registry behavior creates an overlapping boundary.

## Tenant Isolation

`component_registry` is forced RLS with special policies: system rows can be selected globally while tenant custom rows use organization context. `cms_plugins` is global.

## Lifecycle

Enabled/configuration state is mutable. No package-execution, deletion-retention, or historical-version guarantee is inferred.

## Constraints and Indexes

Key uniqueness/indexes and JSON/config checks provide registry integrity. Mixed system/tenant policy functions enforce specialized access.

## Persistence Mapping

Pages and plugin routes use direct SQLx. Shared `ComponentRegistryItem` omits `organization_id` (`MMC-02`).

## Security and Privacy

Configuration and component schemas can influence rendered output and extension behavior; treat writes as privileged.

## Known Risks and Unknowns

Component ownership, key compatibility across page documents, deletion effects, and shared-model coverage are not centrally governed.

## Related Documents

See [Module Data Ownership](../module-data-ownership.md), [Multi-Tenancy](../multi-tenancy.md), and [Marketplace Installations and Runtime Adapters](marketplace-installations-and-runtime-adapters.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Built-In Plugins module](../../backend/modules/built-in-plugins.md).
