use std::io::ErrorKind;
use std::path::Path;

use sqlx::{Postgres, Transaction};
use tokio::fs;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::{file_security, rls};
use crate::state::AppState;

pub async fn enqueue_media_cleanup(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    media_id: Uuid,
    storage_keys: &[String],
    reason: &str,
) -> Result<(), AppError> {
    for storage_key in storage_keys {
        file_security::secure_join(Path::new("."), storage_key)
            .map_err(|error| AppError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO file_cleanup_jobs (
              organization_id, media_id, storage_key, reason
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (organization_id, storage_key)
              WHERE status IN ('pending', 'retry')
            DO NOTHING
            "#,
        )
        .bind(organization_id)
        .bind(media_id)
        .bind(storage_key)
        .bind(reason)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

pub async fn process_tenant_cleanup_jobs(
    state: &AppState,
    tenant: &TenantContext,
    limit: i64,
) -> Result<u64, AppError> {
    let mut db = rls::tenant_connection(&state.db, tenant).await?;
    let jobs = sqlx::query_as::<_, (Uuid, Uuid, Option<Uuid>, String)>(
        r#"
        SELECT id, organization_id, media_id, storage_key
        FROM file_cleanup_jobs
        WHERE organization_id = $1
          AND status IN ('pending', 'retry')
          AND available_at <= now()
        ORDER BY created_at
        LIMIT $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(limit.clamp(1, 100))
    .fetch_all(db.as_mut())
    .await?;

    let mut completed = 0u64;
    for (job_id, organization_id, media_id, storage_key) in jobs {
        let path = match file_security::secure_join_no_symlinks(
            Path::new(&state.config.upload_dir),
            &storage_key,
        ) {
            Ok(path) => path,
            Err(_) => {
                mark_failed(db.as_mut(), job_id, "unsafe_storage_key").await?;
                continue;
            }
        };
        match fs::remove_file(&path).await {
            Ok(()) => {
                mark_complete(db.as_mut(), job_id).await?;
                completed += 1;
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                mark_complete(db.as_mut(), job_id).await?;
                completed += 1;
            }
            Err(_) => {
                mark_retry(db.as_mut(), job_id, "filesystem_remove_failed").await?;
                continue;
            }
        }

        if let Some(media_id) = media_id {
            let remaining: bool = sqlx::query_scalar(
                r#"
                SELECT EXISTS (
                  SELECT 1
                  FROM file_cleanup_jobs
                  WHERE organization_id = $1
                    AND media_id = $2
                    AND status IN ('pending', 'retry', 'failed')
                )
                "#,
            )
            .bind(organization_id)
            .bind(media_id)
            .fetch_one(db.as_mut())
            .await?;
            if !remaining {
                sqlx::query(
                    "DELETE FROM media WHERE organization_id = $1 AND id = $2 AND lifecycle_status IN ('deletion_pending', 'failed')",
                )
                .bind(organization_id)
                .bind(media_id)
                .execute(db.as_mut())
                .await?;
            }
        }
    }
    Ok(completed)
}

pub async fn reconcile_stale_publishing_media(
    state: &AppState,
    tenant: &TenantContext,
    limit: i64,
) -> Result<u64, AppError> {
    let reconciled = enqueue_stale_publishing_cleanup(&state.db, tenant, limit).await?;
    if reconciled > 0 {
        let _ = process_tenant_cleanup_jobs(state, tenant, limit).await;
    }
    Ok(reconciled)
}

pub async fn enqueue_stale_publishing_cleanup(
    pool: &sqlx::PgPool,
    tenant: &TenantContext,
    limit: i64,
) -> Result<u64, AppError> {
    let mut tx = rls::begin_tenant_transaction(pool, tenant).await?;
    let media_ids = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT id
        FROM media
        WHERE organization_id = $1
          AND lifecycle_status = 'publishing'
          AND updated_at <= now() - interval '15 minutes'
        ORDER BY updated_at, id
        LIMIT $2
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .bind(tenant.organization_id)
    .bind(limit.clamp(1, 100))
    .fetch_all(&mut *tx)
    .await?;

    for media_id in &media_ids {
        let storage_keys = sqlx::query_scalar::<_, String>(
            r#"
            SELECT storage_key
            FROM media
            WHERE id = $1 AND organization_id = $2
            UNION ALL
            SELECT storage_key
            FROM media_variants
            WHERE media_id = $1 AND organization_id = $2
            "#,
        )
        .bind(media_id)
        .bind(tenant.organization_id)
        .fetch_all(&mut *tx)
        .await?;
        sqlx::query(
            "UPDATE media SET lifecycle_status = 'failed', updated_at = now() WHERE id = $1 AND organization_id = $2 AND lifecycle_status = 'publishing'",
        )
        .bind(media_id)
        .bind(tenant.organization_id)
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            "UPDATE media_variants SET lifecycle_status = 'failed' WHERE media_id = $1 AND organization_id = $2 AND lifecycle_status = 'publishing'",
        )
        .bind(media_id)
        .bind(tenant.organization_id)
        .execute(&mut *tx)
        .await?;
        enqueue_media_cleanup(
            &mut tx,
            tenant.organization_id,
            *media_id,
            &storage_keys,
            "orphan_reconciliation",
        )
        .await?;
    }
    tx.commit().await?;
    Ok(media_ids.len() as u64)
}

async fn mark_complete(db: &mut sqlx::PgConnection, job_id: Uuid) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE file_cleanup_jobs SET status = 'complete', attempts = attempts + 1, completed_at = now(), last_error_code = NULL, updated_at = now() WHERE id = $1",
    )
    .bind(job_id)
    .execute(db)
    .await?;
    Ok(())
}

async fn mark_retry(
    db: &mut sqlx::PgConnection,
    job_id: Uuid,
    error_code: &str,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE file_cleanup_jobs
        SET status = CASE WHEN attempts >= 4 THEN 'failed' ELSE 'retry' END,
            attempts = attempts + 1,
            available_at = now() + make_interval(secs => LEAST(3600, (attempts + 1) * 30)),
            last_error_code = $2,
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(job_id)
    .bind(error_code)
    .execute(db)
    .await?;
    Ok(())
}

async fn mark_failed(
    db: &mut sqlx::PgConnection,
    job_id: Uuid,
    error_code: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE file_cleanup_jobs SET status = 'failed', attempts = attempts + 1, last_error_code = $2, updated_at = now() WHERE id = $1",
    )
    .bind(job_id)
    .bind(error_code)
    .execute(db)
    .await?;
    Ok(())
}
