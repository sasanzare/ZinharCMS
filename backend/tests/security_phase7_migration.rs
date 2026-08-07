use std::env;
use std::path::PathBuf;

use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use cms_backend::middleware::tenant::TenantContext;
use cms_backend::services::{file_cleanup, quota, rls};

#[tokio::test]
async fn phase7_file_storage_schema_migrates_on_a_fresh_postgres_database() {
    let Ok(database_url) = env::var("PHASE7_TEST_DATABASE_URL") else {
        eprintln!("skipping Phase 7 migration test: PHASE7_TEST_DATABASE_URL is not set");
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connect Phase 7 test database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("apply migrations through Phase 7");

    let media_columns: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'media'
          AND column_name IN (
            'storage_key', 'source_sha256', 'stored_sha256', 'source_size',
            'visibility', 'verification_status', 'malware_scan_status',
            'lifecycle_status', 'security_metadata', 'published_at',
            'retention_until', 'deleted_at'
          )
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("inspect media columns");
    assert_eq!(media_columns, 12);

    let cleanup_rls: (bool, bool) = sqlx::query_as(
        "SELECT relrowsecurity, relforcerowsecurity FROM pg_class WHERE oid = 'file_cleanup_jobs'::regclass",
    )
    .fetch_one(&pool)
    .await
    .expect("inspect cleanup RLS");
    assert_eq!(cleanup_rls, (true, true));

    let required_constraints: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM pg_constraint
        WHERE conname IN (
          'media_storage_key_safe',
          'media_public_verified_only',
          'media_verified_checksum_required',
          'media_variants_storage_key_safe',
          'file_cleanup_jobs_storage_key_safe',
          'marketplace_versions_artifact_state_supported'
        )
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("inspect file security constraints");
    assert_eq!(required_constraints, 6);

    let marketplace_columns: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'marketplace_versions'
          AND column_name IN (
            'artifact_state', 'malware_scan_status', 'archive_inspected_at',
            'artifact_verified_at', 'artifact_retention_until'
          )
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("inspect Marketplace artifact columns");
    assert_eq!(marketplace_columns, 5);

    let role_flags: (bool, bool) =
        sqlx::query_as("SELECT rolsuper, rolbypassrls FROM pg_roles WHERE rolname = current_user")
            .fetch_one(&pool)
            .await
            .expect("inspect application role");
    assert_eq!(role_flags, (false, false));

    let user_id = Uuid::now_v7();
    let organization_a = Uuid::now_v7();
    let organization_b = Uuid::now_v7();
    let media_a = Uuid::now_v7();
    let media_b = Uuid::now_v7();
    let mut seed = pool.acquire().await.expect("acquire seed connection");
    sqlx::query("SELECT set_config('zinhar.rls_bypass', 'true', false)")
        .execute(&mut *seed)
        .await
        .expect("enable seed bypass");
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name) VALUES ($1, $2, 'phase7-test-hash', 'Phase 7 Test')",
    )
    .bind(user_id)
    .bind(format!("phase7-{}@example.invalid", Uuid::now_v7()))
    .execute(&mut *seed)
    .await
    .expect("insert test user");
    sqlx::query(
        "INSERT INTO organizations (id, name, slug, owner_id) VALUES ($1, 'Tenant A', $2, $3), ($4, 'Tenant B', $5, $3)",
    )
    .bind(organization_a)
    .bind(format!("phase7-a-{}", Uuid::now_v7()))
    .bind(user_id)
    .bind(organization_b)
    .bind(format!("phase7-b-{}", Uuid::now_v7()))
    .execute(&mut *seed)
    .await
    .expect("insert organizations");
    for (media_id, organization_id, suffix) in [
        (media_a, organization_a, "a"),
        (media_b, organization_b, "b"),
    ] {
        sqlx::query(
            r#"
            INSERT INTO media (
              id, organization_id, filename, url, mime_type, size, storage_key,
              source_sha256, stored_sha256, source_size, visibility,
              verification_status, lifecycle_status
            )
            VALUES ($1, $2, 'private.txt', $3, 'text/plain', 16, $4,
                    repeat($5, 64), repeat($5, 64), 16, 'restricted', 'verified', 'active')
            "#,
        )
        .bind(media_id)
        .bind(organization_id)
        .bind(format!("/api/media/{media_id}/download"))
        .bind(format!(
            "private/media/{organization_id}/{media_id}/original.txt"
        ))
        .bind(suffix)
        .execute(&mut *seed)
        .await
        .expect("insert private media");
    }
    drop(seed);

    let tenant_a = TenantContext {
        organization_id: organization_a,
        organization_slug: "tenant-a".to_owned(),
        organization_name: "Tenant A".to_owned(),
        role: "admin".to_owned(),
        user_id,
    };
    let mut tenant_db = rls::tenant_connection(&pool, &tenant_a)
        .await
        .expect("open tenant connection");
    let visible_ids: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM media ORDER BY id")
        .fetch_all(tenant_db.as_mut())
        .await
        .expect("load tenant media");
    assert!(visible_ids.contains(&media_a));
    assert!(!visible_ids.contains(&media_b));
    let cross_tenant_lookup: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM media WHERE id = $1")
            .bind(media_b)
            .fetch_optional(tenant_db.as_mut())
            .await
            .expect("perform cross-tenant lookup");
    assert!(cross_tenant_lookup.is_none());
    drop(tenant_db);

    quota::ensure_default_subscription(&pool, &tenant_a)
        .await
        .expect("create tenant subscription");
    let plan_limit: i64 = {
        let plan = quota::load_current_plan(&pool, &tenant_a)
            .await
            .expect("load tenant plan");
        i64::from(plan.media_limit_mb) * 1_048_576
    };
    let mut reserve = rls::begin_tenant_transaction(&pool, &tenant_a)
        .await
        .expect("begin quota seed transaction");
    let current_usage: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(size), 0)::BIGINT FROM media WHERE organization_id = $1",
    )
    .bind(organization_a)
    .fetch_one(&mut *reserve)
    .await
    .expect("load current media usage");
    sqlx::query(
        r#"
        INSERT INTO media (
          organization_id, filename, url, mime_type, size, storage_key,
          source_sha256, stored_sha256, source_size, visibility,
          verification_status, lifecycle_status
        )
        VALUES ($1, 'quota.bin', '/api/media/quota/download', 'text/plain', $2, $3,
                repeat('c', 64), repeat('c', 64), $2, 'restricted', 'verified', 'active')
        "#,
    )
    .bind(organization_a)
    .bind(plan_limit - current_usage - 100)
    .bind(format!(
        "private/media/{organization_a}/{}/original.txt",
        Uuid::now_v7()
    ))
    .execute(&mut *reserve)
    .await
    .expect("seed near-limit usage");
    reserve.commit().await.expect("commit quota seed");

    let reserve_once = |pool: sqlx::PgPool, tenant: TenantContext| async move {
        let media_id = Uuid::now_v7();
        let mut tx = rls::begin_tenant_transaction(&pool, &tenant).await?;
        quota::ensure_media_capacity_in_transaction(&mut tx, tenant.organization_id, 75).await?;
        sqlx::query(
            r#"
            INSERT INTO media (
              id, organization_id, filename, url, mime_type, size, storage_key,
              source_sha256, stored_sha256, source_size, visibility,
              verification_status, lifecycle_status
            )
            VALUES ($1, $2, 'reservation.txt', $3, 'text/plain', 75, $4,
                    repeat('d', 64), repeat('d', 64), 75, 'restricted', 'verified', 'publishing')
            "#,
        )
        .bind(media_id)
        .bind(tenant.organization_id)
        .bind(format!("/api/media/{media_id}/download"))
        .bind(format!(
            "private/media/{}/{media_id}/original.txt",
            tenant.organization_id
        ))
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok::<(), cms_backend::error::AppError>(())
    };
    let (first, second) = tokio::join!(
        reserve_once(pool.clone(), tenant_a.clone()),
        reserve_once(pool.clone(), tenant_a.clone())
    );
    assert_ne!(first.is_ok(), second.is_ok());

    let stale_media_id = Uuid::now_v7();
    let stale_storage_key = format!("private/media/{organization_a}/{stale_media_id}/original.txt");
    let mut stale = rls::begin_tenant_transaction(&pool, &tenant_a)
        .await
        .expect("begin stale publishing fixture");
    sqlx::query(
        r#"
        INSERT INTO media (
          id, organization_id, filename, url, mime_type, size, storage_key,
          source_sha256, stored_sha256, source_size, visibility,
          verification_status, lifecycle_status, updated_at
        )
        VALUES ($1, $2, 'stale.txt', $3, 'text/plain', 10, $4,
                repeat('e', 64), repeat('e', 64), 10, 'restricted',
                'verified', 'publishing', now() - interval '1 hour')
        "#,
    )
    .bind(stale_media_id)
    .bind(organization_a)
    .bind(format!("/api/media/{stale_media_id}/download"))
    .bind(&stale_storage_key)
    .execute(&mut *stale)
    .await
    .expect("insert stale publishing media");
    stale.commit().await.expect("commit stale fixture");

    assert_eq!(
        file_cleanup::enqueue_stale_publishing_cleanup(&pool, &tenant_a, 10)
            .await
            .expect("reconcile stale publishing media"),
        1
    );
    let mut reconciled = rls::tenant_connection(&pool, &tenant_a)
        .await
        .expect("open reconciliation verification connection");
    let lifecycle: String = sqlx::query_scalar("SELECT lifecycle_status FROM media WHERE id = $1")
        .bind(stale_media_id)
        .fetch_one(reconciled.as_mut())
        .await
        .expect("load reconciled lifecycle");
    assert_eq!(lifecycle, "failed");
    let cleanup: (String, String) = sqlx::query_as(
        "SELECT reason, status FROM file_cleanup_jobs WHERE media_id = $1 AND storage_key = $2",
    )
    .bind(stale_media_id)
    .bind(stale_storage_key)
    .fetch_one(reconciled.as_mut())
    .await
    .expect("load reconciliation cleanup job");
    assert_eq!(
        cleanup,
        ("orphan_reconciliation".to_owned(), "pending".to_owned())
    );
    drop(reconciled);

    pool.close().await;
}

#[tokio::test]
async fn phase7_upgrade_classifies_legacy_media_as_restricted_and_unverified() {
    let Ok(database_url) = env::var("PHASE7_UPGRADE_TEST_DATABASE_URL") else {
        eprintln!("skipping Phase 7 upgrade test: PHASE7_UPGRADE_TEST_DATABASE_URL is not set");
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("connect Phase 7 upgrade database");
    let migrations = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut files = std::fs::read_dir(&migrations)
        .expect("read migrations")
        .map(|entry| entry.expect("migration entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".sql") && name < "0030_")
        })
        .collect::<Vec<_>>();
    files.sort();
    for path in files {
        let sql = std::fs::read_to_string(&path).expect("read pre-Phase 7 migration");
        sqlx::raw_sql(&sql)
            .execute(&pool)
            .await
            .unwrap_or_else(|error| panic!("apply {}: {error}", path.display()));
    }

    sqlx::query("SELECT set_config('zinhar.rls_bypass', 'true', false)")
        .execute(&pool)
        .await
        .expect("enable migration-test bypass");
    let organization_id: Uuid =
        sqlx::query_scalar("SELECT id FROM organizations WHERE slug = 'default'")
            .fetch_one(&pool)
            .await
            .expect("load default organization");
    let media_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO media (id, organization_id, filename, url, mime_type, size)
        VALUES ($1, $2, 'legacy-report.pdf', $3, 'application/pdf', 128)
        "#,
    )
    .bind(media_id)
    .bind(organization_id)
    .bind(format!("/uploads/{organization_id}/{media_id}.pdf"))
    .execute(&pool)
    .await
    .expect("insert legacy media");

    let phase7_sql =
        std::fs::read_to_string(migrations.join("0030_security_phase_seven_file_storage.sql"))
            .expect("read Phase 7 migration");
    sqlx::raw_sql(&phase7_sql)
        .execute(&pool)
        .await
        .expect("apply Phase 7 upgrade migration");

    let classification: (String, String, String, String) = sqlx::query_as(
        "SELECT visibility, verification_status, malware_scan_status, storage_key FROM media WHERE id = $1",
    )
    .bind(media_id)
    .fetch_one(&pool)
    .await
    .expect("load migrated media");
    assert_eq!(classification.0, "restricted");
    assert_eq!(classification.1, "legacy_unverified");
    assert_eq!(classification.2, "unavailable");
    assert_eq!(
        classification.3,
        format!("{organization_id}/{media_id}.pdf")
    );

    let public_rejected = sqlx::query("UPDATE media SET visibility = 'public' WHERE id = $1")
        .bind(media_id)
        .execute(&pool)
        .await;
    assert!(public_rejected.is_err());
    pool.close().await;
}
