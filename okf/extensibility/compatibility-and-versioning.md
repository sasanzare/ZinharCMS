---
okf_document_id: "plugin-compatibility-versioning"
title: "Compatibility and Versioning"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/plugins/seo.rs"
related_documents:
  - "plugin-manifest.md"
  - "plugin-lifecycle.md"
  - "marketplace/installation-update-rollback.md"
related_diagrams:
  - "diagrams/marketplace-installation-flow.mmd"
---

# Compatibility and Versioning

SEO Auto reports version 1.0.0 from code. No plugin ABI/API compatibility declaration, dependency solver, minimum host version, migration protocol, or deprecation contract exists for CmsPlugin. PLUGIN_COMPATIBILITY_UNCLEAR applies to compiled-plugin upgrades.

Marketplace uses:

- manifest schema version 2026-07;
- semantic-version validation including prerelease/build syntax;
- required minimum ZinharCMS version and optional maximum;
- product/listing/version/review/security/plan/installability gates;
- explicit pinned installation version;
- newer-version comparison for update;
- retained rollback_version_id and permission snapshot checks.

The host version comparison and plan compatibility report are enforced by Marketplace validation/install code, not a general executable runtime ABI. The adapter contract has version 2026-07 and public hook definitions can default to that contract version.

Automatic update was not found. Update and rollback are explicit, audited actions. NEEDS_OWNER_CONFIRMATION: define long-term support policy for manifest schema, adapter contracts, compiled CmsPlugin trait changes, and stored definitions.

## Compatibility Rule Matrix

| Rule | Enforcement location | Failure behavior | User-visible behavior | Tests | Confidence |
|---|---|---|---|---|---|
| Built-in plugin version | SeoAutoPlugin returns 1.0.0 | No runtime compatibility failure model | Version appears in plugin metadata | No version-behavior test | High for value, low for policy |
| CmsPlugin API compatibility | Rust compilation in same repository | Build fails on incompatible trait implementation | Deployment cannot produce binary | Compiler only | High |
| Manifest version equals 2026-07 | marketplace_manifest.rs | Validation rejection | Upload/validation error | Valid/invalid manifest tests | High |
| Package version is semantic version | Manifest and installation services | Validation/update rejection | Validation reason or no eligible update | Semver tests | High |
| Minimum ZinharCMS version | Manifest compatibility object and gate | Catalog/install ineligible | Compatibility report/reason | Manifest/catalog/validation tests | High |
| Optional maximum ZinharCMS version | Compatibility gate | Catalog/install ineligible | Compatibility report/reason | Validation tests | High |
| Plan compatibility | Catalog/install validation | Product hidden/ineligible or conflict | Compatibility badge/reason | Catalog/validation tests | High |
| Product/listing/version/review/risk status | Install/update/rollback gates | Conflict/filtered candidate | Install/update unavailable | Route/helper tests | High |
| Newer-version update | Semantic precedence comparison | Update rejected | No update or conflict | Installation semver test | High |
| Permission compatibility | Compare current snapshot with target manifest | Reapproval required or update rejected | Permission-change UI and request requirement | Installation/frontend tests | High |
| Rollback compatibility | Same-listing FK, approved/deprecated target, artifact and permission snapshot | Rollback conflict | Error; current version remains | Route/helper tests | High |
| Adapter contract version 2026-07 | marketplace_adapters.rs default/constant | No negotiation path found | Contract value returned | Adapter tests | Medium |
| Component props-schema compatibility | Current registry/page validator | Unknown renderer/property mismatch behavior | Save may pass key/document checks | Page tests do not prove full schema | Low |
| Plugin dependencies/conflicts | No dependency solver or declaration contract | Not applicable | No supported UI | None | High confidence absent |
| Downgrade/deprecation | Explicit rollback may target approved/deprecated same-listing version | Other downgrade paths rejected/absent | Rollback action only | Deprecated rollback test | High |
| Migration compatibility | Application migrations only | Startup/migration failure possible | Deployment failure | Broader migration tests limited | Medium |

PLUGIN_COMPATIBILITY_UNCLEAR remains for stable CmsPlugin interfaces, frontend renderer contracts, manifest deprecation windows, and multi-version adapter support.
