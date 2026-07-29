pub mod auth;
pub mod beta;
pub mod billing;
pub mod comments;
pub mod content;
pub mod delivery;
pub mod marketplace;
pub mod marketplace_adapters;
pub mod marketplace_analytics;
pub mod marketplace_finance;
pub mod marketplace_runtime;
pub mod media;
pub mod organizations;
pub mod pages;
pub mod plugins;
pub mod webhooks;

use axum::extract::DefaultBodyLimit;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use redis::AsyncCommands;
use serde::Serialize;
use sqlx::Executor;
use tower_http::services::ServeDir;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::auth_middleware;
use crate::middleware::tenant::tenant_middleware;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let upload_limit = state.config.max_upload_size.saturating_add(1_048_576) as usize;
    let uploads = Router::new()
        .nest_service("/uploads", ServeDir::new(state.config.upload_dir.clone()))
        .route_layer(middleware::from_fn(restrict_public_uploads));
    let protected = Router::new()
        .merge(auth::protected_router())
        .merge(beta::protected_router())
        .merge(organizations::protected_router())
        .merge(plugins::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let tenant_protected = Router::new()
        .merge(content::router())
        .merge(beta::router())
        .merge(billing::router())
        .merge(media::router())
        .merge(marketplace::router())
        .merge(marketplace_adapters::router())
        .merge(marketplace_analytics::router())
        .merge(marketplace_finance::router())
        .merge(marketplace_runtime::router())
        .merge(organizations::tenant_router())
        .merge(pages::router())
        .merge(comments::router())
        .merge(webhooks::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            tenant_middleware,
        ))
        .layer(DefaultBodyLimit::max(upload_limit));

    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .route("/openapi.json", get(openapi))
        .merge(auth::public_router())
        .merge(billing::public_router())
        .merge(delivery::router())
        .merge(pages::preview_router())
        .merge(protected)
        .merge(tenant_protected)
        .merge(uploads)
        .with_state(state)
}

async fn restrict_public_uploads(request: Request, next: Next) -> Response {
    if is_public_media_path(request.uri().path()) {
        next.run(request).await
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

fn is_public_media_path(path: &str) -> bool {
    let Some(relative) = path.strip_prefix("/uploads/") else {
        return false;
    };
    if relative
        .chars()
        .any(|character| matches!(character, '%' | '\\' | ':'))
    {
        return false;
    }

    let segments = relative.split('/').collect::<Vec<_>>();
    let (organization_id, filename) = match segments.as_slice() {
        [organization_id, filename] => (*organization_id, *filename),
        [organization_id, "variants", filename] => (*organization_id, *filename),
        _ => return false,
    };

    Uuid::parse_str(organization_id).is_ok() && is_safe_public_filename(filename)
}

fn is_safe_public_filename(filename: &str) -> bool {
    !filename.is_empty()
        && filename.len() <= 255
        && filename != "."
        && filename != ".."
        && filename.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_')
        })
}

#[derive(OpenApi)]
#[openapi(
    paths(
        root,
        health,
        readiness,
        auth::module_status,
        auth::register,
        auth::login,
        auth::complete_mfa_login,
        auth::refresh,
        auth::logout,
        auth::me,
        auth::list_sessions,
        auth::revoke_session,
        auth::logout_all,
        auth::privileged_revoke_sessions,
        auth::mfa_status,
        auth::start_mfa_enrollment,
        auth::confirm_mfa_enrollment,
        auth::regenerate_mfa_recovery_codes,
        auth::disable_mfa,
        auth::create_step_up_challenge,
        auth::complete_step_up,
        billing::list_plans,
        billing::get_subscription,
        billing::change_subscription_plan,
        billing::create_checkout_session,
        billing::create_customer_portal_session,
        billing::stripe_webhook,
        billing::get_usage,
        billing::rebuild_usage,
        beta::get_beta_dashboard,
        beta::list_beta_feedback,
        beta::create_beta_feedback,
        beta::update_beta_feedback,
        beta::list_ga_blockers,
        beta::create_ga_blocker,
        beta::update_ga_blocker,
        beta::get_product_dashboard,
        beta::upsert_beta_participant,
        organizations::list_organizations,
        organizations::create_organization,
        organizations::get_current_organization,
        organizations::update_current_organization,
        organizations::list_organization_members,
        organizations::update_organization_member,
        organizations::remove_organization_member,
        organizations::list_organization_invitations,
        organizations::create_organization_invitation,
        organizations::revoke_organization_invitation,
        organizations::accept_invitation,
        organizations::get_workspace_access,
        organizations::list_organization_domains,
        organizations::create_organization_domain,
        organizations::delete_organization_domain,
        organizations::get_rate_limit,
        organizations::update_rate_limit,
        organizations::list_audit_logs,
        organizations::list_email_deliveries,
        organizations::list_saas_alert_rules,
        organizations::leave_organization,
        organizations::transfer_organization_ownership,
        content::list_content_types,
        content::create_content_type,
        content::get_content_type,
        content::update_content_type,
        content::delete_content_type,
        content::list_entries,
        content::create_entry,
        content::get_entry,
        content::update_entry,
        content::delete_entry,
        content::submit_entry_for_review,
        content::publish_entry,
        content::unpublish_entry,
        content::reject_entry,
        content::archive_entry,
        content::restore_entry,
        media::list_media,
        media::upload_media,
        media::get_media,
        media::update_media,
        media::delete_media,
        pages::list_pages,
        pages::create_page,
        pages::get_page,
        pages::get_page_by_slug,
        pages::update_page,
        pages::delete_page,
        pages::submit_page_for_review,
        pages::publish_page,
        pages::unpublish_page,
        pages::reject_page,
        pages::archive_page,
        pages::restore_page,
        pages::list_page_versions,
        pages::restore_page_version,
        pages::list_components,
        pages::create_component,
        pages::get_component,
        pages::update_component,
        pages::delete_component,
        pages::issue_preview_ticket,
        pages::preview_page,
        delivery::list_public_entries,
        delivery::get_public_entry,
        delivery::list_public_pages,
        delivery::get_public_page,
        delivery::public_settings,
        delivery::public_navigation,
        webhooks::list_webhooks,
        webhooks::create_webhook,
        webhooks::get_webhook,
        webhooks::update_webhook,
        webhooks::delete_webhook,
        webhooks::list_deliveries,
        webhooks::test_webhook,
        comments::list_comments,
        comments::create_comment,
        comments::get_comment,
        comments::resolve_comment,
        comments::unresolve_comment,
        comments::delete_comment,
        marketplace::list_installations,
        marketplace::list_product_reviews,
        marketplace::list_product_review_queue,
        marketplace::create_product_review,
        marketplace::moderate_product_review,
        marketplace::create_abuse_report,
        marketplace::list_abuse_reports,
        marketplace::resolve_abuse_report,
        marketplace::install_marketplace_product,
        marketplace::check_installation_updates,
        marketplace::enable_installation,
        marketplace::disable_installation,
        marketplace::uninstall_installation,
        marketplace::update_installation,
        marketplace::rollback_installation,
        marketplace_runtime::list_permissions,
        marketplace_runtime::runtime_status,
        marketplace_runtime::authorize_runtime,
        marketplace_runtime::activate_organization_kill_switch,
        marketplace_runtime::activate_global_kill_switch,
        marketplace_runtime::lift_kill_switch,
        marketplace_adapters::list_marketplace_components,
        marketplace_adapters::preview_template,
        marketplace_adapters::import_template,
        marketplace_adapters::list_marketplace_hooks,
        marketplace_adapters::authorize_marketplace_hook,
        marketplace_finance::list_purchases,
        marketplace_finance::create_checkout,
        marketplace_finance::list_revenue_ledger,
        marketplace_finance::get_payout_account,
        marketplace_finance::onboard_payout_account,
        marketplace_finance::verify_payout_account,
        marketplace_finance::get_creator_balance,
        marketplace_finance::request_payout,
        marketplace_analytics::get_creator_analytics,
        marketplace_analytics::get_admin_analytics,
        plugins::list_plugins,
        plugins::get_plugin,
        plugins::update_plugin,
        plugins::enable_plugin,
        plugins::disable_plugin
    ),
    components(schemas(
        ApiInfo,
        HealthResponse,
        ReadyResponse,
        DependencyCheck,
        auth::AuthModuleStatus,
        auth::RegisterRequest,
        auth::LoginRequest,
        auth::MfaLoginRequiredResponse,
        auth::MfaVerificationRequest,
        auth::MfaProofKindRequest,
        auth::PasswordConfirmationRequest,
        auth::MfaEnrollmentConfirmationRequest,
        auth::MfaEnrollmentResponse,
        auth::RecoveryCodesResponse,
        auth::MfaStatusResponse,
        auth::StepUpChallengeRequest,
        auth::StepUpChallengeResponse,
        auth::StepUpVerificationRequest,
        auth::StepUpGrantResponse,
        auth::MfaDisabledResponse,
        crate::services::mfa_challenges::StepUpScope,
        auth::LogoutResponse,
        auth::AuthResponse,
        auth::AuthUser,
        auth::MeResponse,
        auth::OrganizationMembershipResponse,
        crate::services::sessions::SessionSummary,
        crate::services::sessions::SessionPage,
        crate::services::sessions::RevokeSessionResult,
        crate::services::sessions::LogoutAllResult,
        billing::BillingUsageResponse,
        billing::BillingWebhookResponse,
        billing::ChangePlanRequest,
        billing::CheckoutSessionRequest,
        billing::CheckoutSessionResponse,
        billing::CustomerPortalResponse,
        billing::PlanResponse,
        billing::SubscriptionResponse,
        billing::UsageMetricResponse,
        beta::BetaDashboardResponse,
        beta::BetaFeedbackRequest,
        beta::BetaFeedbackResponse,
        beta::BetaGaBlockerRequest,
        beta::BetaGaBlockerResponse,
        beta::BetaOrganizationDashboardResponse,
        beta::BetaParticipantRequest,
        beta::BetaParticipantResponse,
        beta::BetaProductDashboardResponse,
        beta::BetaProductTotalsResponse,
        beta::UpdateBetaFeedbackRequest,
        beta::UpdateBetaGaBlockerRequest,
        organizations::AcceptInvitationRequest,
        organizations::CreateOrganizationRequest,
        organizations::InviteMemberRequest,
        organizations::AuditLogResponse,
        organizations::EmailDeliveryResponse,
        organizations::OrganizationDomainRequest,
        organizations::OrganizationDomainResponse,
        organizations::OrganizationWorkspaceResponse,
        organizations::RateLimitResponse,
        organizations::SaasAlertRuleResponse,
        organizations::UpdateRateLimitRequest,
        organizations::OrganizationDetailResponse,
        organizations::OrganizationInvitationResponse,
        organizations::OrganizationMemberResponse,
        organizations::OrganizationResponse,
        organizations::PlanLimitResponse,
        organizations::TransferOwnershipRequest,
        organizations::UpdateMemberRoleRequest,
        organizations::UpdateOrganizationRequest,
        crate::middleware::tenant::TenantContext,
        content::ContentTypeRequest,
        content::ContentTypeResponse,
        content::EntryRequest,
        content::ContentEntryResponse,
        content::EntryListResponse,
        media::MediaUpdateRequest,
        media::MediaResponse,
        media::MediaVariantResponse,
        media::MediaDetailResponse,
        media::MediaListResponse,
        pages::PageRequest,
        pages::PageResponse,
        pages::PageListResponse,
        pages::PageVersionResponse,
        pages::ComponentRegistryRequest,
        pages::ComponentRegistryResponse,
        pages::PreviewTicketResponse,
        delivery::PublicEntryResponse,
        delivery::PublicEntryListResponse,
        delivery::PublicPageResponse,
        delivery::PublicPageListResponse,
        delivery::NavigationItemResponse,
        webhooks::WebhookRequest,
        webhooks::WebhookResponse,
        webhooks::WebhookDeliveryResponse,
        webhooks::WebhookTestResponse,
        comments::CommentRequest,
        comments::CommentResponse,
        marketplace::MarketplaceInstallRequest,
        marketplace::MarketplaceProductReviewRequest,
        marketplace::MarketplaceProductReviewModerationRequest,
        marketplace::MarketplaceProductReviewResponse,
        marketplace::MarketplaceProductReviewListResponse,
        marketplace::MarketplaceAbuseReportRequest,
        marketplace::MarketplaceAbuseReportResolutionRequest,
        marketplace::MarketplaceAbuseReportResponse,
        marketplace::MarketplaceInstallationUpdateRequest,
        marketplace::MarketplaceInstallationResponse,
        marketplace::MarketplaceInstallationUpdateResponse,
        marketplace_runtime::MarketplacePermissionResponse,
        marketplace_runtime::MarketplaceKillSwitchResponse,
        marketplace_runtime::MarketplaceRuntimeStatusResponse,
        marketplace_runtime::MarketplaceKillSwitchRequest,
        marketplace_runtime::MarketplaceRuntimeAuthorizeRequest,
        marketplace_runtime::MarketplaceRuntimeAuthorizationResponse,
        marketplace_adapters::MarketplaceComponentResponse,
        marketplace_adapters::TemplateAdapterRequest,
        marketplace_adapters::TemplatePreviewResponse,
        marketplace_adapters::TemplateImportRequest,
        marketplace_adapters::MarketplaceHookResponse,
        marketplace_adapters::MarketplaceHookAuthorizeRequest,
        marketplace_adapters::MarketplaceHookAuthorizationResponse,
        marketplace_finance::MarketplaceCheckoutRequest,
        marketplace_finance::MarketplacePurchaseResponse,
        marketplace_finance::MarketplaceCheckoutResponse,
        marketplace_finance::PayoutOnboardingRequest,
        marketplace_finance::MarketplacePayoutAccountResponse,
        marketplace_finance::MarketplaceRevenueLedgerResponse,
        marketplace_finance::PayoutVerificationRequest,
        marketplace_finance::MarketplaceCreatorBalanceResponse,
        marketplace_finance::MarketplacePayoutResponse,
        marketplace_analytics::MarketplaceCreatorAnalyticsResponse,
        marketplace_analytics::MarketplaceCreatorProductAnalyticsResponse,
        marketplace_analytics::MarketplaceAdminAnalyticsResponse,
        marketplace_analytics::MarketplaceAdminRiskProductResponse,
        plugins::PluginUpdateRequest,
        plugins::PluginResponse
    )),
    tags(
        (name = "system", description = "Phase-zero system endpoints"),
        (name = "auth", description = "Authentication and token management"),
        (name = "billing", description = "Plans, subscriptions, and usage quotas"),
        (name = "beta", description = "Beta release feedback, dashboards, and GA readiness"),
        (name = "content", description = "Content type management"),
        (name = "entries", description = "Content entry management"),
        (name = "media", description = "Media library"),
        (name = "pages", description = "Visual page builder pages"),
        (name = "components", description = "Visual builder component registry"),
        (name = "preview", description = "Live page preview WebSocket"),
        (name = "delivery", description = "Public delivery API"),
        (name = "webhooks", description = "Webhook subscriptions and delivery logs"),
        (name = "comments", description = "Editorial collaboration comments"),
        (name = "organizations", description = "Organization, member, and invitation management"),
        (name = "marketplace", description = "Marketplace installation lifecycle"),
        (name = "plugins", description = "CMS plugin registry and settings")
    )
)]
struct ApiDoc;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub docs: String,
    pub health: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReadyResponse {
    pub status: String,
    pub checks: Vec<DependencyCheck>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DependencyCheck {
    pub name: String,
    pub ok: bool,
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses((status = 200, description = "API metadata", body = ApiInfo))
)]
async fn root() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "ZinharCMS API".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        docs: "/openapi.json".to_owned(),
        health: "/health".to_owned(),
    })
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses((status = 200, description = "Liveness check", body = HealthResponse))
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}

#[utoipa::path(
    get,
    path = "/ready",
    tag = "system",
    responses((status = 200, description = "Readiness check", body = ReadyResponse))
)]
async fn readiness(State(state): State<AppState>) -> Result<Json<ReadyResponse>, AppError> {
    let mut checks = Vec::with_capacity(2);

    let db_ok = match state.db.execute("SELECT 1").await {
        Ok(_) => DependencyCheck {
            name: "postgres".to_owned(),
            ok: true,
            message: "reachable".to_owned(),
        },
        Err(_) => DependencyCheck {
            name: "postgres".to_owned(),
            ok: false,
            message: "unavailable".to_owned(),
        },
    };
    checks.push(db_ok);

    let redis_ok = match state.redis.get_multiplexed_async_connection().await {
        Ok(mut connection) => match connection.ping::<String>().await {
            Ok(_) => DependencyCheck {
                name: "redis".to_owned(),
                ok: true,
                message: "reachable".to_owned(),
            },
            Err(_) => DependencyCheck {
                name: "redis".to_owned(),
                ok: false,
                message: "unavailable".to_owned(),
            },
        },
        Err(_) => DependencyCheck {
            name: "redis".to_owned(),
            ok: false,
            message: "unavailable".to_owned(),
        },
    };
    checks.push(redis_ok);

    let all_ok = checks.iter().all(|check| check.ok);
    let response = ReadyResponse {
        status: (if all_ok { "ready" } else { "degraded" }).to_owned(),
        checks,
    };

    if all_ok {
        Ok(Json(response))
    } else {
        Err(AppError::ServiceUnavailable(
            serde_json::to_string(&response)
                .unwrap_or_else(|_| "dependency check failed".to_owned()),
        ))
    }
}

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::middleware;
    use axum::response::IntoResponse;
    use tower::ServiceExt;
    use uuid::Uuid;

    use super::{is_public_media_path, restrict_public_uploads};

    #[test]
    fn public_upload_mount_exposes_only_generated_media_paths() {
        let organization_id = Uuid::now_v7();
        assert!(is_public_media_path(&format!(
            "/uploads/{organization_id}/01900000-0000-7000-8000-000000000000.png"
        )));
        assert!(is_public_media_path(&format!(
            "/uploads/{organization_id}/variants/01900000-0000-7000-8000-000000000000-thumbnail.webp"
        )));
        assert!(!is_public_media_path(
            "/uploads/marketplace/packages/creator/listing/1.0.0/package.zip"
        ));
        assert!(!is_public_media_path(
            "/uploads/%6darketplace/packages/creator/listing/1.0.0/package.zip"
        ));
        assert!(!is_public_media_path(&format!(
            "/uploads/{organization_id}/../private.zip"
        )));
    }

    #[tokio::test]
    async fn public_upload_router_applies_the_policy_to_the_original_path() {
        let organization_id = Uuid::now_v7();
        let app = Router::new()
            .nest_service(
                "/uploads",
                Router::new().fallback(|| async { StatusCode::OK.into_response() }),
            )
            .route_layer(middleware::from_fn(restrict_public_uploads));

        let allowed = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/uploads/{organization_id}/image.png"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(allowed.status(), StatusCode::OK);

        let blocked = app
            .oneshot(
                Request::builder()
                    .uri("/uploads/marketplace/packages/private.zip")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(blocked.status(), StatusCode::NOT_FOUND);
    }
}
