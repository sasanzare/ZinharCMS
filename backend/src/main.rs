use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method, StatusCode};
use cms_backend::config::Config;
use cms_backend::db;
use cms_backend::middleware::security::security_headers;
use cms_backend::services::{password, rbac};
use cms_backend::state::AppState;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env().context("failed to load configuration")?;
    let db = db::connect_lazy(&config.database_url).context("failed to prepare database pool")?;
    db::run_migrations(&db)
        .await
        .context("failed to run database migrations")?;
    seed_default_admin(&db)
        .await
        .context("failed to seed default admin user")?;
    let redis =
        redis::Client::open(config.redis_url.as_str()).context("failed to create Redis client")?;
    let state = AppState::new(config.clone(), db, redis);

    let cors_origin = HeaderValue::from_str(&config.cors_origin).context("invalid CORS_ORIGIN")?;
    let app = cms_backend::app(state)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(axum::middleware::from_fn(security_headers))
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(cors_origin)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    tracing::info!(%addr, "ZinharCMS API listening");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("server failed")?;

    Ok(())
}

async fn seed_default_admin(db: &sqlx::PgPool) -> anyhow::Result<()> {
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?;

    if user_count > 0 {
        return Ok(());
    }

    let password_hash = password::hash_password("password123")?;

    let mut tx = db.begin().await?;
    let user_id = sqlx::query_scalar::<_, uuid::Uuid>(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind("admin@example.com")
    .bind(&password_hash)
    .bind("Admin")
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        SELECT $1, id
        FROM roles
        WHERE name = $2
        "#,
    )
    .bind(user_id)
    .bind(rbac::SUPER_ADMIN)
    .execute(&mut *tx)
    .await?;

    attach_default_organization_membership(&mut tx, user_id, rbac::SUPER_ADMIN).await?;
    tx.commit().await?;
    tracing::info!("seeded default admin user admin@example.com");
    Ok(())
}
async fn attach_default_organization_membership(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: uuid::Uuid,
    global_role: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
        SELECT id,
               $1,
               CASE $2
                 WHEN 'super_admin' THEN 'owner'::organization_member_role
                 WHEN 'admin' THEN 'admin'::organization_member_role
                 WHEN 'editor' THEN 'editor'::organization_member_role
                 WHEN 'viewer' THEN 'viewer'::organization_member_role
                 ELSE 'author'::organization_member_role
               END,
               'active'::organization_member_status,
               now()
        FROM organizations
        WHERE slug = 'default'
        ON CONFLICT (organization_id, user_id) DO UPDATE
        SET role = EXCLUDED.role,
            status = 'active'::organization_member_status,
            updated_at = now()
        "#,
    )
    .bind(user_id)
    .bind(global_role)
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE organizations
        SET owner_id = $1,
            updated_at = now()
        WHERE slug = 'default'
          AND owner_id IS NULL
          AND $2 IN ('super_admin', 'admin')
        "#,
    )
    .bind(user_id)
    .bind(global_role)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
