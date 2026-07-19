---
okf_document_id: "domain-business-rule-testing"
title: "Business Rule Testing"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src"
  - "frontend/src"
related_documents:
  - "business-rule-catalog.md"
  - "cross-module-workflows.md"
  - "domain-risks.md"
related_diagrams: []
---

# Business Rule Testing

No coverage percentage is reported because no valid coverage report was used.

## Rule and Workflow Matrix

| Rule/workflow | Test path/type | Positive/negative behavior | Auth/tenant/state/transaction coverage | Confidence and apparent gaps |
| --- | --- | --- | --- | --- |
| Workflow transition policy | `backend/src/services/workflow.rs`; unit | Draft-to-review and reviewer bypass positive; author direct-publish negative | No RBAC/tenant/DB/side-effect integration | High pure policy; most transition pairs untested |
| Entry field sanitization | `backend/src/services/security.rs`; unit | Removes script blocks/attributes | No route/schema/tenant coverage | High sanitizer unit; save ordering untested |
| SEO before-save hook | `backend/src/plugins/seo.rs`; unit | Slug normalization and missing-slug mutation | No enable-state/tenant/route coverage | High plugin function; orchestration gap |
| Organization RBAC matrix | `backend/src/services/rbac.rs`; unit | Role-capability matrix | No member mutation/last-owner/tenant DB flow | High helper; workflow gaps |
| Tenant/RLS behavior | static migration and service tests across backend | Policy definitions and selected helpers | Applied catalog and cross-tenant runtime not tested | `TENANT_BEHAVIOR_UNCLEAR` |
| Page Builder UI shell | `frontend/src/pages/PagesPage.test.tsx`; component | System/Marketplace palette, empty canvas, property panel | No backend auth/tenant/state/transaction | Drag/drop, save/autosave, restore, workflow, preview absent |
| Content entries UI | No colocated `EntriesPage` test | None located | None | Major gap |
| Workflow/comments UI | No colocated `WorkflowPage` test | None located | None | Publication/reject/comment/plugin UI gap |
| Media upload/processing | No route/service test located | None | None | MIME, quota, file/DB partial failure, variants untested |
| Webhook policy | `backend/src/services/webhooks.rs`; unit | Safe URL, signature stability; unknown event negative | No tenant/HTTP/retry/transaction integration | High validators; delivery gap |
| Public delivery filters | `backend/src/routes/delivery.rs`; unit | Filter parse and XML escaping | No published-only/cache/tenant runtime test | Query behavior partially covered |
| Billing checkout config | `backend/src/routes/billing.rs`; static/unit | Missing config rejected | No provider/tenant transaction flow | Narrow coverage |
| Quota calculations | `backend/src/services/quota.rs`; unit | Downgrade/unlimited calculations | No concurrent capacity/DB integration | Good calculation coverage; enforcement gap |
| Stripe billing | `backend/src/services/stripe_billing.rs`; unit | Status mapping, event time/order, signature | No live provider/DB callback integration | Strong pure logic; external flow gap |
| Beta validation/query | `backend/src/routes/beta.rs`; unit/static | Choice/object/text validation and query construction | No tenant/RBAC/transaction integration | Moderate |
| Marketplace manifest/package/validation | Marketplace service tests; unit/static | Valid inputs and multiple negative package/security cases | Selected ownership/security static contracts | Strongest domain rule coverage |
| Marketplace review | `marketplace_review.rs`; unit | Decision mapping; blocks high-risk approval; moderation requirements | No full DB route workflow | Strong service policy |
| Marketplace installation/runtime | installation/runtime services plus `MarketplacePage.test.tsx` | Permission equality, reapproval, lifecycle, kill switch, duplicate install, role gate | Good lifecycle/UI; live tenant DB limited | Strong relative coverage |
| Marketplace finance | finance/Stripe services; unit/static | Amount split, invalid pricing/payout, completion/refund contracts | No live provider settlement | Strong pure rules; external flow gap |
| Marketplace feedback | feedback service and Marketplace frontend tests | Rating bounds, abuse taxonomy, eligibility/UI, admin moderation | Static/ mocked API, not DB workflow | Moderate-high |
| Organization provisioning | No integration test located | None | Transaction/audit not executed | Major gap |
| Invitation/ownership | No integration test located | None | Token/email/quota/last-owner/concurrency absent | Major gap |
| Page version transaction/restore | No backend integration test located | None | Snapshot uniqueness and rollback absent | Major gap |

## Strongly Tested Areas

Marketplace policy, validation, installation/runtime permission behavior, Stripe pure logic, workflow transition helpers, webhook validation/signing, RBAC helpers, sanitizer behavior, and selected quota calculations have focused tests.

## Apparent Gaps

- Negative authorization and tenant-isolation integration tests for core CMS workflows.
- Complete state-transition matrices, including rejected pairs.
- Organization provisioning, invitation acceptance, ownership transfer, and last-owner concurrency.
- Database-constraint tests and transaction rollback assertions.
- Content schema changes against existing entries.
- Entry save/publish side-effect partial failures.
- Page tree validation, autosave, version restore, concurrent snapshots, and preview delivery.
- Media file/DB compensation and orphan cleanup.
- Webhook retry/process-loss behavior.
- Publication cache consistency.

## Phase 8 Verification Run

The following checks were executed on 2026-07-19 against commit `5a6f4f3147cc44a22c00ca0f02c8599fd927244f` plus the documentation-only working-tree changes:

- `cargo test --manifest-path backend/Cargo.toml`: 117 backend tests passed; zero failed.
- `npm --prefix frontend test -- --testTimeout=15000`: three test files and 14 tests passed; zero failed. The first run with Vitest's default five-second timeout produced one Marketplace test timeout; the targeted retry and the complete 15-second-timeout run passed without source changes.
- `okf/index.yaml` and all 44 Phase 8 Markdown frontmatter blocks parsed as YAML; all 50 Phase 8 files were registered exactly once.
- 2,183 relative Markdown links across the OKF corpus resolved, including complete Phase 8 README, domain-catalog, and workflow-catalog navigation.
- Phase 8 source paths and related paths passed existence and exact-casing checks.
- Six Mermaid files passed declaration, title/source-comment, note-block, and prohibited `opt`/`else` structural checks. No local Mermaid parser package or CLI was available, so parser/render validation remains deferred to Phase 11.
- The OKF language scan found no characters in the Arabic/Persian Unicode block, and `git diff --check -- okf` passed.
