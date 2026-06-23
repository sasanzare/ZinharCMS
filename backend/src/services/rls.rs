use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;

pub struct TenantConnection {
    connection: PoolConnection<Postgres>,
}

impl TenantConnection {
    pub fn as_mut(&mut self) -> &mut PgConnection {
        self.connection.as_mut()
    }
}

pub async fn tenant_connection(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<TenantConnection, AppError> {
    organization_connection(pool, tenant.organization_id, Some(tenant.user_id)).await
}

pub async fn organization_connection(
    pool: &PgPool,
    organization_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<TenantConnection, AppError> {
    let mut connection = pool.acquire().await?;
    set_context_on_connection(connection.as_mut(), organization_id, user_id).await?;
    connection.close_on_drop();
    Ok(TenantConnection { connection })
}

pub async fn begin_tenant_transaction(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<Transaction<'static, Postgres>, AppError> {
    begin_organization_transaction(pool, tenant.organization_id, Some(tenant.user_id)).await
}

pub async fn begin_organization_transaction(
    pool: &PgPool,
    organization_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<Transaction<'static, Postgres>, AppError> {
    let mut tx = pool.begin().await?;
    set_context_on_transaction(&mut tx, organization_id, user_id).await?;
    Ok(tx)
}

async fn set_context_on_connection(
    connection: &mut PgConnection,
    organization_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        SELECT set_config('zinhar.organization_id', $1, false),
               set_config('zinhar.user_id', $2, false),
               set_config('zinhar.rls_bypass', 'false', false)
        "#,
    )
    .bind(organization_id.to_string())
    .bind(user_id.map(|id| id.to_string()).unwrap_or_default())
    .execute(connection)
    .await?;

    Ok(())
}

async fn set_context_on_transaction(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        SELECT set_config('zinhar.organization_id', $1, true),
               set_config('zinhar.user_id', $2, true),
               set_config('zinhar.rls_bypass', 'false', true)
        "#,
    )
    .bind(organization_id.to_string())
    .bind(user_id.map(|id| id.to_string()).unwrap_or_default())
    .execute(&mut **tx)
    .await?;

    Ok(())
}
