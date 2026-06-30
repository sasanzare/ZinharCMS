use axum::extract::{Extension, Path, Query, State};
use axum::routing::{get, patch, post, put};
use axum::{Json, Router};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::services::{audit, quota, rbac, rls};
use crate::state::AppState;

const FEEDBACK_CATEGORIES: &[&str] = &[
    "bug",
    "ux",
    "billing",
    "performance",
    "tenant_isolation",
    "onboarding",
    "other",
];
const FEEDBACK_SEVERITIES: &[&str] = &["low", "medium", "high", "critical"];
const FEEDBACK_STATUSES: &[&str] = &["open", "triaged", "planned", "fixed", "closed"];
const BLOCKER_PRIORITIES: &[&str] = &["p0", "p1", "p2", "p3"];
const BLOCKER_STATUSES: &[&str] = &["open", "in_progress", "blocked", "resolved", "deferred"];
const PARTICIPANT_STATUSES: &[&str] = &[
    "candidate",
    "invited",
    "onboarding",
    "active",
    "paused",
    "graduated",
    "rejected",
];

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/beta/dashboard", get(get_beta_dashboard))
        .route("/api/beta/feedback", get(list_beta_feedback))
        .route("/api/beta/feedback", post(create_beta_feedback))
        .route(
            "/api/beta/feedback/{feedback_id}",
            patch(update_beta_feedback),
        )
        .route("/api/beta/ga-blockers", get(list_ga_blockers))
        .route("/api/beta/ga-blockers", post(create_ga_blocker))
        .route(
            "/api/beta/ga-blockers/{blocker_id}",
            patch(update_ga_blocker),
        )
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/api/beta/product-dashboard", get(get_product_dashboard))
        .route(
            "/api/beta/participants/{organization_id}",
            put(upsert_beta_participant),
        )
}

#[derive(Debug, Deserialize)]
pub struct LimitQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BetaFeedbackRequest {
    pub category: Option<String>,
    pub severity: Option<String>,
    pub title: String,
    pub description: String,
    pub page_url: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBetaFeedbackRequest {
    pub status: Option<String>,
    pub severity: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BetaGaBlockerRequest {
    pub feedback_id: Option<Uuid>,
    pub priority: Option<String>,
    pub area: String,
    pub title: String,
    pub owner: Option<String>,
    pub due_at: Option<NaiveDate>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBetaGaBlockerRequest {
    pub priority: Option<String>,
    pub status: Option<String>,
    pub owner: Option<String>,
    pub due_at: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BetaParticipantRequest {
    pub cohort_label: Option<String>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BetaFeedbackResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub submitted_by: Option<Uuid>,
    pub submitted_by_email: Option<String>,
    pub category: String,
    pub severity: String,
    pub status: String,
    pub title: String,
    pub description: String,
    pub page_url: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BetaGaBlockerResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub feedback_id: Option<Uuid>,
    pub priority: String,
    pub area: String,
    pub title: String,
    pub status: String,
    pub owner: Option<String>,
    pub due_at: Option<NaiveDate>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BetaParticipantResponse {
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub cohort_label: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub status: String,
    pub onboarded_at: Option<DateTime<Utc>>,
    pub last_check_in_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BetaOrganizationDashboardResponse {
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub participant_status: Option<String>,
    pub cohort_label: Option<String>,
    pub current_plan: Option<String>,
    pub current_plan_slug: Option<String>,
    pub open_feedback: i64,
    pub critical_feedback: i64,
    pub open_ga_blockers: i64,
    pub failed_billing_events: i64,
    pub failed_email_deliveries: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BetaDashboardResponse {
    pub organization: BetaOrganizationDashboardResponse,
    pub exceeded_usage_metrics: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BetaProductTotalsResponse {
    pub beta_organizations: i64,
    pub active_organizations: i64,
    pub open_feedback: i64,
    pub critical_feedback: i64,
    pub open_ga_blockers: i64,
    pub failed_billing_events: i64,
    pub failed_email_deliveries: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BetaProductDashboardResponse {
    pub organizations: Vec<BetaOrganizationDashboardResponse>,
    pub totals: BetaProductTotalsResponse,
}

#[utoipa::path(
    get,
    path = "/api/beta/dashboard",
    tag = "beta",
    responses((status = 200, description = "Current organization beta dashboard", body = BetaDashboardResponse))
)]
pub async fn get_beta_dashboard(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<BetaDashboardResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let organization = load_organization_dashboard(db.as_mut(), tenant.organization_id).await?;
    let exceeded_usage_metrics = exceeded_usage_metrics(&state, &tenant).await?;
    Ok(Json(BetaDashboardResponse {
        organization,
        exceeded_usage_metrics,
    }))
}

#[utoipa::path(
    get,
    path = "/api/beta/feedback",
    tag = "beta",
    responses((status = 200, description = "Current organization beta feedback", body = [BetaFeedbackResponse]))
)]
pub async fn list_beta_feedback(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<Vec<BetaFeedbackResponse>>, AppError> {
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, BetaFeedbackResponse>(
        r#"
        SELECT feedback.id,
               feedback.organization_id,
               feedback.submitted_by,
               users.email::text as submitted_by_email,
               feedback.category,
               feedback.severity,
               feedback.status,
               feedback.title,
               feedback.description,
               feedback.page_url,
               feedback.metadata,
               feedback.created_at,
               feedback.updated_at
        FROM beta_feedback feedback
        LEFT JOIN users ON users.id = feedback.submitted_by
        WHERE feedback.organization_id = $1
        ORDER BY feedback.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(limit)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(rows))
}

#[utoipa::path(
    post,
    path = "/api/beta/feedback",
    tag = "beta",
    request_body = BetaFeedbackRequest,
    responses((status = 200, description = "Created beta feedback", body = BetaFeedbackResponse))
)]
pub async fn create_beta_feedback(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<BetaFeedbackRequest>,
) -> Result<Json<BetaFeedbackResponse>, AppError> {
    let category = validate_choice(
        payload.category.as_deref().unwrap_or("ux"),
        FEEDBACK_CATEGORIES,
        "feedback category",
    )?;
    let severity = validate_choice(
        payload.severity.as_deref().unwrap_or("medium"),
        FEEDBACK_SEVERITIES,
        "feedback severity",
    )?;
    let title = normalize_required(&payload.title, "feedback title", 160)?;
    let description = normalize_required(&payload.description, "feedback description", 4_000)?;
    let metadata = metadata_object(payload.metadata)?;
    let page_url = normalize_optional(payload.page_url, 500);

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let row = sqlx::query_as::<_, BetaFeedbackResponse>(
        r#"
        INSERT INTO beta_feedback (
          organization_id,
          submitted_by,
          category,
          severity,
          title,
          description,
          page_url,
          metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id,
                  organization_id,
                  submitted_by,
                  (SELECT email::text FROM users WHERE id = submitted_by) as submitted_by_email,
                  category,
                  severity,
                  status,
                  title,
                  description,
                  page_url,
                  metadata,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(tenant.user_id)
    .bind(&category)
    .bind(&severity)
    .bind(&title)
    .bind(&description)
    .bind(&page_url)
    .bind(&metadata)
    .fetch_one(&mut *tx)
    .await?;

    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "beta.feedback.create",
        "beta_feedback",
        Some(row.id),
        serde_json::json!({ "category": &row.category, "severity": &row.severity, "title": &row.title }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(row))
}

#[utoipa::path(
    patch,
    path = "/api/beta/feedback/{feedback_id}",
    tag = "beta",
    params(("feedback_id" = Uuid, Path, description = "Beta feedback id")),
    request_body = UpdateBetaFeedbackRequest,
    responses((status = 200, description = "Updated beta feedback", body = BetaFeedbackResponse))
)]
pub async fn update_beta_feedback(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(feedback_id): Path<Uuid>,
    Json(payload): Json<UpdateBetaFeedbackRequest>,
) -> Result<Json<BetaFeedbackResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_EDITOR])?;
    let status = match payload.status {
        Some(value) => Some(validate_choice(
            &value,
            FEEDBACK_STATUSES,
            "feedback status",
        )?),
        None => None,
    };
    let severity = match payload.severity {
        Some(value) => Some(validate_choice(
            &value,
            FEEDBACK_SEVERITIES,
            "feedback severity",
        )?),
        None => None,
    };
    if status.is_none() && severity.is_none() {
        return Err(AppError::Validation(
            "feedback status or severity is required".to_owned(),
        ));
    }

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let row = sqlx::query_as::<_, BetaFeedbackResponse>(
        r#"
        UPDATE beta_feedback feedback
        SET status = COALESCE($3, feedback.status),
            severity = COALESCE($4, feedback.severity),
            updated_at = now()
        WHERE feedback.organization_id = $1
          AND feedback.id = $2
        RETURNING feedback.id,
                  feedback.organization_id,
                  feedback.submitted_by,
                  (SELECT email::text FROM users WHERE id = feedback.submitted_by) as submitted_by_email,
                  feedback.category,
                  feedback.severity,
                  feedback.status,
                  feedback.title,
                  feedback.description,
                  feedback.page_url,
                  feedback.metadata,
                  feedback.created_at,
                  feedback.updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(feedback_id)
    .bind(status.as_deref())
    .bind(severity.as_deref())
    .fetch_one(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "beta.feedback.update",
        "beta_feedback",
        Some(row.id),
        serde_json::json!({ "status": &row.status, "severity": &row.severity }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/beta/ga-blockers",
    tag = "beta",
    responses((status = 200, description = "Current organization GA blockers", body = [BetaGaBlockerResponse]))
)]
pub async fn list_ga_blockers(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<Vec<BetaGaBlockerResponse>>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_EDITOR])?;
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, BetaGaBlockerResponse>(
        r#"
        SELECT id,
               organization_id,
               feedback_id,
               priority,
               area,
               title,
               status,
               owner,
               due_at,
               metadata,
               created_at,
               updated_at
        FROM beta_ga_blockers
        WHERE organization_id = $1
        ORDER BY CASE priority WHEN 'p0' THEN 1 WHEN 'p1' THEN 2 WHEN 'p2' THEN 3 ELSE 4 END,
                 created_at DESC
        LIMIT $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(limit)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(rows))
}

#[utoipa::path(
    post,
    path = "/api/beta/ga-blockers",
    tag = "beta",
    request_body = BetaGaBlockerRequest,
    responses((status = 200, description = "Created GA blocker", body = BetaGaBlockerResponse))
)]
pub async fn create_ga_blocker(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<BetaGaBlockerRequest>,
) -> Result<Json<BetaGaBlockerResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_EDITOR])?;
    let priority = validate_choice(
        payload.priority.as_deref().unwrap_or("p2"),
        BLOCKER_PRIORITIES,
        "blocker priority",
    )?;
    let area = normalize_required(&payload.area, "blocker area", 80)?;
    let title = normalize_required(&payload.title, "blocker title", 200)?;
    let metadata = metadata_object(payload.metadata)?;
    let owner = normalize_optional(payload.owner, 120);

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    if let Some(feedback_id) = payload.feedback_id {
        ensure_feedback_in_current_org(&mut tx, tenant.organization_id, feedback_id).await?;
    }

    let row = sqlx::query_as::<_, BetaGaBlockerResponse>(
        r#"
        INSERT INTO beta_ga_blockers (
          organization_id,
          feedback_id,
          priority,
          area,
          title,
          owner,
          due_at,
          metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id,
                  organization_id,
                  feedback_id,
                  priority,
                  area,
                  title,
                  status,
                  owner,
                  due_at,
                  metadata,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.feedback_id)
    .bind(&priority)
    .bind(&area)
    .bind(&title)
    .bind(&owner)
    .bind(payload.due_at)
    .bind(&metadata)
    .fetch_one(&mut *tx)
    .await?;

    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "beta.ga_blocker.create",
        "beta_ga_blocker",
        Some(row.id),
        serde_json::json!({ "priority": &row.priority, "area": &row.area, "title": &row.title }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(row))
}

#[utoipa::path(
    patch,
    path = "/api/beta/ga-blockers/{blocker_id}",
    tag = "beta",
    params(("blocker_id" = Uuid, Path, description = "GA blocker id")),
    request_body = UpdateBetaGaBlockerRequest,
    responses((status = 200, description = "Updated GA blocker", body = BetaGaBlockerResponse))
)]
pub async fn update_ga_blocker(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(blocker_id): Path<Uuid>,
    Json(payload): Json<UpdateBetaGaBlockerRequest>,
) -> Result<Json<BetaGaBlockerResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_EDITOR])?;
    let priority = match payload.priority {
        Some(value) => Some(validate_choice(
            &value,
            BLOCKER_PRIORITIES,
            "blocker priority",
        )?),
        None => None,
    };
    let status = match payload.status {
        Some(value) => Some(validate_choice(&value, BLOCKER_STATUSES, "blocker status")?),
        None => None,
    };
    let owner = normalize_optional(payload.owner, 120);

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let row = sqlx::query_as::<_, BetaGaBlockerResponse>(
        r#"
        UPDATE beta_ga_blockers blocker
        SET priority = COALESCE($3, blocker.priority),
            status = COALESCE($4, blocker.status),
            owner = COALESCE($5, blocker.owner),
            due_at = COALESCE($6, blocker.due_at),
            updated_at = now()
        WHERE blocker.organization_id = $1
          AND blocker.id = $2
        RETURNING id,
                  organization_id,
                  feedback_id,
                  priority,
                  area,
                  title,
                  status,
                  owner,
                  due_at,
                  metadata,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(blocker_id)
    .bind(priority.as_deref())
    .bind(status.as_deref())
    .bind(owner.as_deref())
    .bind(payload.due_at)
    .fetch_one(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "beta.ga_blocker.update",
        "beta_ga_blocker",
        Some(row.id),
        serde_json::json!({ "priority": &row.priority, "status": &row.status }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/beta/product-dashboard",
    tag = "beta",
    responses((status = 200, description = "Cross-organization beta dashboard", body = BetaProductDashboardResponse))
)]
pub async fn get_product_dashboard(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<BetaProductDashboardResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let organizations =
        sqlx::query_as::<_, BetaOrganizationDashboardResponse>(beta_dashboard_query("").as_str())
            .fetch_all(&mut *tx)
            .await?;
    tx.commit().await?;

    let totals = BetaProductTotalsResponse {
        beta_organizations: organizations.len() as i64,
        active_organizations: organizations
            .iter()
            .filter(|item| item.participant_status.as_deref() == Some("active"))
            .count() as i64,
        open_feedback: organizations.iter().map(|item| item.open_feedback).sum(),
        critical_feedback: organizations
            .iter()
            .map(|item| item.critical_feedback)
            .sum(),
        open_ga_blockers: organizations.iter().map(|item| item.open_ga_blockers).sum(),
        failed_billing_events: organizations
            .iter()
            .map(|item| item.failed_billing_events)
            .sum(),
        failed_email_deliveries: organizations
            .iter()
            .map(|item| item.failed_email_deliveries)
            .sum(),
    };
    Ok(Json(BetaProductDashboardResponse {
        organizations,
        totals,
    }))
}

#[utoipa::path(
    put,
    path = "/api/beta/participants/{organization_id}",
    tag = "beta",
    params(("organization_id" = Uuid, Path, description = "Organization id")),
    request_body = BetaParticipantRequest,
    responses((status = 200, description = "Updated beta participant", body = BetaParticipantResponse))
)]
pub async fn upsert_beta_participant(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(organization_id): Path<Uuid>,
    Json(payload): Json<BetaParticipantRequest>,
) -> Result<Json<BetaParticipantResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let status = validate_choice(&payload.status, PARTICIPANT_STATUSES, "participant status")?;
    let cohort_label = normalize_required(
        payload.cohort_label.as_deref().unwrap_or("private-beta"),
        "cohort label",
        120,
    )?;
    let contact_name = normalize_optional(payload.contact_name, 160);
    let contact_email = normalize_optional(payload.contact_email, 255);
    let notes = normalize_optional(payload.notes, 4_000);
    let metadata = metadata_object(payload.metadata)?;

    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let row = sqlx::query_as::<_, BetaParticipantResponse>(
        r#"
        INSERT INTO beta_participants (
          organization_id,
          cohort_label,
          contact_name,
          contact_email,
          status,
          onboarded_at,
          last_check_in_at,
          notes,
          metadata
        )
        SELECT org.id,
               $2,
               $3,
               $4,
               $5,
               CASE WHEN $5 IN ('onboarding', 'active', 'graduated') THEN COALESCE(existing.onboarded_at, now()) ELSE existing.onboarded_at END,
               now(),
               $6,
               $7
        FROM organizations org
        LEFT JOIN beta_participants existing ON existing.organization_id = org.id
        WHERE org.id = $1
        ON CONFLICT (organization_id) DO UPDATE
        SET cohort_label = EXCLUDED.cohort_label,
            contact_name = EXCLUDED.contact_name,
            contact_email = EXCLUDED.contact_email,
            status = EXCLUDED.status,
            onboarded_at = EXCLUDED.onboarded_at,
            last_check_in_at = now(),
            notes = EXCLUDED.notes,
            metadata = EXCLUDED.metadata,
            updated_at = now()
        RETURNING organization_id,
                  (SELECT name FROM organizations WHERE id = organization_id) as organization_name,
                  (SELECT slug FROM organizations WHERE id = organization_id) as organization_slug,
                  cohort_label,
                  contact_name,
                  contact_email,
                  status,
                  onboarded_at,
                  last_check_in_at,
                  notes,
                  metadata,
                  created_at,
                  updated_at
        "#,
    )
    .bind(organization_id)
    .bind(&cohort_label)
    .bind(&contact_name)
    .bind(&contact_email)
    .bind(&status)
    .bind(&notes)
    .bind(&metadata)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(row))
}

async fn load_organization_dashboard(
    executor: &mut sqlx::PgConnection,
    organization_id: Uuid,
) -> Result<BetaOrganizationDashboardResponse, AppError> {
    sqlx::query_as::<_, BetaOrganizationDashboardResponse>(
        format!("{} WHERE org.id = $1", beta_dashboard_query("LEFT")).as_str(),
    )
    .bind(organization_id)
    .fetch_one(executor)
    .await
    .map_err(AppError::from)
}

fn beta_dashboard_query(participant_join: &str) -> String {
    format!(
        r#"
        SELECT org.id as organization_id,
               org.name as organization_name,
               org.slug as organization_slug,
               participant.status as participant_status,
               participant.cohort_label,
               plans.name as current_plan,
               plans.slug as current_plan_slug,
               (
                 SELECT COUNT(*)
                 FROM beta_feedback feedback
                 WHERE feedback.organization_id = org.id
                   AND feedback.status NOT IN ('fixed', 'closed')
               ) as open_feedback,
               (
                 SELECT COUNT(*)
                 FROM beta_feedback feedback
                 WHERE feedback.organization_id = org.id
                   AND feedback.status NOT IN ('fixed', 'closed')
                   AND feedback.severity IN ('high', 'critical')
               ) as critical_feedback,
               (
                 SELECT COUNT(*)
                 FROM beta_ga_blockers blocker
                 WHERE blocker.organization_id = org.id
                   AND blocker.status NOT IN ('resolved', 'deferred')
               ) as open_ga_blockers,
               (
                 SELECT COUNT(*)
                 FROM billing_events event
                 WHERE event.organization_id = org.id
                   AND event.status = 'failed'
                   AND event.created_at >= now() - interval '30 days'
               ) as failed_billing_events,
               (
                 SELECT COUNT(*)
                 FROM email_deliveries delivery
                 WHERE delivery.organization_id = org.id
                   AND delivery.status = 'failed'
                   AND delivery.created_at >= now() - interval '30 days'
               ) as failed_email_deliveries
        FROM organizations org
        {participant_join} JOIN beta_participants participant ON participant.organization_id = org.id
        LEFT JOIN organization_subscriptions subscription ON subscription.organization_id = org.id
        LEFT JOIN plans ON plans.id = subscription.plan_id
        "#,
    )
}

async fn exceeded_usage_metrics(
    state: &AppState,
    tenant: &TenantContext,
) -> Result<Vec<String>, AppError> {
    let usage = quota::usage_summary(&state.db, tenant).await?;
    let metrics = [
        usage.members,
        usage.content_records,
        usage.media_bytes,
        usage.api_requests,
    ];
    Ok(metrics
        .into_iter()
        .filter(|metric| metric.exceeded)
        .map(|metric| metric.metric.to_owned())
        .collect())
}

async fn ensure_feedback_in_current_org(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    organization_id: Uuid,
    feedback_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM beta_feedback WHERE organization_id = $1 AND id = $2",
    )
    .bind(organization_id)
    .bind(feedback_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(())
}

fn validate_choice(value: &str, allowed: &[&str], field: &str) -> Result<String, AppError> {
    let normalized = value.trim().to_ascii_lowercase();
    if allowed.iter().any(|allowed| *allowed == normalized) {
        Ok(normalized)
    } else {
        Err(AppError::Validation(format!(
            "{field} must be one of: {}",
            allowed.join(", ")
        )))
    }
}

fn normalize_required(value: &str, field: &str, max_len: usize) -> Result<String, AppError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(AppError::Validation(format!("{field} is required")));
    }
    if normalized.chars().count() > max_len {
        return Err(AppError::Validation(format!(
            "{field} must be at most {max_len} characters"
        )));
    }
    Ok(normalized.to_owned())
}

fn normalize_optional(value: Option<String>, max_len: usize) -> Option<String> {
    value.and_then(|value| {
        let normalized = value.trim();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized.chars().take(max_len).collect())
        }
    })
}

fn metadata_object(value: Option<Value>) -> Result<Value, AppError> {
    let value = value.unwrap_or_else(|| serde_json::json!({}));
    if value.is_object() {
        Ok(value)
    } else {
        Err(AppError::Validation(
            "metadata must be a JSON object".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_supported_beta_feedback_category() {
        assert_eq!(
            validate_choice(" Tenant_Isolation ", FEEDBACK_CATEGORIES, "category").unwrap(),
            "tenant_isolation"
        );
        assert!(validate_choice("sales", FEEDBACK_CATEGORIES, "category").is_err());
    }

    #[test]
    fn rejects_non_object_metadata() {
        assert!(metadata_object(Some(serde_json::json!([]))).is_err());
        assert!(metadata_object(Some(serde_json::json!({ "ok": true }))).is_ok());
    }

    #[test]
    fn required_text_is_trimmed_and_bounded() {
        assert_eq!(
            normalize_required("  Onboarding issue  ", "title", 80).unwrap(),
            "Onboarding issue"
        );
        assert!(normalize_required("   ", "title", 80).is_err());
    }

    #[test]
    fn beta_dashboard_query_does_not_duplicate_join_keyword() {
        let inner = beta_dashboard_query("");
        let left = beta_dashboard_query("LEFT");

        assert!(!inner.contains("JOIN JOIN"));
        assert!(inner.contains("JOIN beta_participants"));
        assert!(left.contains("LEFT JOIN beta_participants"));
    }
}
