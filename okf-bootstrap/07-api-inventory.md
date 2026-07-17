# API Inventory

## Surface Summary

The backend composes an Axum HTTP API with 140 route declarations observed in route source. The Utoipa OpenAPI builder registers 149 annotated handlers, but coverage is partial: several older Marketplace catalog, creator, listing, submission, and review handlers plus sitemap and robots endpoints are not represented by annotations.

The primary prefixes are /api for administrative and tenant operations, /api/v1 for public delivery, /uploads for public filesystem bytes, and /openapi.json for generated schema output. No repository-wide API versioning or deprecation policy is documented.

## Route Groups

| Domain | Representative paths | Boundary | Authorization and tenancy | Main implementation | Documentation and test status |
| --- | --- | --- | --- | --- | --- |
| Runtime | /, /health, /ready, /openapi.json | Public | Health and readiness expose service state without tenant context | routes/mod.rs, main.rs | Current in README/API; readiness and static tests exist |
| Authentication | /api/auth/register, /login, /refresh, /logout, /me | Mixed public/authenticated | Access bearer token; rotating refresh cookie or body token | routes/auth.rs, middleware/auth.rs | Documented; service and security tests are partial |
| Organizations | /api/organizations, /current, members, invitations, domains, rates | Authenticated and tenant | Membership and tenant role checks; some global-admin operations | routes/organizations.rs | Broad manual documentation; limited end-to-end route tests |
| Content | /api/content-types, /api/entries and actions | Tenant | TenantContext, RBAC, quotas, RLS | routes/content.rs | Manual API current; OpenAPI coverage present; validation/workflow tests |
| Media | /api/media, /uploads | Tenant management plus public bytes | Mutations use tenant/RBAC; byte serving is public | routes/media.rs, services/media_processing.rs | Documented; security helper tests; storage boundary needs owner policy |
| Pages and preview | /api/pages, component-registry, /api/preview/ws | Tenant | Tenant/RBAC/RLS; WebSocket accepts organization preview context | routes/pages.rs | Documented; page UI and workflow tests; preview is process-local |
| Comments | /api/comments | Tenant | Tenant/RBAC/RLS | routes/comments.rs | Documented; dedicated route integration coverage is limited |
| Built-in plugins | /api/plugins | Authenticated | User authentication and plugin-specific management rules | routes/plugins.rs | Documented; registry/plugin unit tests |
| Public delivery | /api/v1/content, pages, settings, navigation, sitemap, robots | Public | Uses RLS connection for the active organization, currently the slug default | routes/delivery.rs | Manual documentation mostly current; sitemap/robots absent from OpenAPI |
| CMS webhooks | /api/webhooks and deliveries | Tenant | Tenant/RBAC/RLS; outbound payload signatures | routes/webhooks.rs | Documented; signature and URL tests; no durable retry API |
| Billing | /api/billing/plans, usage, subscription, checkout, portal, stripe/webhook | Mixed tenant/public provider webhook | Tenant/RBAC for customer operations; Stripe signature for provider webhook | routes/billing.rs | OpenAPI and manual docs present; see DCC-09 |
| Beta and GA operations | /api/beta and readiness surfaces | Tenant/global admin by operation | Tenant roles or global-admin gates | routes/beta.rs, readiness services | Phase docs and contract tests; operational sign-off remains owner-owned |
| Marketplace creator and submissions | /api/marketplace/creator, listings, uploads, submissions | Authenticated/global review | Creator ownership and global-admin review gates | routes/marketplace.rs | Manual and phase docs; important older handlers are absent from OpenAPI |
| Marketplace catalog | /api/marketplace/catalog | Tenant-authenticated | Organization plan and compatibility context | marketplace_catalog.rs, routes/marketplace.rs | Catalog docs/tests; not actually anonymous despite public product language |
| Marketplace installations | /api/marketplace/installations | Tenant | Tenant role, RLS, policy, entitlement, and version gates | marketplace_installation.rs | Current docs/tests; diagram 33 is stale |
| Marketplace runtime and adapters | permission catalog, runtime authorize, kill switches, components, templates, hooks | Tenant/global admin by operation | Host-enforced permission policy; no external package code execution | marketplace_runtime.rs, marketplace_adapters.rs | Current phase docs and unit/contract tests |
| Marketplace finance | purchases, checkout, entitlements, ledger, payout account, payouts | Tenant/creator/global admin/provider | Tenant ownership, creator ownership, global review, Stripe signature | routes/marketplace_finance.rs, marketplace_finance.rs | Current phase docs/OpenAPI/tests |
| Marketplace feedback and analytics | reviews, abuse reports, notifications, creator/admin analytics | Tenant/creator/global admin | Ownership and moderation gates by route | marketplace_feedback.rs, marketplace_analytics.rs, route modules | Current phase docs and tests |

## Security Stacks

Three principal route stacks are composed:

1. Public routes include root probes, registration, login, refresh, Stripe webhook reception, public delivery, and uploaded-byte serving.
2. Authenticated routes require valid access-token identity but do not always require a selected organization. Examples include logout, current user, organization list/create, invitation acceptance, built-in plugin access, and some beta administration.
3. Tenant routes require authentication plus organization resolution, membership validation, rate-limit and quota checks, and TenantContext injection.

Global-admin, organization-role, creator-ownership, entitlement, and Marketplace moderation checks are enforced inside middleware or handlers according to the route.

## Authentication and Session Contract

- Passwords are hashed with Argon2.
- Access tokens are HMAC-SHA256 JWT bearer tokens.
- Refresh tokens are stored hashed and rotated.
- The refresh token is supported through an HttpOnly, SameSite=Lax cookie and an optional request body token.
- Browser requests include credentials. The frontend stores access token, user, and organization data in localStorage.
- The frontend API client has no automatic refresh-and-retry interceptor. Session recovery behavior is therefore distributed across UI flows.

## Tenant Context Contract

Tenant middleware reads X-Organization-Id, with a preview-specific query alternative, verifies membership, applies rate and quota controls, and inserts TenantContext. Database helpers set zinhar.organization_id, zinhar.user_id, and bypass settings on PostgreSQL connections before protected queries.

Public delivery does not select an organization from a public domain or route slug. It resolves the active organization with the hard-coded slug default. Whether this is the intended production contract is NEEDS_OWNER_CONFIRMATION.

## Requests, Responses, Errors, and Validation

- JSON request and response DTOs are defined in Rust route or service modules and mirrored manually by TypeScript client types.
- Multipart upload is used for media and Marketplace packages.
- WebSocket is used for page preview updates.
- AppError produces a stable JSON envelope with error and message fields.
- Common status mappings include 400, 401, 403, 404, 409, 422, 429, 500, and 503.
- Validation is distributed across extractors, route checks, dedicated services, SQL constraints, state-transition services, and provider signature verification.
- Pagination, filtering, sorting, and idempotency are domain-specific rather than governed by a single shared API convention.

## API Documentation Coverage

docs/API.md is the most complete human-readable route reference. Generated /openapi.json provides valuable DTO and handler detail but is not a complete inventory. These two sources should not be described as equally stale.

The final OKF API documentation should use the composed router as the route source of truth, generated OpenAPI as a partially automated schema source, and docs/API.md as curated behavioral context. A coverage check should compare router handlers with OpenAPI annotations.

DOCUMENTATION_CODE_CONFLICT DCC-09: one docs/API.md paragraph names /api/billing/webhook, while the composed route is /api/billing/stripe/webhook.

## API Risks and Gaps

| Priority | Finding | Impact | Recommended later action |
| --- | --- | --- | --- |
| High | OpenAPI is partial | Generated clients and reviewers can miss real routes | Add a route-to-OpenAPI coverage check and annotate missing handlers |
| High | Public delivery is fixed to organization slug default | Multi-domain or multi-site behavior is unclear | Resolve owner intent and document routing contract |
| High | Public /uploads access and production storage policy are unresolved | Authorization, privacy, and scaling guarantees cannot be stated | Define public/private asset policy and storage topology |
| Medium | No explicit API versioning/deprecation policy | Future breaking changes lack a governed path | Obtain owner decision and document compatibility rules |
| Medium | Frontend types are manually synchronized | DTO drift can reach runtime | Generate or contract-test shared schemas |
| Medium | No automatic browser refresh interceptor | Token expiry behavior may vary by screen | Document and standardize session recovery |
| Medium | Pagination and idempotency conventions vary | Consumers need domain-specific knowledge | Create a cross-API convention matrix |
| Medium | Background webhook delivery is not durable | Accepted mutations can outlive a failed side effect | Document failure semantics and future queue boundary |

## Recommended Final API Documentation

1. route index grouped by boundary and domain;
2. authentication, tenant selection, and authorization matrices;
3. error envelope and status-code catalog;
4. pagination, filtering, sorting, idempotency, and upload conventions;
5. WebSocket and provider-webhook contracts;
6. router-to-OpenAPI coverage results;
7. versioning and deprecation policy after owner confirmation;
8. explicit documentation of public delivery and asset security boundaries.
