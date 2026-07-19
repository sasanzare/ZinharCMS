---
okf_document_id: "extensibility-testing"
title: "Extensibility Testing"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "partially_verified"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/plugins/seo.rs"
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "frontend/src/pages/MarketplacePage.test.tsx"
  - "frontend/src/pages/PagesPage.test.tsx"
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
related_documents:
  - "../backend/testing-map.md"
  - "../frontend/testing-map.md"
  - "../api/api-testing.md"
  - "plugins/seo-auto.md"
  - "extension-points/cms-entry-before-save.md"
  - "extension-points/marketplace-runtime-authorization.md"
related_diagrams:
  - "diagrams/plugin-lifecycle.mmd"
---

# Extensibility Testing

| Surface | Observed tests | Important gap |
|---|---|---|
| SEO built-in | Slug normalization and missing-slug mutation unit tests | No real content-route/DB integration or failure propagation test |
| Plugin registry/routes | No colocated tests found | Sync preservation, RBAC, settings, enable/disable, stale rows |
| Manifest/package | Unit tests for required fields, semver, paths, checksums, package rules | No end-to-end upload/review/install with real DB/storage |
| Installation lifecycle | Pure transition/permission/version tests plus filesystem artifact tests | No tenant database transaction/concurrency suite |
| Runtime authorization | Permission, product, path, size, kill-switch state helper tests | No executable runtime exists to test |
| Host adapters | Component/hook/template extraction and selected route helper tests | Registry cleanup/collisions and real page import/RLS integration |
| Marketplace frontend | MarketplacePage behavior tests | Backend contract/database integration |
| Page Builder frontend | PagesPage tests include Marketplace adapter interactions | General declarative component renderer compatibility |
| Finance/feedback/analytics/readiness | Numerous pure/service tests | Provider, RLS, transaction, concurrency, and operational verification |

## Required Test Matrix

| Concern | Test path | Test type | Positive behavior | Negative behavior | Permission case | Tenant case | Compatibility case | Failure case | Confidence | Apparent gap |
|---|---|---|---|---|---|---|---|---|---|---|
| Plugin registration | No direct test found | Gap | SEO metadata appears through source methods | Not covered | Manager routes not covered | Global scope not contrasted with tenant | Source version not tested | Duplicate/stale key not covered | Low | Registry sync, preservation, stale cleanup |
| Manifest validation | backend/src/services/marketplace_manifest.rs | Unit | Valid manifest accepted | Missing/unsupported fields rejected | Unknown permission rejected | Tenant scope intentionally absent from manifest | Semver/range shape checked | Invalid field returns error | High | Server upload integration |
| Package validation | marketplace_package.rs; marketplace_validation.rs | Unit/helper | Checksum/path/assets accepted | Traversal, size, missing assets, risk blocked | Declared permission inputs checked in manifest | Compatibility report uses tenant plan | Package/version constraints checked | Invalid artifact rejected | High | Real upload/storage/review transaction |
| Installation | marketplace_installation.rs; routes/marketplace.rs | Unit/helper | Eligible transition/artifact accepted | Unsupported type/tamper rejected | Exact snapshot required | Tenant persistence not executed live | Install gate and semver tested | Filesystem verification errors tested | Medium-high | Real DB/RLS/concurrency |
| Activation | marketplace_installation.rs; marketplace_runtime.rs | Unit | Disabled to active allowed | Blocked/uninstalled denied | Installer/kill switch logic partially helper-tested | Organization runtime state represented | Current version retained | Invalid transition rejected | Medium | Route/RBAC/database integration |
| Deactivation | marketplace_installation.rs | Unit | Active to disabled | Invalid states denied | Installer route not integrated | Tenant scope not live-tested | Version unchanged | Conflict behavior helper-only | Medium | Real route/audit verification |
| Removal | marketplace_installation.rs | Unit | Active/disabled/blocked to uninstalled | Terminal transitions denied | Installer route not integrated | Soft tenant record modeled | Version/history preserved | Cleanup failures not represented | Medium | Derived component/artifact/data cleanup |
| Compatibility | manifest/catalog/validation/installation services | Unit | Compatible host/plan/version accepted | Invalid range/plan/version rejected | Permission change tied to update | Tenant plan input covered in pure tests | Min/max/semver/update/rollback checked | Conflict/report reasons checked | High | Deployed host version and renderer compatibility |
| Permissions | marketplace_runtime.rs; marketplace_installation.rs | Unit | Approved declared operation allowed | Escalation/unknown operation denied | Exact grant and reapproval tested | Snapshot contains tenant installation context | Permission changes on update | Denial reason bounded | High | Host operation execution and route RBAC matrix |
| Tenant scope | RLS migrations and route code | Static/structural | Organization IDs and forced RLS declared | No live cross-tenant request test | RBAC helpers documented | Tenant context/RLS wiring observed | Not applicable | Bypass/mis-context behavior unexecuted | Medium-low | Real two-tenant integration suite |
| Component registration | marketplace_adapters.rs; PagesPage.test.tsx | Unit/frontend | Definitions namespaced and loaded | Invalid/private definitions rejected | Component/page permission path observed | Tenant installation namespace | Contract version/default schema | Collision/stale cleanup not tested | Medium | DB upsert/RLS/renderer matrix |
| Page Builder blocks | PagesPage.test.tsx; routes/pages.rs tests where present | Frontend/unit | Builder shell and registered keys work | Structural validation rejects invalid page JSON | Page/component actions gated | Tenant components queried | Props/render compatibility partial | Deleted/missing component references | Medium | All 28 schemas and published renderer |
| Hooks and events | marketplace_adapters.rs; marketplace_runtime.rs | Unit | Public definitions extracted/authorized | Private type/permission/path denied | Runtime capability checked | Tenant installation checked in code | Contract version default | Delivery absent, so retry/order untested | Medium | Frontend/delivery implementation |
| Plugin data | SEO unit tests; migration/schema evidence | Unit/static | SEO mutates missing slug | Existing slug preserved | No plugin data permission test | PluginContext unused by SEO | Same-build only | DB rollback/data retention untested | Medium | Real content transaction and ownership |
| Migrations | Marketplace phase contract tests and SQL files | Static/unit assertions | Required tables/constraints found | Selected missing-contract checks | RLS definitions inspected | Forced RLS source checked | Additive migration order | Applied-state failure untested | Medium | Live migrate/upgrade/rollback |
| Isolation | marketplace_runtime.rs policy tests | Unit | Safe declared request authorized | Unsafe path/large payload/blocked state denied | Capability policy covered | Kill switches covered | Policy version fixed | No executor exists to crash/contain | High for policy; none for execution | Executable sandbox tests impossible until implemented |
| Marketplace end-to-end | MarketplacePage.test.tsx and service tests | Frontend/unit | UI flows and pure rules | Many denial paths | Role/permission UI cases | Active organization role tested | Update permission/change UX | Backend transaction/provider failures unintegrated | Medium | Creator-to-review-to-install-to-adapter real stack |
| Frontend plugin behavior | PagesPage.test.tsx; MarketplacePage.test.tsx | Frontend | Host UI consumes adapter/catalog data | Unsupported/runtime gates shown | Role/action controls | Organization context used | Permission-change UI | One initial combined-run timeout passed on rerun | Medium-high | No third-party frontend code exists |
| Example plugin | backend/src/plugins/seo.rs | Unit | Generates missing slug | Preserves existing slug implicitly through code; only missing case explicit | No capability model | Tenant context not exercised | Version metadata untested | Callback error impossible in current implementation | Medium | Route/DB/enablement/Unicode tests |

No valid coverage report was found, so no coverage percentage is claimed. Test names and presence demonstrate selected rule coverage only.

Priority scenarios are cross-tenant isolation, full creator-to-install flow, permission escalation on update, kill-switch races, soft-uninstall derived-data behavior, malformed adapter definitions, and hook callbacks failing inside content transactions.

## Phase 9 Verification Run

Verification on 2026-07-19 used repository HEAD 56d733985fdd7aa3f25ee6981b88cf29c52f65c9:

- cargo test --manifest-path backend/Cargo.toml --all-features marketplace: 79 passed, 0 failed.
- cargo test --manifest-path backend/Cargo.toml --all-features plugins::seo: 2 passed, 0 failed.
- npm --prefix frontend test -- src/pages/MarketplacePage.test.tsx: 12 passed, 0 failed.
- PagesPage.test.tsx passed its single test during the initial combined frontend run.
- The initial combined frontend run had one five-second timeout in the first Marketplace test; that test passed in isolation and the complete Marketplace file then passed.
- Phase 9 metadata, YAML, evidence paths, local relations, Markdown links, required files, index registration, diagram references, language, and repository scope checks passed for 39 Markdown files, 7 Mermaid files, and 46 Phase 9 index entries.

## Phase 10 Integration

The general backend CI includes plugin/Marketplace tests through `cargo test --all-features`; targeted filters remain useful locally. CI does not run Marketplace readiness/load scripts, real package upload/installation/adapters, browser E2E, database/RLS isolation, or external-provider smoke. See [Testing Workflow](../development/testing-workflow.md), [CI Job Catalog](../delivery/ci-job-catalog.md), and [Operational Risks](../operations/operational-risks.md).
