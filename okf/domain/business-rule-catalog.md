---
okf_document_id: "domain-business-rule-catalog"
title: "Business Rule Catalog"
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
  - "backend/migrations"
related_documents:
  - "domain-catalog.md"
  - "invariants.md"
  - "cross-module-workflows.md"
related_diagrams:
  - "diagrams/domain-map.mmd"
  - "diagrams/cross-module-orchestration.mmd"
---

# Business Rule Catalog

This catalog contains 50 evidence-backed rules. A status describes the strongest observed enforcement characteristic; it does not claim that every entry point has equivalent enforcement.

## Identity and Access

### `BR-IDENTITY-001` — Email identity is case-insensitively unique

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; A user email identifies at most one user; `DATABASE_ENFORCED_RULE` through `CITEXT UNIQUE`.
- Evidence: `backend/migrations/0001_initial_schema.sql`; `backend/src/routes/auth.rs`.
- Relations: `users`; authentication route, password/JWT services.
- Surface: registration/login endpoints; `AuthPage`; authentication workflow; no live duplicate-registration test located.
- Exceptions/confidence/owner: database-enforced for persisted users; High; owner confirmation not required.

### `BR-IDENTITY-002` — Inactive users cannot establish or refresh a session

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Login and refresh require `users.is_active`; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/auth.rs`.
- Relations: users and refresh tokens; authentication module.
- Surface: `/api/auth/login`, `/api/auth/refresh`; `AuthPage` and API refresh helper; Phase 7 security testing map.
- Exceptions/confidence/owner: existing access tokens are not continuously rechecked against user status; Medium; `NEEDS_OWNER_CONFIRMATION` for intended revocation timing.

### `BR-IDENTITY-003` — Refresh tokens are opaque, hashed, expiring, and individually revocable

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Only a hash is persisted and refresh/logout updates individual token records; `APPLICATION_ENFORCED_RULE` plus database uniqueness.
- Evidence: `backend/src/routes/auth.rs`; `backend/src/services/jwt.rs`; `backend/migrations/0001_initial_schema.sql`.
- Relations: `refresh_tokens`; authentication/JWT modules.
- Surface: refresh/logout endpoints; frontend cookie-based refresh; authentication workflow; token family/reuse tests not found.
- Exceptions/confidence/owner: no refresh family, bulk revocation, or reuse detection; High for observed behavior; owner confirmation required for broader session policy.

## Organizations and Membership

### `BR-TENANT-001` — Organization creation establishes an owner and default subscription atomically

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Creating an organization inserts the organization, active owner membership, and default subscription in one transaction; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/organizations.rs`; `backend/src/services/quota.rs`.
- Relations: organizations, members, subscriptions; organization/quota modules.
- Surface: `POST /api/organizations`; `OrganizationPage`; [organization provisioning](workflows/organization-provisioning.md); no database integration test located.
- Exceptions/confidence/owner: audit is recorded after commit; High; owner confirmation not required for observed sequence.

### `BR-TENANT-002` — Tenant routes require active organization and active membership

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Tenant middleware rejects missing/inactive tenant context; `APPLICATION_ENFORCED_RULE` backed by forced RLS for tenant data.
- Evidence: `backend/src/middleware/tenant.rs`; `backend/src/services/rls.rs`; `backend/migrations/0009_v2_phase_three_rls.sql`.
- Relations: organizations and members; tenant/RLS modules.
- Surface: protected tenant route tree; workspace selector; all tenant workflows; live cross-tenant test not found.
- Exceptions/confidence/owner: bypass callers and preview query context are separate paths; High; applied-database confirmation remains `TENANT_BEHAVIOR_UNCLEAR`.

### `BR-TENANT-003` — Pending invitations are unique per organization and email

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Only one pending invitation may exist for the same organization/email; `DATABASE_ENFORCED_RULE` with application upsert.
- Evidence: `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/routes/organizations.rs`.
- Relations: organization invitations; organization route.
- Surface: invitation create endpoint; `OrganizationPage`; [membership workflow](workflows/tenant-invitation-and-membership.md); no concurrent test located.
- Exceptions/confidence/owner: accepted/revoked/expired history can coexist; High; owner confirmation not required.

### `BR-TENANT-004` — Invitation acceptance is bound to token, recipient email, pending state, and expiry

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; The authenticated user's email must match a valid unexpired pending invitation; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/organizations.rs`.
- Relations: users, invitations, memberships; organization/quota modules.
- Surface: `POST /api/organization-invitations/accept`; workspace redirect; [membership workflow](workflows/tenant-invitation-and-membership.md); negative integration tests not found.
- Exceptions/confidence/owner: capacity is checked outside the acceptance transaction; High for predicates, Medium for concurrent quota guarantee; owner confirmation not required.

### `BR-TENANT-005` — The last active owner cannot be removed, downgraded, or leave

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Owner-changing operations call `ensure_not_last_owner`; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/organizations.rs`.
- Relations: organization members; organization route.
- Surface: member update/remove and leave endpoints; `OrganizationPage`; ownership/membership workflows; concurrency test not found.
- Exceptions/confidence/owner: count check and mutation are not locked in one verified transaction, so concurrent operations may race; Medium; `OWNERSHIP_RULE_UNCLEAR` for concurrency.

### `BR-TENANT-006` — Only an owner can assign owner role or transfer ownership

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Owner role assignment and transfer require current organization role `owner`; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/organizations.rs`; `backend/src/services/rbac.rs`.
- Relations: organizations and members; organization/RBAC modules.
- Surface: member role update and ownership transfer endpoints; `OrganizationPage`; [ownership transfer](workflows/organization-ownership-transfer.md); RBAC helper tests cover the matrix, not full workflow.
- Exceptions/confidence/owner: global `super_admin` does not bypass missing tenant membership; High; owner confirmation not required.

### `BR-TENANT-007` — Ownership transfer changes both membership roles and `organizations.owner_id` atomically

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Current owner becomes admin, target active member becomes owner, and root owner reference changes in one transaction; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/organizations.rs`.
- Relations: organizations and members; organization module.
- Surface: ownership transfer endpoint; organization administration UI; [ownership transfer](workflows/organization-ownership-transfer.md); no transaction integration test located.
- Exceptions/confidence/owner: audit occurs after commit; High; owner confirmation not required.

## Content and Editorial Workflow

### `BR-CONTENT-001` — Content type names and slugs must be valid and fields parse as a supported schema

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Name is non-empty, slug uses lowercase slug syntax, fields is an object with supported field definitions; `APPLICATION_ENFORCED_RULE` plus database checks.
- Evidence: `backend/src/routes/content.rs`; `backend/src/services/entry_validation.rs`; `backend/migrations/0001_initial_schema.sql`.
- Relations: content types; content/entry-validation modules.
- Surface: content type create/update endpoints; `ContentTypesPage`; [content save](workflows/content-entry-save.md); no content-route tests found.
- Exceptions/confidence/owner: frontend exposes fewer field types than backend supports; High; `VALIDATION_RULE_UNCLEAR` for UI parity.

### `BR-CONTENT-002` — Content type slugs are unique within an organization

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Tenant-scoped uniqueness is final persistence authority; `DATABASE_ENFORCED_RULE`.
- Evidence: `backend/migrations/0008_v2_phase_one_organizations.sql`.
- Relations: content types and organizations.
- Surface: content type create/update; frontend form; content workflow; duplicate error-shape test not found.
- Exceptions/confidence/owner: none in persisted rows; High; owner confirmation not required.

### `BR-CONTENT-003` — Entry data is transformed, sanitized, then schema-validated before persistence

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Enabled built-in plugins run first, rich text is sanitized, then required/type/range/slug rules are checked; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/content.rs`; `backend/src/plugins/mod.rs`; `backend/src/services/security.rs`; `backend/src/services/entry_validation.rs`.
- Relations: content entries/types and CMS plugins.
- Surface: entry create/update; `DynamicForm`; [content entry save](workflows/content-entry-save.md); sanitizer/plugin tests exist, route-level ordering test does not.
- Exceptions/confidence/owner: relation/media references are shape-checked but target existence is not verified by this validator; High; `VALIDATION_RULE_UNCLEAR` for referential semantics.

### `BR-CONTENT-004` — Content mutations increment the numeric entry version

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Entry update and every accepted workflow transition increment `version`; `APPLICATION_ENFORCED_RULE` with positive database check.
- Evidence: `backend/src/routes/content.rs`; `backend/migrations/0001_initial_schema.sql`.
- Relations: content entries; content/workflow modules.
- Surface: entry update and workflow endpoints; Entries/Workflow pages; content-save/publication workflows; transition unit tests do not assert version increments.
- Exceptions/confidence/owner: there is no immutable entry snapshot or optimistic concurrency check; High; `REVISION_BEHAVIOR_UNCLEAR` for historical meaning.

### `BR-CONTENT-005` — Content and page lifecycle transitions use the shared workflow policy

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Allowed transitions are draft-to-review, review-to-published/draft, published-to-archived/draft, archived-to-draft, plus reviewer draft-to-published bypass; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/workflow.rs`; content/pages routes.
- Relations: entries and pages; workflow/content/pages modules.
- Surface: submit, publish, reject, archive, restore, unpublish endpoints; Entries/Pages/Workflow UI; workflow unit tests cover three paths.
- Exceptions/confidence/owner: direct SQL can set any enum value; High for handler paths; owner confirmation not required.

### `BR-CONTENT-006` — Content type deletion is confirmed and cascades entries

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; API requires `confirm=true`; database FK deletes entries for that type; `PARTIALLY_ENFORCED_RULE`.
- Evidence: `backend/src/routes/content.rs`; `backend/migrations/0001_initial_schema.sql`.
- Relations: content types and entries.
- Surface: content type delete; frontend confirmation and API helper; deletion behavior; no cascade test found.
- Exceptions/confidence/owner: confirmation is API-only and entry audit/cache side effects are not replayed for cascaded rows; High; `DELETION_BEHAVIOR_UNCLEAR` for external effects.

## Pages and Page Builder

### `BR-PAGE-001` — Page title, slug, and page document must validate before save

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Title is non-empty, slug is valid, and `page_json` has a valid root/layout; `APPLICATION_ENFORCED_RULE` plus database shape/slug checks.
- Evidence: `backend/src/routes/pages.rs`; migrations 0001/0008.
- Relations: pages and component registry.
- Surface: page create/update and version restore; `PagesPage`; page save/restore workflows; frontend test covers shell, not invalid documents.
- Exceptions/confidence/owner: property values are not validated against each component `props_schema`; High; `VALIDATION_RULE_UNCLEAR` for prop semantics.

### `BR-PAGE-002` — Page layout uses one root, registered component types, bounded depth, and bounded node count

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Root type is `root`; descendants cannot be root, must be registered, depth is at most 12, and node count at most 500; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/pages.rs`.
- Relations: pages and component registry; pages module.
- Surface: page create/update/restore and template import validation; `PagesPage`; Page Builder workflows; dedicated validator tests not found.
- Exceptions/confidence/owner: duplicate node IDs and allowed child-type relationships are not checked; High; owner confirmation required for intended tree semantics.

### `BR-PAGE-003` — Page create, update, and version restore append a snapshot atomically

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Page write and next numbered snapshot commit together; `APPLICATION_ENFORCED_RULE` backed by unique `(page_id, version)`.
- Evidence: `backend/src/routes/pages.rs`; `backend/migrations/0001_initial_schema.sql`.
- Relations: pages and page versions.
- Surface: create/update/restore endpoints; `PagesPage`; page save/restore workflows; no database integration test found.
- Exceptions/confidence/owner: concurrent `MAX(version)+1` writers can contend and fail uniqueness; High for atomicity, Medium for concurrency.

### `BR-PAGE-004` — Restoring a page version creates a new snapshot and resets publication state to draft

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Selected snapshot replaces `page_json`, clears `published_at`, sets `draft`, then appends a new snapshot; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/pages.rs`.
- Relations: pages and page versions.
- Surface: version restore endpoint; `PagesPage`; [page version restoration](workflows/page-version-restoration.md); no route integration test found.
- Exceptions/confidence/owner: no restore of title/slug because snapshots contain only `page_json`; High; owner confirmation not required.

### `BR-PAGE-005` — Tenant code cannot mutate system component records

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Tenant component writes target `is_system = FALSE`, with RLS component-write policy; `DATABASE_ENFORCED_RULE` and application filtering.
- Evidence: `backend/src/routes/pages.rs`; `backend/migrations/0009_v2_phase_three_rls.sql`.
- Relations: component registry and organizations.
- Surface: component registry endpoints; Page Builder palette; page workflow; RLS live test not found.
- Exceptions/confidence/owner: global seed/sync paths can manage system records; High; owner confirmation not required.

### `BR-PAGE-006` — Existing dirty pages autosave after a frontend delay, while new pages require manual save

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; `PagesPage` schedules save for an existing dirty page and has no ID for a new draft; `FRONTEND_ONLY_RULE`.
- Evidence: `frontend/src/pages/PagesPage.tsx`.
- Relations: local page draft and pages API.
- Surface: Page Builder; [page save](workflows/page-builder-save-and-version.md); autosave timing/error tests not found.
- Exceptions/confidence/owner: backend has no autosave concept or concurrency token; High for UI behavior; owner confirmation required for product autosave guarantees.

### `BR-PAGE-007` — Page writes and state transitions broadcast current page JSON to in-process preview subscribers

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Successful page changes publish serialized JSON through a Tokio broadcast channel when one exists; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/pages.rs`; `backend/src/state.rs`.
- Relations: pages and in-memory preview channels.
- Surface: page write/workflow endpoints and preview WebSocket; copy-preview UI; no integration test found.
- Exceptions/confidence/owner: delivery is non-durable, process-local, and ignores send failure; High; owner confirmation not required for observed limitation.

## Media Library

### `BR-MEDIA-001` — Uploaded files are limited by configured size, plan capacity, and detected content type

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Upload rejects oversized/quota-exceeding files and accepts only JPEG, PNG, WebP, PDF, or UTF-8 text detected from bytes; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/media.rs`; `backend/src/services/quota.rs`.
- Relations: media, usage, plan; media/quota modules.
- Surface: media upload endpoint; `MediaPage`; media workflow; dedicated route tests not found.
- Exceptions/confidence/owner: detection is a small signature/UTF-8 classifier, not full content scanning; High; owner confirmation required for security policy depth.

### `BR-MEDIA-002` — Supported images produce four WebP variants synchronously

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Image upload generates `thumbnail`, `small`, `medium`, and `large` variants before response completion; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/media_processing.rs`; `backend/src/routes/media.rs`.
- Relations: media and variants; media processing module.
- Surface: upload endpoint; media UI; [media workflow](workflows/media-upload-and-processing.md); processing tests not found.
- Exceptions/confidence/owner: CPU work uses `spawn_blocking` but no durable job or retry; High; owner confirmation not required.

### `BR-MEDIA-003` — Variant names are unique per media item

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; The database permits one row per `(media_id, variant_name)`; `DATABASE_ENFORCED_RULE`.
- Evidence: `backend/migrations/0001_initial_schema.sql`.
- Relations: media variants.
- Surface: upload processing; no direct frontend operation; media workflow; no constraint test found.
- Exceptions/confidence/owner: none for persisted rows; High; owner confirmation not required.

### `BR-MEDIA-004` — Media deletion removes the database row before best-effort files and audit

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Delete cascades variant rows, then attempts to remove source/variant files and writes audit; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/media.rs`; `backend/migrations/0001_initial_schema.sql`.
- Relations: media, variants, filesystem, audit.
- Surface: media delete endpoint; `MediaPage`; deletion behavior; failure-path tests not found.
- Exceptions/confidence/owner: file removal errors are ignored and DB/files/audit are not atomic; High; `DELETION_BEHAVIOR_UNCLEAR` for orphan cleanup.

## Delivery, Settings, and Webhooks

### `BR-DELIVERY-001` — Public delivery returns only published entries and pages

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Public SQL explicitly filters `status = 'published'`; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/delivery.rs`.
- Relations: content entries and pages; delivery module.
- Surface: `/api/v1/entries*`, `/api/v1/pages*`, sitemap; public consumers; publication workflow; filter parser unit tests only.
- Exceptions/confidence/owner: the public tenant is the active organization with slug `default`; High; `TENANT_BEHAVIOR_UNCLEAR` for intended multi-tenant public routing.

### `BR-DELIVERY-002` — Published content/page changes invalidate relevant Redis delivery caches best-effort

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Publish, unpublish, archive, delete, or update of published objects calls cache invalidation; `PARTIALLY_ENFORCED_RULE`.
- Evidence: content/pages/delivery routes; cache service.
- Relations: entries, pages, Redis keys; content/pages/delivery modules.
- Surface: mutation endpoints; no direct frontend control; publication workflow; no invalidation integration tests found.
- Exceptions/confidence/owner: invalidation errors are not returned and direct DB changes bypass it; Medium; owner confirmation required for cache consistency target.

### `BR-DELIVERY-003` — Webhook subscriptions accept only four publication events and safe HTTP(S) targets

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Events are entry/page publish/unpublish; target URLs reject credentials and known local/private hosts; `APPLICATION_ENFORCED_RULE` plus database checks.
- Evidence: `backend/src/services/webhooks.rs`; `backend/src/routes/webhooks.rs`; migration 0005.
- Relations: webhooks and deliveries.
- Surface: webhook CRUD/test endpoints; `SettingsPage`; webhook workflow; validator unit tests exist.
- Exceptions/confidence/owner: hostname DNS rebinding behavior is not revalidated at connection time; High for lexical checks; owner confirmation required for network policy.

### `BR-DELIVERY-004` — Publication webhook attempts are signed and persisted as delivered or failed

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; HMAC-SHA256 headers are sent with a ten-second timeout and every completed attempt is recorded; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/webhooks.rs`; migration 0005.
- Relations: webhooks and webhook deliveries.
- Surface: publication triggers and test endpoint; webhook UI; [webhook delivery](workflows/publication-webhook-delivery.md); signature unit test exists.
- Exceptions/confidence/owner: no retry, outbox, or durable queue; High; owner confirmation not required for observed behavior.

### `BR-DELIVERY-005` — Public setting keys and navigation locales have database-defined formats

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Setting keys use lowercase/underscore; navigation locale uses language or language-region format; `DATABASE_ENFORCED_RULE`.
- Evidence: `backend/migrations/0005_phase_five_delivery_api.sql`.
- Relations: public settings and navigation items.
- Surface: public read endpoints; no complete current admin UI/API found; delivery behavior; management tests not found.
- Exceptions/confidence/owner: `WORKFLOW_UNCLEAR` because mutation ownership is not exposed in inspected current routes; High for stored shape.

## Billing and Quotas

### `BR-BILLING-001` — Each organization has at most one current subscription

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; `organization_id` is the subscription primary key; `DATABASE_ENFORCED_RULE`.
- Evidence: migration 0010; quota service.
- Relations: organizations, plans, subscriptions.
- Surface: subscription read/change/checkout; `BillingPage`; billing workflow; quota tests cover calculations, not uniqueness.
- Exceptions/confidence/owner: event history is separate in billing events; High; owner confirmation not required.

### `BR-BILLING-002` — Active/trialing/past-due subscriptions provide current plan limits; otherwise fallback applies

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Plan loading filters subscription status and uses configured/default plan behavior; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/quota.rs`.
- Relations: subscriptions, plans, content/media/member/API usage.
- Surface: tenant middleware and feature capacity checks; billing/organization pages; billing and all capacity workflows; quota unit tests cover selected calculations.
- Exceptions/confidence/owner: policy for allowing `past_due` is explicit code but business rationale is `BUSINESS_RULE_UNVERIFIED`; High.

### `BR-BILLING-003` — Stripe webhook events are idempotent by provider event identity

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Unique `(provider, provider_event_id)` and event-processing transaction prevent duplicate application; `DATABASE_ENFORCED_RULE` plus application transaction.
- Evidence: `backend/src/services/stripe_billing.rs`; migration 0011.
- Relations: billing events, subscriptions, Marketplace finance.
- Surface: Stripe callback; no frontend; billing/purchase workflows; signature/idempotency-related unit tests exist, no live provider test.
- Exceptions/confidence/owner: external provider side effects precede callback; High; owner confirmation not required.

### `BR-BILLING-004` — Older provider subscription events do not overwrite newer state

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Upsert applies only when stored provider event time is absent or not newer; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/stripe_billing.rs`.
- Relations: organization subscriptions and billing events.
- Surface: Stripe callback; billing UI reads result; billing workflow; ordering unit test exists.
- Exceptions/confidence/owner: events without reliable timestamps follow fallback behavior; High for timestamped events; owner confirmation not required.

## SaaS Operations and Beta

### `BR-SAAS-001` — Organization settings must be a JSON object and are replaced as one value

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Organization update rejects non-object settings and assigns the submitted object; `APPLICATION_ENFORCED_RULE` plus database check.
- Evidence: organization route; migration 0008.
- Relations: organizations; organization module.
- Surface: current organization update; `OrganizationPage`; settings behavior; no tests found.
- Exceptions/confidence/owner: no typed setting schema, inheritance, or merge behavior; High; `VALIDATION_RULE_UNCLEAR` for key semantics.

### `BR-SAAS-002` — Beta feedback and blockers accept only bounded enumerated values

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Category/severity/status/priority values and required text are normalized and bounded; `APPLICATION_ENFORCED_RULE` plus database checks.
- Evidence: beta route; migration 0014.
- Relations: beta feedback and GA blockers.
- Surface: beta CRUD endpoints; `BetaPage`; beta workflow; beta validation unit tests exist.
- Exceptions/confidence/owner: allowed status updates are value validation, not a state-transition graph; High; `STATE_TRANSITION_UNCLEAR`.

### `BR-SAAS-003` — Beta dashboard readiness aggregates unresolved operational signals

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Dashboard counts open feedback/blockers, failed billing/webhook records, and exceeded usage metrics; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/routes/beta.rs`; quota service.
- Relations: beta, billing, webhooks, usage.
- Surface: organization/global beta dashboards; `BetaPage`; [beta workflow](workflows/beta-feedback-and-readiness.md); query-shape unit test exists.
- Exceptions/confidence/owner: repository cannot prove an actual launch decision; Medium; `IMPLEMENTATION_STATUS_UNCLEAR`.

## Plugins and Components

### `BR-PLUGIN-001` — Only enabled built-in plugin keys execute content hooks

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Runtime enumerates compiled built-ins and runs those whose persisted global record is enabled; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/plugins/mod.rs`; plugin route.
- Relations: CMS plugins and content entries.
- Surface: plugin enable/disable and entry save/publish; Workflow UI; content save/publication workflows; SEO plugin tests exist.
- Exceptions/confidence/owner: enabled state is global, not tenant-specific; High; owner confirmation required for intended scope.

### `BR-PLUGIN-002` — The SEO plugin supplies a slug only when entry slug is missing and title is a string

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Existing slug is preserved; otherwise title is ASCII-normalized or `untitled`; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/plugins/seo.rs`.
- Relations: content entry JSON and SEO built-in plugin.
- Surface: entry create/update; no dedicated UI; content save workflow; two unit tests exist.
- Exceptions/confidence/owner: non-ASCII title characters are not retained by the algorithm; High; product localization intent is `BUSINESS_RULE_UNVERIFIED`.

### `BR-PLUGIN-003` — Uploaded Marketplace package code is not executed

- Statement/status/enforcement: `INFERRED_BUSINESS_RULE`; Runtime uses declared permissions and host-owned adapters; arbitrary server-side execution is absent; `UNENFORCED_DOCUMENTED_RULE` supported by implementation absence and contract tests.
- Evidence: Marketplace runtime/adapters services; V3 Phase 7/8 docs and tests.
- Relations: Marketplace versions/installations, component/template/hook adapters.
- Surface: Marketplace runtime/adapter APIs and `MarketplacePage`; installation workflow; contract tests assert boundaries.
- Exceptions/confidence/owner: static absence is not a formal language-level sandbox proof; High; `PLANNED_NOT_IMPLEMENTED` for arbitrary execution.

## Marketplace

### `BR-MARKET-001` — Only approved creators can submit public products

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Creator submission service rejects non-approved creator status; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/marketplace_submission.rs`; Marketplace routes.
- Relations: creators, listings, versions, submissions.
- Surface: creator/submission endpoints; `MarketplacePage`; [product publication](workflows/marketplace-product-publication.md); service unit test exists.
- Exceptions/confidence/owner: global admin moderation is a separate actor; High; owner confirmation not required.

### `BR-MARKET-002` — Submitted Marketplace artifacts become immutable in protected states

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Database trigger rejects artifact identity/content changes after submission/validation/approval/deprecation/blocking; `DATABASE_ENFORCED_RULE`.
- Evidence: migrations 0015 and 0016.
- Relations: Marketplace versions.
- Surface: upload/submission/review endpoints; Marketplace creator UI; publication workflow; static migration tests exist.
- Exceptions/confidence/owner: mutable status/review metadata remain outside protected artifact fields; High; owner confirmation not required.

### `BR-MARKET-003` — High-risk or failed Marketplace packages cannot be approved

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Validation/review services block approval based on validation and risk results; `APPLICATION_ENFORCED_RULE`.
- Evidence: Marketplace validation/review services; migration 0017.
- Relations: versions, submissions, validation reports, review events.
- Surface: review endpoints and admin UI; product publication workflow; validation/review/security contract tests exist.
- Exceptions/confidence/owner: compatibility ineligibility does not itself block review publication; High; owner confirmation not required.

### `BR-MARKET-004` — Permission approval must exactly match the installed manifest

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Initial approval is exact and changed permissions require reapproval; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/marketplace_installation.rs`; Marketplace routes.
- Relations: versions, installations, installation permissions.
- Surface: install/update endpoints; Marketplace UI; [installation lifecycle](workflows/marketplace-installation-lifecycle.md); focused unit and frontend tests exist.
- Exceptions/confidence/owner: runtime only recognizes the current finite operation catalog; High; owner confirmation not required.

### `BR-MARKET-005` — Runtime authorization requires active installation, ready runtime, declared operation, and approved permission

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Disabled/uninstalled/blocked or kill-switched installations and permission escalation are denied; `APPLICATION_ENFORCED_RULE`.
- Evidence: `backend/src/services/marketplace_runtime.rs`; runtime route; migration 0020.
- Relations: installations and permissions.
- Surface: runtime authorization/status/kill-switch endpoints; Marketplace UI; installation workflow; runtime unit tests exist.
- Exceptions/confidence/owner: authorization grants host-owned operations, not arbitrary code execution; High; owner confirmation not required.

### `BR-MARKET-006` — Marketplace installation versions remain explicitly pinned

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Installation records set `version_pinned`, updates are explicit, and no background auto-update path exists; `APPLICATION_ENFORCED_RULE` plus persisted flag.
- Evidence: migration 0019; installation service/routes.
- Relations: installations and versions.
- Surface: install/update/rollback endpoints; Marketplace UI; installation workflow; lifecycle tests exist.
- Exceptions/confidence/owner: owner product policy for future auto-update is not inferred; High; owner confirmation not required.

### `BR-MARKET-007` — Completed paid purchase grants one entitlement and refund revokes active entitlement

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Provider callback completes purchase, creates entitlement/ledger effects, and refund changes purchase/entitlement state in a transaction; `APPLICATION_ENFORCED_RULE` plus unique purchase entitlement.
- Evidence: Stripe service; Marketplace finance service/routes; migrations 0022/0023.
- Relations: purchases, entitlements, ledger, billing events.
- Surface: checkout/history/provider callback; Marketplace UI; [purchase workflow](workflows/marketplace-purchase-and-entitlement.md); finance/service unit and static tests exist.
- Exceptions/confidence/owner: partial-refund and dispute automation are `PLANNED_NOT_IMPLEMENTED`; High.

### `BR-MARKET-008` — Customer reviews require eligible use and ratings from one to five

- Statement/status/enforcement: `EXPLICIT_BUSINESS_RULE`; Review creation requires installed or purchased eligibility; database bounds rating and one review per qualifying identity/product relation; `APPLICATION_ENFORCED_RULE` plus constraints.
- Evidence: Marketplace route/feedback service; migration 0024.
- Relations: reviews, installations, purchases, listings, versions.
- Surface: review endpoints and Marketplace UI; Marketplace feedback flow; service and frontend tests exist.
- Exceptions/confidence/owner: moderation lifecycle is separate and status transitions are not one central state machine; High; `STATE_TRANSITION_UNCLEAR` for moderation sequencing.
