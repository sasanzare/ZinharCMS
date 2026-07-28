use std::borrow::Cow;
use std::env;
use std::str::FromStr;

use cms_backend::services::rls;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, PgConnection, PgPool, Row};
use uuid::Uuid;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

struct Fixture {
    organization_a: Uuid,
    organization_b: Uuid,
    owner_a: Uuid,
    member_a: Uuid,
    owner_b: Uuid,
    content_a: Uuid,
    content_b: Uuid,
    slug_prefix: String,
}

async fn phase2_pool(max_connections: u32) -> Option<PgPool> {
    let database_url = env::var("PHASE2_TEST_DATABASE_URL").ok()?;
    Some(
        PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url)
            .await
            .expect("the dedicated Phase 2 database must be reachable"),
    )
}

async fn create_user(pool: &PgPool, label: &str) -> Uuid {
    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, 'phase2-test-password-hash', $2)
        RETURNING id
        "#,
    )
    .bind(format!("{label}-{}@example.invalid", Uuid::now_v7()))
    .bind(label)
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        SELECT $1, id FROM roles WHERE name = 'author'
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await
    .unwrap();
    user_id
}

async fn create_fixture(pool: &PgPool) -> Fixture {
    let owner_a = create_user(pool, "phase2-owner-a").await;
    let member_a = create_user(pool, "phase2-member-a").await;
    let owner_b = create_user(pool, "phase2-owner-b").await;
    let slug_prefix = format!("phase2-{}", Uuid::now_v7().simple());
    let organization_a: Uuid = sqlx::query_scalar(
        "INSERT INTO organizations (name, slug, owner_id) VALUES ('Phase 2 A', $1, $2) RETURNING id",
    )
    .bind(format!("{slug_prefix}-a"))
    .bind(owner_a)
    .fetch_one(pool)
    .await
    .unwrap();
    let organization_b: Uuid = sqlx::query_scalar(
        "INSERT INTO organizations (name, slug, owner_id) VALUES ('Phase 2 B', $1, $2) RETURNING id",
    )
    .bind(format!("{slug_prefix}-b"))
    .bind(owner_b)
    .fetch_one(pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
        VALUES
          ($1, $2, 'owner', 'active', now()),
          ($1, $3, 'author', 'active', now()),
          ($4, $5, 'owner', 'active', now())
        "#,
    )
    .bind(organization_a)
    .bind(owner_a)
    .bind(member_a)
    .bind(organization_b)
    .bind(owner_b)
    .execute(pool)
    .await
    .unwrap();

    let mut bypass = rls::begin_bypass_transaction(pool).await.unwrap();
    let content_a: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO content_types (organization_id, name, slug, created_by)
        VALUES ($1, 'Phase 2 Content A', $2, $3)
        RETURNING id
        "#,
    )
    .bind(organization_a)
    .bind(format!("{slug_prefix}-content-a"))
    .bind(owner_a)
    .fetch_one(&mut *bypass)
    .await
    .unwrap();
    let content_b: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO content_types (organization_id, name, slug, created_by)
        VALUES ($1, 'Phase 2 Content B', $2, $3)
        RETURNING id
        "#,
    )
    .bind(organization_b)
    .bind(format!("{slug_prefix}-content-b"))
    .bind(owner_b)
    .fetch_one(&mut *bypass)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO public_settings (organization_id, key, value)
        VALUES
          ($1, $3, '"a"'::jsonb),
          ($2, $3, '"b"'::jsonb)
        "#,
    )
    .bind(organization_a)
    .bind(organization_b)
    .bind(format!("{}_setting", slug_prefix.replace('-', "_")))
    .execute(&mut *bypass)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO webhooks (organization_id, name, url, events, secret)
        VALUES
          ($1, 'Phase 2 Webhook A', 'https://a.example.invalid/hook', ARRAY['page.publish'], 'phase2-test-secret-a'),
          ($2, 'Phase 2 Webhook B', 'https://b.example.invalid/hook', ARRAY['page.publish'], 'phase2-test-secret-b')
        "#,
    )
    .bind(organization_a)
    .bind(organization_b)
    .execute(&mut *bypass)
    .await
    .unwrap();
    bypass.commit().await.unwrap();

    Fixture {
        organization_a,
        organization_b,
        owner_a,
        member_a,
        owner_b,
        content_a,
        content_b,
        slug_prefix,
    }
}

async fn cleanup_fixture(pool: &PgPool, fixture: &Fixture) {
    sqlx::query("DELETE FROM organizations WHERE id = ANY($1)")
        .bind(vec![fixture.organization_a, fixture.organization_b])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE id = ANY($1)")
        .bind(vec![fixture.owner_a, fixture.member_a, fixture.owner_b])
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn live_rls_catalog_crud_and_context_cleanup_matrix() {
    let Some(pool) = phase2_pool(10).await else {
        return;
    };
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let fixture = create_fixture(&pool).await;
    let is_superuser: bool =
        sqlx::query_scalar("SELECT rolsuper FROM pg_roles WHERE rolname = current_user")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        !is_superuser,
        "the live RLS matrix must not run as a superuser"
    );

    let enabled_tables: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::bigint
        FROM pg_class table_class
        JOIN pg_namespace namespace ON namespace.oid = table_class.relnamespace
        WHERE namespace.nspname = 'public'
          AND table_class.relkind = 'r'
          AND table_class.relrowsecurity
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let forced_tables: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::bigint
        FROM pg_class table_class
        JOIN pg_namespace namespace ON namespace.oid = table_class.relnamespace
        WHERE namespace.nspname = 'public'
          AND table_class.relkind = 'r'
          AND table_class.relforcerowsecurity
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let policy_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::bigint
        FROM pg_policy policy
        JOIN pg_class table_class ON table_class.oid = policy.polrelid
        JOIN pg_namespace namespace ON namespace.oid = table_class.relnamespace
        WHERE namespace.nspname = 'public'
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let tenant_owned_tables: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT table_name)::bigint
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND column_name = 'organization_id'
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let tenant_tables_without_rls: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT DISTINCT columns.table_name
        FROM information_schema.columns columns
        JOIN pg_class table_class ON table_class.relname = columns.table_name
        JOIN pg_namespace namespace ON namespace.oid = table_class.relnamespace
        WHERE columns.table_schema = 'public'
          AND columns.column_name = 'organization_id'
          AND namespace.nspname = 'public'
          AND NOT table_class.relrowsecurity
        ORDER BY columns.table_name
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert!(enabled_tables > 0);
    assert_eq!(forced_tables, enabled_tables);
    assert!(policy_count >= enabled_tables);

    let mut matrix_cases = 1_i64;
    let mut tenant_a =
        rls::organization_connection(&pool, fixture.organization_a, Some(fixture.owner_a))
            .await
            .unwrap();
    let visible_a: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM content_types WHERE slug LIKE $1 ORDER BY id")
            .bind(format!("{}%", fixture.slug_prefix))
            .fetch_all(tenant_a.as_mut())
            .await
            .unwrap();
    assert!(visible_a.contains(&fixture.content_a));
    assert!(!visible_a.contains(&fixture.content_b));
    matrix_cases += 2;

    let updated_b = sqlx::query("UPDATE content_types SET name = 'blocked' WHERE id = $1")
        .bind(fixture.content_b)
        .execute(tenant_a.as_mut())
        .await
        .unwrap();
    assert_eq!(updated_b.rows_affected(), 0);
    matrix_cases += 1;
    let deleted_b = sqlx::query("DELETE FROM content_types WHERE id = $1")
        .bind(fixture.content_b)
        .execute(tenant_a.as_mut())
        .await
        .unwrap();
    assert_eq!(deleted_b.rows_affected(), 0);
    matrix_cases += 1;
    let cross_tenant_insert = sqlx::query(
        "INSERT INTO content_types (organization_id, name, slug) VALUES ($1, 'blocked', $2)",
    )
    .bind(fixture.organization_b)
    .bind(format!("{}-blocked", fixture.slug_prefix))
    .execute(tenant_a.as_mut())
    .await;
    assert!(cross_tenant_insert.is_err());
    matrix_cases += 1;
    sqlx::query(
        "INSERT INTO content_types (organization_id, name, slug) VALUES ($1, 'allowed', $2)",
    )
    .bind(fixture.organization_a)
    .bind(format!("{}-allowed-a", fixture.slug_prefix))
    .execute(tenant_a.as_mut())
    .await
    .unwrap();
    matrix_cases += 1;
    drop(tenant_a);

    let mut tenant_b =
        rls::organization_connection(&pool, fixture.organization_b, Some(fixture.owner_b))
            .await
            .unwrap();
    let visible_b: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM content_types WHERE slug LIKE $1 ORDER BY id")
            .bind(format!("{}%", fixture.slug_prefix))
            .fetch_all(tenant_b.as_mut())
            .await
            .unwrap();
    assert!(visible_b.contains(&fixture.content_b));
    assert!(!visible_b.contains(&fixture.content_a));
    matrix_cases += 2;
    let updated_a = sqlx::query("UPDATE content_types SET name = 'blocked' WHERE id = $1")
        .bind(fixture.content_a)
        .execute(tenant_b.as_mut())
        .await
        .unwrap();
    assert_eq!(updated_a.rows_affected(), 0);
    matrix_cases += 1;
    drop(tenant_b);

    let missing_context_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM content_types WHERE slug LIKE $1")
            .bind(format!("{}%", fixture.slug_prefix))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(missing_context_count, 0);
    matrix_cases += 1;
    let mut invalid_context = rls::organization_connection(&pool, Uuid::now_v7(), None)
        .await
        .unwrap();
    let invalid_context_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM content_types WHERE slug LIKE $1")
            .bind(format!("{}%", fixture.slug_prefix))
            .fetch_one(invalid_context.as_mut())
            .await
            .unwrap();
    assert_eq!(invalid_context_count, 0);
    matrix_cases += 1;
    drop(invalid_context);

    let cleanup_pool = phase2_pool(1).await.unwrap();
    let mut tenant_tx = rls::begin_organization_transaction(
        &cleanup_pool,
        fixture.organization_a,
        Some(fixture.member_a),
    )
    .await
    .unwrap();
    let current_user: Option<Uuid> = sqlx::query_scalar("SELECT app_current_user_id()")
        .fetch_one(&mut *tenant_tx)
        .await
        .unwrap();
    let bypass: bool = sqlx::query_scalar("SELECT app_rls_bypass_enabled()")
        .fetch_one(&mut *tenant_tx)
        .await
        .unwrap();
    assert_eq!(current_user, Some(fixture.member_a));
    assert!(!bypass);
    tenant_tx.commit().await.unwrap();
    matrix_cases += 2;

    let current_org: Option<Uuid> = sqlx::query_scalar("SELECT app_current_organization_id()")
        .fetch_one(&cleanup_pool)
        .await
        .unwrap();
    let bypass: bool = sqlx::query_scalar("SELECT app_rls_bypass_enabled()")
        .fetch_one(&cleanup_pool)
        .await
        .unwrap();
    assert!(current_org.is_none());
    assert!(!bypass);
    matrix_cases += 2;

    let mut failed_tx = rls::begin_organization_transaction(
        &cleanup_pool,
        fixture.organization_a,
        Some(fixture.owner_a),
    )
    .await
    .unwrap();
    assert!(
        sqlx::query("SELECT definitely_missing_phase2_column FROM content_types")
            .execute(&mut *failed_tx)
            .await
            .is_err()
    );
    failed_tx.rollback().await.unwrap();
    let current_org: Option<Uuid> = sqlx::query_scalar("SELECT app_current_organization_id()")
        .fetch_one(&cleanup_pool)
        .await
        .unwrap();
    assert!(current_org.is_none());
    matrix_cases += 1;

    let mut bypass_tx = rls::begin_bypass_transaction(&cleanup_pool).await.unwrap();
    let bypass_visible: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM content_types WHERE slug LIKE $1")
            .bind(format!("{}%", fixture.slug_prefix))
            .fetch_one(&mut *bypass_tx)
            .await
            .unwrap();
    assert!(bypass_visible >= 3);
    bypass_tx.commit().await.unwrap();
    let bypass: bool = sqlx::query_scalar("SELECT app_rls_bypass_enabled()")
        .fetch_one(&cleanup_pool)
        .await
        .unwrap();
    assert!(!bypass);
    matrix_cases += 2;

    let mut bypass_rollback = rls::begin_bypass_transaction(&cleanup_pool).await.unwrap();
    assert!(
        sqlx::query_scalar::<_, bool>("SELECT app_rls_bypass_enabled()")
            .fetch_one(&mut *bypass_rollback)
            .await
            .unwrap()
    );
    bypass_rollback.rollback().await.unwrap();
    let bypass: bool = sqlx::query_scalar("SELECT app_rls_bypass_enabled()")
        .fetch_one(&cleanup_pool)
        .await
        .unwrap();
    assert!(!bypass);
    matrix_cases += 1;

    let operation_counts = sqlx::query(
        r#"
        SELECT polcmd, COUNT(*)::bigint AS count
        FROM pg_policy policy
        JOIN pg_class table_class ON table_class.oid = policy.polrelid
        JOIN pg_namespace namespace ON namespace.oid = table_class.relnamespace
        WHERE namespace.nspname = 'public'
        GROUP BY polcmd
        ORDER BY polcmd
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| {
        format!(
            "{}={}",
            row.get::<i8, _>("polcmd") as u8 as char,
            row.get::<i64, _>("count")
        )
    })
    .collect::<Vec<_>>()
    .join(",");

    println!(
        "PHASE2_RLS_METRICS enabled={enabled_tables} forced={forced_tables} policies={policy_count} tenant_owned={tenant_owned_tables} cases={matrix_cases} passed={matrix_cases} operations={operation_counts} non_rls_tenant_tables={}",
        tenant_tables_without_rls.join(",")
    );

    cleanup_fixture(&pool, &fixture).await;
    cleanup_pool.close().await;
    pool.close().await;
}

#[tokio::test]
async fn phase2_migration_upgrades_legacy_refresh_rows_safely() {
    let Ok(bootstrap_url) = env::var("PHASE2_BOOTSTRAP_DATABASE_URL") else {
        return;
    };
    let Ok(app_url) = env::var("PHASE2_TEST_DATABASE_URL") else {
        return;
    };
    let app_user = env::var("PHASE2_DB_USER").unwrap();
    assert!(
        app_user
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    );
    let database_name = format!("zinhar_phase2_upgrade_{}", Uuid::now_v7().simple());
    let mut bootstrap_options = PgConnectOptions::from_str(&bootstrap_url).unwrap();
    bootstrap_options = bootstrap_options.database("postgres");
    let mut bootstrap = PgConnection::connect_with(&bootstrap_options)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE DATABASE {database_name} OWNER {app_user}"))
        .execute(&mut bootstrap)
        .await
        .unwrap();

    let outcome = run_upgrade_path(&app_url, &database_name).await;

    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1 AND pid <> pg_backend_pid()",
    )
    .bind(&database_name)
    .execute(&mut bootstrap)
    .await
    .unwrap();
    assert!(database_name.starts_with("zinhar_phase2_upgrade_"));
    sqlx::query(&format!("DROP DATABASE {database_name}"))
        .execute(&mut bootstrap)
        .await
        .unwrap();
    bootstrap.close().await.unwrap();

    outcome.unwrap();
}

async fn run_upgrade_path(app_url: &str, database_name: &str) -> anyhow::Result<()> {
    let mut app_options = PgConnectOptions::from_str(app_url)?;
    app_options = app_options.database(database_name);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(app_options)
        .await?;
    let phase1_migrator = Migrator {
        migrations: Cow::Owned(
            MIGRATOR
                .iter()
                .filter(|migration| migration.version <= 26)
                .cloned()
                .collect(),
        ),
        ignore_missing: false,
        locking: true,
        no_tx: false,
    };
    phase1_migrator.run(&pool).await?;

    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, 'legacy-password-hash', 'Legacy Session User')
        RETURNING id
        "#,
    )
    .bind(format!("legacy-{}@example.invalid", Uuid::now_v7()))
    .fetch_one(&pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        SELECT $1, id FROM roles WHERE name = 'author'
        "#,
    )
    .bind(user_id)
    .execute(&pool)
    .await?;
    let legacy_token_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, 'legacy-token-hash-only', now() + INTERVAL '1 day')
        RETURNING id
        "#,
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    MIGRATOR.run(&pool).await?;
    let row = sqlx::query(
        r#"
        SELECT family_id, revoked_at
        FROM refresh_tokens
        WHERE id = $1
        "#,
    )
    .bind(legacy_token_id)
    .fetch_one(&pool)
    .await?;
    anyhow::ensure!(row.get::<Uuid, _>("family_id") == legacy_token_id);
    anyhow::ensure!(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("revoked_at")
            .is_some()
    );
    let family_revoked: Option<chrono::DateTime<chrono::Utc>> =
        sqlx::query_scalar("SELECT revoked_at FROM refresh_token_families WHERE id = $1")
            .bind(legacy_token_id)
            .fetch_one(&pool)
            .await?;
    anyhow::ensure!(family_revoked.is_some());
    let auth_version: i64 = sqlx::query_scalar("SELECT auth_version FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await?;
    anyhow::ensure!(auth_version == 1);
    let migration_version: i64 = sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
        .fetch_one(&pool)
        .await?;
    anyhow::ensure!(migration_version == 28);

    pool.close().await;
    Ok(())
}
