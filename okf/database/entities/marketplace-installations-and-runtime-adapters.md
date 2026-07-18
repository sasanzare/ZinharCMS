---
okf_document_id: "database-entity-marketplace-installations-runtime"
title: "Marketplace Installations and Runtime Adapters"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-015"
entity_name: "Marketplace Installations and Runtime Adapters"
entity_domain: "Marketplace Runtime"
schema_objects: ["marketplace_installations", "marketplace_permission_catalog", "marketplace_kill_switches", "marketplace_template_imports", "marketplace_plugin_hooks"]
owning_module: "Marketplace Installation and Runtime"
tenant_scope: "mixed"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0019_v3_phase_six_installation_lifecycle.sql", "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql", "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql", "backend/src/services/marketplace_installation.rs", "backend/src/services/marketplace_policy.rs", "backend/src/services/marketplace_adapters.rs"]
related_documents: ["database/multi-tenancy.md", "database/transactions-and-consistency.md", "backend/modules/marketplace-catalog-installation.md", "backend/modules/marketplace-runtime-adapters.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["DOCUMENTATION_CODE_CONFLICT DCC-12", "MODULE_OWNERSHIP_UNCLEAR MOU-05", "SOFT_DELETE_BEHAVIOR_UNCLEAR SDBU-01"]
---

# Marketplace Installations and Runtime Adapters

## Entity Identity

`DB-ENT-015` groups tenant installation lifecycle, the global permission catalog, tenant kill switches, template-import mappings, and plugin hook registrations.

## Purpose

Verified purpose: install approved Marketplace versions, record requested/granted permissions, block unsafe runtime use, and connect packages to Pages/plugin host adapters. Arbitrary server-side package execution is not implemented.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| installation org/listing/version/status | UUID FKs/checked text | Tenant-installed artifact |
| configuration/permissions/timestamps | JSONB/arrays/time | Runtime configuration and lifecycle |
| permission key/risk/description | Text PK/checks | Global permission vocabulary |
| kill-switch scope/reason/state | UUID FKs/text/boolean/time | Runtime block control |
| template import org/install/page/mapping | UUID FKs/JSONB | Pages adapter state |
| plugin hook org/install/hook/config | UUID FKs/text/JSONB | Plugin adapter state |

## Identifiers

Most rows use UUID PKs; permission uses `permission_key`. Partial unique indexes constrain active installation, switch, import, and hook combinations.

## Relationships

Installations connect organizations to listings and versions. Kill switches have global or organization scope and do not reference an installation or listing. Imports connect installations to pages. Hooks belong to installations. Entitlements may gate installation in application logic.

## Ownership

Marketplace installation/runtime services own writes; Pages, catalog, entitlement, audit, and policy services participate (`MOU-05`).

## Tenant Isolation

Installation, kill switch, template import, and hook tables are forced RLS; permission catalog is global. The hardening checker omits the three later adapter/policy tables and other later Marketplace RLS tables (`DCC-12`).

## Lifecycle

Install/update/rollback/status operations use tenant transactions. Uninstall preserves a row with status and `uninstalled_at`, functioning as soft uninstall rather than universal soft delete.

## Constraints and Indexes

FKs, checked statuses, permission validation, JSON checks, active partial uniqueness, tenant indexes, and RLS constrain runtime records.

## Persistence Mapping

Marketplace services use direct SQLx/local rows; artifact verification precedes selected transaction writes. Filesystem artifact tests exist but are not a database harness.

## Security and Privacy

Permissions, configuration, hooks, imported page IDs, and kill-switch reasons are security-sensitive. Bypass access and runtime adapters need strict review.

## Known Risks and Unknowns

RLS test coverage, uninstalled-row retention, cross-tenant adapter coherence, and official sandbox guarantees are unresolved.

## Related Documents

See [Multi-Tenancy](../multi-tenancy.md), [Marketplace Purchases and Entitlements](marketplace-purchases-and-entitlements.md), and [Database Testing](../database-testing.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with [Marketplace Catalog and Installation](../../backend/modules/marketplace-catalog-installation.md) and [Marketplace Runtime Adapters](../../backend/modules/marketplace-runtime-adapters.md).
