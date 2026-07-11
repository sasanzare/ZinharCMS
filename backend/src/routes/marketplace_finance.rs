use axum::extract::{Extension, Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::services::{audit, marketplace_finance, rbac, rls, stripe_billing};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/marketplace/purchases", get(list_purchases))
        .route("/api/marketplace/purchases/checkout", post(create_checkout))
        .route("/api/marketplace/revenue-ledger", get(list_revenue_ledger))
        .route(
            "/api/marketplace/creators/{creator_id}/payout",
            get(get_payout_account).post(onboard_payout_account),
        )
        .route(
            "/api/marketplace/creators/{creator_id}/payout/verify",
            post(verify_payout_account),
        )
        .route(
            "/api/marketplace/creators/{creator_id}/balance",
            get(get_creator_balance),
        )
        .route(
            "/api/marketplace/creators/{creator_id}/payout/request",
            post(request_payout),
        )
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceCheckoutRequest {
    pub listing_id: Uuid,
    pub version_id: Uuid,
    #[serde(default = "default_currency")]
    pub currency: String,
}

#[derive(Debug, Serialize, FromRow, ToSchema, Clone)]
pub struct MarketplacePurchaseResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub listing_id: Uuid,
    pub version_id: Uuid,
    pub pricing_type: String,
    pub currency: String,
    pub subtotal_cents: i32,
    pub tax_cents: i32,
    pub total_cents: i32,
    pub provider: String,
    pub provider_checkout_id: Option<String>,
    pub status: String,
    pub receipt_number: String,
    pub provider_metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceCheckoutResponse {
    pub purchase: MarketplacePurchaseResponse,
    pub checkout_url: Option<String>,
    pub entitlement_granted: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PayoutOnboardingRequest {
    pub provider_account_id: String,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplacePayoutAccountResponse {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub provider: String,
    pub provider_account_id: Option<String>,
    pub status: String,
    pub country: Option<String>,
    pub charges_enabled: bool,
    pub payouts_enabled: bool,
    pub details_submitted: bool,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceRevenueLedgerResponse {
    pub id: Uuid,
    pub purchase_id: Uuid,
    pub creator_id: Uuid,
    pub listing_id: Uuid,
    pub entry_type: String,
    pub provider_event_id: Option<String>,
    pub currency: String,
    pub gross_cents: i32,
    pub tax_cents: i32,
    pub commission_bps: i32,
    pub platform_fee_cents: i32,
    pub creator_share_cents: i32,
    pub adjustment_cents: i32,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceCreatorBalanceResponse {
    pub creator_id: Uuid,
    pub currency: String,
    pub pending_cents: i64,
    pub available_cents: i64,
    pub paid_cents: i64,
    pub net_earned_cents: i64,
    pub settlement_days: i32,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplacePayoutResponse {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub provider_account_id: Option<String>,
    pub settlement_at: Option<DateTime<Utc>>,
    pub provider_transfer_id: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const SETTLEMENT_DAYS: i32 = 7;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PayoutVerificationRequest {
    pub status: String,
    pub charges_enabled: bool,
    pub payouts_enabled: bool,
    pub details_submitted: bool,
}

fn default_currency() -> String {
    "usd".to_owned()
}

#[utoipa::path(
    get,
    path = "/api/marketplace/purchases",
    tag = "marketplace",
    responses((status = 200, description = "Organization Marketplace purchases and receipts", body = [MarketplacePurchaseResponse]))
)]
pub async fn list_purchases(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<MarketplacePurchaseResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, MarketplacePurchaseResponse>(
        "SELECT id, organization_id, listing_id, version_id, pricing_type, currency, subtotal_cents, tax_cents, total_cents, provider, provider_checkout_id, status, receipt_number, provider_metadata, created_at, updated_at FROM marketplace_purchases WHERE organization_id = $1 ORDER BY created_at DESC",
    ).bind(tenant.organization_id).fetch_all(db.as_mut()).await?;
    Ok(Json(rows))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/revenue-ledger",
    tag = "marketplace",
    responses((status = 200, description = "Auditable organization Marketplace revenue ledger", body = [MarketplaceRevenueLedgerResponse]))
)]
pub async fn list_revenue_ledger(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<MarketplaceRevenueLedgerResponse>>, AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, MarketplaceRevenueLedgerResponse>(
        "SELECT id, purchase_id, creator_id, listing_id, entry_type, provider_event_id, currency, gross_cents, tax_cents, commission_bps, platform_fee_cents, creator_share_cents, adjustment_cents, metadata, created_at FROM marketplace_revenue_ledger WHERE organization_id = $1 ORDER BY created_at DESC",
    )
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(rows))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/creators/{creator_id}/balance",
    tag = "marketplace",
    params(("creator_id" = Uuid, Path, description = "Marketplace creator id")),
    responses((status = 200, description = "Creator revenue balance after settlement window", body = MarketplaceCreatorBalanceResponse))
)]
pub async fn get_creator_balance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(creator_id): Path<Uuid>,
) -> Result<Json<MarketplaceCreatorBalanceResponse>, AppError> {
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    ensure_creator_owner(&mut *tx, creator_id, claims.sub).await?;
    let row = sqlx::query_as::<_, (Option<String>, i64, i64, i64)>(
        "SELECT MAX(currency), COALESCE(SUM(creator_share_cents + adjustment_cents) FILTER (WHERE created_at < now() - make_interval(days => $2)), 0), COALESCE(SUM(creator_share_cents + adjustment_cents) FILTER (WHERE created_at >= now() - make_interval(days => $2)), 0), COALESCE(SUM(creator_share_cents + adjustment_cents), 0) FROM marketplace_revenue_ledger WHERE creator_id = $1",
    )
    .bind(creator_id)
    .bind(SETTLEMENT_DAYS)
    .fetch_one(&mut *tx)
    .await?;
    let paid_cents: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount_cents), 0)::bigint FROM marketplace_payouts WHERE creator_id = $1 AND status = 'paid'",
    )
    .bind(creator_id)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(MarketplaceCreatorBalanceResponse {
        creator_id,
        currency: row.0.unwrap_or_else(|| "usd".to_owned()),
        available_cents: row.1,
        pending_cents: row.2,
        net_earned_cents: row.3,
        paid_cents,
        settlement_days: SETTLEMENT_DAYS,
    }))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/purchases/checkout",
    tag = "marketplace",
    request_body = MarketplaceCheckoutRequest,
    responses((status = 201, description = "Free entitlement or paid checkout session", body = MarketplaceCheckoutResponse))
)]
pub async fn create_checkout(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<MarketplaceCheckoutRequest>,
) -> Result<(axum::http::StatusCode, Json<MarketplaceCheckoutResponse>), AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    let currency = payload.currency.trim().to_ascii_lowercase();
    if currency.len() != 3 || !currency.chars().all(|ch| ch.is_ascii_lowercase()) {
        return Err(AppError::Validation(
            "currency must be a three-letter lowercase code".to_owned(),
        ));
    }
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let candidate = sqlx::query_as::<_, (Uuid, Uuid, String, String, i32, String, String, String, Uuid, Uuid)>(
        "SELECT listing.id, version.id, listing.pricing_type, listing.status, listing.price_cents, version.status, version.validation_status, version.security_risk_level, listing.creator_id, creator.user_id FROM marketplace_listings listing JOIN marketplace_versions version ON version.listing_id = listing.id JOIN marketplace_creators creator ON creator.id = listing.creator_id WHERE listing.id = $1 AND version.id = $2",
    ).bind(payload.listing_id).bind(payload.version_id).fetch_optional(&mut *tx).await?.ok_or_else(|| AppError::NotFound("Marketplace listing/version not found".to_owned()))?;
    marketplace_finance::validate_purchase_amount(candidate.4, &candidate.2)
        .map_err(AppError::Validation)?;
    if candidate.3 != "approved"
        || candidate.5 != "approved"
        || !matches!(candidate.6.as_str(), "passed" | "warning")
        || !matches!(candidate.7.as_str(), "low" | "medium")
    {
        return Err(AppError::Conflict(
            "Marketplace product is not eligible for purchase".to_owned(),
        ));
    }
    if candidate.9 == tenant.user_id {
        return Err(AppError::Conflict(
            "Creators cannot purchase their own Marketplace products".to_owned(),
        ));
    }
    let existing: Option<MarketplacePurchaseResponse> = sqlx::query_as("SELECT id, organization_id, listing_id, version_id, pricing_type, currency, subtotal_cents, tax_cents, total_cents, provider, provider_checkout_id, status, receipt_number, provider_metadata, created_at, updated_at FROM marketplace_purchases WHERE organization_id = $1 AND listing_id = $2 AND version_id = $3 AND status IN ('pending', 'completed') ORDER BY created_at DESC LIMIT 1")
        .bind(tenant.organization_id).bind(payload.listing_id).bind(payload.version_id).fetch_optional(&mut *tx).await?;
    if let Some(purchase) = existing {
        let entitlement_granted = purchase.status == "completed";
        let checkout_url = purchase
            .provider_metadata
            .get("checkout_url")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        tx.commit().await?;
        return Ok((
            axum::http::StatusCode::OK,
            Json(MarketplaceCheckoutResponse {
                purchase,
                checkout_url,
                entitlement_granted,
            }),
        ));
    }
    let tax_cents = 0;
    let total_cents = candidate.4 + tax_cents;
    let provider = if candidate.2 == "free" {
        "none"
    } else {
        "stripe"
    };
    let status = if candidate.2 == "free" {
        "completed"
    } else {
        "pending"
    };
    let purchase = sqlx::query_as::<_, MarketplacePurchaseResponse>(
        "INSERT INTO marketplace_purchases (organization_id, listing_id, version_id, buyer_id, pricing_type, currency, subtotal_cents, tax_cents, total_cents, provider, status, receipt_number, provider_metadata) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'MKT-' || upper(substr(replace(gen_random_uuid()::text, '-', ''), 1, 16)), $12) RETURNING id, organization_id, listing_id, version_id, pricing_type, currency, subtotal_cents, tax_cents, total_cents, provider, provider_checkout_id, status, receipt_number, provider_metadata, created_at, updated_at",
    ).bind(tenant.organization_id).bind(candidate.0).bind(candidate.1).bind(tenant.user_id).bind(&candidate.2).bind(&currency).bind(candidate.4).bind(tax_cents).bind(total_cents).bind(provider).bind(status).bind(marketplace_finance::tax_metadata(&currency, tax_cents)).fetch_one(&mut *tx).await?;
    if candidate.2 == "free" {
        sqlx::query("INSERT INTO marketplace_entitlements (organization_id, purchase_id, listing_id, version_id, metadata) VALUES ($1, $2, $3, $4, $5)")
            .bind(tenant.organization_id).bind(purchase.id).bind(candidate.0).bind(candidate.1).bind(serde_json::json!({"source":"free_product_checkout"})).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO marketplace_revenue_ledger (organization_id, purchase_id, creator_id, listing_id, entry_type, currency, gross_cents, tax_cents, commission_bps, platform_fee_cents, creator_share_cents, metadata) VALUES ($1, $2, $3, $4, 'purchase', $5, 0, 0, $6, 0, 0, $7)")
            .bind(tenant.organization_id).bind(purchase.id).bind(candidate.8).bind(candidate.0).bind(&currency).bind(marketplace_finance::DEFAULT_COMMISSION_BPS).bind(serde_json::json!({"source":"free_product"})).execute(&mut *tx).await?;
        audit::record_in_transaction(&mut tx, tenant.organization_id, Some(tenant.user_id), "marketplace.purchase.free", "marketplace_purchase", Some(purchase.id), serde_json::json!({"listing_id": candidate.0, "version_id": candidate.1, "receipt": purchase.receipt_number})).await?;
        tx.commit().await?;
        return Ok((
            axum::http::StatusCode::CREATED,
            Json(MarketplaceCheckoutResponse {
                purchase,
                checkout_url: None,
                entitlement_granted: true,
            }),
        ));
    }
    tx.commit().await?;
    let session = stripe_billing::create_marketplace_checkout_session(
        &state.config,
        purchase.id,
        tenant.organization_id,
        "Marketplace product",
        total_cents,
        &currency,
    )
    .await;
    let session = match session {
        Ok(session) => session,
        Err(error) => {
            let mut db = rls::tenant_connection(&state.db, &tenant).await?;
            sqlx::query("UPDATE marketplace_purchases SET status = 'failed', provider_metadata = provider_metadata || $2, updated_at = now() WHERE id = $1 AND organization_id = $3")
                .bind(purchase.id)
                .bind(serde_json::json!({"checkout_error":"provider_session_failed"}))
                .bind(tenant.organization_id)
                .execute(db.as_mut())
                .await?;
            return Err(error);
        }
    };
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let updated = sqlx::query_as::<_, MarketplacePurchaseResponse>("UPDATE marketplace_purchases SET provider_checkout_id = $2, provider_metadata = provider_metadata || $3, updated_at = now() WHERE id = $1 AND organization_id = $4 RETURNING id, organization_id, listing_id, version_id, pricing_type, currency, subtotal_cents, tax_cents, total_cents, provider, provider_checkout_id, status, receipt_number, provider_metadata, created_at, updated_at")
        .bind(purchase.id).bind(&session.session_id).bind(serde_json::json!({"checkout_url": session.url})).bind(tenant.organization_id).fetch_one(db.as_mut()).await?;
    Ok((
        axum::http::StatusCode::CREATED,
        Json(MarketplaceCheckoutResponse {
            purchase: updated,
            checkout_url: Some(session.url),
            entitlement_granted: false,
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/creators/{creator_id}/payout",
    tag = "marketplace",
    params(("creator_id" = Uuid, Path, description = "Marketplace creator id")),
    responses((status = 200, description = "Creator payout account", body = MarketplacePayoutAccountResponse))
)]
pub async fn get_payout_account(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(creator_id): Path<Uuid>,
) -> Result<Json<MarketplacePayoutAccountResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    ensure_creator_owner(db.as_mut(), creator_id, tenant.user_id).await?;
    sqlx::query_as::<_, MarketplacePayoutAccountResponse>("SELECT id, creator_id, provider, provider_account_id, status, country, charges_enabled, payouts_enabled, details_submitted, metadata, created_at, updated_at FROM marketplace_payout_accounts WHERE creator_id = $1").bind(creator_id).fetch_one(db.as_mut()).await.map(Json).map_err(AppError::from)
}

#[utoipa::path(
    post,
    path = "/api/marketplace/creators/{creator_id}/payout",
    tag = "marketplace",
    params(("creator_id" = Uuid, Path, description = "Marketplace creator id")),
    request_body = PayoutOnboardingRequest,
    responses((status = 200, description = "Creator payout account onboarding state", body = MarketplacePayoutAccountResponse))
)]
pub async fn onboard_payout_account(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(creator_id): Path<Uuid>,
    Json(payload): Json<PayoutOnboardingRequest>,
) -> Result<Json<MarketplacePayoutAccountResponse>, AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    if payload.provider_account_id.trim().is_empty() || payload.provider_account_id.len() > 128 {
        return Err(AppError::Validation(
            "provider_account_id is required and bounded".to_owned(),
        ));
    }
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    ensure_creator_owner(db.as_mut(), creator_id, tenant.user_id).await?;
    let row = sqlx::query_as::<_, MarketplacePayoutAccountResponse>("INSERT INTO marketplace_payout_accounts (creator_id, provider_account_id, status, country, metadata) VALUES ($1, $2, 'pending', $3, $4) ON CONFLICT (creator_id) DO UPDATE SET provider_account_id = EXCLUDED.provider_account_id, status = 'pending', country = EXCLUDED.country, updated_at = now() RETURNING id, creator_id, provider, provider_account_id, status, country, charges_enabled, payouts_enabled, details_submitted, metadata, created_at, updated_at")
        .bind(creator_id).bind(payload.provider_account_id.trim()).bind(payload.country.as_deref()).bind(serde_json::json!({"onboarding":"provider_attested"})).fetch_one(db.as_mut()).await?;
    Ok(Json(row))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/creators/{creator_id}/payout/verify",
    tag = "marketplace",
    params(("creator_id" = Uuid, Path, description = "Marketplace creator id")),
    request_body = PayoutVerificationRequest,
    responses((status = 200, description = "Provider-attested payout verification state", body = MarketplacePayoutAccountResponse))
)]
pub async fn verify_payout_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(creator_id): Path<Uuid>,
    Json(payload): Json<PayoutVerificationRequest>,
) -> Result<Json<MarketplacePayoutAccountResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    if !matches!(
        payload.status.as_str(),
        "pending" | "verified" | "restricted"
    ) {
        return Err(AppError::Validation(
            "payout status must be pending, verified, or restricted".to_owned(),
        ));
    }
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let creator_status: String =
        sqlx::query_scalar("SELECT status FROM marketplace_creators WHERE id = $1")
            .bind(creator_id)
            .fetch_optional(db.as_mut())
            .await?
            .ok_or_else(|| AppError::NotFound("Marketplace creator not found".to_owned()))?;
    if payload.status == "verified" {
        if !payload.details_submitted {
            return Err(AppError::Conflict(
                "provider details must be submitted before payout verification".to_owned(),
            ));
        }
        marketplace_finance::payout_eligibility(
            &creator_status,
            &payload.status,
            payload.payouts_enabled,
        )
        .map_err(AppError::Conflict)?;
    }
    let row = sqlx::query_as::<_, MarketplacePayoutAccountResponse>(
        "UPDATE marketplace_payout_accounts SET status = $2, charges_enabled = $3, payouts_enabled = $4, details_submitted = $5, metadata = metadata || $6, updated_at = now() WHERE creator_id = $1 RETURNING id, creator_id, provider, provider_account_id, status, country, charges_enabled, payouts_enabled, details_submitted, metadata, created_at, updated_at",
    )
    .bind(creator_id)
    .bind(&payload.status)
    .bind(payload.charges_enabled)
    .bind(payload.payouts_enabled)
    .bind(payload.details_submitted)
    .bind(serde_json::json!({"verification":"provider_attested"}))
    .fetch_optional(db.as_mut())
    .await?
    .ok_or_else(|| AppError::NotFound("Payout onboarding is not configured".to_owned()))?;
    sqlx::query(
        "UPDATE marketplace_creators SET payout_status = $2, updated_at = now() WHERE id = $1",
    )
    .bind(creator_id)
    .bind(&payload.status)
    .execute(db.as_mut())
    .await?;
    Ok(Json(row))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/creators/{creator_id}/payout/request",
    tag = "marketplace",
    params(("creator_id" = Uuid, Path, description = "Marketplace creator id")),
    responses((status = 201, description = "Eligible creator payout request", body = MarketplacePayoutResponse))
)]
pub async fn request_payout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(creator_id): Path<Uuid>,
) -> Result<(axum::http::StatusCode, Json<MarketplacePayoutResponse>), AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    ensure_creator_owner(&mut *tx, creator_id, claims.sub).await?;
    let account = sqlx::query_as::<_, (String, String, bool, bool, Option<String>)>(
        "SELECT creator.status, account.status, account.payouts_enabled, account.details_submitted, account.provider_account_id FROM marketplace_creators creator JOIN marketplace_payout_accounts account ON account.creator_id = creator.id WHERE creator.id = $1",
    )
    .bind(creator_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::Conflict("verified payout onboarding is required before requesting a payout".to_owned()))?;
    marketplace_finance::payout_eligibility(&account.0, &account.1, account.2)
        .map_err(AppError::Conflict)?;
    if !account.3 || account.4.is_none() {
        return Err(AppError::Conflict(
            "provider account details are incomplete".to_owned(),
        ));
    }
    let available_cents: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(creator_share_cents + adjustment_cents), 0)::bigint FROM marketplace_revenue_ledger WHERE creator_id = $1 AND created_at < now() - make_interval(days => $2)",
    )
    .bind(creator_id)
    .bind(SETTLEMENT_DAYS)
    .fetch_one(&mut *tx)
    .await?;
    let reserved_cents: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount_cents), 0)::bigint FROM marketplace_payouts WHERE creator_id = $1 AND status IN ('pending', 'eligible', 'paid')",
    )
    .bind(creator_id)
    .fetch_one(&mut *tx)
    .await?;
    let amount_cents = available_cents - reserved_cents;
    if amount_cents <= 0 || amount_cents > i32::MAX as i64 {
        return Err(AppError::Conflict(
            "no settled creator balance is available for payout".to_owned(),
        ));
    }
    let row = sqlx::query_as::<_, MarketplacePayoutResponse>(
        "INSERT INTO marketplace_payouts (creator_id, amount_cents, currency, status, provider_account_id, settlement_at, metadata) VALUES ($1, $2, 'usd', 'eligible', $3, now(), $4) RETURNING id, creator_id, amount_cents, currency, status, provider_account_id, settlement_at, provider_transfer_id, metadata, created_at, updated_at",
    )
    .bind(creator_id)
    .bind(amount_cents as i32)
    .bind(account.4)
    .bind(serde_json::json!({"source":"creator_balance","settlement_days":SETTLEMENT_DAYS}))
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok((axum::http::StatusCode::CREATED, Json(row)))
}

async fn ensure_creator_owner(
    db: &mut sqlx::PgConnection,
    creator_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let owner: Option<Uuid> =
        sqlx::query_scalar("SELECT user_id FROM marketplace_creators WHERE id = $1")
            .bind(creator_id)
            .fetch_optional(db)
            .await?;
    if owner == Some(user_id) {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "only the creator owner can access payout onboarding".to_owned(),
        ))
    }
}
