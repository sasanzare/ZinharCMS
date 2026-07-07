# Ambiguity Register

This register records evidence-based interpretation decisions for future Mermaid diagrams.
An item is considered resolved only when current migrations, backend routes/services,
frontend code, or tests provide enough evidence. Naming alone is not treated as proof.

## AMB-001

- ID: AMB-001
- Domain: Page Builder
- Exact question: Is the page builder still future work, or is it implemented in the current application?
- Documentation claim: `docs/PHASE_THREE.md` says the visual drag-and-drop page builder remains phase four work; `docs/PHASE_FOUR.md` describes the page builder as implemented.
- Implementation evidence: `backend/src/routes/pages.rs` exposes page JSON, component registry, versions, publishing, and `/api/preview/{page_id}`; `frontend/src/pages/PagesPage.tsx` includes the visual builder shell, canvas, drag/drop blocks, props editor, and live preview controls.
- Database evidence: `backend/migrations/0004_phase_two_page_builder.sql` adds the component registry and component metadata used by builder blocks.
- Frontend evidence: `frontend/src/pages/PagesPage.tsx` renders the builder UI and uses page/component APIs.
- Test evidence: `frontend/src/pages/PagesPage.test.tsx` asserts the builder UI, component palette, canvas, and props editor.
- Conflict or missing information: Historical phase documentation is stale when read without the later phase document and current code.
- Safest interpretation: Treat Page Builder as implemented, with historical documentation conflict noted.
- Representation to use in diagrams: Use `[IMPLEMENTED]` and add an evidence comment that phase three documentation is historical.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future page-builder diagrams.

## AMB-002

- ID: AMB-002
- Domain: Media and Marketplace package storage
- Exact question: Does the current implementation use local storage or S3-compatible object storage?
- Documentation claim: Marketplace docs discuss object keys and artifact storage concepts, but no current document proves S3 runtime integration.
- Implementation evidence: `backend/src/config.rs` defines `UPLOAD_DIR`; `backend/src/routes/media.rs` writes uploads with local filesystem paths; `backend/src/routes/marketplace.rs` persists package artifacts under the upload directory and records local storage metadata.
- Database evidence: `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` stores `artifact_object_key` and `storage_metadata`, not S3 bucket credentials or provider state.
- Frontend evidence: `frontend/src/pages/MediaPage.tsx` and `frontend/src/pages/MarketplacePage.tsx` upload files through backend APIs and do not select an external storage provider.
- Test evidence: No storage-provider integration test was found; current tests exercise local upload and validation flows only indirectly.
- Conflict or missing information: The term `object_key` could suggest object storage, but code writes to local disk.
- Safest interpretation: Treat storage as local filesystem storage with logical object keys.
- Representation to use in diagrams: Use `[IMPLEMENTED] Local filesystem storage`; do not draw S3, CDN, or external object storage.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `04-container-architecture.mmd`, `05-local-development-runtime.mmd`, `06-production-deployment.mmd`, future media, package, deployment, and marketplace diagrams.

## AMB-003

- ID: AMB-003
- Domain: Marketplace Catalog
- Exact question: Is the Marketplace catalog public and unauthenticated, or tenant-protected?
- Documentation claim: `docs/V3_PHASE_FIVE.md` describes a catalog experience, while gap docs separate future purchase/install behavior.
- Implementation evidence: `backend/src/routes/mod.rs` mounts `marketplace::router()` inside the tenant-protected route stack; `backend/src/routes/marketplace.rs` exposes catalog endpoints under `/api/marketplace/catalog`.
- Database evidence: Catalog reads approved listing/version data from Marketplace tables created in `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` through `backend/migrations/0018_v3_phase_four_review_moderation.sql`.
- Frontend evidence: `frontend/src/services/api.ts` calls catalog endpoints with authenticated requests and `X-Organization-Id`; `frontend/src/pages/MarketplacePage.tsx` renders catalog tabs in the admin UI.
- Test evidence: No test was found proving unauthenticated catalog access.
- Conflict or missing information: "Public catalog" can mean visible product catalog, not unauthenticated internet access.
- Safest interpretation: Treat the catalog as an authenticated, tenant-aware admin catalog.
- Representation to use in diagrams: Use `[IMPLEMENTED] tenant-aware catalog`; do not draw a public anonymous catalog route.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `04-container-architecture.mmd`, `08-route-boundaries.mmd`, `09-request-middleware-pipeline.mmd`, future marketplace catalog and tenant-routing diagrams.

## AMB-004

- ID: AMB-004
- Domain: Authorization
- Exact question: Are global roles and organization membership roles the same security model?
- Documentation claim: V1 docs describe global RBAC; V2 docs introduce organization membership roles.
- Implementation evidence: `backend/src/services/rbac.rs` contains checks for global roles and organization roles; route handlers call both `require_any` and `require_org_any` depending on the domain.
- Database evidence: `backend/migrations/0001_initial_schema.sql` creates `roles` and `user_roles`; `backend/migrations/0008_v2_phase_one_organizations.sql` creates `organization_members` with `organization_member_role`.
- Frontend evidence: `frontend/src/stores/useAppStore.ts` tracks active organization context; admin pages rely on backend authorization results.
- Test evidence: Security and route tests cover selected auth/RBAC paths, but no single full matrix test for global-role versus organization-role precedence was found.
- Conflict or missing information: Some labels such as `admin` exist in both conceptual layers and can be confused.
- Safest interpretation: Treat global RBAC and organization membership roles as separate authorization layers.
- Representation to use in diagrams: Show global role checks separately from tenant membership checks.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future auth, tenant, and organization diagrams.

## AMB-005

- ID: AMB-005
- Domain: Tenancy and Super Admin
- Exact question: Does a super admin bypass tenant middleware automatically?
- Documentation claim: Documentation identifies `super_admin` as a powerful global role but does not prove automatic tenant bypass.
- Implementation evidence: `backend/src/middleware/tenant.rs` requires an active organization membership for tenant routes; `backend/src/main.rs` and `backend/src/routes/auth.rs` attach default organization membership for seeded/admin users; `backend/src/services/rls.rs` has explicit bypass helpers for service-level operations.
- Database evidence: `backend/migrations/0008_v2_phase_one_organizations.sql` maps super admin users into organization owner membership.
- Frontend evidence: `frontend/src/services/api.ts` sends `X-Organization-Id`; `frontend/src/components/AppShell.tsx` exposes organization selection.
- Test evidence: No test was found showing a global super admin can access tenant APIs without organization context.
- Conflict or missing information: The role name could imply implicit bypass, but middleware evidence does not.
- Safest interpretation: Super admin still needs organization context for tenant routes; bypass is explicit in backend service helpers only.
- Representation to use in diagrams: Show super admin as a global role plus organization membership, not as a tenant middleware bypass.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future tenant and RLS diagrams.

## AMB-006

- ID: AMB-006
- Domain: Plugins
- Exact question: Is plugin support only configuration, or is executable plugin runtime implemented?
- Documentation claim: `docs/PHASE_SIX.md` describes built-in plugins and future extension limits.
- Implementation evidence: `backend/src/plugins/mod.rs` defines plugin hooks and built-in registry execution; `backend/src/plugins/seo.rs` implements a built-in plugin; `backend/src/routes/plugins.rs` manages plugin enablement; `backend/src/routes/content.rs` runs plugin hooks around entry saves and publishes.
- Database evidence: `backend/migrations/0006_phase_six_workflow_collaboration.sql` creates `cms_plugins` and seeds built-in plugin configuration.
- Frontend evidence: `frontend/src/pages/WorkflowPage.tsx` exposes plugin management UI.
- Test evidence: No external plugin sandbox/runtime test was found.
- Conflict or missing information: Built-in executable hooks exist, but external packaged plugin runtime is absent.
- Safest interpretation: Treat plugins as partial: built-in runtime is implemented; external plugin execution is not.
- Representation to use in diagrams: Use `[PARTIAL] built-in hooks only`; do not draw a plugin sandbox or external runtime.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future workflow and plugin diagrams.

## AMB-007

- ID: AMB-007
- Domain: Marketplace Installations
- Exact question: Does the Marketplace installation table mean products can currently be installed?
- Documentation claim: `docs/V3_PHASE_FIVE.md` and `docs/V3_MARKETPLACE_GAP_LIST.md` state installation runtime is deferred.
- Implementation evidence: `backend/src/routes/marketplace.rs` reads installation counts and updates active installations during emergency block, but no install/uninstall/update endpoint was found.
- Database evidence: `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` creates `marketplace_installations` with statuses and tenant RLS.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` shows install-related catalog messaging but does not perform installation; UI text indicates install is deferred.
- Test evidence: No installation endpoint or install-flow test was found.
- Conflict or missing information: Schema exists before runtime behavior.
- Safest interpretation: Treat installation as partial schema/supporting state only.
- Representation to use in diagrams: Use `[PARTIAL] installation table and moderation updates, no install API`.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future marketplace installation diagrams.

## AMB-008

- ID: AMB-008
- Domain: Marketplace Purchases
- Exact question: Do paid listing fields mean purchase flow is implemented?
- Documentation claim: Marketplace readiness and gap docs describe purchases as future work.
- Implementation evidence: `backend/src/routes/marketplace.rs` exposes pricing metadata but no checkout, purchase, entitlement, or payment route for Marketplace products.
- Database evidence: `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` stores `pricing_type` and `price_cents`; no purchase table was found.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` renders prices but does not start a product purchase flow.
- Test evidence: No Marketplace purchase test was found.
- Conflict or missing information: Paid listing metadata exists without purchase runtime.
- Safest interpretation: Treat Marketplace purchases as planned only.
- Representation to use in diagrams: Use `[PLANNED] Marketplace purchase`; do not draw purchase tables, checkout routes, or entitlements.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future purchase and billing diagrams.

## AMB-009

- ID: AMB-009
- Domain: Creator Payouts
- Exact question: Is creator payout implemented or only represented by creator status metadata?
- Documentation claim: `docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md` and `docs/V3_MARKETPLACE_GAP_LIST.md` identify payouts and Stripe Connect style flows as future work.
- Implementation evidence: No payout service, payout route, ledger, or provider integration was found in `backend/src/routes/marketplace.rs` or service modules.
- Database evidence: `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` stores `marketplace_creators.payout_status`, but no payout or ledger table was found.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` does not expose payout management or payout history.
- Test evidence: No payout test was found.
- Conflict or missing information: Payout status field exists without payout processing.
- Safest interpretation: Treat creator payout as planned only.
- Representation to use in diagrams: Use `[PLANNED] payout status only`; do not invent payout provider or payout entity.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future payout and Marketplace finance diagrams.

## AMB-010

- ID: AMB-010
- Domain: Email Deliveries
- Exact question: What email provider implementation exists?
- Documentation claim: V2 SaaS operations docs describe persisted email delivery and provider modes.
- Implementation evidence: `backend/src/services/email.rs` supports `log`, `disabled`, and `webhook` provider behavior; `backend/src/config.rs` reads email provider settings.
- Database evidence: `backend/migrations/0012_v2_phase_seven_saas_ops.sql` creates `email_deliveries` with provider, status, attempts, and payload fields.
- Frontend evidence: Organization invitation flows in `frontend/src/pages/OrganizationPage.tsx` call backend APIs that create email delivery records.
- Test evidence: No SMTP, SES, or third-party email provider test was found.
- Conflict or missing information: Email delivery is implemented, but as log/webhook bridge rather than a full SMTP/provider integration.
- Safest interpretation: Treat email deliveries as implemented with log/webhook provider modes.
- Representation to use in diagrams: Use `[IMPLEMENTED] log/webhook email delivery`; do not draw SMTP or hosted email provider unless configured as external webhook.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `04-container-architecture.mmd`, `06-production-deployment.mmd`, future SaaS operations and organization diagrams.

## AMB-011

- ID: AMB-011
- Domain: Webhooks and Background Processing
- Exact question: Are webhook retries and durable background processing implemented?
- Documentation claim: Phase docs describe webhook delivery and delivery logging, not a durable queue.
- Implementation evidence: `backend/src/services/webhooks.rs` dispatches with `tokio::spawn` and records delivery status; no queue module, worker process, retry scheduler, or durable job table was found.
- Database evidence: `backend/migrations/0005_phase_five_delivery_api.sql` creates `webhook_deliveries`, but no retry count, next-at schedule, or job queue table.
- Frontend evidence: `frontend/src/pages/SettingsPage.tsx` manages webhook subscriptions and test dispatches, not retry policy.
- Test evidence: No durable retry or worker test was found.
- Conflict or missing information: Async dispatch exists, but durable background processing does not.
- Safest interpretation: Treat webhooks as implemented with transient async dispatch and delivery logging only.
- Representation to use in diagrams: Use `[PARTIAL] async dispatch, no durable queue/retry worker`; do not invent a queue or worker.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `04-container-architecture.mmd`, `06-production-deployment.mmd`, future webhook and operations diagrams.

## AMB-012

- ID: AMB-012
- Domain: Redis
- Exact question: Is Redis optional everywhere, or only for selected features?
- Documentation claim: Delivery docs describe Redis-backed cache with PostgreSQL fallback.
- Implementation evidence: `backend/src/services/cache.rs` falls back to direct fetch on Redis errors; `backend/src/services/rate_limit.rs` returns service errors for Redis failures; `backend/src/routes/mod.rs` reports degraded readiness when Redis is unavailable.
- Database evidence: No Redis-specific database schema exists.
- Frontend evidence: No frontend Redis logic exists; frontend only receives API or readiness failures.
- Test evidence: Cache behavior is represented in service code; no full failure-mode integration test was found.
- Conflict or missing information: Redis fallback is true for delivery cache, not for all Redis-backed features.
- Safest interpretation: Treat Redis as cache with delivery fallback and rate-limit/readiness dependency.
- Representation to use in diagrams: Use `[PARTIAL] Redis fallback for cache only`; show rate limiting as dependent on Redis.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `04-container-architecture.mmd`, `05-local-development-runtime.mmd`, `06-production-deployment.mmd`, future delivery, security, and deployment diagrams.

## AMB-013

- ID: AMB-013
- Domain: Row Level Security
- Exact question: Are all tables covered by RLS?
- Documentation claim: V2 docs emphasize forced RLS for tenant-owned data; Marketplace docs distinguish platform/global state from tenant state.
- Implementation evidence: `backend/src/services/rls.rs` sets tenant context and has explicit bypass helpers; tenant routes use middleware before handlers.
- Database evidence: `backend/migrations/0009_v2_phase_three_rls.sql`, `0010_v2_phase_five_billing_quota.sql`, `0012_v2_phase_seven_saas_ops.sql`, `0014_v2_phase_nine_beta_release.sql`, and `0015_v3_phase_one_marketplace_foundation.sql` force RLS on tenant-owned tables. Global Marketplace creator/listing/version/submission/review tables are not all tenant-owned.
- Frontend evidence: `frontend/src/services/api.ts` sends active organization context to tenant APIs.
- Test evidence: RLS hardening checks exist in backend services/scripts, but no complete all-table proof was found in one test.
- Conflict or missing information: "RLS implemented" does not mean every table is tenant-scoped.
- Safest interpretation: Treat RLS as implemented for tenant-owned data, with global/platform tables handled separately.
- Representation to use in diagrams: Use `[IMPLEMENTED] tenant RLS` and mark global Marketplace tables as platform scoped.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future database, tenant, and Marketplace diagrams.

## AMB-014

- ID: AMB-014
- Domain: Marketplace Package Immutability
- Exact question: Is package artifact immutability enforced?
- Documentation claim: V3 validation docs require immutable uploaded artifacts after submission and validation states.
- Implementation evidence: `backend/src/services/marketplace_package.rs` validates package metadata and object keys; `backend/src/routes/marketplace.rs` inserts package versions and submissions around validation.
- Database evidence: `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` and `0016_v3_phase_two_creator_submission.sql` define version artifact fields and immutability trigger behavior for submitted/validated states.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` uploads package versions through backend APIs rather than editing stored artifact fields directly.
- Test evidence: No dedicated immutability trigger test was found in application tests.
- Conflict or missing information: Runtime upload code and database trigger both contribute; immutability should be represented at database boundary.
- Safest interpretation: Treat artifact immutability as implemented by database constraints/triggers.
- Representation to use in diagrams: Use `[IMPLEMENTED] DB-enforced artifact immutability`.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future package validation and database diagrams.

## AMB-015

- ID: AMB-015
- Domain: Marketplace Artifact Cleanup
- Exact question: Are stored package artifacts cleaned up after validation failure or later database persistence failure?
- Documentation claim: Current docs discuss validation reports and blocking unsafe packages; cleanup behavior is not clearly specified.
- Implementation evidence: `backend/src/routes/marketplace.rs` persists the package artifact before validation and database writes; no Marketplace artifact `remove_file` cleanup path was found.
- Database evidence: Package version rows can store failed/blocked validation state; database schema does not model orphaned artifact cleanup.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` shows validation status but no cleanup or retry cleanup control.
- Test evidence: No cleanup-after-failure test was found.
- Conflict or missing information: It is unclear whether retained failed artifacts are intentional audit evidence or an unimplemented cleanup policy.
- Safest interpretation: Do not represent automatic artifact cleanup as implemented.
- Representation to use in diagrams: Use `[DECISION REQUIRED] cleanup policy`; show failed validation as stored report/artifact retention unless future evidence changes.
- Confidence: MEDIUM
- Status: DECISION_REQUIRED
- Affected diagram files: `00-implementation-status-map.mmd`, future validation and storage lifecycle diagrams.

## AMB-016

- ID: AMB-016
- Domain: Marketplace Review and Moderation
- Exact question: Is there an appeal or restoration flow after rejection, suspension, unpublish, or emergency block?
- Documentation claim: V3 review docs describe review decisions and moderation actions; no appeal or restore workflow was found in current docs.
- Implementation evidence: `backend/src/services/marketplace_review.rs` validates approve, reject, request changes, suspend, unpublish, and emergency block actions; no appeal/restore route was found in `backend/src/routes/marketplace.rs`.
- Database evidence: `backend/migrations/0018_v3_phase_four_review_moderation.sql` stores review events and moderation flags, but no appeal table or restoration request table.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` exposes review and moderation actions but no appeal/restoration UI.
- Test evidence: No appeal or restoration test was found.
- Conflict or missing information: Emergency block can change installations, but no reverse workflow is represented.
- Safest interpretation: Treat appeal/restoration as not implemented and product-design dependent.
- Representation to use in diagrams: Use `[DECISION REQUIRED] no appeal/restoration flow`; do not draw a restore path.
- Confidence: HIGH
- Status: DECISION_REQUIRED
- Affected diagram files: `00-implementation-status-map.mmd`, future review and moderation diagrams.

## AMB-017

- ID: AMB-017
- Domain: Marketplace Reviews
- Exact question: Are customer ratings implemented, or only internal review events?
- Documentation claim: Catalog docs mention ratings/reviews in catalog responses, while review/moderation docs focus on internal review events.
- Implementation evidence: `backend/src/routes/marketplace.rs` returns catalog rating fields and an empty review list, while internal review events are implemented through review services and routes.
- Database evidence: `backend/migrations/0018_v3_phase_four_review_moderation.sql` creates `marketplace_review_events`; no customer review/rating table was found.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` displays rating fields and review placeholders, but no customer review submission UI.
- Test evidence: No customer review write test was found.
- Conflict or missing information: Internal Marketplace review and customer product reviews are different concepts.
- Safest interpretation: Treat internal review events as implemented and customer ratings as placeholder/read-only fields.
- Representation to use in diagrams: Use `[PARTIAL] internal review implemented, customer ratings placeholder`.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future catalog and review diagrams.

## AMB-018

- ID: AMB-018
- Domain: Stripe Billing
- Exact question: Is Stripe event ordering handled?
- Documentation claim: V2 hardening docs describe Stripe webhook hardening and event ordering.
- Implementation evidence: `backend/src/services/stripe_billing.rs` stores provider event creation time and applies subscription updates only when the incoming event is current enough.
- Database evidence: `backend/migrations/0013_v2_phase_eight_hardening.sql` adds `provider_event_created_at` fields and indexes for billing/subscription event ordering.
- Frontend evidence: `frontend/src/pages/BillingPage.tsx` consumes billing state after backend webhook processing.
- Test evidence: `backend/src/services/stripe_billing.rs` includes unit tests for event timestamp parsing and rejecting older provider events.
- Conflict or missing information: Ordering is implemented for organization subscription billing, not Marketplace purchases.
- Safest interpretation: Treat Stripe event ordering as implemented for V2 billing only.
- Representation to use in diagrams: Use `[IMPLEMENTED] subscription event ordering`; keep Marketplace purchase separate as planned.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future billing and Stripe diagrams.

## AMB-019

- ID: AMB-019
- Domain: Stripe Billing
- Exact question: Is Stripe webhook idempotency implemented?
- Documentation claim: V2 billing/hardening docs describe webhook event persistence.
- Implementation evidence: `backend/src/services/stripe_billing.rs` inserts `billing_events` with `ON CONFLICT (provider, provider_event_id) DO NOTHING` and returns an already-processed result for duplicate events.
- Database evidence: `backend/migrations/0011_v2_phase_six_stripe_billing.sql` enforces unique `(provider, provider_event_id)` on `billing_events`.
- Frontend evidence: Frontend billing UI does not handle idempotency directly.
- Test evidence: No full HTTP duplicate-webhook integration test was found, but service code contains duplicate-event handling.
- Conflict or missing information: Idempotency applies to organization billing webhooks, not Marketplace purchases.
- Safest interpretation: Treat Stripe idempotency as implemented for V2 subscription billing only.
- Representation to use in diagrams: Use `[IMPLEMENTED] subscription webhook idempotency`; do not apply to Marketplace purchases.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, future billing and Stripe diagrams.

## AMB-020

- ID: AMB-020
- Domain: Live Preview
- Exact question: Does live preview support horizontal scaling across multiple backend instances?
- Documentation claim: Page builder docs describe live preview behavior, not multi-node coordination.
- Implementation evidence: `backend/src/state.rs` stores preview channels in an in-memory `Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>`; `backend/src/routes/pages.rs` broadcasts page JSON through that in-process channel.
- Database evidence: No preview subscription or cross-instance event table exists.
- Frontend evidence: `frontend/src/pages/PagesPage.tsx` connects directly to the preview WebSocket route for the current backend.
- Test evidence: No multi-instance live preview test was found.
- Conflict or missing information: Live preview is implemented for one backend process, but no external bus exists.
- Safest interpretation: Treat live preview as implemented for single-process runtime only.
- Representation to use in diagrams: Use `[PARTIAL] in-process WebSocket broadcast`; do not draw Redis pub/sub, queue, or external broker.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `06-production-deployment.mmd`, future page builder and deployment diagrams.

## AMB-021

- ID: AMB-021
- Domain: Static Uploads
- Exact question: Are `/uploads` files protected by authentication or tenant authorization?
- Documentation claim: Media docs describe media library behavior and upload URLs, but not access control on static file serving.
- Implementation evidence: `backend/src/routes/mod.rs` mounts `ServeDir` at `/uploads` on the top-level router; `backend/src/routes/media.rs` stores media URLs under `/uploads/{organization_id}/...`.
- Database evidence: `media_assets` rows are tenant-owned, but static file serving does not consult database rows.
- Frontend evidence: `frontend/src/pages/MediaPage.tsx` displays stored media URLs directly.
- Test evidence: No auth-protected static upload test was found.
- Conflict or missing information: Metadata is tenant-protected, but file bytes are served statically.
- Safest interpretation: Treat `/uploads` as public static file serving by path.
- Representation to use in diagrams: Use `[PARTIAL] tenant metadata, public static file URLs`; do not draw auth/RLS checks for static file bytes.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `04-container-architecture.mmd`, `05-local-development-runtime.mmd`, `06-production-deployment.mmd`, `08-route-boundaries.mmd`, `09-request-middleware-pipeline.mmd`, future media, security, and deployment diagrams.

## AMB-022

- ID: AMB-022
- Domain: API Documentation
- Exact question: Is the current API documentation complete for V2 and V3 runtime routes?
- Documentation claim: `docs/API.md` documents earlier auth/content/media/pages/delivery/comment/plugin/security/webhook APIs.
- Implementation evidence: `backend/src/routes/mod.rs` mounts organizations, billing, beta, and marketplace routes; Marketplace paths are not fully represented in central OpenAPI registration.
- Database evidence: Migrations `0008` through `0018` add V2/V3 tables that are not covered by `docs/API.md`.
- Frontend evidence: `frontend/src/services/api.ts` calls V2/V3 endpoints that are absent from the older API document.
- Test evidence: No documentation completeness test was found.
- Conflict or missing information: Runtime APIs have outgrown the central API document.
- Safest interpretation: Treat `docs/API.md` and OpenAPI as incomplete for V2/V3.
- Representation to use in diagrams: Use backend route code as API boundary evidence and mark API docs as `[CONFLICT]` where stale.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `00-implementation-status-map.mmd`, `08-route-boundaries.mmd`, `09-request-middleware-pipeline.mmd`, future API and route diagrams.

## AMB-023

- ID: AMB-023
- Domain: Marketplace Actors and RBAC
- Exact question: Are Marketplace reviewer and Marketplace moderator implemented as dedicated roles?
- Documentation claim: `docs/V3_PHASE_FOUR.md` describes internal Marketplace reviewers and moderation actions.
- Implementation evidence: `backend/src/routes/marketplace.rs` gates review queue, review decisions, review events, review reports, and moderation with `rbac::require_any(&claims, &[rbac::ADMIN])`.
- Database evidence: `backend/migrations/0018_v3_phase_four_review_moderation.sql` stores `marketplace_review_events.actor_id`, but no reviewer or moderator role table/enum was found.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` renders reviewer and moderation controls in the Marketplace page, but frontend routing does not create backend security roles.
- Test evidence: No test was found for dedicated Marketplace reviewer or moderator roles.
- Conflict or missing information: Product wording names reviewer and moderator actors, while backend authorization currently uses global administrator checks.
- Safest interpretation: Treat Marketplace reviewer and moderator as operational actor labels backed by the global `admin` role, not separate RBAC roles.
- Representation to use in diagrams: Use `Marketplace reviewer [IMPLEMENTED] AMB-023` and `Marketplace moderator [IMPLEMENTED] AMB-023` with notes that both are implemented through global admin checks.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `02-system-context.mmd`, future RBAC and Marketplace review diagrams.

## AMB-024

- ID: AMB-024
- Domain: Beta and Support Operations Actors
- Exact question: Is beta/support operation implemented as a dedicated support role?
- Documentation claim: `docs/V2_PHASE_NINE.md` and `docs/V2_PHASE_TEN.md` describe beta, support, and GA operational workflows.
- Implementation evidence: `backend/src/routes/beta.rs` allows organization admin/editor roles to update tenant beta feedback and blockers, while product dashboard and participant updates require global `admin` through `rbac::require_any(&claims, &[rbac::ADMIN])`.
- Database evidence: `backend/migrations/0014_v2_phase_nine_beta_release.sql` creates `beta_participants`, `beta_feedback`, and `beta_ga_blockers`, but no support role or operator table was found.
- Frontend evidence: `frontend/src/pages/BetaPage.tsx` exposes beta feedback, blockers, participants, and product dashboard through the React admin app.
- Test evidence: No test was found for a dedicated beta operator or support operator role.
- Conflict or missing information: Operational docs mention support activities, but implementation maps those activities to existing global and organization roles.
- Safest interpretation: Treat beta/support operator as an operational actor implemented by global admin for product-level operations and organization admin/editor for tenant triage.
- Representation to use in diagrams: Use `Beta or support operator [IMPLEMENTED] AMB-024` with role mapping in the label; do not draw a separate support role.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `02-system-context.mmd`, future operations and RBAC diagrams.

## AMB-025

- ID: AMB-025
- Domain: RBAC Permission Model
- Exact question: Are `roles.permissions` arrays the runtime authorization engine?
- Documentation claim: Foundation migrations seed role permission arrays for global roles.
- Implementation evidence: `backend/src/services/rbac.rs` authorizes through hard-coded role constants and helper functions; route handlers call role helper functions rather than querying `roles.permissions`.
- Database evidence: `backend/migrations/0001_initial_schema.sql` defines `roles.permissions`; `backend/migrations/0002_seed_foundation_data.sql` and `backend/migrations/0003_phase_one_core.sql` seed permission arrays.
- Frontend evidence: `frontend/src/stores/useAppStore.ts` and page code use role strings from the current user or membership, not permission arrays.
- Test evidence: `backend/src/services/rbac.rs` tests role-helper behavior; no test was found for dynamic permission-array evaluation.
- Conflict or missing information: Permission arrays exist in the database, but runtime authorization is role-helper based.
- Safest interpretation: Diagram authorization boundaries as hard-coded role helper checks; treat `roles.permissions` as stored metadata, not the authoritative runtime permission engine.
- Representation to use in diagrams: Use `roles.permissions array [PARTIAL] AMB-025` and connect runtime permissions to RBAC helper/service nodes.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `03-identity-and-authorization-boundaries.mmd`, future RBAC diagrams.

## AMB-026

- ID: AMB-026
- Domain: Frontend Authorization Controls
- Exact question: Are `RequireAuth`, navigation visibility, and disabled buttons authoritative security boundaries?
- Documentation claim: Frontend routes and pages include auth guards and role-based UI checks.
- Implementation evidence: `frontend/src/components/RequireAuth.tsx` only checks a stored access token; `frontend/src/components/AppShell.tsx` renders all navigation items for authenticated users; page-level booleans hide or disable selected actions.
- Database evidence: No database enforcement is created by frontend guards.
- Frontend evidence: `frontend/src/pages/BillingPage.tsx`, `OrganizationPage.tsx`, `BetaPage.tsx`, and `MarketplacePage.tsx` compute UI booleans from stored user and organization membership state.
- Test evidence: No frontend guard test was found that proves backend authorization; backend route handlers and middleware remain the authoritative checks.
- Conflict or missing information: UI role checks can improve ergonomics, but they do not protect backend APIs.
- Safest interpretation: Treat frontend guards and navigation as UX-only controls.
- Representation to use in diagrams: Use `[FRONTEND ONLY] AMB-026`; connect frontend-only nodes with dashed non-authoritative edges to backend permission boundaries.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `03-identity-and-authorization-boundaries.mmd`, future frontend and security diagrams.

## AMB-027

- ID: AMB-027
- Domain: Marketplace Installation Authorization
- Exact question: Are install-time permission approval and installation runtime authorization implemented?
- Documentation claim: V3 gap documentation says Marketplace install runtime, permission approval enforcement, and runtime permission enforcement are future work.
- Implementation evidence: `backend/src/routes/marketplace.rs` exposes catalog, creator, listing, version, review, and moderation routes; no install/uninstall/update route was found. Moderation can update existing installation rows during emergency block.
- Database evidence: `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` creates `marketplace_installations` with `permissions_json`, `permission_approved_by`, `permission_approved_at`, status values, and forced RLS policies.
- Frontend evidence: `frontend/src/pages/MarketplacePage.tsx` displays catalog compatibility and an install-deferred message; no install action is wired to an API call.
- Test evidence: No install-time authorization, permission approval, or runtime permission enforcement test was found.
- Conflict or missing information: Installation authorization fields exist in schema, but runtime authorization behavior is not implemented.
- Safest interpretation: Treat Marketplace installation authorization as partial schema/RLS only until install APIs and permission enforcement exist.
- Representation to use in diagrams: Use `[PARTIAL] AMB-027` for installation records and `[NOT FOUND]` for install-time authorization route/enforcement.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `03-identity-and-authorization-boundaries.mmd`, future Marketplace install and permission diagrams.

## Step 2 Report

- Ambiguities added: 22
- Ambiguities resolved: 20
- Decision-required items: AMB-015, AMB-016
- Diagram conventions established: See `docs/diagrams/DIAGRAM_CONVENTIONS.md`
- Files created or modified in this step: `docs/diagrams/AMBIGUITIES.md`, `docs/diagrams/DIAGRAM_CONVENTIONS.md`, `docs/diagrams/00-implementation-status-map.mmd`, `docs/diagrams/ARCHITECTURE_AUDIT.md`
- Production behavior changed: No

## Step 3 Update

- Ambiguities added in step 3: AMB-023, AMB-024
- Total ambiguity entries after step 3: 24
- Actor or boundary conflicts newly recorded: Marketplace reviewer/moderator role mapping; beta/support operator role mapping
- Production behavior changed: No

## Step 4 Update

- Ambiguities added in step 4: AMB-025, AMB-026, AMB-027
- Total ambiguity entries after step 4: 27
- Authorization gaps newly recorded: `roles.permissions` is metadata rather than runtime permission engine; frontend controls are UX-only; Marketplace install-time permission enforcement is not implemented.
- Production behavior changed: No

## Step 5 Container Architecture Update

- Updated ambiguity records used by `04-container-architecture.mmd`: AMB-002, AMB-003, AMB-010, AMB-011, AMB-012, and AMB-021.
- No new ambiguity IDs were required for this step; existing records cover Marketplace catalog accessibility, object storage, email delivery, webhook execution, Redis fallback, and static upload security.
- Representation decision: show only implemented containers, same-process backend modules, local filesystem storage, and verified external integrations; keep S3, CDN, separate API gateway, durable queue or worker, and durable preview broker as documented-only exclusions.
- Production behavior changed: No.
## AMB-028

- ID: AMB-028
- Domain: Production Network Edge
- Exact question: Is TLS termination or reverse proxying implemented by the repository deployment files?
- Documentation claim: Operational docs describe deployment and health checks, but do not define a concrete TLS or reverse-proxy component.
- Implementation evidence: `docker-compose.prod.yml` exposes backend port `8080:8080` and frontend port `5173:80`; `frontend/nginx.conf` serves static SPA files and does not proxy API traffic or configure TLS.
- Database evidence: Not applicable.
- Frontend evidence: `frontend/src/services/api.ts` uses `VITE_API_URL` as the browser API base URL; API routing is not provided by the frontend Nginx configuration.
- Test evidence: No deployment or TLS integration test was found.
- Conflict or missing information: Public HTTPS entrypoint, certificate management, and reverse proxy routing are not represented in the repository.
- Safest interpretation: Treat TLS termination and public reverse proxying as a deployment decision outside the current repo.
- Representation to use in diagrams: Use `[DECISION REQUIRED]` for TLS/reverse proxy; do not invent Nginx reverse proxy, Traefik, cloud load balancer, or API gateway.
- Confidence: HIGH
- Status: DECISION_REQUIRED
- Affected diagram files: `06-production-deployment.mmd`, future deployment and operations diagrams.

## AMB-029

- ID: AMB-029
- Domain: Production Health Checks
- Exact question: Are backend and frontend health checks wired into production Compose?
- Documentation claim: The runbook says to run `/health` and `/ready` checks during deployment.
- Implementation evidence: `backend/src/routes/mod.rs` exposes `/health` and `/ready`, but `docker-compose.prod.yml` defines health checks only for PostgreSQL and Redis. The frontend service has no healthcheck in Compose.
- Database evidence: Not applicable.
- Frontend evidence: `frontend/nginx.conf` serves the SPA but does not expose a dedicated health endpoint.
- Test evidence: CI runs backend and frontend tests, but no Compose healthcheck test was found.
- Conflict or missing information: Application readiness endpoints exist, but container orchestration does not consume them.
- Safest interpretation: Treat app checks as implemented and Compose-level backend/frontend health checks as not configured.
- Representation to use in diagrams: Use `[PARTIAL]` for health checks and show missing backend/frontend Compose healthcheck separately.
- Confidence: HIGH
- Status: RESOLVED
- Affected diagram files: `06-production-deployment.mmd`, future operations diagrams.

## AMB-030

- ID: AMB-030
- Domain: Migration Execution
- Exact question: Is production migration execution coordinated separately from backend startup?
- Documentation claim: The runbook lists database backup, migration deployment, then backend deployment as separate operational steps.
- Implementation evidence: `backend/src/main.rs` calls `db::run_migrations(&db)` during backend startup before binding the server; no separate migration job, script, or Compose one-shot migration service was found.
- Database evidence: SQLx migration history is tracked by `_sqlx_migrations`, but this does not define deployment orchestration.
- Frontend evidence: Not applicable.
- Test evidence: Backend CI runs tests against PostgreSQL, but no multi-replica migration coordination test was found.
- Conflict or missing information: Startup migrations are implemented for a single backend startup path, while production rollout sequencing remains operational.
- Safest interpretation: Treat startup migration execution as implemented, and multi-replica migration coordination as a deployment decision.
- Representation to use in diagrams: Use `[IMPLEMENTED]` for startup migrations and `[DECISION REQUIRED]` for separate migration job or multi-replica rollout coordination.
- Confidence: HIGH
- Status: DECISION_REQUIRED
- Affected diagram files: `05-local-development-runtime.mmd`, `06-production-deployment.mmd`, future deployment diagrams.

## AMB-031

- ID: AMB-031
- Domain: Backup and Restore
- Exact question: Is backup automation implemented for database, Redis, uploads, and Marketplace artifacts?
- Documentation claim: `docs/V2_OPERATIONS_RUNBOOK.md` and `docs/V2_MIGRATION_GUIDE.md` require database and uploaded media backups before release or migration.
- Implementation evidence: `docker-compose.prod.yml` defines named volumes for PostgreSQL, Redis, and uploads, but no backup sidecar, scheduled job, CI workflow, or artifact backup command was found.
- Database evidence: Not applicable beyond PostgreSQL data stored in `postgres_data`.
- Frontend evidence: Not applicable.
- Test evidence: No backup or restore test was found.
- Conflict or missing information: Backup is operationally required by docs, but not automated in repository deployment files.
- Safest interpretation: Treat backup and restore as manual/operational responsibilities until automation is added.
- Representation to use in diagrams: Use `[DECISION REQUIRED]` for database, Redis, upload, and Marketplace artifact backup automation.
- Confidence: HIGH
- Status: DECISION_REQUIRED
- Affected diagram files: `06-production-deployment.mmd`, future operations and storage diagrams.
## Step 6 Deployment Runtime Update

- Ambiguities added in step 6: AMB-028, AMB-029, AMB-030, AMB-031.
- Existing ambiguity records linked to deployment diagrams: AMB-002, AMB-010, AMB-011, AMB-012, AMB-020, and AMB-021.
- Deployment decisions newly recorded: TLS/reverse proxy, backend/frontend Compose health checks, migration coordination for multi-replica rollout, and backup automation for database/cache/uploads/artifacts.
- Production behavior changed: No.
## Step 8 Route Boundary Update

- Updated ambiguity records used by `08-route-boundaries.mmd` and `09-request-middleware-pipeline.mmd`: AMB-003, AMB-021, and AMB-022.
- Boundary conflicts represented: Marketplace catalog is tenant-protected despite public-catalog wording; `/uploads` file bytes are public static serving by path; API documentation is incomplete for V2/V3 route coverage.
- Effective middleware representation: matched route service first, then global Tower route layers in effective order, followed by auth-only or tenant route-stack middleware where applicable.
- Production behavior changed: No.