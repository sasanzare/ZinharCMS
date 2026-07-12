# ZinharCMS

ZinharCMS is a Rust/Axum + React headless CMS with a visual page builder roadmap.
This repository currently implements phases zero through ten from the original project
proposal plus V3 Marketplace phases 0.1 through 12: a runnable monorepo foundation, local infrastructure, CI, environment
configuration, auth, RBAC, content type CRUD, entry CRUD, media library APIs,
page JSON storage, component registry, page versioning, live preview streaming,
delivery APIs, webhooks, editorial workflow, collaboration comments, plugin
management, security hardening, i18n-ready admin localization, V2 multi-tenant SaaS operations, beta feedback tooling, GA release operations, a React admin panel for those capabilities, the V3 Marketplace phase 4 review, decision, and moderation workflow, and the phase 5 public catalog, search, and listing detail experience.

## Phase Zero Scope

- `backend/`: Rust 2024 + Axum 0.8 API scaffold with health/readiness endpoints.
- `backend/src/routes/`: phase-one auth, content, entry, and media APIs.
- `frontend/`: React 19 + Vite 6 admin workspace scaffold.
- `docker-compose.yml`: PostgreSQL 16, Redis 7, and pgAdmin only.
- `.github/workflows/`: initial backend and frontend CI.
- `backend/migrations/`: database schema based on the proposal ERD.
- `docs/`: architecture, API, and phase-zero notes.
- Internationalization: typed frontend locale dictionaries for en and fa-IR, persisted language selection, and RTL document direction support. See docs/I18N.md.

## Phase One Scope

- Auth: register, login, refresh, logout, and current-user endpoints.
- Security: Argon2id password hashing and HMAC-SHA256 access tokens.
- RBAC: `super_admin`, `admin`, `editor`, `author`, and `viewer` role checks.
- Content Types: admin-managed field schemas stored in PostgreSQL `JSONB`.
- Entries: CRUD, schema validation, pagination, sorting, publish, and unpublish.
- Media: multipart upload, metadata editing, listing, details, deletion, and image variants.

## Phase Two Scope

- Component Registry: seeded system components plus custom component CRUD.
- Pages: JSON layout CRUD, slug lookup, pagination, status filtering, publish, and unpublish.
- Validation: page metadata, layout tree, registered component types, props/styles objects, and size/depth limits.
- Versions: snapshots on create/update/restore, version history, and restore-to-draft.
- Live Preview: authenticated WebSocket stream at `/api/preview/{page_id}`.

## Phase Three Scope

- Auth UI: login/register, stored session, protected admin routes, and logout.
- Dashboard: backend health plus live CMS counts.
- Content Types: schema list/create/edit/delete with a field builder.
- Entries: dynamic forms generated from content type field schemas, CRUD, publish, and unpublish.
- Media: upload, search, metadata editing, copy URL, and delete.
- Pages: JSON editor for phase-two pages, publish/unpublish, versions, restore, and preview WebSocket URL copy.

## Phase Four Scope

- Page Builder: component palette, drag-and-drop canvas, sortable blocks, and generated props editor.
- Preview: live local preview of the current page JSON plus WebSocket preview URL copy for saved pages.
- Persistence: manual save for new pages and debounced autosave for existing page drafts.
- Compatibility: uses the existing `page_json`, page versions, publish/unpublish, and component registry APIs.

## Phase Five Scope

- Delivery API: public `/api/v1` endpoints for published content, pages, settings, and navigation.
- SEO: sitemap and robots endpoints generated from published pages and entry slugs.
- Cache: Redis-backed delivery responses with publish/update invalidation and PostgreSQL fallback when Redis is unavailable.
- Webhooks: admin-managed subscriptions for entry/page publish and unpublish events with HMAC signatures and delivery logs.

## Phase Six Scope

- Workflow: draft, pending review, published, and archived transitions for entries and pages.
- Collaboration: comments on entries/pages with resolve and reopen actions.
- Plugins: built-in plugin registry and a `seo-auto` before-save hook for entry slugs.
- Admin UI: Workflow page for review queues, comments, and plugin toggles.

## Phase Seven Scope

- Auth security: failed-login rate limiting and HttpOnly refresh-token cookies.
- API security: credentialed CORS and CSP/security response headers.
- Content security: richtext sanitization before saving entries.
- Webhook security: SSRF-focused URL validation for webhook registration.
- Upload security: allowlisted MIME types verified from file content signatures.

## V2 Beta Scope

- Multi-tenant organization model with membership-based access.
- Tenant-aware API context, PostgreSQL RLS, billing plans, quotas, and Stripe lifecycle hooks.
- Organization operations for invitations, workspace URLs, domains, rate limits, audit logs, email deliveries, and SaaS alerts.
- Phase 8 hardening coverage for tenant isolation, billing webhook ordering, security headers, and production readiness checks.
- Phase 9 beta release tooling for selected organizations, in-product feedback, product dashboarding, and GA blocker tracking.

## V2 GA Scope

- Phase 10 release notes, migration guide, admin guide, billing guide, operational runbook, and support/rollback plan.
- GA release checklist for freeze, final migration, post-release monitoring, paid plan enablement, billing support, account access support, and rollback readiness.
- `scripts/v2-ga-check.ps1` for release-candidate backend tests, frontend lint/build, and optional live health/readiness checks.
- Backend static tests that keep the GA documentation set and release checklist from silently regressing.

## V3 Marketplace Phase 0.1 Scope

- Marketplace scope lock for the first V3 implementation.
- Product taxonomy for Component Pack, Design Template, Integration Plugin, Backend Extension, and Unsupported submissions.
- Initial Marketplace review, approval, rejection, quarantine, moderation, and takedown policy.
- Backend static tests that keep the phase 0.1 scope and policy documents from silently regressing.

## V3 Marketplace Phase 0.2 Scope

- V2 readiness audit for Marketplace dependencies on organizations, billing, RBAC, audit logs, and RLS.
- Dependency matrix that fixes ownership, entitlement, permission, and audit decisions before domain modeling.
- Gap list for plugin install, creator payment, Marketplace permissions, audit taxonomy, and operational runbooks.
- Backend static tests that keep the phase 0.2 readiness audit and gap list from silently regressing.

## V3 Marketplace Phase 1 Scope

- Marketplace domain model for Creator, Listing, Package, Version, Submission, Installation, and future Purchase.
- Standard Marketplace manifest contract with required fields, supported product types, permissions, compatibility, entry points, and assets.
- Base migration for `marketplace_creators`, `marketplace_listings`, `marketplace_versions`, `marketplace_submissions`, and `marketplace_installations`.
- Package artifact storage contract with object keys, SHA-256 checksum validation, size limits, metadata, and immutable approved artifacts.
- Backend helpers and tests for manifest validation, package checksum/object-key rules, migration coverage, and tenant-owned installation RLS.

## V3 Marketplace Phase 2 Scope

- Creator profile request workflow with slug, display name, bio, support email, and pending verification status.
- Creator verification states for pending, approved, suspended, and rejected Marketplace creators.
- Listing submission metadata for product type, title, summary, description, category, screenshots, price, license, and support URL.
- Package version upload API with manifest validation, SHA-256 checksum calculation, local artifact persistence, and review submission creation.
- Admin panel Marketplace page for creator profile, listings, review submission, and package upload.

## V3 Marketplace Phase 3 Scope

- Static package validation for ZIP file tree, unsafe paths, asset limits, manifest entry points, semantic versions, and dependency metadata.
- Initial security scan for forbidden files, executable artifacts, external references, sensitive permissions, remote dependencies, and backend extensions.
- Compatibility reports for ZinharCMS version bounds, required Marketplace features, required plan, and machine-readable install eligibility.
- Validation report persistence on package versions and review submissions.
- Creator-facing and reviewer-facing Marketplace report APIs plus admin UI report panels.

## V3 Marketplace Phase 4 Scope

- Admin-only review queue for queued, validating, blocked, and submitted Marketplace submissions.
- Review decision flow for approve, reject, and request changes with internal comments and creator-facing messages.
- Append-only Marketplace review event log plus audit log entries for every decision.
- Moderation and takedown actions for listing suspension, version unpublish, and emergency blocking.
- Admin UI controls for review decisions, moderation actions, and recent event history.
## V3 Marketplace Phase 5 Scope

- Tenant-aware public catalog API for approved, safe, compatible Marketplace listings.
- Search and filters for query text, category, product type, and pricing type while excluding suspended or incompatible products.
- Listing detail API and admin UI panels for description, screenshots, changelog, permissions, compatible versions, customer reviews, license, and support links.
- Compatibility reports recalculated against the active organization's plan before catalog display.

## V3 Marketplace Phase 6 Scope

- Organization-owned install records for approved, safe, compatible free Component Packs and Design Templates.
- Exact install-time permission approval plus package existence, size, and SHA-256 integrity gates.
- Owner/admin enable, disable, soft-uninstall, semver update, version pinning, and safe rollback operations.
- Tenant-transactional lifecycle audit logs, preserved organization data, lifecycle timestamps, and forced-RLS isolation.
- Paid/custom Marketplace installs remain blocked until purchase and entitlement support is implemented.

## V3 Marketplace Phase 7 Scope

- Permission catalog for content, page, media, webhook, settings, and external integration capabilities with risk and product-type metadata.
- Allowlisted sandbox host API policy for declared runtime operations, safe entry-point paths, approved permission snapshots, and bounded JSON payloads.
- Runtime authorization decisions that never execute uploaded package code and deny inactive, incompatible, unapproved, unsafe, or oversized requests.
- Organization and global Marketplace kill switches with reason, status, lift, forced-RLS organization handling, cross-organization admin bypass boundaries, and audit records.

## V3 Marketplace Phase 8 Scope

- Component Pack manifest definitions materialized into the organization-scoped Page Builder palette.
- Design Template preview and independent page clone with organization-owned media asset mapping.
- Public Plugin Hook MVP registry for sidebar items, dashboard widgets, form fields, and webhook adapters.
- Adapter endpoints remain host-owned and policy-checked; uploaded Marketplace package code is never executed.

## V3 Marketplace Phase 9 Scope

- Free and paid organization purchases with receipts, tax metadata, and Stripe one-time Checkout for paid products.
- Active entitlements gate paid install, re-enable, update, and rollback operations; full refunds revoke access.
- Idempotent revenue split ledger entries record platform commission, creator share, and refund reversals.
- Creator payout-provider onboarding and admin verification prevent unverified creators from becoming payout eligible.
- Marketplace finance remains separate from organization subscription billing.

## V3 Marketplace Phase 10 Scope

- Organization-installed or purchased products can receive one customer rating and review per organization, with pending/published/rejected moderation.
- Catalog ratings use only published reviews; the ownership gate is enforced by the API and recorded in tenant audit logs.
- Abuse reports collect violation type, severity, description, and JSON evidence in a global-admin moderation queue.
- Critical reports create a persisted unread internal admin notification and audit event; external notification delivery and automatic takedown remain future work.

## V3 Marketplace Phase 11 Scope

- Creator analytics summarize each creator's own listings, installs, active installs, revenue, conversion, ratings, reports, and persisted error signals.
- Internal admin analytics summarize Marketplace submission rate, approval time, installs, refunds, reports, critical reports, blocked packages, and risky/repetitive products.
- Analytics are read-only projections over existing Marketplace tables; runtime execution error telemetry, warehouse export, and anomaly alerting remain future work.

## V3 Marketplace Phase 12 Scope

- Creator-side CLI tooling validates Marketplace manifests, package file trees, permissions, compatibility, adapter declarations, and security findings before upload.
- The CLI packs validated package directories into ZIP artifacts and can submit them to the existing Marketplace version upload API.
- Creator documentation covers manifest requirements, permissions, review policy, CLI workflow, and sample packages.
- Component Pack and Integration Plugin sample packages are available under `docs/marketplace-samples`.
- Backend review remains the final authority and uploaded Marketplace package code is still never executed.

Creator tooling quick check:

```powershell
npm run marketplace -- validate docs/marketplace-samples/component-pack
npm run marketplace -- pack docs/marketplace-samples/component-pack --force
```

## Quick Start

Copy the environment template and start the local stack:

```powershell
Copy-Item .env.example .env
docker compose up -d postgres redis pgadmin
```

Local services:

- pgAdmin: http://localhost:5050
- PostgreSQL: localhost:5432
- Redis: localhost:6379
- API: http://localhost:8080
- Admin UI: http://localhost:5173

## Local Development Without Docker

Start the infrastructure:

Run the backend:

```powershell
cd backend
cargo run
```

Run the frontend:

```powershell
cd frontend
npm install
npm run dev
```
