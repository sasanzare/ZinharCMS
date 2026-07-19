---
okf_document_id: "marketplace-workflows"
title: "Marketplace Workflows"
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
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/routes/marketplace_analytics.rs"
related_documents:
  - "marketplace-architecture.md"
  - "installation-and-removal.md"
  - "activation-and-deactivation.md"
related_diagrams:
  - "diagrams/marketplace-installation-flow.mmd"
  - "diagrams/plugin-permission-flow.mmd"
---

# Marketplace Workflows

| Workflow | Main sequence | Terminal or result |
|---|---|---|
| Creator publication | Creator profile to listing to upload to submission to validation/review | Approved catalog version or rejected/changes requested |
| Free install | Select version, validate gates, approve exact permissions, verify artifact, persist | Active tenant installation |
| Paid install | Completed purchase/entitlement plus install gates | Active tenant installation |
| Update | Find newer approved compatible version, confirm changelog, reapprove changed permissions | Version changes; previous version retained |
| Rollback | Validate stored target, compatibility, artifact, and permission snapshot | Current and rollback versions swap |
| Disable/enable | Validate state and kill-switch gate | Disabled or active |
| Uninstall | Validate state and soft transition | Terminal uninstalled record; data preserved |
| Component use | Active ready component-pack installation, extract definitions, upsert registry, load builder | Declarative component available |
| Template import | Active ready template, map tenant assets, validate JSON/quota, transactionally create page/version/import audit | New page |
| Public hook | Active ready integration definition, validate type/context/runtime | Allowed/denied authorization; not executed |
| Emergency block | Privileged global/tenant kill-switch action | New install, re-enable, and runtime authorization blocked |
| Feedback | Eligible organization review/report and moderator action | Published/rejected review or resolved report |

MARKETPLACE_BEHAVIOR_UNCLEAR remains for production provider behavior, stale derived-data cleanup, and package execution because the latter is absent.

## Workflow Contract Matrix

| Workflow ID | Actor | Preconditions | Main steps | Permissions | Scope | Entities | APIs | Frontend | Side effects | Failure behavior | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MP-WF-01 Creator registration | Authenticated user/admin | Active identity; safe profile data | Request/update creator; admin verifies | Creator self-service/global admin | Global creator identity | creators | Creator and verification routes | Marketplace creator form | Audit/state updates | Validation/auth error | Submission/domain tests | High |
| MP-WF-02 Listing creation | Approved creator | Verified creator and complete metadata | Create/update listing | Creator ownership | Global catalog draft | listings | Listing routes | Listing form/list | Draft metadata | Validation/ownership denial | Submission tests | High |
| MP-WF-03 Package upload | Approved creator | Owned listing; file and manifest | Parse multipart, validate manifest/type, checksum/store artifact, create version/submission | Creator ownership | Global version | versions, submissions, artifact | Version upload | Version upload form | File plus DB state | Validation/storage/transaction errors | Manifest/package/submission tests | High |
| MP-WF-04 Review/publication | Reviewer/admin | Submitted version and validation report | Review, approve/reject/request changes; moderate if required | Global reviewer/admin | Global | submissions, versions, listings, review events | Review queue/decision/moderation | Admin review UI | Catalog eligibility/audit | Invalid transition/risk gate denial | Review/policy tests | High |
| MP-WF-05 Catalog search/detail | Tenant member | Approved compatible listing/version | Filter/search, project compatibility, show detail | Authenticated tenant reader | Global catalog evaluated for tenant | listings, versions, install counts | Catalog routes | Marketplace catalog | Cache/aggregate reads | Filtered/empty/error | Catalog tests | High |
| MP-WF-06 Install | Tenant installer/approver | Eligible version, exact permissions, artifact, entitlement if paid, no kill switch | Validate, persist active pinned installation, audit | Installer and permission approver | Tenant | installations, entitlement, audit | Installations POST | Install confirmation | DB/audit state | Transaction rollback/conflict | Installation/runtime/frontend tests | High |
| MP-WF-07 Update | Tenant installer/approver | Active/disabled; newer compatible version; changelog; permissions | Check, approve changes, verify, switch version, retain rollback | Installer/approver | Tenant | installation/version/audit | Update check/POST | Update controls | Snapshot/version change | Conflict/rollback | Installation/frontend tests | High |
| MP-WF-08 Rollback | Tenant installer | Safe retained target and permissions | Validate and swap versions/snapshots | Installer | Tenant | installation/version/audit | Rollback POST | Rollback control | Version reversal | Conflict/current retained | Installation tests | High |
| MP-WF-09 Disable/enable/uninstall | Tenant installer | Valid current state; no enable kill switch | Transition and audit; uninstall soft | Installer | Tenant | installation/audit | Lifecycle POST routes | Installed-app controls | Eligibility change, retained data | Invalid transition conflict | Lifecycle/frontend tests | High |
| MP-WF-10 Component/template use | Page editor/manager | Active-ready eligible installation | Extract/sync definitions or preview/map/import template | Runtime/install plus page/component RBAC | Tenant | component registry, pages, versions, imports | Adapter routes | PagesPage | Page/registry/audit writes | Validation/transaction error | Adapter/Page tests | Medium-high |
| MP-WF-11 Public hook authorize | Tenant host consumer | Matching active-ready hook declaration | List, validate, authorize | Runtime permission snapshot | Tenant | manifest/hooks/install | Hook routes | No verified renderer | Decision only | Denied; not executed | Adapter/runtime tests | Medium |
| MP-WF-12 Purchase/entitlement | Tenant buyer | Paid listing and provider readiness | Checkout/payment confirmation, purchase/entitlement, install eligibility | Billing/Marketplace finance roles | Tenant | purchases, entitlements, ledger | Finance routes | Marketplace checkout | Provider/DB/audit effects | Provider/transaction error | Finance tests | Medium |
| MP-WF-13 Rating/review/report | Eligible tenant member/moderator | Installed/purchased for review; valid evidence for report | Submit; moderate/publish/reject/investigate/resolve | Member plus global moderation | Tenant input/global moderation | reviews, reports | Feedback routes | Marketplace feedback/admin queues | Operational signals | Validation/auth conflict | Feedback/frontend tests | High |
| MP-WF-14 Delist/deprecate/block | Global moderator/admin | Reason and target | Suspend/unpublish/deprecate/emergency block | Global Marketplace admin | Global with tenant runtime impact | listings, versions, installations, kill switches | Moderation/runtime routes | Admin controls | Catalog/runtime denial and audit | Invalid action/error | Review/runtime tests | High |

Compatibility warnings are implemented through catalog/install/update reports. Arbitrary plugin execution is excluded from every workflow.
