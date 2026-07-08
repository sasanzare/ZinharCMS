# Architecture Audit

Inspection date: 2026-07-04.

Status values:

- `IMPLEMENTED`: Current schema and runtime code support the feature.
- `PARTIALLY_IMPLEMENTED`: Some core pieces exist, but important runtime/API/UI paths are absent or limited.
- `PLANNED_ONLY`: Explicitly planned for future phases, with no current implementation evidence.
- `DOCUMENTED_ONLY`: Present in documentation only and not found in code/schema.
- `CODE_ONLY`: Present in code/schema but not represented in current docs or central API docs.
- `DOCUMENTATION_CONFLICT`: Documentation contradicts current implementation evidence.
- `UNVERIFIED`: Not enough evidence was found in this pass.

## Implementation Matrix

| Domain | Status | Evidence paths | Confidence | Notes |
| --- | --- | --- | --- | --- |
| Authentication | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/src/routes/auth.rs`; `backend/src/services/password.rs`; `backend/src/services/jwt.rs`; `backend/src/middleware/auth.rs`; `frontend/src/pages/AuthPage.tsx`; `frontend/src/services/api.ts` | High | Register, login, refresh, logout, and me endpoints exist with password hashing and JWT claims. |
| Global RBAC | PARTIALLY_IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/migrations/0002_seed_foundation_data.sql`; `backend/src/services/rbac.rs`; `backend/src/routes/auth.rs`; `docs/PHASE_THREE.md` | High | Role checks are implemented, but user/role management CRUD is documented as planned and no admin user-management route was found. |
| Refresh token rotation | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/src/routes/auth.rs`; `backend/src/services/jwt.rs` | High | Refresh tokens are hashed in `refresh_tokens`; refresh revokes the old token and issues a new cookie-backed token. |
| Content types | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/routes/content.rs`; `backend/src/services/entry_validation.rs`; `frontend/src/pages/ContentTypesPage.tsx`; `frontend/src/services/api.ts` | High | Tenant-scoped schema CRUD and field validation are present. |
| Entries | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/migrations/0006_phase_six_workflow_collaboration.sql`; `backend/src/routes/content.rs`; `backend/src/services/workflow.rs`; `frontend/src/pages/EntriesPage.tsx`; `frontend/src/components/DynamicForm.tsx` | High | Entry CRUD, dynamic forms, validation, and workflow transitions exist. |
| Media | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/routes/media.rs`; `backend/src/services/media_processing.rs`; `frontend/src/pages/MediaPage.tsx` | High | Uploads, metadata update/delete, organization paths, and image variants exist. |
| Page builder | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/migrations/0004_phase_two_page_builder.sql`; `backend/src/routes/pages.rs`; `frontend/src/pages/PagesPage.tsx`; `frontend/src/pages/PagesPage.test.tsx` | High | Component registry, page JSON, visual builder shell, drag/drop UI, and props editor exist. |
| Page versions | IMPLEMENTED | `backend/migrations/0001_initial_schema.sql`; `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/routes/pages.rs`; `frontend/src/pages/PagesPage.tsx`; `frontend/src/services/api.ts` | High | Page versions are created and can be listed/restored. |
| Live preview | IMPLEMENTED | `backend/src/routes/pages.rs`; `backend/src/state.rs`; `backend/src/middleware/auth.rs`; `frontend/src/pages/PagesPage.tsx` | High | Preview WebSocket route exists and accepts bearer or query tokens for browser WebSocket clients. |
| Delivery API | IMPLEMENTED | `backend/migrations/0005_phase_five_delivery_api.sql`; `backend/src/routes/delivery.rs`; `backend/src/services/cache.rs`; `docs/API.md`; `docs/PHASE_FIVE.md` | High | Public content/pages/settings/navigation/sitemap/robots routes exist with published-content filtering. |
| Redis caching | IMPLEMENTED | `backend/src/services/cache.rs`; `backend/src/routes/delivery.rs`; `backend/src/routes/mod.rs`; `backend/src/services/rate_limit.rs`; `docker-compose.yml`; `.github/workflows/backend-ci.yml` | High | Delivery cache and rate limiting use Redis with fallback behavior for delivery cache. |
| Webhooks | IMPLEMENTED | `backend/migrations/0005_phase_five_delivery_api.sql`; `backend/src/routes/webhooks.rs`; `backend/src/services/webhooks.rs`; `frontend/src/pages/SettingsPage.tsx`; `frontend/src/services/api.ts` | High | Subscription CRUD, URL validation, HMAC signing, delivery recording, and test dispatch exist. |
| Workflow | IMPLEMENTED | `backend/migrations/0006_phase_six_workflow_collaboration.sql`; `backend/src/services/workflow.rs`; `backend/src/routes/content.rs`; `backend/src/routes/pages.rs`; `frontend/src/pages/WorkflowPage.tsx` | High | Draft, pending review, publish, reject, archive, and restore transitions are enforced for entries/pages. |
| Comments | IMPLEMENTED | `backend/migrations/0006_phase_six_workflow_collaboration.sql`; `backend/src/routes/comments.rs`; `frontend/src/pages/WorkflowPage.tsx`; `frontend/src/services/api.ts` | High | Comments support entity linkage, resolve/unresolve, delete, and role checks. |
| Plugins | PARTIALLY_IMPLEMENTED | `backend/migrations/0006_phase_six_workflow_collaboration.sql`; `backend/src/plugins/mod.rs`; `backend/src/plugins/seo.rs`; `backend/src/routes/plugins.rs`; `frontend/src/pages/WorkflowPage.tsx`; `docs/PHASE_SIX.md` | High | Built-in plugin registry and hook execution exist; external install/sandbox/plugin package runtime is not implemented. |
| Security hardening | IMPLEMENTED | `backend/migrations/0007_phase_seven_security.sql`; `backend/src/middleware/security.rs`; `backend/src/services/security.rs`; `backend/src/services/webhooks.rs`; `backend/src/services/hardening.rs`; `docs/PHASE_SEVEN.md`; `docs/V2_PHASE_EIGHT.md` | High | Login throttling, security headers, sanitization, webhook SSRF checks, MIME checks, and static hardening tests exist. |
| Localization | PARTIALLY_IMPLEMENTED | `frontend/src/i18n/*`; `frontend/src/styles/index.css`; `frontend/src/components/AppShell.tsx`; `frontend/src/pages/PagesPage.tsx`; `frontend/src/pages/SettingsPage.tsx`; `docs/I18N.md`; AMB-032 | High | Frontend i18n supports English/Persian, persisted locale selection, and RTL document direction. Some admin UI labels remain hard-coded, and backend/content localization is limited to delivery locale filters rather than a full translation system. |
| Organizations | IMPLEMENTED | `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/routes/organizations.rs`; `backend/src/models/organization.rs`; `frontend/src/pages/OrganizationPage.tsx`; `frontend/src/stores/useAppStore.ts` | High | Organization CRUD/current org, workspace slug, members, domains, and tenant-owned data model exist. |
| Membership and invitations | IMPLEMENTED | `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/routes/organizations.rs`; `backend/src/services/email.rs`; `frontend/src/pages/OrganizationPage.tsx` | High | Members, role updates/removal, invitations, acceptance, and invitation email persistence exist. |
| Tenant middleware | IMPLEMENTED | `backend/src/routes/mod.rs`; `backend/src/middleware/tenant.rs`; `backend/src/services/rate_limit.rs`; `backend/src/services/quota.rs`; `frontend/src/services/api.ts`; `frontend/src/stores/useAppStore.ts` | High | Tenant routes require `X-Organization-Id`, active membership, rate limit, and API quota checks. |
| RLS | IMPLEMENTED | `backend/migrations/0009_v2_phase_three_rls.sql`; `backend/migrations/0010_v2_phase_five_billing_quota.sql`; `backend/migrations/0012_v2_phase_seven_saas_ops.sql`; `backend/migrations/0014_v2_phase_nine_beta_release.sql`; `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/src/services/rls.rs`; `backend/src/services/hardening.rs` | High | Tenant-owned tables use forced RLS and runtime context helpers. Global Marketplace listing/creator tables are intentionally not tenant-owned. |
| Billing plans | IMPLEMENTED | `backend/migrations/0010_v2_phase_five_billing_quota.sql`; `backend/src/routes/billing.rs`; `backend/src/services/quota.rs`; `frontend/src/pages/BillingPage.tsx`; `docs/V2_BILLING_GUIDE.md` | High | Free, Pro, and Enterprise plans exist; manual plan changes are implemented. |
| Stripe | PARTIALLY_IMPLEMENTED | `backend/migrations/0011_v2_phase_six_stripe_billing.sql`; `backend/src/routes/billing.rs`; `backend/src/services/stripe_billing.rs`; `.env.example`; `frontend/src/pages/BillingPage.tsx` | High | Organization subscription checkout, portal, and webhooks exist. Marketplace purchases, entitlements, and creator payouts are not implemented. |
| Quotas | IMPLEMENTED | `backend/migrations/0010_v2_phase_five_billing_quota.sql`; `backend/src/services/quota.rs`; `backend/src/middleware/tenant.rs`; `backend/src/routes/billing.rs`; `frontend/src/pages/BillingPage.tsx` | High | Member/content/media/API usage checks and usage rebuild are implemented. |
| Audit logs | IMPLEMENTED | `backend/migrations/0012_v2_phase_seven_saas_ops.sql`; `backend/src/services/audit.rs`; `backend/src/routes/organizations.rs`; `frontend/src/pages/OrganizationPage.tsx` | High | Tenant audit log storage/listing and audit writes from sensitive actions exist. |
| Email deliveries | IMPLEMENTED | `backend/migrations/0012_v2_phase_seven_saas_ops.sql`; `backend/src/services/email.rs`; `backend/src/routes/organizations.rs`; `frontend/src/pages/OrganizationPage.tsx`; `.env.example` | High | Invitation and billing notification delivery records are persisted; webhook provider is optional. |
| SaaS alerts | PARTIALLY_IMPLEMENTED | `backend/migrations/0012_v2_phase_seven_saas_ops.sql`; `backend/src/routes/organizations.rs`; `frontend/src/pages/OrganizationPage.tsx`; `docs/V2_PHASE_SEVEN.md` | High | Alert rules are seeded and listable, but no alert CRUD/update route or alert execution engine was found. |
| Beta feedback | IMPLEMENTED | `backend/migrations/0014_v2_phase_nine_beta_release.sql`; `backend/src/routes/beta.rs`; `frontend/src/pages/BetaPage.tsx`; `docs/V2_PHASE_NINE.md` | High | Participants, feedback, GA blockers, organization dashboard, and product dashboard exist. |
| GA readiness | IMPLEMENTED | `backend/src/services/ga_readiness.rs`; `scripts/v2-ga-check.ps1`; `docs/V2_PHASE_TEN.md`; `docs/V2_RELEASE_NOTES.md`; `docs/V2_OPERATIONS_RUNBOOK.md` | High | GA readiness is implemented as docs, scripts, and static tests rather than a runtime feature. |
| Marketplace creators | IMPLEMENTED | `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/migrations/0016_v3_phase_two_creator_submission.sql`; `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_submission.rs`; `frontend/src/pages/MarketplacePage.tsx` | High | Creator profiles, verification states, validation, and UI exist. |
| Listings | IMPLEMENTED | `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/migrations/0016_v3_phase_two_creator_submission.sql`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx` | High | Creator listing CRUD, metadata, submit action, and listing table UI exist. |
| Versions/packages | IMPLEMENTED | `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/migrations/0016_v3_phase_two_creator_submission.sql`; `backend/src/services/marketplace_package.rs`; `backend/src/services/marketplace_manifest.rs`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx` | High | Package upload, object key/checksum/size rules, manifest validation, and immutable version trigger exist. |
| Validation | IMPLEMENTED | `backend/migrations/0017_v3_phase_three_validation_pipeline.sql`; `backend/src/services/marketplace_validation.rs`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx`; `docs/V3_PHASE_THREE.md` | High | Static package validation and security scan produce stored reports and block unsafe submissions. |
| Compatibility | IMPLEMENTED | `backend/migrations/0017_v3_phase_three_validation_pipeline.sql`; `backend/src/services/marketplace_validation.rs`; `backend/src/services/marketplace_catalog.rs`; `frontend/src/pages/MarketplacePage.tsx` | High | Compatibility reports include plan/version/feature checks and `install_eligible`. |
| Reviews | PARTIALLY_IMPLEMENTED | `backend/migrations/0018_v3_phase_four_review_moderation.sql`; `backend/src/services/marketplace_review.rs`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx` | High | Internal Marketplace review workflow exists. Public catalog product reviews have response/UI placeholders but no persistence or write API was found. |
| Moderation | IMPLEMENTED | `backend/migrations/0018_v3_phase_four_review_moderation.sql`; `backend/src/services/marketplace_review.rs`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx`; `docs/V3_PHASE_FOUR.md` | High | Admin moderation supports suspend listing, unpublish version, and emergency block; emergency block updates active installations. |
| Catalog | IMPLEMENTED | `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_catalog.rs`; `frontend/src/pages/MarketplacePage.tsx`; `docs/V3_PHASE_FIVE.md`; `README.md` | High | Tenant-aware catalog and detail APIs filter approved/safe/compatible listings and render catalog UI. Central OpenAPI docs are stale. |
| Installations | PARTIALLY_IMPLEMENTED | `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx`; `docs/V3_PHASE_THREE.md`; `docs/V3_PHASE_FIVE.md`; `docs/V3_MARKETPLACE_GAP_LIST.md` | High | Installation table, RLS, active-install counts, and emergency-block updates exist. No install/uninstall/update endpoint or active install button was found; UI says install is deferred. |
| Purchases | PLANNED_ONLY | `docs/V3_PHASE_ONE.md`; `docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md`; `docs/V3_MARKETPLACE_GAP_LIST.md`; absence from `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; absence from `backend/src/routes/marketplace.rs` | High | Purchase is a future paid-product concept. No purchase table, route, service, or UI action was found. |
| Creator payouts | PLANNED_ONLY | `docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md`; `docs/V3_MARKETPLACE_GAP_LIST.md`; `backend/src/routes/marketplace.rs`; `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` | High | Creator payout status exists on creators, but payout provider, ledger, payout records, and payout routes are not implemented. |

## Initial Documentation Conflicts

| Conflict | Evidence | Impact |
| --- | --- | --- |
| Marketplace routes are mounted but missing from central OpenAPI registration. | `backend/src/routes/mod.rs`; `backend/src/routes/marketplace.rs` | API diagrams should treat Marketplace runtime routes as implemented, but OpenAPI documentation as stale. |
| `docs/API.md` does not cover V2 organizations/billing/beta or V3 Marketplace routes. | `docs/API.md`; `backend/src/routes/organizations.rs`; `backend/src/routes/billing.rs`; `backend/src/routes/beta.rs`; `backend/src/routes/marketplace.rs` | API reference diagram should not rely on `docs/API.md` alone. |
| `env.example` is smaller than `.env.example` and omits several current backend config fields. | `env.example`; `.env.example`; `backend/src/config.rs` | Deployment/config diagram should use `.env.example` plus `config.rs` as primary evidence. |
| `docker-compose.prod.yml` exposes only a subset of backend environment variables. | `docker-compose.prod.yml`; `.env.example`; `backend/src/config.rs` | Production deployment diagram should call out optional/unset Stripe/email/rate-limit settings. |
| Marketplace install/payment concepts appear in planning docs, while runtime install and payment paths are not implemented. | `docs/V3_PHASE_ONE.md`; `docs/V3_PHASE_THREE.md`; `docs/V3_PHASE_FIVE.md`; `docs/V3_MARKETPLACE_GAP_LIST.md`; `backend/src/routes/marketplace.rs` | Future diagrams must separate schema/planning from implemented install/purchase runtime behavior. |

## Ambiguity Register Links

| Ambiguity ID | Domain | Audit impact |
| --- | --- | --- |
| AMB-001 | Page Builder | Page Builder remains `IMPLEMENTED`; phase-three future wording is historical. |
| AMB-002 | Media and Marketplace package storage | Use local filesystem storage with logical object keys; do not draw S3/CDN. |
| AMB-003 | Marketplace Catalog | Catalog is tenant-aware and authenticated through tenant middleware. |
| AMB-004 | Authorization | Global RBAC and organization membership roles must be diagrammed as separate layers. |
| AMB-005 | Tenancy and Super Admin | Super admin does not automatically bypass tenant middleware without organization context. |
| AMB-006 | Plugins | Built-in plugin hooks are executable; external plugin runtime remains absent. |
| AMB-007 | Marketplace Installations | Installation schema/counts exist, but install/uninstall runtime is not implemented. |
| AMB-008 | Marketplace Purchases | Paid listing metadata exists, but purchase runtime is planned only. |
| AMB-009 | Creator Payouts | Payout status exists, but payout provider/ledger/runtime is planned only. |
| AMB-010 | Email Deliveries | Email delivery is implemented through log/disabled/webhook provider modes. |
| AMB-011 | Webhooks and Background Processing | Webhooks are transient async dispatch with delivery logs; no durable queue/worker/retry. |
| AMB-012 | Redis | Delivery cache has fallback; rate limiting/readiness still depend on Redis availability. |
| AMB-013 | Row Level Security | Forced RLS applies to tenant-owned tables; global Marketplace tables are separate platform state. |
| AMB-014 | Marketplace Package Immutability | Artifact immutability is database-enforced for submitted/validated states. |
| AMB-015 | Marketplace Artifact Cleanup | Cleanup after validation or persistence failure is decision-required; do not draw automatic cleanup. |
| AMB-016 | Marketplace Review and Moderation | Appeal/restoration after moderation is decision-required; no restore path should be drawn. |
| AMB-017 | Marketplace Reviews | Internal review events are implemented; customer ratings/reviews are placeholder behavior. |
| AMB-018 | Stripe Billing | Event ordering is implemented for organization subscription billing only. |
| AMB-019 | Stripe Billing | Webhook idempotency is implemented for organization subscription billing only. |
| AMB-020 | Live Preview | Live preview is in-process WebSocket broadcast; do not draw cross-node pub/sub. |
| AMB-021 | Static Uploads | Media metadata is tenant-owned, but `/uploads` static file bytes are publicly served by path. |
| AMB-022 | API Documentation | API docs/OpenAPI are incomplete for V2/V3; route code is the source of truth. |
| AMB-023 | Marketplace Actors and RBAC | Marketplace reviewer/moderator are operational actors backed by global admin checks, not dedicated roles. |
| AMB-024 | Beta and Support Operations Actors | Beta/support operation maps to global admin and organization admin/editor roles, not a separate support role. |
| AMB-032 | Frontend Localization Coverage | Admin i18n is broad but not complete; untranslated UI labels and API/content localization limits must be shown as partial coverage. |
## Background Tasks And Queue Finding

No durable background task or queue worker was found in the inspected code. Webhook
delivery is dispatched inline by `backend/src/services/webhooks.rs`; preview updates
use in-memory broadcast channels in `backend/src/state.rs` and `backend/src/routes/pages.rs`;
Stripe webhook processing is synchronous in `backend/src/services/stripe_billing.rs`.

## Hard-Coded Or Important Assumptions

| Assumption | Evidence | Diagram impact |
| --- | --- | --- |
| Public delivery defaults to the active organization with slug `default`. | `backend/src/routes/delivery.rs` | Public delivery diagram should show this default-org lookup. |
| Tenant routes require `X-Organization-Id`, with query fallback for preview sockets only. | `backend/src/middleware/tenant.rs` | Tenant request diagrams should include header requirement. |
| Refresh tokens are stored server-side as hashes and browser-side as HttpOnly cookies under `/api/auth`. | `backend/src/routes/auth.rs`; `backend/src/services/jwt.rs` | Auth diagrams should show dual access token and refresh cookie handling. |
| Redis failures do not block delivery cache fallback, but rate-limit Redis errors become service errors. | `backend/src/services/cache.rs`; `backend/src/services/rate_limit.rs` | Reliability diagrams should distinguish cache fallback from rate-limit dependency. |
| Enterprise billing price can be custom/manual unless Stripe price IDs are configured. | `backend/migrations/0010_v2_phase_five_billing_quota.sql`; `backend/src/services/stripe_billing.rs`; `.env.example` | Billing diagrams should separate manual plan changes from Stripe checkout availability. |
