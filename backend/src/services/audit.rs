use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::rls;

pub async fn record(
    pool: &PgPool,
    tenant: &TenantContext,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    metadata: Value,
) -> Result<(), AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query(
        r#"
        INSERT INTO audit_logs (organization_id, actor_id, action, entity_type, entity_id, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(tenant.organization_id)
    .bind(tenant.user_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(metadata)
    .execute(db.as_mut())
    .await?;
    Ok(())
}

pub async fn record_for_organization(
    pool: &PgPool,
    organization_id: Uuid,
    actor_id: Option<Uuid>,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    metadata: Value,
) -> Result<(), AppError> {
    let mut tx = rls::begin_organization_transaction(pool, organization_id, actor_id).await?;
    record_in_transaction(
        &mut tx,
        organization_id,
        actor_id,
        action,
        entity_type,
        entity_id,
        metadata,
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn record_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    actor_id: Option<Uuid>,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    metadata: Value,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO audit_logs (organization_id, actor_id, action, entity_type, entity_id, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(organization_id)
    .bind(actor_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(metadata)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
