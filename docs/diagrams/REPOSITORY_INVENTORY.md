# Repository Inventory For Future Diagrams

Inspection date: 2026-07-04.

This inventory lists architecture-relevant files that were inspected for the
diagram evidence pass. It intentionally explains why files matter instead of
only mirroring the directory tree.

## Root, Runtime, And CI Configuration

| Path | Responsibility | Domain | Kind | Main symbols, services, or settings | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `README.md` | Current project overview and quick start. Useful as context only after code/schema evidence. | Product scope, local dev | Documentation | V1/V2/V3 phase summary, ports, default services | Context map, release scope timeline |
| `.env.example` | Primary local backend/frontend environment contract. | Runtime configuration | Configuration | `DATABASE_URL`, `REDIS_URL`, `JWT_*`, `COOKIE_SECURE`, `CORS_ORIGIN`, `STRIPE_*`, `EMAIL_WEBHOOK_URL`, rate limits | Deployment/config diagram |
| `env.example` | Older sample environment file with a smaller variable set. | Runtime configuration | Configuration | Database, Redis, JWT, upload, CORS, Stripe variables | Configuration drift diagram |
| `docker-compose.yml` | Local infrastructure only. | Local infrastructure | Configuration | PostgreSQL 16, Redis 7, pgAdmin, named volumes | Local development diagram |
| `docker-compose.prod.yml` | Containerized production-like stack. | Deployment | Configuration | backend, frontend, postgres, redis, nginx, health checks | Deployment topology diagram |
| `package.json` | Root workspace orchestration scripts. | Developer workflow | Configuration | `dev:infra`, `dev:backend`, `dev:frontend`, test/build scripts | Developer workflow diagram |
| `.github/workflows/backend-ci.yml` | Backend quality gate. | CI/CD | Configuration | PostgreSQL and Redis services, `cargo fmt`, `cargo clippy`, `cargo test` | CI pipeline diagram |
| `.github/workflows/frontend-ci.yml` | Frontend quality gate. | CI/CD | Configuration | Node 22, `npm install`, lint, typecheck, test, build | CI pipeline diagram |
| `scripts/v2-ga-check.ps1` | GA release checklist runner. | GA readiness | Script/test | Backend tests, frontend lint/build, health/ready checks | Release readiness workflow |
| `scripts/phase8-load-smoke.ps1` | Tenant and load smoke helper. | Hardening, tenant checks | Script/test | Smoke paths, local endpoint checks | Hardening test flow |
| `scripts/marketplace-phase13-load-smoke.ps1` | Marketplace catalog/listing/install latency smoke helper. | Marketplace QA/performance | Script/test | Authenticated catalog/search/listing checks, optional install mutation, P95 budget reporting | Marketplace QA/performance diagram |
| `scripts/marketplace-phase14-beta-readiness.ps1` | Read-only Marketplace beta evidence helper. | Marketplace beta readiness | Script/test | Existing beta, install, purchase, report, and analytics endpoint checks with report-only mode | Marketplace beta readiness diagram |
| `scripts/marketplace-phase15-ga-check.ps1` | Marketplace Launch Readiness and GA checklist runner. | Marketplace launch/GA readiness | Script/test | Phase 15 backend contracts, Marketplace regression, frontend lint/build, optional health/ready/install/purchase/report/analytics checks | Marketplace launch/GA diagram |

## Backend Cargo And Container Files

| Path | Responsibility | Domain | Kind | Main symbols, services, or settings | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/Cargo.toml` | Backend crate dependency and test configuration. | Backend runtime | Configuration | Axum, SQLx, Redis, reqwest, image, utoipa, tokio | Backend dependency diagram |
| `backend/Cargo.lock` | Locked backend dependency graph. | Build reproducibility | Configuration | Exact Rust crate versions | Supply chain diagram |
| `backend/Dockerfile` | Development/backend container build. | Deployment | Configuration | Rust backend image build path | Backend deployment diagram |
| `backend/Dockerfile.prod` | Production backend image build. | Deployment | Configuration | Release build and runtime image | Production deployment diagram |

## SQL Migrations

| Path | Responsibility | Domain | Kind | Main tables, enums, constraints, or policies | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/migrations/0001_initial_schema.sql` | Base schema for users, roles, tokens, content, pages, components, and media. | Auth, RBAC, CMS core | Schema | `users`, `roles`, `user_roles`, `refresh_tokens`, `content_types`, `content_entries`, `pages`, `page_versions`, `component_registry`, `media`, `media_variants`, `content_status`, `page_status` | ERD, auth flow, CMS core flow |
| `backend/migrations/0002_seed_foundation_data.sql` | Default roles, permissions, and system components. | RBAC, page builder | Schema/data | Role permissions, component registry seed data | RBAC matrix, component registry diagram |
| `backend/migrations/0003_phase_one_core.sql` | Phase one role and content refinements. | Auth, content core | Schema/data | Additional permissions, status and field rules | RBAC and content lifecycle diagrams |
| `backend/migrations/0004_phase_two_page_builder.sql` | Page-builder component key and page metadata additions. | Page builder | Schema | Component key constraints, page metadata | Page builder data diagram |
| `backend/migrations/0005_phase_five_delivery_api.sql` | Public delivery support, webhook persistence, settings, and navigation. | Delivery API, webhooks | Schema | `webhooks`, `webhook_deliveries`, `public_settings`, `navigation_items` | Delivery and webhook sequence diagrams |
| `backend/migrations/0006_phase_six_workflow_collaboration.sql` | Editorial workflow, comments, and plugin registry. | Workflow, comments, plugins | Schema | `pending_review`, `comments`, `cms_plugins`, plugin permissions | Workflow state diagram |
| `backend/migrations/0007_phase_seven_security.sql` | Login rate-limit persistence. | Security hardening | Schema | `login_attempts` | Auth security diagram |
| `backend/migrations/0008_v2_phase_one_organizations.sql` | Organization tenancy base. | Organizations, memberships, tenant ownership | Schema | `organizations`, `organization_members`, `organization_invitations`, tenant `organization_id` columns and triggers | Multi-tenant ERD |
| `backend/migrations/0009_v2_phase_three_rls.sql` | PostgreSQL tenant isolation policy layer. | RLS | Schema/security | `app_current_organization_id`, `app_rls_bypass_enabled`, `app_rls_tenant_matches`, forced RLS policies | RLS enforcement diagram |
| `backend/migrations/0010_v2_phase_five_billing_quota.sql` | Billing plans, subscriptions, and usage counters. | Billing, quotas | Schema/data | `plans`, `organization_subscriptions`, `usage_counters`, RLS policies | Billing and quota ERD |
| `backend/migrations/0011_v2_phase_six_stripe_billing.sql` | Stripe identifiers and billing webhook events. | Stripe billing | Schema | `plans.stripe_*`, `billing_events`, RLS policies | Stripe webhook sequence diagram |
| `backend/migrations/0012_v2_phase_seven_saas_ops.sql` | SaaS operations model. | Domains, rate limits, audit, email, alerts | Schema/data | `organization_domains`, `organization_rate_limits`, `audit_logs`, `email_deliveries`, `saas_alert_rules`, RLS policies | SaaS ops ERD |
| `backend/migrations/0013_v2_phase_eight_hardening.sql` | Additive hardening fields for billing/order and operational checks. | Hardening | Schema | Provider event ordering fields and hardening support | Hardening controls diagram |
| `backend/migrations/0014_v2_phase_nine_beta_release.sql` | Beta release feedback and GA blocker data. | Beta, GA readiness | Schema | `beta_participants`, `beta_feedback`, `beta_ga_blockers`, forced RLS policies | Beta feedback workflow |
| `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` | Marketplace base domain tables. | Marketplace core | Schema | `marketplace_creators`, `marketplace_listings`, `marketplace_versions`, `marketplace_submissions`, `marketplace_installations`, installation RLS | Marketplace ERD |
| `backend/migrations/0016_v3_phase_two_creator_submission.sql` | Creator verification and listing submission refinements. | Marketplace creators/listings | Schema | Creator statuses, listing metadata constraints, immutable version trigger | Marketplace submission diagram |
| `backend/migrations/0017_v3_phase_three_validation_pipeline.sql` | Validation and compatibility report storage. | Marketplace validation | Schema | `validation_status`, `security_risk_level`, `validation_report`, `compatibility_report` | Package validation sequence |
| `backend/migrations/0018_v3_phase_four_review_moderation.sql` | Marketplace review and moderation event log. | Marketplace review/moderation | Schema | `marketplace_review_events`, supported actions including `emergency_block` | Review moderation state diagram |
| `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql` | Marketplace installation lifecycle additions. | Marketplace installations | Schema | cleanup policy, version pinning, lifecycle timestamps, same-listing rollback FK | Marketplace lifecycle/data model diagrams |
| `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql` | Marketplace permission catalog, runtime status, and kill-switch state. | Marketplace security runtime | Schema | permission catalog, runtime blocking, global/organization kill switches, forced RLS | Marketplace security runtime diagram |
| `backend/migrations/0021_v3_phase_eight_runtime_adapters.sql` | Component registry installation link, template import records, and public plugin hook records. | Marketplace runtime adapters | Schema | Component Pack registry, tenant template imports, public hook contracts, forced RLS | Marketplace runtime adapter diagram |
| `backend/migrations/0022_v3_phase_nine_marketplace_finance.sql` | Marketplace purchases, paid entitlements, payouts, and finance ledgers. | Marketplace finance | Schema | purchases, entitlements, payout accounts/requests, account ledger, forced RLS | Marketplace finance diagram |
| `backend/migrations/0023_v3_phase_nine_finance_hardening.sql` | Marketplace finance abuse guards and webhook idempotency. | Marketplace finance/security | Schema | provider event uniqueness, append-only ledger trigger, checkout/payout guards | Marketplace finance hardening diagram |
| `backend/migrations/0024_v3_phase_ten_ratings_abuse.sql` | Ratings, reviews, abuse reports, and review moderation queues. | Marketplace feedback/trust | Schema | reviews, abuse reports, moderation queues, evidence objects, forced RLS | Marketplace feedback and abuse diagram |
| `backend/migrations/0025_v3_phase_ten_internal_notifications.sql` | Internal notification persistence for Marketplace moderation signals. | Marketplace notifications | Schema | notification events and read state | Marketplace feedback and abuse diagram |
| `backend/migrations/0026_v3_phase_thirteen_marketplace_qa_performance.sql` | Marketplace catalog/search/install performance indexes. | Marketplace QA/performance | Schema | `pg_trgm`, catalog/search/listing/install indexes | Marketplace QA/performance diagram |

## Backend Application Composition

| Path | Responsibility | Domain | Kind | Main symbols, route groups, or components | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/src/lib.rs` | Exposes backend module tree for tests and binary. | Backend composition | Executable code | module exports | Backend module map |
| `backend/src/main.rs` | Starts runtime, migrations, default admin and organization seed, router, and shutdown. | Runtime bootstrap | Executable code | `db::run_migrations`, Redis client, `seed_default_admin`, `ensure_default_organization_membership` | Startup sequence diagram |
| `backend/src/config.rs` | Environment parsing and defaults. | Runtime configuration | Executable code | `Config`, Stripe, CORS, JWT, upload, rate limit, email settings | Config source diagram |
| `backend/src/state.rs` | Shared application state. | Runtime composition | Executable code | `AppState`, `PagePreviewChannels`, Redis client, PostgreSQL pool | Runtime object graph |
| `backend/src/error.rs` | API error representation. | API boundary | Executable code | `AppError`, `ErrorBody`, HTTP status mapping | Error flow diagram |
| `backend/src/db/mod.rs` | Database pool and migration runner. | Persistence | Executable code | `connect_lazy`, `run_migrations` | Startup and migration sequence |

## Backend Middleware

| Path | Responsibility | Domain | Kind | Main symbols, checks, or assumptions | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/src/middleware/auth.rs` | Verifies bearer/preview access tokens and injects claims. | Authentication | Executable code | `auth_middleware`, preview token query support | Auth request flow |
| `backend/src/middleware/security.rs` | Adds HTTP security headers. | Security hardening | Executable code | CSP, frame, content type, referrer and permission headers | Security controls diagram |
| `backend/src/middleware/tenant.rs` | Enforces active organization membership, tenant header, rate limits, and API quota. | Tenancy, quota | Executable code | `TenantContext`, `X-Organization-Id`, `tenant_middleware`, preview query fallback | Tenant request flow |
| `backend/src/middleware/mod.rs` | Middleware module export. | Backend composition | Executable code | module exports | Backend module map |

## Backend Routes

| Path | Responsibility | Domain | Kind | Main route groups or handlers | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/src/routes/mod.rs` | Global router composition, public/protected/tenant layers, upload serving, OpenAPI registration. | API composition | Executable code | `/health`, `/ready`, `/openapi.json`, protected routes, tenant routes, `/uploads` | API boundary diagram |
| `backend/src/routes/auth.rs` | Public and protected authentication endpoints. | Auth, refresh tokens | Executable code | `/api/auth/register`, `/login`, `/refresh`, `/logout`, `/me`; refresh cookie helpers | Auth sequence diagram |
| `backend/src/routes/content.rs` | Content type and entry CRUD plus workflow actions. | Content types, entries, workflow | Executable code | `/api/content-types`, `/api/entries/{type_slug}`, submit/publish/unpublish/reject/archive/restore | Content lifecycle diagram |
| `backend/src/routes/media.rs` | Media library CRUD and upload pipeline. | Media | Executable code | `/api/media`, `/api/media/upload`, upload directory by organization | Media upload sequence |
| `backend/src/routes/pages.rs` | Page CRUD, page versions, component registry, and preview socket. | Page builder, versions, live preview | Executable code | `/api/pages`, `/api/component-registry`, `/api/preview/{page_id}` | Page builder and preview diagrams |
| `backend/src/routes/delivery.rs` | Public content, pages, settings, navigation, sitemap, and robots endpoints. | Delivery API, cache | Executable code | `/api/v1/content`, `/api/v1/pages`, `/api/v1/settings/public`, `/api/v1/navigation`, `/sitemap.xml`, `/robots.txt` | Public delivery diagram |
| `backend/src/routes/webhooks.rs` | Tenant webhook subscription CRUD and delivery inspection/test endpoint. | Webhooks | Executable code | `/api/webhooks`, `/deliveries`, `/test` | Webhook management diagram |
| `backend/src/routes/comments.rs` | Entity comments and resolution state. | Collaboration | Executable code | `/api/comments`, `/api/comments/{id}/resolve` | Comment workflow diagram |
| `backend/src/routes/plugins.rs` | Built-in plugin registry management. | Plugins | Executable code | `/api/plugins`, enable/disable/update | Plugin lifecycle diagram |
| `backend/src/routes/organizations.rs` | Organizations, members, invitations, domains, rate limits, audit, email, alerts, leave and ownership transfer. | Organizations, SaaS ops | Executable code | `/api/organizations`, `/api/organizations/current/*` | Organization admin diagram |
| `backend/src/routes/billing.rs` | Plans, subscriptions, checkout, portal, usage, rebuild, and Stripe webhook. | Billing, Stripe, quota | Executable code | `/api/billing/*`, `/api/billing/stripe/webhook` | Billing sequence diagram |
| `backend/src/routes/beta.rs` | Beta dashboard, feedback, blockers, and participant selection. | Beta feedback, GA blockers | Executable code | `/api/beta/dashboard`, `/feedback`, `/ga-blockers`, `/product-dashboard`, `/participants/{organization_id}` | Beta feedback workflow |
| `backend/src/routes/marketplace.rs` | Marketplace catalog, creator profile, listings, package upload, validation reports, review decisions, moderation, and installation lifecycle. | Marketplace | Executable code | `/api/marketplace/catalog`, `/creator`, `/listings`, `/versions/upload`, `/review/*`, `/installations/*` | Marketplace catalog/submission/review/lifecycle diagrams |
| `backend/src/routes/marketplace_adapters.rs` | Component Pack registry, Template preview/import, and public Plugin Hook authorization. | Marketplace runtime adapters | Executable code | `/api/marketplace/runtime/components`, `/templates/*`, `/hooks/*` | Marketplace runtime adapter diagram |
| `backend/src/routes/marketplace_runtime.rs` | Permission catalog, sandbox authorization, runtime status, and kill-switch handlers. | Marketplace security runtime | Executable code | `/api/marketplace/permissions`, `/runtime/status`, `/runtime/authorize`, `/kill-switches/*` | Marketplace security runtime diagram |
| `backend/src/routes/marketplace_finance.rs` | Marketplace purchases, checkout, paid install gate, payouts, and finance admin flows. | Marketplace finance | Executable code | `/api/marketplace/purchases`, `/checkout`, `/install`, payout account/request/admin verification routes | Marketplace finance and beta readiness diagrams |
| `backend/src/routes/marketplace_analytics.rs` | Creator-owned analytics and admin Marketplace health/risk analytics. | Marketplace analytics | Executable code | `/api/marketplace/creators/{creator_id}/analytics`, `/api/marketplace/analytics/admin` | Marketplace analytics and beta readiness diagrams |

## Backend Services And Plugins

| Path | Responsibility | Domain | Kind | Main symbols or behaviors | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/src/services/jwt.rs` | Access token signing/verification and refresh token generation/hash. | Auth | Executable code | `sign_access_token`, `verify_access_token`, `generate_refresh_token`, `hash_refresh_token` | Auth token diagram |
| `backend/src/services/password.rs` | Password hashing and verification. | Auth | Executable code | Argon2 helpers | Auth sequence diagram |
| `backend/src/services/security.rs` | Login throttling and rich text sanitization. | Security hardening | Executable code | `require_login_allowed`, `record_login_attempt`, `sanitize_entry_data`, `sanitize_richtext` | Security controls diagram |
| `backend/src/services/rbac.rs` | Global and organization role authorization helpers. | RBAC | Executable code/test | `require_any`, `require_org_any`, org role matrix tests | RBAC matrix |
| `backend/src/services/rls.rs` | Sets tenant or bypass context on SQL connections/transactions. | RLS | Executable code | `tenant_connection`, `begin_tenant_transaction`, `begin_bypass_transaction` | RLS request diagram |
| `backend/src/services/rate_limit.rs` | Redis-backed organization/user rate limiting. | Security, SaaS ops | Executable code/test | `check_and_record_request`, Redis counters | Rate limit flow |
| `backend/src/services/quota.rs` | Plan limits, subscription state, usage counters, quota checks and rebuilds. | Billing, quotas | Executable code/test | `list_plans`, `change_plan`, `check_and_record_api_request`, `rebuild_usage_counters` | Quota flow |
| `backend/src/services/stripe_billing.rs` | Stripe checkout, portal, webhook processing, signature validation, event ordering. | Stripe billing | Executable code/test | `create_checkout_session`, `handle_webhook`, `verify_signature`, provider ordering tests | Stripe webhook sequence |
| `backend/src/services/audit.rs` | Audit log writes inside normal and transactional flows. | Audit logs | Executable code | `record`, `record_for_organization`, `record_in_transaction` | Audit event diagram |
| `backend/src/services/email.rs` | Invitation and billing notification delivery with persistence. | Email deliveries | Executable code/test | `send_invitation_email`, `send_billing_notification`, `email_deliveries` updates | Email sequence diagram |
| `backend/src/services/cache.rs` | Redis JSON cache with database fallback and prefix invalidation. | Redis caching, delivery | Executable code | `get_or_set_json`, `invalidate`, `invalidate_prefix` | Cache flow diagram |
| `backend/src/services/webhooks.rs` | Webhook validation, HMAC signing, delivery dispatch, and delivery records. | Webhooks | Executable code/test | SSRF host checks, `trigger_event`, `dispatch_webhook`, `webhook_deliveries` | Webhook delivery diagram |
| `backend/src/services/workflow.rs` | Shared content/page status transition rules. | Workflow | Executable code/test | `WorkflowStatus`, `require_transition` | Workflow state diagram |
| `backend/src/services/entry_validation.rs` | Dynamic content field schema and entry data validation. | Content types, entries | Executable code | `parse_fields`, `validate_entry_data`, slug/identifier validation | Content validation diagram |
| `backend/src/services/media_processing.rs` | Image mime and variant generation. | Media | Executable code | supported image MIME check, WebP variants | Media processing diagram |
| `backend/src/services/health.rs` | Readiness dependency response helper. | Health | Executable code | dependency result helpers | Health/readiness diagram |
| `backend/src/services/hardening.rs` | Static tests for RLS and hardening coverage. | Security hardening | Test code | include migration/script assertions | Hardening test evidence |
| `backend/src/services/ga_readiness.rs` | Static tests for GA docs and release checklist. | GA readiness | Test code | include docs/scripts assertions | GA readiness diagram |
| `backend/src/services/marketplace_policy.rs` | Static tests for V3 phase 0.1 scope and policy docs. | Marketplace policy | Test code | include policy/taxonomy/scope docs | Marketplace governance diagram |
| `backend/src/services/marketplace_readiness.rs` | Static tests for V3 phase 0.2 readiness and gap docs. | Marketplace readiness | Test code | include audit/dependency/gap docs | Marketplace dependency diagram |
| `backend/src/services/marketplace_domain.rs` | Static tests for domain docs, manifest, package storage, and base migration. | Marketplace domain | Test code | include 0015 migration and docs | Marketplace ERD |
| `backend/src/services/marketplace_manifest.rs` | Manifest schema validation. | Marketplace package validation | Executable code/test | `validate_marketplace_manifest`, semver validation | Package validation diagram |
| `backend/src/services/marketplace_package.rs` | Package object key, checksum, and size rules. | Marketplace packages | Executable code/test | `marketplace_package_object_key`, `sha256_hex`, `validate_package_size` | Package storage diagram |
| `backend/src/services/marketplace_submission.rs` | Creator/listing input normalization and review eligibility validation. | Marketplace submissions | Executable code/test | `validate_creator_profile`, `validate_listing_for_review` | Submission workflow diagram |
| `backend/src/services/marketplace_validation.rs` | ZIP parsing, static validation, security scan, compatibility report. | Marketplace validation | Executable code/test | `evaluate_marketplace_package`, `install_eligible` report, high-risk blocking | Validation pipeline diagram |
| `backend/src/services/marketplace_review.rs` | Review and moderation transition validation. | Marketplace review/moderation | Executable code/test | `validate_review_decision`, `validate_moderation_action` | Review state diagram |
| `backend/src/services/marketplace_catalog.rs` | Catalog compatibility and install eligibility derivation. | Marketplace catalog | Executable code/test | `catalog_compatibility_report`, `is_catalog_compatible` | Catalog filtering diagram |
| `backend/src/services/marketplace_installation.rs` | Installation lifecycle gates and artifact verification. | Marketplace installations | Executable code/test | permission snapshots, lifecycle transitions, SemVer, artifact size/SHA gates | Marketplace lifecycle diagrams |
| `backend/src/services/marketplace_adapters.rs` | Pure manifest component, template, asset mapping, and public hook contract rules. | Marketplace runtime adapters | Executable code/test | component definitions, template JSON, tenant asset mapping, public hook allowlist | Marketplace runtime adapter diagram |
| `backend/src/services/marketplace_runtime.rs` | Allowlisted runtime operations and sandbox policy validation. | Marketplace security runtime | Executable code/test | permission-bound operations, safe entry points, payload limit, runtime decision | Marketplace security runtime diagram |
| `backend/src/services/marketplace_finance.rs` | Marketplace pricing, checkout, entitlement, payout, and finance invariants. | Marketplace finance | Executable code/test | pricing validation, checkout requests, entitlement checks, creator payout ownership | Marketplace finance diagram |
| `backend/src/services/marketplace_feedback.rs` | Marketplace ratings, reviews, abuse reports, moderation queues, and internal notification contracts. | Marketplace feedback/trust | Test code | static coverage for review ownership, abuse evidence, moderation status, notification events | Marketplace feedback and abuse diagram |
| `backend/src/services/marketplace_analytics.rs` | Marketplace analytics dashboard and creator tooling contract checks. | Marketplace analytics/creator tooling | Test code | install/purchase/review aggregations, creator samples, CLI evidence | Marketplace analytics diagram |
| `backend/src/services/marketplace_performance.rs` | Marketplace cache policy, index, and latency budget contracts. | Marketplace QA/performance | Executable code/test | catalog cache headers, P95 targets, 0026 index assertions, load-smoke script evidence | Marketplace QA/performance diagram |
| `backend/src/services/marketplace_phase_thirteen.rs` | Marketplace security QA regression contracts. | Marketplace QA/security | Test code | IDOR, permission bypass, malicious package, refund abuse, review abuse assertions | Marketplace QA/performance diagram |
| `backend/src/services/marketplace_phase_fourteen.rs` | Marketplace beta readiness static contract checks. | Marketplace beta readiness | Test code | Private Creator Beta and Customer Beta evidence assertions against docs, script, diagram, and existing routes | Marketplace beta readiness diagram |
| `backend/src/services/marketplace_phase_fifteen.rs` | Marketplace launch readiness and GA static contract checks. | Marketplace launch/GA readiness | Test code | Launch Readiness, runbook, final policy, support workflow, rollback, incident checklist, release notes, monitoring dashboard, and support plan assertions | Marketplace launch/GA diagram |
| `backend/src/services/mod.rs` | Service module export. | Backend composition | Executable code | module exports | Backend module map |
| `backend/src/plugins/mod.rs` | Built-in plugin trait and hook runner. | Plugins | Executable code | `CmsPlugin`, `builtin_plugins`, hook runners | Plugin hook diagram |
| `backend/src/plugins/seo.rs` | Built-in SEO slug plugin. | Plugins, entries | Executable code/test | `SeoAutoPlugin`, `slugify` | Plugin hook diagram |

## Backend Models

| Path | Responsibility | Domain | Kind | Main symbols | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `backend/src/models/user.rs` | User DTOs and persistence mappings. | Auth, users | Executable code | user structs | Auth/user ERD |
| `backend/src/models/content.rs` | Content type and entry models. | Content | Executable code | content structs | CMS core ERD |
| `backend/src/models/media.rs` | Media and variant models. | Media | Executable code | media structs | Media ERD |
| `backend/src/models/page.rs` | Page and version models. | Pages | Executable code | page structs | Page ERD |
| `backend/src/models/organization.rs` | Organization, membership, and invitation models. | Organizations | Executable code | org structs | Organization ERD |
| `backend/src/models/mod.rs` | Model module export. | Backend composition | Executable code | module exports | Backend module map |

## Frontend Configuration

| Path | Responsibility | Domain | Kind | Main settings | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `frontend/package.json` | Frontend scripts and dependencies. | Frontend build/test | Configuration | Vite, React, React Router, Zustand, Vitest, Tailwind, lucide | Frontend dependency diagram |
| `frontend/package-lock.json` | Locked frontend dependency graph. | Build reproducibility | Configuration | npm lock data | Supply chain diagram |
| `frontend/Dockerfile` | Development/frontend image. | Deployment | Configuration | Node build path | Frontend deployment diagram |
| `frontend/Dockerfile.prod` | Production frontend image. | Deployment | Configuration | Build then nginx serve | Production deployment diagram |
| `frontend/nginx.conf` | Static frontend serving and fallback routing. | Deployment | Configuration | SPA fallback and nginx config | Deployment topology diagram |
| `frontend/index.html` | Vite document shell. | Frontend runtime | Configuration | root element | Frontend runtime diagram |
| `frontend/vite.config.ts` | Frontend build/dev server config. | Frontend build | Configuration | Vite config | Frontend build diagram |
| `frontend/vitest.config.ts` | Frontend test config. | Frontend tests | Configuration | Vitest/jsdom setup | Test pipeline diagram |
| `frontend/eslint.config.js` | Frontend lint rules. | Quality gate | Configuration | ESLint config | CI pipeline diagram |
| `frontend/tsconfig.json` | Shared TypeScript config. | Frontend build | Configuration | Project references | Frontend build diagram |
| `frontend/tsconfig.app.json` | App TypeScript config. | Frontend build | Configuration | React app typing | Frontend build diagram |
| `frontend/tsconfig.node.json` | Node/tooling TypeScript config. | Frontend build | Configuration | Vite tooling typing | Frontend build diagram |

## Frontend Source

| Path | Responsibility | Domain | Kind | Main symbols, components, or routes | Related future diagrams |
| --- | --- | --- | --- | --- | --- |
| `frontend/src/main.tsx` | React app bootstrapping. | Frontend runtime | Executable code | `RouterProvider`, `I18nProvider` | Frontend app shell diagram |
| `frontend/src/router.tsx` | SPA route registration. | Frontend navigation | Executable code | `/login`, `/`, content, media, marketplace, pages, workflow, organization, billing, beta, settings | Frontend route diagram |
| `frontend/src/services/api.ts` | Typed API client and auth/org headers. | Frontend API | Executable code | `api.auth`, `api.organizations`, `api.billing`, `api.marketplace`, `api.pages`, `api.webhooks` | Frontend-to-backend API diagram |
| `frontend/src/stores/useAppStore.ts` | Auth session and active organization storage. | Auth, tenancy | Executable code | local storage keys, tokens, organizations | Client state diagram |
| `frontend/src/hooks/useHealth.ts` | Backend readiness polling. | Health | Executable code | health/ready hook | Health UI diagram |
| `frontend/src/types/api.ts` | Shared frontend response/request types. | Frontend API contract | Executable code/types | CMS, org, billing, marketplace, beta DTOs | API type map |
| `frontend/src/styles/index.css` | Global UI layout and responsive styles. | Frontend UI | Executable code/style | app shell, panels, page builder, marketplace UI, RTL rules | UI layout diagram |
| `frontend/src/vite-env.d.ts` | Vite ambient types. | Frontend build | Executable code/types | Vite env typing | Build diagram |
| `frontend/src/components/AppShell.tsx` | Authenticated admin shell and navigation. | Frontend shell, navigation | Executable code | sidebar nav, organization selector, language selector | Admin shell diagram |
| `frontend/src/components/RequireAuth.tsx` | Auth gate for protected routes. | Auth | Executable code | redirect to login, session check | Client auth flow |
| `frontend/src/components/DynamicForm.tsx` | Dynamic entry form rendering from content type fields. | Entries | Executable code | field renderer | Entry form diagram |
| `frontend/src/components/StatusBadge.tsx` | Shared status display component. | UI | Executable code | status badge | UI component diagram |
| `frontend/src/i18n/context.ts` | i18n React context. | Localization | Executable code | context definitions | Localization diagram |
| `frontend/src/i18n/I18nProvider.tsx` | i18n state, locale persistence, document direction. | Localization | Executable code | `I18nProvider`, `lang`, `dir` | Localization flow |
| `frontend/src/i18n/LanguageSelect.tsx` | Language selector UI. | Localization | Executable code | language selector | Localization UI diagram |
| `frontend/src/i18n/index.ts` | i18n exports. | Localization | Executable code | module exports | Localization module map |
| `frontend/src/i18n/labels.ts` | Locale label helpers. | Localization | Executable code | localized labels | Localization map |
| `frontend/src/i18n/locales.ts` | Supported locale metadata. | Localization | Executable code | English, Persian, RTL metadata | Localization map |
| `frontend/src/i18n/messages.ts` | Translation message catalog. | Localization | Executable code/data | English/Persian message keys | Localization coverage diagram |
| `frontend/src/i18n/useI18n.ts` | i18n hook. | Localization | Executable code | `useI18n` | Localization flow |
| `frontend/src/pages/AuthPage.tsx` | Login/register page. | Auth | Executable code | auth form, default organization handling | Auth UI flow |
| `frontend/src/pages/DashboardPage.tsx` | Foundation dashboard. | Admin overview | Executable code | cards and health context | Dashboard diagram |
| `frontend/src/pages/ContentTypesPage.tsx` | Content type schema management UI. | Content types | Executable code | schema form, field editor | Content type UI flow |
| `frontend/src/pages/EntriesPage.tsx` | Entry CRUD and workflow UI. | Entries, workflow | Executable code | dynamic form, list filters, workflow actions | Entry lifecycle UI |
| `frontend/src/pages/MediaPage.tsx` | Media library and upload UI. | Media | Executable code | upload form, library list | Media UI flow |
| `frontend/src/pages/PagesPage.tsx` | Page builder, versions, component registry, preview URL. | Page builder, live preview | Executable code/tested | drag/drop builder, props editor, version restore, preview socket URL | Page builder UI diagram |
| `frontend/src/pages/WorkflowPage.tsx` | Review queues, comments, plugin toggles. | Workflow, comments, plugins | Executable code | entry/page review queues, comment actions, plugins | Workflow UI diagram |
| `frontend/src/pages/OrganizationPage.tsx` | Organization admin UI. | Organizations, memberships, SaaS ops | Executable code | members, invitations, domains, rate limits, audit logs, email deliveries, alerts | Organization admin flow |
| `frontend/src/pages/BillingPage.tsx` | Subscription, quota, plan selection, Stripe availability UI. | Billing, quotas, Stripe | Executable code | current plan, usage, plans, checkout/portal | Billing UI flow |
| `frontend/src/pages/BetaPage.tsx` | Beta feedback and GA blocker UI. | Beta, GA readiness | Executable code | feedback, blockers, product dashboard | Beta workflow UI |
| `frontend/src/pages/MarketplacePage.tsx` | Marketplace catalog, creator profile, listing submission, upload, reports, review, moderation, install dialog, Installed Apps, runtime status, permission catalog, and kill-switch UI. | Marketplace | Executable code | catalog/detail, creator form, package upload, review actions, permission approval, lifecycle controls, runtime safety and kill-switch controls | Marketplace UI diagrams |
| `frontend/src/pages/PagesPage.tsx` | Page Builder palette plus installed Component Pack registry and Template preview/import controls. | Page builder, Marketplace runtime adapters | Executable code | component palette, template preview/import, page clone | Page builder and Marketplace runtime adapter diagrams |
| `frontend/src/pages/SettingsPage.tsx` | Webhook management UI. | Webhooks, settings | Executable code | webhook form, event toggles, delivery actions | Webhook UI flow |
| `frontend/src/pages/WorkspaceRedirectPage.tsx` | Workspace slug route bridge. | Organizations | Executable code | workspace slug redirect | Workspace flow |
| `frontend/src/pages/DashboardPage.test.tsx` | Frontend dashboard smoke test. | Frontend tests | Test | foundation cards | Test coverage diagram |
| `frontend/src/pages/PagesPage.test.tsx` | Page builder shell smoke test. | Frontend tests, page builder | Test | builder shell, component, drop area, props editor | Test coverage diagram |
| `frontend/src/test/setup.ts` | Frontend test environment setup. | Frontend tests | Test configuration | testing-library setup | Test pipeline diagram |

## Existing Documentation Sources

| Path | Responsibility | Domain | Kind | Why it matters for diagrams |
| --- | --- | --- | --- | --- |
| `docs/ARCHITECTURE.md` | Current architecture overview. | Architecture | Documentation | Useful for terminology, lower priority than code/schema. |
| `docs/API.md` | Current API documentation for earlier phases and Marketplace lifecycle. | API docs | Documentation | Manual reference for legacy handlers plus Phase-6 routes; generated coverage is partial for older handlers. |
| `docs/I18N.md` | i18n design and extension guidance. | Localization | Documentation | Supports localization diagram after frontend evidence. |
| `docs/sample-data.sql` | Local sample data. | Seed/testing | Documentation/schema data | Shows intended demo state but not runtime behavior. |
| `docs/PHASE_ZERO.md` | Phase zero foundation notes. | Historical phase | Documentation | Historical context only. |
| `docs/PHASE_ONE.md` | Phase one core notes. | Historical phase | Documentation | Historical context for auth/content/media. |
| `docs/PHASE_TWO.md` | Phase two page engine notes. | Historical phase | Documentation | Historical context for page JSON. |
| `docs/PHASE_THREE.md` | Phase three admin UI notes. | Historical phase | Documentation | Mentions planned user management; confirms docs-only gap. |
| `docs/PHASE_FOUR.md` | Visual page builder notes. | Historical phase | Documentation | Context for PagesPage and page builder. |
| `docs/PHASE_FIVE.md` | Delivery/cache/webhook notes. | Historical phase | Documentation | Context for public delivery and webhooks. |
| `docs/PHASE_SIX.md` | Workflow/comments/plugins notes. | Historical phase | Documentation | Notes built-in-only plugin scope. |
| `docs/PHASE_SEVEN.md` | Security hardening notes. | Historical phase | Documentation | Context for security controls. |
| `docs/V2_PHASE_ZERO.md` | V2 evaluation/design notes. | V2 planning | Documentation | Historical SaaS context. |
| `docs/V2_PHASE_ONE.md` | V2 organization migration notes. | Organizations | Documentation | Context for organization model. |
| `docs/V2_PHASE_TWO.md` | V2 organization UI/API notes. | Organizations | Documentation | Context for organization admin. |
| `docs/V2_PHASE_THREE.md` | V2 RLS notes. | RLS | Documentation | Context for tenant isolation. |
| `docs/V2_PHASE_FOUR.md` | V2 tenant middleware/frontend notes. | Tenant middleware | Documentation | Context for tenant request flow. |
| `docs/V2_PHASE_FIVE.md` | V2 billing/quota notes. | Billing, quotas | Documentation | Context for plan and usage diagrams. |
| `docs/V2_PHASE_SIX.md` | V2 Stripe billing notes. | Stripe | Documentation | Context for Stripe flow. |
| `docs/V2_PHASE_SEVEN.md` | V2 SaaS ops notes. | SaaS ops | Documentation | Context for domains/rate/audit/email/alerts. |
| `docs/V2_PHASE_EIGHT.md` | V2 hardening notes. | Security | Documentation | Context for hardening tests. |
| `docs/V2_PHASE_EIGHT_FIXTURE.sql` | Tenant fixture for hardening. | Test data | Documentation/schema data | Useful for tenant isolation test diagram. |
| `docs/V2_PHASE_NINE.md` | Beta release notes. | Beta | Documentation | Context for beta data/API. |
| `docs/V2_PHASE_TEN.md` | GA release notes. | GA readiness | Documentation | Context for GA readiness. |
| `docs/V2_RELEASE_NOTES.md` | Release notes for V2. | Release docs | Documentation | Context only. |
| `docs/V2_MIGRATION_GUIDE.md` | V1 to V2 migration guide. | Operations | Documentation | Operational flow source. |
| `docs/V2_ADMIN_GUIDE.md` | Organization admin guide. | Organizations, operations | Documentation | Operational admin flow source. |
| `docs/V2_BILLING_GUIDE.md` | Billing operations guide. | Billing, Stripe | Documentation | Operational billing flow source. |
| `docs/V2_OPERATIONS_RUNBOOK.md` | V2 operational runbook. | Operations | Documentation | Incident flow source. |
| `docs/V3_PHASE_0_1.md` | Marketplace scope and policy phase. | Marketplace planning | Documentation | Scope lock source. |
| `docs/V3_MARKETPLACE_SCOPE.md` | Marketplace MVP boundaries. | Marketplace planning | Documentation | Defines out-of-scope install/payment areas. |
| `docs/V3_PRODUCT_TAXONOMY.md` | Marketplace product taxonomy. | Marketplace planning | Documentation | Product-type vocabulary. |
| `docs/V3_MARKETPLACE_POLICY.md` | Review and moderation policy. | Marketplace governance | Documentation | Review policy context. |
| `docs/V3_PHASE_0_2.md` | Marketplace V2 dependency audit phase. | Marketplace readiness | Documentation | Dependency context. |
| `docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md` | V2 readiness audit for Marketplace. | Marketplace readiness | Documentation | Explicitly calls install/payment gaps. |
| `docs/V3_MARKETPLACE_V2_DEPENDENCY_MATRIX.md` | Dependency matrix. | Marketplace readiness | Documentation | Future permission/entitlement source. |
| `docs/V3_MARKETPLACE_GAP_LIST.md` | Known Marketplace gaps. | Marketplace readiness | Documentation | Confirms install/payment/payout gaps. |
| `docs/V3_PHASE_ONE.md` | Marketplace domain and migration notes. | Marketplace | Documentation | Context for tables and future purchase note. |
| `docs/V3_MARKETPLACE_DOMAIN_MODEL.md` | Marketplace domain model. | Marketplace | Documentation | Domain concepts, lower priority than migration. |
| `docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md` | Manifest schema contract. | Marketplace packages | Documentation | Validation source paired with service. |
| `docs/V3_PACKAGE_STORAGE.md` | Package storage rules. | Marketplace packages | Documentation | Object-key/checksum/size contract. |
| `docs/V3_PHASE_TWO.md` | Creator onboarding and submission notes. | Marketplace | Documentation | Context for creator/listing/upload UI/API. |
| `docs/V3_PHASE_THREE.md` | Validation/security/compatibility notes. | Marketplace validation | Documentation | Context for validation reports. |
| `docs/V3_PHASE_FOUR.md` | Review and moderation notes. | Marketplace review | Documentation | Context for review event log and moderation. |
| `docs/V3_PHASE_FIVE.md` | Catalog notes. | Marketplace catalog | Documentation | Context for public catalog and deferred install. |
| `docs/V3_PHASE_SIX.md` | Installation lifecycle notes. | Marketplace installations | Documentation | Context for free install, permission approval, lifecycle, update, rollback, audit, and deferred paid/runtime boundaries. |
| `docs/V3_PHASE_SEVEN.md` | Permission, sandbox policy, and kill-switch notes. | Marketplace security runtime | Documentation | Context for permission catalog, bounded host API decisions, runtime blocking, and global/organization emergency controls. |
| `docs/V3_PHASE_EIGHT.md` | Runtime adapter notes. | Marketplace runtime adapters | Documentation | Context for Component Pack, Template, and public Plugin Hook adapter contracts. |
| `docs/V3_PHASE_NINE.md` | Marketplace finance notes. | Marketplace finance | Documentation | Context for pricing, checkout, entitlement, ledger, and payout hardening. |
| `docs/V3_PHASE_TEN.md` | Ratings, reviews, and abuse-management notes. | Marketplace feedback/trust | Documentation | Context for reviews, abuse queue, and internal notification contracts. |
| `docs/V3_PHASE_ELEVEN.md` | Marketplace analytics notes. | Marketplace analytics | Documentation | Context for dashboard metrics and analytics evidence. |
| `docs/V3_PHASE_TWELVE.md` | Creator tooling and sample package notes. | Marketplace creator tooling | Documentation | Context for CLI packaging/validation helpers and sample marketplace assets. |
| `docs/V3_PHASE_THIRTEEN.md` | Marketplace security QA and performance notes. | Marketplace QA/performance | Documentation | Context for abuse-path regression tests, catalog cache policy, index tuning, and load baseline. |
| `docs/V3_PHASE_FOURTEEN.md` | Private Creator Beta and Customer Beta readiness notes. | Marketplace beta readiness | Documentation | Context for beta cohort evidence, creator feedback, install/uninstall/purchase/support/report gates, and read-only readiness script. |
| `docs/V3_PHASE_FIFTEEN.md` | Marketplace Launch Readiness and General Availability notes. | Marketplace launch/GA readiness | Documentation | Context for runbook, final policy, support workflow, rollback, incident checklist, release notes, public docs, monitoring dashboard, and support plan. |
| `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md` | Marketplace operational runbook. | Marketplace launch/GA readiness | Documentation | Support workflow, broken install, malicious product, wrong payment, refund/dispute/payout, abuse report, emergency block, rollback, and monitoring. |
| `docs/V3_MARKETPLACE_RELEASE_NOTES.md` | Marketplace GA release notes. | Marketplace launch/GA readiness | Documentation | Public docs, monitoring dashboard, support plan, known limitations, go/no-go criteria, and approved-product production enablement. |

## Search And Inspection Notes

- Status strings are constrained heavily in SQL migrations for content/page/workflow, billing, beta, and Marketplace.
- Authorization checks are concentrated in `backend/src/services/rbac.rs` and applied by route handlers.
- Organization filters are present in tenant route SQL predicates and RLS-scoped connections.
- RLS setup is migration-driven and runtime context is set by `backend/src/services/rls.rs`.
- Filesystem paths appear in upload handling, package artifact object keys, Docker volumes, and static `/uploads` serving.
- Redis is used for delivery cache, rate limiting, and readiness checks.
- Stripe is implemented for organization subscription billing and one-time Marketplace checkout; automated Marketplace payout transfer execution remains deferred.
- Webhook delivery is performed inline by backend services; no durable background queue was found.
- Marketplace free/paid installation persistence, paid entitlements, and lifecycle runtime exist with tenant RLS; executable package runtime remains deferred.
- Phase 14 Marketplace beta readiness is implemented as read-only evidence over existing beta and Marketplace APIs, not as a new schema or route group.
- Phase 15 Marketplace launch readiness and GA are implemented as operational docs, static contract tests, a GA check script, and diagram evidence over existing Marketplace APIs, not as a new schema or route group.
