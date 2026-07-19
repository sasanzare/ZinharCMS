---
okf_document_id: "domain-cross-module-workflows"
title: "Cross-Module Workflows"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "frontend/src/pages"
related_documents:
  - "domain-catalog.md"
  - "business-rule-catalog.md"
  - "business-rule-testing.md"
related_diagrams:
  - "diagrams/cross-module-orchestration.mmd"
---

# Cross-Module Workflows

## Workflow Catalog

| ID | Workflow | Trigger / actor / preconditions | Modules and entities | Transaction / side effects / completion | Tests and confidence | Document |
| --- | --- | --- | --- | --- | --- | --- |
| `WF-ORG-PROVISION` | Organization provisioning | Authenticated user; valid unique name/slug | Organizations, quota, audit; organization/member/subscription | Three core rows in transaction; audit after commit; complete when detail/membership load succeeds | No integration test; High code confidence | [Open](workflows/organization-provisioning.md) |
| `WF-MEMBERSHIP` | Tenant invitation and membership | Owner/admin creates; recipient accepts valid token | Organization, JWT hash helper, quota, email, audit | Acceptance membership+invitation transaction; email/audit separate | No end-to-end test; High predicates | [Open](workflows/tenant-invitation-and-membership.md) |
| `WF-OWNERSHIP` | Organization ownership transfer | Current owner selects active member | Organization, RBAC, audit; organization/members | Three ownership writes in one transaction; audit after commit | No integration test; High | [Open](workflows/organization-ownership-transfer.md) |
| `WF-CONTENT-SAVE` | Content entry save | Tenant writer; capacity and schema valid | Content, quota, plugin, sanitizer, validation, RLS | Single entry write; synchronous plugin; complete on returned row | Unit coverage for validators/plugins only; High | [Open](workflows/content-entry-save.md) |
| `WF-PUBLISH` | Editorial publication | Writer/reviewer/publisher; allowed current state | Content/pages, workflow, delivery cache, plugin, webhooks, preview | State write commits before non-atomic side effects | Workflow unit tests, little route coverage; High state policy | [Open](workflows/editorial-publication.md) |
| `WF-PAGE-SAVE` | Page Builder save and version | Tenant page writer; valid component tree | Pages, registry, versions, preview, quota/cache | Page+snapshot transaction; broadcast/cache after commit | Frontend shell tests only; High backend sequence | [Open](workflows/page-builder-save-and-version.md) |
| `WF-PAGE-RESTORE` | Page version restoration | Page manager; positive existing version | Pages, versions, registry, preview | Page reset+new snapshot transaction; broadcast after commit | No integration test; High | [Open](workflows/page-version-restoration.md) |
| `WF-MEDIA-UPLOAD` | Media upload and processing | Media writer; size/type/quota valid | Media, quota, filesystem, image service, RLS | Files and DB writes are not one transaction; four image variants | No workflow tests; High observed behavior | [Open](workflows/media-upload-and-processing.md) |
| `WF-WEBHOOK` | Publication webhook delivery | Successful entry/page publish or unpublish | Content/pages, delivery, webhook service, HTTP, delivery log | In-process task after state write; no retry/outbox | Validator/signature unit tests; Medium delivery guarantee | [Open](workflows/publication-webhook-delivery.md) |
| `WF-BILLING` | Billing subscription | Billing manager checkout/manual change or signed Stripe event | Billing, quota, Stripe, audit/email; subscription/events | Provider callback is idempotent transaction; checkout crosses external boundary | Unit tests for signature/status/order; Medium end-to-end | [Open](workflows/billing-subscription.md) |
| `WF-BETA` | Beta feedback and readiness | Tenant member/editor/admin or global admin | Beta, quota, billing, webhook, audit | Feedback/blocker writes plus audit in transaction; dashboards aggregate | Validator/query static tests; Medium | [Open](workflows/beta-feedback-and-readiness.md) |
| `WF-MARKET-PUBLISH` | Marketplace product publication | Approved creator submits; global admin reviews | Submission, package, validation, review, catalog, files, events | Multi-stage writes; artifact immutability; review decision publishes or rejects | Strong service/static tests; High rules, Medium full flow | [Open](workflows/marketplace-product-publication.md) |
| `WF-MARKET-INSTALL` | Marketplace installation lifecycle | Tenant admin confirms organization and permissions | Catalog, installation, runtime, adapters, entitlement, audit | Explicit pinned lifecycle; data-preserving uninstall; no auto-update | Strong service/frontend tests; High | [Open](workflows/marketplace-installation-lifecycle.md) |
| `WF-MARKET-PURCHASE` | Marketplace purchase and entitlement | Tenant billing actor checks out; provider confirms | Marketplace finance, Stripe, entitlement, ledger, catalog | Provider callback transaction grants/revokes; external checkout precedes it | Finance and Stripe tests; Medium full provider flow | [Open](workflows/marketplace-purchase-and-entitlement.md) |

## Orchestration Ownership

- Handler-level orchestration owns organization, content, page, media, beta, and many Marketplace API workflows.
- Domain/application services own reusable workflow policy, validation, quotas, Stripe event processing, webhook dispatch, media processing, and Marketplace policies.
- Database triggers/constraints finalize tenant propagation, immutability, uniqueness, and allowed stored values.
- In-process plugin hooks, preview broadcasts, webhook tasks, and image processing are not a general event/worker architecture.
- `CROSS_MODULE_ORCHESTRATION_UNCLEAR`: there is no uniform rule for whether audit, cache, email, files, network delivery, or provider calls must succeed before a workflow is considered complete.

