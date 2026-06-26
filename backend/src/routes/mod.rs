pub mod auth;
pub mod comments;
pub mod content;
pub mod delivery;
pub mod media;
pub mod organizations;
pub mod pages;
pub mod plugins;
pub mod webhooks;

use axum::extract::DefaultBodyLimit;
use axum::extract::State;
use axum::middleware;
use axum::routing::get;
use axum::{Json, Router};
use redis::AsyncCommands;
use serde::Serialize;
use sqlx::Executor;
use tower_http::services::ServeDir;
use utoipa::{OpenApi, ToSchema};

use crate::error::AppError;
use crate::middleware::auth::auth_middleware;
use crate::middleware::tenant::tenant_middleware;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let upload_limit = state.config.max_upload_size.saturating_add(1_048_576) as usize;
    let uploads = ServeDir::new(state.config.upload_dir.clone());
    let protected = Router::new()
        .merge(auth::protected_router())
        .merge(organizations::protected_router())
        .merge(plugins::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let tenant_protected = Router::new()
        .merge(content::router())
        .merge(media::router())
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
        .merge(delivery::router())
        .merge(protected)
        .merge(tenant_protected)
        .nest_service("/uploads", uploads)
        .with_state(state)
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
        auth::refresh,
        auth::logout,
        auth::me,
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
        auth::RefreshRequest,
        auth::LogoutRequest,
        auth::LogoutResponse,
        auth::AuthResponse,
        auth::AuthUser,
        auth::MeResponse,
        auth::OrganizationMembershipResponse,
        organizations::AcceptInvitationRequest,
        organizations::CreateOrganizationRequest,
        organizations::CreatedInvitationResponse,
        organizations::InviteMemberRequest,
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
        plugins::PluginUpdateRequest,
        plugins::PluginResponse
    )),
    tags(
        (name = "system", description = "Phase-zero system endpoints"),
        (name = "auth", description = "Authentication and token management"),
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
        Err(error) => DependencyCheck {
            name: "postgres".to_owned(),
            ok: false,
            message: error.to_string(),
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
            Err(error) => DependencyCheck {
                name: "redis".to_owned(),
                ok: false,
                message: error.to_string(),
            },
        },
        Err(error) => DependencyCheck {
            name: "redis".to_owned(),
            ok: false,
            message: error.to_string(),
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
