---
okf_document_id: "api-endpoint-catalog"
title: "API Endpoint Catalog"
project: "ZinharCMS"
category: "api"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/mod.rs"
  - "backend/src/routes"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/README.md"
  - "api/route-group-catalog.md"
  - "api/request-contracts.md"
  - "api/response-contracts.md"
  - "api/openapi-consistency.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
  - "REQUEST_CONTRACT_UNCLEAR RCU-01"
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-01"
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# API Endpoint Catalog

## Scope and Identity Rule

This catalog contains every one of the 168 registered handler-method endpoints in the verified route tree. A stable endpoint ID uses `<route-group>.<operation>`, where the operation is the registered Rust handler name converted to kebab case. The ID remains stable unless the semantic operation itself changes; a source refactor should not casually rename it.

Static `/uploads/*` is documented in [System, Health, OpenAPI, and Static Endpoints](endpoints/system-health-openapi-and-static.md) but is excluded from the handler count because it is a `ServeDir` mount. Group and family documents provide detailed authorization, persistence, frontend, testing, and uncertainty context.

## Coverage Summary

- 17 public handler endpoints.
- 12 bearer-authenticated non-tenant endpoints.
- 139 bearer-and-tenant endpoints.
- 149 handlers represented in OpenAPI and 19 missing.
- 141 handlers called by the shared frontend API client and 27 without a direct wrapper.
- No duplicate registered method/path pairs.

## Column Guide

`Input` describes transport extractors and omits internal `State`, `Claims`, and `TenantContext`. `Output / success` preserves the Rust return type and observed explicit success status, defaulting to 200 when the handler does not return an explicit success status. `OpenAPI` reports handler inclusion, not schema completeness. Follow the family link for role, ownership, persistence, side effects, errors, and tests.

## System and Static

[Group contract](groups/system-and-static.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `system-static.root` | **GET** `/` | `backend/src/routes/mod.rs#root` | Public | none | `Json<ApiInfo>`; 200 | Included | `api.info` | [Contract](endpoints/system-health-openapi-and-static.md) |
| `system-static.health` | **GET** `/health` | `backend/src/routes/mod.rs#health` | Public | none | `Json<HealthResponse>`; 200 | Included | `api.health` | [Contract](endpoints/system-health-openapi-and-static.md) |
| `system-static.readiness` | **GET** `/ready` | `backend/src/routes/mod.rs#readiness` | Public | none | `Result<Json<ReadyResponse>, AppError>`; 200 | Included | `api.readiness` | [Contract](endpoints/system-health-openapi-and-static.md) |
| `system-static.openapi` | **GET** `/openapi.json` | `backend/src/routes/mod.rs#openapi` | Public | none | `Json<utoipa::openapi::OpenApi>`; 200 | **Missing** | None | [Contract](endpoints/system-health-openapi-and-static.md) |

## Authentication

[Group contract](groups/authentication.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `authentication.module-status` | **GET** `/api/auth` | `backend/src/routes/auth.rs#module_status` | Public | none | `Json<AuthModuleStatus>`; 200 | Included | None | [Contract](endpoints/authentication-and-session.md) |
| `authentication.register` | **POST** `/api/auth/register` | `backend/src/routes/auth.rs#register` | Public | JSON `RegisterRequest` | `Result<AuthResult, AppError>`; 200 | Included | `api.auth.register` | [Contract](endpoints/authentication-and-session.md) |
| `authentication.login` | **POST** `/api/auth/login` | `backend/src/routes/auth.rs#login` | Public | JSON `LoginRequest`; headers; remote address | `Result<AuthResult, AppError>`; 200 | Included | `api.auth.login` | [Contract](endpoints/authentication-and-session.md) |
| `authentication.refresh` | **POST** `/api/auth/refresh` | `backend/src/routes/auth.rs#refresh` | Public | headers; raw bytes | `Result<AuthResult, AppError>`; 200 | Included | `api.auth.refresh` | [Contract](endpoints/authentication-and-session.md) |
| `authentication.logout` | **POST** `/api/auth/logout` | `backend/src/routes/auth.rs#logout` | Bearer | headers; raw bytes | `Result<(HeaderMap, Json<LogoutResponse>), AppError>`; 200 | Included | `api.auth.logout` | [Contract](endpoints/authentication-and-session.md) |
| `authentication.me` | **GET** `/api/auth/me` | `backend/src/routes/auth.rs#me` | Bearer | none | `Result<Json<MeResponse>, AppError>`; 200 | Included | `api.auth.me` | [Contract](endpoints/authentication-and-session.md) |

## Beta Release

[Group contract](groups/beta-release.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `beta-release.get-beta-dashboard` | **GET** `/api/beta/dashboard` | `backend/src/routes/beta.rs#get_beta_dashboard` | Bearer + tenant | none | `Result<Json<BetaDashboardResponse>, AppError>`; 200 | Included | `api.beta.dashboard` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.list-beta-feedback` | **GET** `/api/beta/feedback` | `backend/src/routes/beta.rs#list_beta_feedback` | Bearer + tenant | query `LimitQuery` | `Result<Json<Vec<BetaFeedbackResponse>>, AppError>`; 200 | Included | `api.beta.feedback` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.create-beta-feedback` | **POST** `/api/beta/feedback` | `backend/src/routes/beta.rs#create_beta_feedback` | Bearer + tenant | JSON `BetaFeedbackRequest` | `Result<Json<BetaFeedbackResponse>, AppError>`; 200 | Included | `api.beta.createFeedback` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.update-beta-feedback` | **PATCH** `/api/beta/feedback/{feedback_id}` | `backend/src/routes/beta.rs#update_beta_feedback` | Bearer + tenant | path `Uuid`; JSON `UpdateBetaFeedbackRequest` | `Result<Json<BetaFeedbackResponse>, AppError>`; 200 | Included | `api.beta.updateFeedback` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.list-ga-blockers` | **GET** `/api/beta/ga-blockers` | `backend/src/routes/beta.rs#list_ga_blockers` | Bearer + tenant | query `LimitQuery` | `Result<Json<Vec<BetaGaBlockerResponse>>, AppError>`; 200 | Included | `api.beta.blockers` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.create-ga-blocker` | **POST** `/api/beta/ga-blockers` | `backend/src/routes/beta.rs#create_ga_blocker` | Bearer + tenant | JSON `BetaGaBlockerRequest` | `Result<Json<BetaGaBlockerResponse>, AppError>`; 200 | Included | `api.beta.createBlocker` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.update-ga-blocker` | **PATCH** `/api/beta/ga-blockers/{blocker_id}` | `backend/src/routes/beta.rs#update_ga_blocker` | Bearer + tenant | path `Uuid`; JSON `UpdateBetaGaBlockerRequest` | `Result<Json<BetaGaBlockerResponse>, AppError>`; 200 | Included | `api.beta.updateBlocker` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.get-product-dashboard` | **GET** `/api/beta/product-dashboard` | `backend/src/routes/beta.rs#get_product_dashboard` | Bearer | none | `Result<Json<BetaProductDashboardResponse>, AppError>`; 200 | Included | `api.beta.productDashboard` | [Contract](endpoints/beta-release-operations.md) |
| `beta-release.upsert-beta-participant` | **PUT** `/api/beta/participants/{organization_id}` | `backend/src/routes/beta.rs#upsert_beta_participant` | Bearer | path `Uuid`; JSON `BetaParticipantRequest` | `Result<Json<BetaParticipantResponse>, AppError>`; 200 | Included | `api.beta.upsertParticipant` | [Contract](endpoints/beta-release-operations.md) |

## Billing and Quota

[Group contract](groups/billing-and-quota.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `billing-quota.list-plans` | **GET** `/api/billing/plans` | `backend/src/routes/billing.rs#list_plans` | Bearer + tenant | none | `Result<Json<Vec<PlanResponse>>, AppError>`; 200 | Included | `api.billing.plans` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.get-subscription` | **GET** `/api/billing/subscription` | `backend/src/routes/billing.rs#get_subscription` | Bearer + tenant | none | `Result<Json<SubscriptionResponse>, AppError>`; 200 | Included | `api.billing.subscription` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.change-subscription-plan` | **PUT** `/api/billing/subscription` | `backend/src/routes/billing.rs#change_subscription_plan` | Bearer + tenant | JSON `ChangePlanRequest` | `Result<Json<SubscriptionResponse>, AppError>`; 200 | Included | `api.billing.changePlan` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.create-checkout-session` | **POST** `/api/billing/checkout` | `backend/src/routes/billing.rs#create_checkout_session` | Bearer + tenant | JSON `CheckoutSessionRequest` | `Result<Json<CheckoutSessionResponse>, AppError>`; 200 | Included | `api.billing.checkout` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.create-customer-portal-session` | **POST** `/api/billing/portal` | `backend/src/routes/billing.rs#create_customer_portal_session` | Bearer + tenant | none | `Result<Json<CustomerPortalResponse>, AppError>`; 200 | Included | `api.billing.portal` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.get-usage` | **GET** `/api/billing/usage` | `backend/src/routes/billing.rs#get_usage` | Bearer + tenant | none | `Result<Json<BillingUsageResponse>, AppError>`; 200 | Included | `api.billing.usage` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.rebuild-usage` | **POST** `/api/billing/usage/rebuild` | `backend/src/routes/billing.rs#rebuild_usage` | Bearer + tenant | none | `Result<Json<BillingUsageResponse>, AppError>`; 200 | Included | `api.billing.rebuildUsage` | [Contract](endpoints/billing-subscription-and-usage.md) |
| `billing-quota.stripe-webhook` | **POST** `/api/billing/stripe/webhook` | `backend/src/routes/billing.rs#stripe_webhook` | Public signed callback | headers; raw bytes | `Result<Json<BillingWebhookResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/billing-subscription-and-usage.md) |

## Editorial Comments

[Group contract](groups/editorial-comments.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `editorial-comments.list-comments` | **GET** `/api/comments` | `backend/src/routes/comments.rs#list_comments` | Bearer + tenant | query `CommentListQuery` | `Result<Json<Vec<CommentResponse>>, AppError>`; 200 | Included | `api.comments.list` | [Contract](endpoints/editorial-comments.md) |
| `editorial-comments.create-comment` | **POST** `/api/comments` | `backend/src/routes/comments.rs#create_comment` | Bearer + tenant | JSON `CommentRequest` | `Result<Json<CommentResponse>, AppError>`; 200 | Included | `api.comments.create` | [Contract](endpoints/editorial-comments.md) |
| `editorial-comments.resolve-comment` | **POST** `/api/comments/{id}/resolve` | `backend/src/routes/comments.rs#resolve_comment` | Bearer + tenant | path `Uuid` | `Result<Json<CommentResponse>, AppError>`; 200 | Included | `api.comments.resolve` | [Contract](endpoints/editorial-comments.md) |
| `editorial-comments.unresolve-comment` | **DELETE** `/api/comments/{id}/resolve` | `backend/src/routes/comments.rs#unresolve_comment` | Bearer + tenant | path `Uuid` | `Result<Json<CommentResponse>, AppError>`; 200 | Included | `api.comments.unresolve` | [Contract](endpoints/editorial-comments.md) |
| `editorial-comments.get-comment` | **GET** `/api/comments/{id}` | `backend/src/routes/comments.rs#get_comment` | Bearer + tenant | path `Uuid` | `Result<Json<CommentResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/editorial-comments.md) |
| `editorial-comments.delete-comment` | **DELETE** `/api/comments/{id}` | `backend/src/routes/comments.rs#delete_comment` | Bearer + tenant | path `Uuid` | `Result<Json<CommentResponse>, AppError>`; 200 | Included | `api.comments.delete` | [Contract](endpoints/editorial-comments.md) |

## Content and Workflow

[Group contract](groups/content-and-workflow.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `content-workflow.list-content-types` | **GET** `/api/content-types` | `backend/src/routes/content.rs#list_content_types` | Bearer + tenant | none | `Result<Json<Vec<ContentTypeResponse>>, AppError>`; 200 | Included | `api.contentTypes.list` | [Contract](endpoints/content-types.md) |
| `content-workflow.create-content-type` | **POST** `/api/content-types` | `backend/src/routes/content.rs#create_content_type` | Bearer + tenant | JSON `ContentTypeRequest` | `Result<Json<ContentTypeResponse>, AppError>`; 200 | Included | `api.contentTypes.create` | [Contract](endpoints/content-types.md) |
| `content-workflow.get-content-type` | **GET** `/api/content-types/{id}` | `backend/src/routes/content.rs#get_content_type` | Bearer + tenant | path `Uuid` | `Result<Json<ContentTypeResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/content-types.md) |
| `content-workflow.update-content-type` | **PUT** `/api/content-types/{id}` | `backend/src/routes/content.rs#update_content_type` | Bearer + tenant | path `Uuid`; JSON `ContentTypeRequest` | `Result<Json<ContentTypeResponse>, AppError>`; 200 | Included | `api.contentTypes.update` | [Contract](endpoints/content-types.md) |
| `content-workflow.delete-content-type` | **DELETE** `/api/content-types/{id}` | `backend/src/routes/content.rs#delete_content_type` | Bearer + tenant | path `Uuid`; query `DeleteConfirm` | `Result<Json<ContentTypeResponse>, AppError>`; 200 | Included | `api.contentTypes.delete` | [Contract](endpoints/content-types.md) |
| `content-workflow.list-entries` | **GET** `/api/entries/{type_slug}` | `backend/src/routes/content.rs#list_entries` | Bearer + tenant | path `String`; query `EntryListQuery` | `Result<Json<EntryListResponse>, AppError>`; 200 | Included | `api.entries.list` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.create-entry` | **POST** `/api/entries/{type_slug}` | `backend/src/routes/content.rs#create_entry` | Bearer + tenant | path `String`; JSON `EntryRequest` | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.create` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.get-entry` | **GET** `/api/entries/{type_slug}/{id}` | `backend/src/routes/content.rs#get_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.update-entry` | **PUT** `/api/entries/{type_slug}/{id}` | `backend/src/routes/content.rs#update_entry` | Bearer + tenant | JSON `EntryRequest` | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.update` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.delete-entry` | **DELETE** `/api/entries/{type_slug}/{id}` | `backend/src/routes/content.rs#delete_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.delete` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.submit-entry-for-review` | **POST** `/api/entries/{type_slug}/{id}/submit-review` | `backend/src/routes/content.rs#submit_entry_for_review` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.submitReview` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.publish-entry` | **POST** `/api/entries/{type_slug}/{id}/publish` | `backend/src/routes/content.rs#publish_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.publish` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.reject-entry` | **POST** `/api/entries/{type_slug}/{id}/reject` | `backend/src/routes/content.rs#reject_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.reject` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.archive-entry` | **POST** `/api/entries/{type_slug}/{id}/archive` | `backend/src/routes/content.rs#archive_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.archive` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.restore-entry` | **POST** `/api/entries/{type_slug}/{id}/restore` | `backend/src/routes/content.rs#restore_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.restore` | [Contract](endpoints/content-entries-and-workflow.md) |
| `content-workflow.unpublish-entry` | **POST** `/api/entries/{type_slug}/{id}/unpublish` | `backend/src/routes/content.rs#unpublish_entry` | Bearer + tenant | none | `Result<Json<ContentEntryResponse>, AppError>`; 200 | Included | `api.entries.unpublish` | [Contract](endpoints/content-entries-and-workflow.md) |

## Public Delivery

[Group contract](groups/public-delivery.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `public-delivery.list-public-entries` | **GET** `/api/v1/content/{type_slug}` | `backend/src/routes/delivery.rs#list_public_entries` | Public | path `String`; query `DeliveryListQuery` | `Result<Json<PublicEntryListResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.get-public-entry` | **GET** `/api/v1/content/{type_slug}/{id_or_slug}` | `backend/src/routes/delivery.rs#get_public_entry` | Public | query `DeliveryDetailQuery` | `Result<Json<PublicEntryResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.list-public-pages` | **GET** `/api/v1/pages` | `backend/src/routes/delivery.rs#list_public_pages` | Public | query `DeliveryListQuery` | `Result<Json<PublicPageListResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.get-public-page` | **GET** `/api/v1/pages/{slug}` | `backend/src/routes/delivery.rs#get_public_page` | Public | path `String` | `Result<Json<PublicPageResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.public-settings` | **GET** `/api/v1/settings/public` | `backend/src/routes/delivery.rs#public_settings` | Public | none | `Result<Json<Value>, AppError>`; 200 | Included | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.public-navigation` | **GET** `/api/v1/navigation` | `backend/src/routes/delivery.rs#public_navigation` | Public | query `NavigationQuery` | `Result<Json<Vec<NavigationItemResponse>>, AppError>`; 200 | Included | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.sitemap` | **GET** `/api/v1/sitemap.xml` | `backend/src/routes/delivery.rs#sitemap` | Public | none | `Result<impl IntoResponse, AppError>`; 200 | **Missing** | None | [Contract](endpoints/public-delivery.md) |
| `public-delivery.robots` | **GET** `/api/v1/robots.txt` | `backend/src/routes/delivery.rs#robots` | Public | none | `Result<impl IntoResponse, AppError>`; 200 | **Missing** | None | [Contract](endpoints/public-delivery.md) |

## Marketplace Core

[Group contract](groups/marketplace-core.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `marketplace-core.list-catalog` | **GET** `/api/marketplace/catalog` | `backend/src/routes/marketplace.rs#list_catalog` | Bearer + tenant | query `CatalogQuery` | `Result<(HeaderMap, Json<Vec<MarketplaceCatalogItemResponse>>), AppError>`; 200 | **Missing** | `api.marketplace.catalog` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.get-catalog-listing` | **GET** `/api/marketplace/catalog/{listing_slug}` | `backend/src/routes/marketplace.rs#get_catalog_listing` | Bearer + tenant | path `String` | `Result<(HeaderMap, Json<MarketplaceCatalogDetailResponse>), AppError>`; 200 | **Missing** | `api.marketplace.catalogDetail` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.list-product-reviews` | **GET** `/api/marketplace/listings/{listing_id}/reviews` | `backend/src/routes/marketplace.rs#list_product_reviews` | Bearer + tenant | path `Uuid` | `Result<Json<Vec<MarketplaceProductReviewListResponse>>, AppError>`; 200 | Included | `api.marketplace.reviews` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.create-product-review` | **POST** `/api/marketplace/listings/{listing_id}/reviews` | `backend/src/routes/marketplace.rs#create_product_review` | Bearer + tenant | path `Uuid`; JSON `MarketplaceProductReviewRequest` | `Result<(StatusCode, Json<MarketplaceProductReviewResponse>), AppError>`; 201 | Included | `api.marketplace.submitReview` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.moderate-product-review` | **PATCH** `/api/marketplace/reviews/{review_id}/moderation` | `backend/src/routes/marketplace.rs#moderate_product_review` | Bearer + tenant | path `Uuid`; JSON `MarketplaceProductReviewModerationRequest` | `Result<Json<MarketplaceProductReviewResponse>, AppError>`; 200 | Included | `api.marketplace.moderateReview` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.list-product-review-queue` | **GET** `/api/marketplace/reviews` | `backend/src/routes/marketplace.rs#list_product_review_queue` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceProductReviewResponse>>, AppError>`; 200 | Included | `api.marketplace.reviewModerationQueue` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.create-abuse-report` | **POST** `/api/marketplace/listings/{listing_id}/reports` | `backend/src/routes/marketplace.rs#create_abuse_report` | Bearer + tenant | path `Uuid`; JSON `MarketplaceAbuseReportRequest` | `Result<(StatusCode, Json<MarketplaceAbuseReportResponse>), AppError>`; 201 | Included | `api.marketplace.submitAbuseReport` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.list-abuse-reports` | **GET** `/api/marketplace/reports` | `backend/src/routes/marketplace.rs#list_abuse_reports` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceAbuseReportResponse>>, AppError>`; 200 | Included | `api.marketplace.abuseReports` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.resolve-abuse-report` | **PATCH** `/api/marketplace/reports/{report_id}` | `backend/src/routes/marketplace.rs#resolve_abuse_report` | Bearer + tenant | path `Uuid`; JSON `MarketplaceAbuseReportResolutionRequest` | `Result<Json<MarketplaceAbuseReportResponse>, AppError>`; 200 | Included | `api.marketplace.resolveAbuseReport` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-core.list-installations` | **GET** `/api/marketplace/installations` | `backend/src/routes/marketplace.rs#list_installations` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceInstallationResponse>>, AppError>`; 200 | Included | `api.marketplace.installations` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.install-marketplace-product` | **POST** `/api/marketplace/installations` | `backend/src/routes/marketplace.rs#install_marketplace_product` | Bearer + tenant | JSON `MarketplaceInstallRequest` | `Result<(StatusCode, Json<MarketplaceInstallationResponse>), AppError>`; 201 | Included | `api.marketplace.install` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.check-installation-updates` | **GET** `/api/marketplace/installations/{installation_id}/updates` | `backend/src/routes/marketplace.rs#check_installation_updates` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceInstallationUpdateResponse>, AppError>`; 200 | Included | `api.marketplace.installationUpdates` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.enable-installation` | **POST** `/api/marketplace/installations/{installation_id}/enable` | `backend/src/routes/marketplace.rs#enable_installation` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceInstallationResponse>, AppError>`; 200 | Included | `api.marketplace.enableInstallation` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.disable-installation` | **POST** `/api/marketplace/installations/{installation_id}/disable` | `backend/src/routes/marketplace.rs#disable_installation` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceInstallationResponse>, AppError>`; 200 | Included | `api.marketplace.disableInstallation` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.uninstall-installation` | **POST** `/api/marketplace/installations/{installation_id}/uninstall` | `backend/src/routes/marketplace.rs#uninstall_installation` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceInstallationResponse>, AppError>`; 200 | Included | `api.marketplace.uninstallInstallation` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.update-installation` | **POST** `/api/marketplace/installations/{installation_id}/update` | `backend/src/routes/marketplace.rs#update_installation` | Bearer + tenant | path `Uuid`; JSON `MarketplaceInstallationUpdateRequest` | `Result<Json<MarketplaceInstallationResponse>, AppError>`; 200 | Included | `api.marketplace.updateInstallation` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.rollback-installation` | **POST** `/api/marketplace/installations/{installation_id}/rollback` | `backend/src/routes/marketplace.rs#rollback_installation` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceInstallationResponse>, AppError>`; 200 | Included | `api.marketplace.rollbackInstallation` | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| `marketplace-core.get-creator` | **GET** `/api/marketplace/creator` | `backend/src/routes/marketplace.rs#get_creator` | Bearer + tenant | none | `Result<Json<CreatorStateResponse>, AppError>`; 200 | **Missing** | `api.marketplace.creator` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.request-creator` | **POST** `/api/marketplace/creator` | `backend/src/routes/marketplace.rs#request_creator` | Bearer + tenant | JSON `CreatorProfileRequest` | `Result<Json<CreatorProfileResponse>, AppError>`; 200 | **Missing** | `api.marketplace.requestCreator` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.update-creator-verification` | **PATCH** `/api/marketplace/creators/{creator_id}/verification` | `backend/src/routes/marketplace.rs#update_creator_verification` | Bearer + tenant | path `Uuid`; JSON `CreatorVerificationRequest` | `Result<Json<CreatorProfileResponse>, AppError>`; 200 | **Missing** | None | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.list-creator-listings` | **GET** `/api/marketplace/listings` | `backend/src/routes/marketplace.rs#list_creator_listings` | Bearer + tenant | none | `Result<Json<Vec<ListingResponse>>, AppError>`; 200 | **Missing** | `api.marketplace.listings` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.create-listing` | **POST** `/api/marketplace/listings` | `backend/src/routes/marketplace.rs#create_listing` | Bearer + tenant | JSON `ListingRequest` | `Result<Json<ListingResponse>, AppError>`; 200 | **Missing** | `api.marketplace.createListing` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.update-listing` | **PUT** `/api/marketplace/listings/{listing_id}` | `backend/src/routes/marketplace.rs#update_listing` | Bearer + tenant | path `Uuid`; JSON `ListingRequest` | `Result<Json<ListingResponse>, AppError>`; 200 | **Missing** | `api.marketplace.updateListing` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.submit-listing` | **POST** `/api/marketplace/listings/{listing_id}/submit` | `backend/src/routes/marketplace.rs#submit_listing` | Bearer + tenant | path `Uuid` | `Result<Json<ListingResponse>, AppError>`; 200 | **Missing** | `api.marketplace.submitListing` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.upload-listing-version` | **POST** `/api/marketplace/listings/{listing_id}/versions/upload` | `backend/src/routes/marketplace.rs#upload_listing_version` | Bearer + tenant | path `Uuid`; multipart | `Result<Json<VersionSubmissionResponse>, AppError>`; 200 | **Missing** | `api.marketplace.uploadVersion` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.list-listing-submissions` | **GET** `/api/marketplace/listings/{listing_id}/submissions` | `backend/src/routes/marketplace.rs#list_listing_submissions` | Bearer + tenant | path `Uuid` | `Result<Json<Vec<MarketplaceValidationReportResponse>>, AppError>`; 200 | **Missing** | `api.marketplace.submissions` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.list-review-queue` | **GET** `/api/marketplace/review/queue` | `backend/src/routes/marketplace.rs#list_review_queue` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceValidationReportResponse>>, AppError>`; 200 | **Missing** | `api.marketplace.reviewQueue` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.list-review-events` | **GET** `/api/marketplace/review/events` | `backend/src/routes/marketplace.rs#list_review_events` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceReviewEventResponse>>, AppError>`; 200 | **Missing** | `api.marketplace.reviewEvents` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.list-review-reports` | **GET** `/api/marketplace/review/reports` | `backend/src/routes/marketplace.rs#list_review_reports` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceValidationReportResponse>>, AppError>`; 200 | **Missing** | `api.marketplace.reviewReports` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.review-submission` | **PATCH** `/api/marketplace/review/submissions/{submission_id}` | `backend/src/routes/marketplace.rs#review_submission` | Bearer + tenant | path `Uuid`; JSON `ReviewDecisionRequest` | `Result<Json<MarketplaceReviewEventResponse>, AppError>`; 200 | **Missing** | `api.marketplace.reviewDecision` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| `marketplace-core.moderate-listing` | **POST** `/api/marketplace/review/listings/{listing_id}/moderation` | `backend/src/routes/marketplace.rs#moderate_listing` | Bearer + tenant | path `Uuid`; JSON `ModerationRequest` | `Result<Json<MarketplaceReviewEventResponse>, AppError>`; 200 | **Missing** | `api.marketplace.moderateListing` | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |

## Marketplace Adapters

[Group contract](groups/marketplace-adapters.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `marketplace-adapters.list-marketplace-components` | **GET** `/api/marketplace/runtime/components` | `backend/src/routes/marketplace_adapters.rs#list_marketplace_components` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceComponentResponse>>, AppError>`; 200 | Included | `api.marketplaceAdapters.components` | [Contract](endpoints/marketplace-host-adapters.md) |
| `marketplace-adapters.preview-template` | **POST** `/api/marketplace/templates/{installation_id}/preview` | `backend/src/routes/marketplace_adapters.rs#preview_template` | Bearer + tenant | path `Uuid`; JSON `TemplateAdapterRequest` | `Result<Json<TemplatePreviewResponse>, AppError>`; 200 | Included | `api.marketplaceAdapters.previewTemplate` | [Contract](endpoints/marketplace-host-adapters.md) |
| `marketplace-adapters.import-template` | **POST** `/api/marketplace/templates/{installation_id}/import` | `backend/src/routes/marketplace_adapters.rs#import_template` | Bearer + tenant | path `Uuid`; JSON `TemplateImportRequest` | `Result<(axum::http::StatusCode, Json<PageResponse>), AppError>`; 201 | Included | `api.marketplaceAdapters.importTemplate` | [Contract](endpoints/marketplace-host-adapters.md) |
| `marketplace-adapters.list-marketplace-hooks` | **GET** `/api/marketplace/hooks` | `backend/src/routes/marketplace_adapters.rs#list_marketplace_hooks` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceHookResponse>>, AppError>`; 200 | Included | `api.marketplaceAdapters.hooks` | [Contract](endpoints/marketplace-host-adapters.md) |
| `marketplace-adapters.authorize-marketplace-hook` | **POST** `/api/marketplace/hooks/{hook_type}/authorize` | `backend/src/routes/marketplace_adapters.rs#authorize_marketplace_hook` | Bearer + tenant | path `String`; JSON `MarketplaceHookAuthorizeRequest` | `Result<Json<MarketplaceHookAuthorizationResponse>, AppError>`; 200 | Included | `api.marketplaceAdapters.authorizeHook` | [Contract](endpoints/marketplace-host-adapters.md) |

## Marketplace Analytics

[Group contract](groups/marketplace-analytics.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `marketplace-analytics.get-creator-analytics` | **GET** `/api/marketplace/creators/{creator_id}/analytics` | `backend/src/routes/marketplace_analytics.rs#get_creator_analytics` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceCreatorAnalyticsResponse>, AppError>`; 200 | Included | `api.marketplace.creatorAnalytics` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| `marketplace-analytics.get-admin-analytics` | **GET** `/api/marketplace/analytics/admin` | `backend/src/routes/marketplace_analytics.rs#get_admin_analytics` | Bearer + tenant | none | `Result<Json<MarketplaceAdminAnalyticsResponse>, AppError>`; 200 | Included | `api.marketplace.adminAnalytics` | [Contract](endpoints/marketplace-feedback-and-analytics.md) |

## Marketplace Finance

[Group contract](groups/marketplace-finance.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `marketplace-finance.list-purchases` | **GET** `/api/marketplace/purchases` | `backend/src/routes/marketplace_finance.rs#list_purchases` | Bearer + tenant | none | `Result<Json<Vec<MarketplacePurchaseResponse>>, AppError>`; 200 | Included | `api.marketplace.purchases` | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.create-checkout` | **POST** `/api/marketplace/purchases/checkout` | `backend/src/routes/marketplace_finance.rs#create_checkout` | Bearer + tenant | JSON `MarketplaceCheckoutRequest` | `Result<(axum::http::StatusCode, Json<MarketplaceCheckoutResponse>), AppError>`; 200/201 | Included | `api.marketplace.checkout` | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.list-revenue-ledger` | **GET** `/api/marketplace/revenue-ledger` | `backend/src/routes/marketplace_finance.rs#list_revenue_ledger` | Bearer + tenant | none | `Result<Json<Vec<MarketplaceRevenueLedgerResponse>>, AppError>`; 200 | Included | None | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.get-payout-account` | **GET** `/api/marketplace/creators/{creator_id}/payout` | `backend/src/routes/marketplace_finance.rs#get_payout_account` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplacePayoutAccountResponse>, AppError>`; 200 | Included | `api.marketplace.payoutAccount` | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.onboard-payout-account` | **POST** `/api/marketplace/creators/{creator_id}/payout` | `backend/src/routes/marketplace_finance.rs#onboard_payout_account` | Bearer + tenant | path `Uuid`; JSON `PayoutOnboardingRequest` | `Result<Json<MarketplacePayoutAccountResponse>, AppError>`; 200 | Included | `api.marketplace.onboardPayout` | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.verify-payout-account` | **POST** `/api/marketplace/creators/{creator_id}/payout/verify` | `backend/src/routes/marketplace_finance.rs#verify_payout_account` | Bearer + tenant | path `Uuid`; JSON `PayoutVerificationRequest` | `Result<Json<MarketplacePayoutAccountResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.get-creator-balance` | **GET** `/api/marketplace/creators/{creator_id}/balance` | `backend/src/routes/marketplace_finance.rs#get_creator_balance` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceCreatorBalanceResponse>, AppError>`; 200 | Included | `api.marketplace.creatorBalance` | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| `marketplace-finance.request-payout` | **POST** `/api/marketplace/creators/{creator_id}/payout/request` | `backend/src/routes/marketplace_finance.rs#request_payout` | Bearer + tenant | path `Uuid` | `Result<(axum::http::StatusCode, Json<MarketplacePayoutResponse>), AppError>`; 201 | Included | `api.marketplace.requestPayout` | [Contract](endpoints/marketplace-commerce-and-payouts.md) |

## Marketplace Runtime

[Group contract](groups/marketplace-runtime.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `marketplace-runtime.list-permissions` | **GET** `/api/marketplace/permissions` | `backend/src/routes/marketplace_runtime.rs#list_permissions` | Bearer + tenant | none | `Result<Json<Vec<MarketplacePermissionResponse>>, AppError>`; 200 | Included | `api.marketplace.permissions` | [Contract](endpoints/marketplace-runtime-security.md) |
| `marketplace-runtime.runtime-status` | **GET** `/api/marketplace/runtime/status` | `backend/src/routes/marketplace_runtime.rs#runtime_status` | Bearer + tenant | none | `Result<Json<MarketplaceRuntimeStatusResponse>, AppError>`; 200 | Included | `api.marketplace.runtimeStatus` | [Contract](endpoints/marketplace-runtime-security.md) |
| `marketplace-runtime.authorize-runtime` | **POST** `/api/marketplace/installations/{installation_id}/runtime/authorize` | `backend/src/routes/marketplace_runtime.rs#authorize_runtime` | Bearer + tenant | path `Uuid`; JSON `MarketplaceRuntimeAuthorizeRequest` | `Result<Json<MarketplaceRuntimeAuthorizationResponse>, AppError>`; 200 | Included | `api.marketplace.authorizeRuntime` | [Contract](endpoints/marketplace-runtime-security.md) |
| `marketplace-runtime.activate-organization-kill-switch` | **POST** `/api/marketplace/kill-switches/organization` | `backend/src/routes/marketplace_runtime.rs#activate_organization_kill_switch` | Bearer + tenant | JSON `MarketplaceKillSwitchRequest` | `Result<Json<MarketplaceKillSwitchResponse>, AppError>`; 200 | Included | `api.marketplace.activateOrganizationKillSwitch` | [Contract](endpoints/marketplace-runtime-security.md) |
| `marketplace-runtime.activate-global-kill-switch` | **POST** `/api/marketplace/kill-switches/global` | `backend/src/routes/marketplace_runtime.rs#activate_global_kill_switch` | Bearer + tenant | JSON `MarketplaceKillSwitchRequest` | `Result<Json<MarketplaceKillSwitchResponse>, AppError>`; 200 | Included | `api.marketplace.activateGlobalKillSwitch` | [Contract](endpoints/marketplace-runtime-security.md) |
| `marketplace-runtime.lift-kill-switch` | **POST** `/api/marketplace/kill-switches/{kill_switch_id}/lift` | `backend/src/routes/marketplace_runtime.rs#lift_kill_switch` | Bearer + tenant | path `Uuid` | `Result<Json<MarketplaceKillSwitchResponse>, AppError>`; 200 | Included | `api.marketplace.liftKillSwitch` | [Contract](endpoints/marketplace-runtime-security.md) |

## Media

[Group contract](groups/media.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `media.list-media` | **GET** `/api/media` | `backend/src/routes/media.rs#list_media` | Bearer + tenant | query `MediaListQuery` | `Result<Json<MediaListResponse>, AppError>`; 200 | Included | `api.media.list` | [Contract](endpoints/media-library.md) |
| `media.upload-media` | **POST** `/api/media/upload` | `backend/src/routes/media.rs#upload_media` | Bearer + tenant | multipart | `Result<Json<MediaDetailResponse>, AppError>`; 200 | Included | `api.media.upload` | [Contract](endpoints/media-library.md) |
| `media.get-media` | **GET** `/api/media/{id}` | `backend/src/routes/media.rs#get_media` | Bearer + tenant | path `Uuid` | `Result<Json<MediaDetailResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/media-library.md) |
| `media.update-media` | **PUT** `/api/media/{id}` | `backend/src/routes/media.rs#update_media` | Bearer + tenant | path `Uuid`; JSON `MediaUpdateRequest` | `Result<Json<MediaDetailResponse>, AppError>`; 200 | Included | `api.media.update` | [Contract](endpoints/media-library.md) |
| `media.delete-media` | **DELETE** `/api/media/{id}` | `backend/src/routes/media.rs#delete_media` | Bearer + tenant | path `Uuid` | `Result<Json<MediaDetailResponse>, AppError>`; 200 | Included | `api.media.delete` | [Contract](endpoints/media-library.md) |

## Organizations and SaaS

[Group contract](groups/organizations-and-saas.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `organizations-saas.list-organizations` | **GET** `/api/organizations` | `backend/src/routes/organizations.rs#list_organizations` | Bearer | none | `Result<Json<Vec<OrganizationMembershipResponse>>, AppError>`; 200 | Included | `api.organizations.list` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.create-organization` | **POST** `/api/organizations` | `backend/src/routes/organizations.rs#create_organization` | Bearer | JSON `CreateOrganizationRequest` | `Result<Json<OrganizationDetailResponse>, AppError>`; 200 | Included | `api.organizations.create` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.accept-invitation` | **POST** `/api/organization-invitations/accept` | `backend/src/routes/organizations.rs#accept_invitation` | Bearer | JSON `AcceptInvitationRequest` | `Result<Json<OrganizationMembershipResponse>, AppError>`; 200 | Included | `api.organizations.acceptInvitation` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.get-current-organization` | **GET** `/api/organizations/current` | `backend/src/routes/organizations.rs#get_current_organization` | Bearer + tenant | none | `Result<Json<OrganizationDetailResponse>, AppError>`; 200 | Included | `api.organizations.current` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.update-current-organization` | **PUT** `/api/organizations/current` | `backend/src/routes/organizations.rs#update_current_organization` | Bearer + tenant | JSON `UpdateOrganizationRequest` | `Result<Json<OrganizationDetailResponse>, AppError>`; 200 | Included | `api.organizations.updateCurrent` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.list-organization-members` | **GET** `/api/organizations/current/members` | `backend/src/routes/organizations.rs#list_organization_members` | Bearer + tenant | none | `Result<Json<Vec<OrganizationMemberResponse>>, AppError>`; 200 | Included | `api.organizations.members` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.update-organization-member` | **PATCH** `/api/organizations/current/members/{user_id}` | `backend/src/routes/organizations.rs#update_organization_member` | Bearer + tenant | path `Uuid`; JSON `UpdateMemberRoleRequest` | `Result<Json<OrganizationMemberResponse>, AppError>`; 200 | Included | `api.organizations.updateMember` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.remove-organization-member` | **DELETE** `/api/organizations/current/members/{user_id}` | `backend/src/routes/organizations.rs#remove_organization_member` | Bearer + tenant | path `Uuid` | `Result<Json<OrganizationMemberResponse>, AppError>`; 200 | Included | `api.organizations.removeMember` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.list-organization-invitations` | **GET** `/api/organizations/current/invitations` | `backend/src/routes/organizations.rs#list_organization_invitations` | Bearer + tenant | none | `Result<Json<Vec<OrganizationInvitationResponse>>, AppError>`; 200 | Included | `api.organizations.invitations` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.create-organization-invitation` | **POST** `/api/organizations/current/invitations` | `backend/src/routes/organizations.rs#create_organization_invitation` | Bearer + tenant | JSON `InviteMemberRequest` | `Result<Json<CreatedInvitationResponse>, AppError>`; 200 | Included | `api.organizations.invite` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.revoke-organization-invitation` | **DELETE** `/api/organizations/current/invitations/{invitation_id}` | `backend/src/routes/organizations.rs#revoke_organization_invitation` | Bearer + tenant | path `Uuid` | `Result<Json<OrganizationInvitationResponse>, AppError>`; 200 | Included | `api.organizations.revokeInvitation` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.get-workspace-access` | **GET** `/api/organizations/current/workspace` | `backend/src/routes/organizations.rs#get_workspace_access` | Bearer + tenant | none | `Result<Json<OrganizationWorkspaceResponse>, AppError>`; 200 | Included | `api.organizations.workspace` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.list-organization-domains` | **GET** `/api/organizations/current/domains` | `backend/src/routes/organizations.rs#list_organization_domains` | Bearer + tenant | none | `Result<Json<Vec<OrganizationDomainResponse>>, AppError>`; 200 | Included | `api.organizations.domains` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.create-organization-domain` | **POST** `/api/organizations/current/domains` | `backend/src/routes/organizations.rs#create_organization_domain` | Bearer + tenant | JSON `OrganizationDomainRequest` | `Result<Json<OrganizationDomainResponse>, AppError>`; 200 | Included | `api.organizations.createDomain` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.delete-organization-domain` | **DELETE** `/api/organizations/current/domains/{domain_id}` | `backend/src/routes/organizations.rs#delete_organization_domain` | Bearer + tenant | path `Uuid` | `Result<Json<OrganizationDomainResponse>, AppError>`; 200 | Included | `api.organizations.deleteDomain` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.get-rate-limit` | **GET** `/api/organizations/current/rate-limit` | `backend/src/routes/organizations.rs#get_rate_limit` | Bearer + tenant | none | `Result<Json<RateLimitResponse>, AppError>`; 200 | Included | `api.organizations.rateLimit` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.update-rate-limit` | **PUT** `/api/organizations/current/rate-limit` | `backend/src/routes/organizations.rs#update_rate_limit` | Bearer + tenant | JSON `UpdateRateLimitRequest` | `Result<Json<RateLimitResponse>, AppError>`; 200 | Included | `api.organizations.updateRateLimit` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.list-audit-logs` | **GET** `/api/organizations/current/audit-logs` | `backend/src/routes/organizations.rs#list_audit_logs` | Bearer + tenant | query `LimitQuery` | `Result<Json<Vec<AuditLogResponse>>, AppError>`; 200 | Included | `api.organizations.auditLogs` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.list-email-deliveries` | **GET** `/api/organizations/current/email-deliveries` | `backend/src/routes/organizations.rs#list_email_deliveries` | Bearer + tenant | query `LimitQuery` | `Result<Json<Vec<EmailDeliveryResponse>>, AppError>`; 200 | Included | `api.organizations.emailDeliveries` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.list-saas-alert-rules` | **GET** `/api/organizations/current/alerts` | `backend/src/routes/organizations.rs#list_saas_alert_rules` | Bearer + tenant | none | `Result<Json<Vec<SaasAlertRuleResponse>>, AppError>`; 200 | Included | `api.organizations.alerts` | [Contract](endpoints/saas-operations.md) |
| `organizations-saas.leave-organization` | **POST** `/api/organizations/current/leave` | `backend/src/routes/organizations.rs#leave_organization` | Bearer + tenant | none | `Result<Json<OrganizationMemberResponse>, AppError>`; 200 | Included | `api.organizations.leave` | [Contract](endpoints/organizations-membership-and-invitations.md) |
| `organizations-saas.transfer-organization-ownership` | **POST** `/api/organizations/current/transfer-ownership` | `backend/src/routes/organizations.rs#transfer_organization_ownership` | Bearer + tenant | JSON `TransferOwnershipRequest` | `Result<Json<OrganizationMemberResponse>, AppError>`; 200 | Included | `api.organizations.transferOwnership` | [Contract](endpoints/organizations-membership-and-invitations.md) |

## Pages, Components, and Preview

[Group contract](groups/pages-components-and-preview.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `pages-components-preview.list-pages` | **GET** `/api/pages` | `backend/src/routes/pages.rs#list_pages` | Bearer + tenant | query `PageListQuery` | `Result<Json<PageListResponse>, AppError>`; 200 | Included | `api.pages.list` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.create-page` | **POST** `/api/pages` | `backend/src/routes/pages.rs#create_page` | Bearer + tenant | JSON `PageRequest` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.create` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.get-page-by-slug` | **GET** `/api/pages/slug/{slug}` | `backend/src/routes/pages.rs#get_page_by_slug` | Bearer + tenant | path `String` | `Result<Json<PageResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.get-page` | **GET** `/api/pages/{id}` | `backend/src/routes/pages.rs#get_page` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.update-page` | **PUT** `/api/pages/{id}` | `backend/src/routes/pages.rs#update_page` | Bearer + tenant | path `Uuid`; JSON `PageRequest` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.update` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.delete-page` | **DELETE** `/api/pages/{id}` | `backend/src/routes/pages.rs#delete_page` | Bearer + tenant | path `Uuid`; query `DeleteConfirm` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.delete` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.submit-page-for-review` | **POST** `/api/pages/{id}/submit-review` | `backend/src/routes/pages.rs#submit_page_for_review` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.submitReview` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.publish-page` | **POST** `/api/pages/{id}/publish` | `backend/src/routes/pages.rs#publish_page` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.publish` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.reject-page` | **POST** `/api/pages/{id}/reject` | `backend/src/routes/pages.rs#reject_page` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.reject` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.archive-page` | **POST** `/api/pages/{id}/archive` | `backend/src/routes/pages.rs#archive_page` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.archive` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.restore-page` | **POST** `/api/pages/{id}/restore` | `backend/src/routes/pages.rs#restore_page` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.restoreStatus` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.unpublish-page` | **POST** `/api/pages/{id}/unpublish` | `backend/src/routes/pages.rs#unpublish_page` | Bearer + tenant | path `Uuid` | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.unpublish` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.list-page-versions` | **GET** `/api/pages/{id}/versions` | `backend/src/routes/pages.rs#list_page_versions` | Bearer + tenant | path `Uuid` | `Result<Json<Vec<PageVersionResponse>>, AppError>`; 200 | Included | `api.pages.versions` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.restore-page-version` | **POST** `/api/pages/{id}/versions/{version}/restore` | `backend/src/routes/pages.rs#restore_page_version` | Bearer + tenant | none | `Result<Json<PageResponse>, AppError>`; 200 | Included | `api.pages.restore` | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| `pages-components-preview.list-components` | **GET** `/api/component-registry` | `backend/src/routes/pages.rs#list_components` | Bearer + tenant | query `ComponentListQuery` | `Result<Json<Vec<ComponentRegistryResponse>>, AppError>`; 200 | Included | `api.components.list` | [Contract](endpoints/component-registry.md) |
| `pages-components-preview.create-component` | **POST** `/api/component-registry` | `backend/src/routes/pages.rs#create_component` | Bearer + tenant | JSON `ComponentRegistryRequest` | `Result<Json<ComponentRegistryResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/component-registry.md) |
| `pages-components-preview.get-component` | **GET** `/api/component-registry/{component_key}` | `backend/src/routes/pages.rs#get_component` | Bearer + tenant | path `String` | `Result<Json<ComponentRegistryResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/component-registry.md) |
| `pages-components-preview.update-component` | **PUT** `/api/component-registry/{component_key}` | `backend/src/routes/pages.rs#update_component` | Bearer + tenant | path `String`; JSON `ComponentRegistryRequest` | `Result<Json<ComponentRegistryResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/component-registry.md) |
| `pages-components-preview.delete-component` | **DELETE** `/api/component-registry/{component_key}` | `backend/src/routes/pages.rs#delete_component` | Bearer + tenant | path `String`; query `DeleteConfirm` | `Result<Json<ComponentRegistryResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/component-registry.md) |
| `pages-components-preview.preview-page` | **GET** `/api/preview/{page_id}` | `backend/src/routes/pages.rs#preview_page` | Bearer + tenant | path `Uuid`; WebSocket upgrade | `Response`; 200 | Included | None | [Contract](endpoints/pages-workflow-versions-and-preview.md) |

## Built-In Plugins

[Group contract](groups/built-in-plugins.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `built-in-plugins.list-plugins` | **GET** `/api/plugins` | `backend/src/routes/plugins.rs#list_plugins` | Bearer | none | `Result<Json<Vec<PluginResponse>>, AppError>`; 200 | Included | `api.plugins.list` | [Contract](endpoints/built-in-plugins.md) |
| `built-in-plugins.get-plugin` | **GET** `/api/plugins/{plugin_key}` | `backend/src/routes/plugins.rs#get_plugin` | Bearer | path `String` | `Result<Json<PluginResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/built-in-plugins.md) |
| `built-in-plugins.update-plugin` | **PUT** `/api/plugins/{plugin_key}` | `backend/src/routes/plugins.rs#update_plugin` | Bearer | path `String`; JSON `PluginUpdateRequest` | `Result<Json<PluginResponse>, AppError>`; 200 | Included | `api.plugins.update` | [Contract](endpoints/built-in-plugins.md) |
| `built-in-plugins.enable-plugin` | **POST** `/api/plugins/{plugin_key}/enable` | `backend/src/routes/plugins.rs#enable_plugin` | Bearer | path `String` | `Result<Json<PluginResponse>, AppError>`; 200 | Included | `api.plugins.enable` | [Contract](endpoints/built-in-plugins.md) |
| `built-in-plugins.disable-plugin` | **POST** `/api/plugins/{plugin_key}/disable` | `backend/src/routes/plugins.rs#disable_plugin` | Bearer | path `String` | `Result<Json<PluginResponse>, AppError>`; 200 | Included | `api.plugins.disable` | [Contract](endpoints/built-in-plugins.md) |

## CMS Webhooks

[Group contract](groups/cms-webhooks.md)

| Endpoint ID | Method and path | Handler source | Access | Input | Output / success | OpenAPI | Frontend | Family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `cms-webhooks.list-webhooks` | **GET** `/api/webhooks` | `backend/src/routes/webhooks.rs#list_webhooks` | Bearer + tenant | none | `Result<Json<Vec<WebhookResponse>>, AppError>`; 200 | Included | `api.webhooks.list` | [Contract](endpoints/cms-webhooks.md) |
| `cms-webhooks.create-webhook` | **POST** `/api/webhooks` | `backend/src/routes/webhooks.rs#create_webhook` | Bearer + tenant | JSON `WebhookRequest` | `Result<Json<WebhookResponse>, AppError>`; 200 | Included | `api.webhooks.create` | [Contract](endpoints/cms-webhooks.md) |
| `cms-webhooks.get-webhook` | **GET** `/api/webhooks/{id}` | `backend/src/routes/webhooks.rs#get_webhook` | Bearer + tenant | path `Uuid` | `Result<Json<WebhookResponse>, AppError>`; 200 | Included | None | [Contract](endpoints/cms-webhooks.md) |
| `cms-webhooks.update-webhook` | **PUT** `/api/webhooks/{id}` | `backend/src/routes/webhooks.rs#update_webhook` | Bearer + tenant | path `Uuid`; JSON `WebhookRequest` | `Result<Json<WebhookResponse>, AppError>`; 200 | Included | `api.webhooks.update` | [Contract](endpoints/cms-webhooks.md) |
| `cms-webhooks.delete-webhook` | **DELETE** `/api/webhooks/{id}` | `backend/src/routes/webhooks.rs#delete_webhook` | Bearer + tenant | path `Uuid`; query `DeleteConfirm` | `Result<Json<WebhookResponse>, AppError>`; 200 | Included | `api.webhooks.delete` | [Contract](endpoints/cms-webhooks.md) |
| `cms-webhooks.list-deliveries` | **GET** `/api/webhooks/{id}/deliveries` | `backend/src/routes/webhooks.rs#list_deliveries` | Bearer + tenant | path `Uuid`; query `DeliveryListQuery` | `Result<Json<Vec<WebhookDeliveryResponse>>, AppError>`; 200 | Included | `api.webhooks.deliveries` | [Contract](endpoints/cms-webhooks.md) |
| `cms-webhooks.test-webhook` | **POST** `/api/webhooks/{id}/test` | `backend/src/routes/webhooks.rs#test_webhook` | Bearer + tenant | path `Uuid` | `Result<Json<WebhookTestResponse>, AppError>`; 200 | Included | `api.webhooks.test` | [Contract](endpoints/cms-webhooks.md) |

## Endpoint Family Index

| Family | Endpoint count | Document |
| --- | ---: | --- |
| Authentication And Session | 6 | [Contract](endpoints/authentication-and-session.md) |
| Beta Release Operations | 9 | [Contract](endpoints/beta-release-operations.md) |
| Billing Subscription And Usage | 8 | [Contract](endpoints/billing-subscription-and-usage.md) |
| Built In Plugins | 5 | [Contract](endpoints/built-in-plugins.md) |
| Cms Webhooks | 7 | [Contract](endpoints/cms-webhooks.md) |
| Component Registry | 5 | [Contract](endpoints/component-registry.md) |
| Content Entries And Workflow | 11 | [Contract](endpoints/content-entries-and-workflow.md) |
| Content Types | 5 | [Contract](endpoints/content-types.md) |
| Editorial Comments | 6 | [Contract](endpoints/editorial-comments.md) |
| Marketplace Commerce And Payouts | 8 | [Contract](endpoints/marketplace-commerce-and-payouts.md) |
| Marketplace Creator Catalog And Review | 16 | [Contract](endpoints/marketplace-creator-catalog-and-review.md) |
| Marketplace Feedback And Analytics | 9 | [Contract](endpoints/marketplace-feedback-and-analytics.md) |
| Marketplace Host Adapters | 5 | [Contract](endpoints/marketplace-host-adapters.md) |
| Marketplace Installation Lifecycle | 8 | [Contract](endpoints/marketplace-installation-lifecycle.md) |
| Marketplace Runtime Security | 6 | [Contract](endpoints/marketplace-runtime-security.md) |
| Media Library | 5 | [Contract](endpoints/media-library.md) |
| Organizations Membership And Invitations | 13 | [Contract](endpoints/organizations-membership-and-invitations.md) |
| Pages Workflow Versions And Preview | 15 | [Contract](endpoints/pages-workflow-versions-and-preview.md) |
| Public Delivery | 8 | [Contract](endpoints/public-delivery.md) |
| Saas Operations | 9 | [Contract](endpoints/saas-operations.md) |
| System Health Openapi And Static | 4 | [Contract](endpoints/system-health-openapi-and-static.md) |

## Maintenance

Regenerate or manually compare this inventory whenever route registration changes. Then review the owning family and group, DTOs, middleware zone, RBAC/ownership checks, RLS context, frontend wrapper/type, OpenAPI annotation, tests, and database side effects. Never infer reachability solely from a handler or Utoipa annotation.
