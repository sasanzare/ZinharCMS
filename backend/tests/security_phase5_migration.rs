use std::env;
use std::path::PathBuf;

use sqlx::Row;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[tokio::test]
async fn phase5_upgrade_preserves_existing_session_families_under_unprivileged_role() {
    let Ok(database_url) = env::var("PHASE5_UPGRADE_TEST_DATABASE_URL") else {
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("the disposable Phase 5 upgrade database must be reachable");
    let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut migrations = std::fs::read_dir(&migrations_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    migrations.sort();

    for path in migrations.iter().filter(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name <= "0027_security_phase_two_sessions.sql")
    }) {
        let sql = std::fs::read_to_string(path).unwrap();
        sqlx::raw_sql(&sql).execute(&pool).await.unwrap();
    }

    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, 'upgrade-test-password-hash', 'Phase 5 Upgrade Test')
        RETURNING id
        "#,
    )
    .bind(format!("phase5-upgrade-{}@example.invalid", Uuid::now_v7()))
    .fetch_one(&pool)
    .await
    .unwrap();
    let family_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO refresh_token_families (id, user_id, expires_at)
        VALUES ($1, $2, now() + INTERVAL '1 day')
        "#,
    )
    .bind(family_id)
    .bind(user_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, family_id, token_hash, expires_at)
        VALUES ($1, $2, $3, now() + INTERVAL '1 day')
        "#,
    )
    .bind(user_id)
    .bind(family_id)
    .bind(format!("upgrade-test-hash-{}", Uuid::now_v7()))
    .execute(&pool)
    .await
    .unwrap();

    let phase5_sql = std::fs::read_to_string(
        migrations_dir.join("0028_security_phase_five_key_session_recovery.sql"),
    )
    .unwrap();
    sqlx::raw_sql(&phase5_sql).execute(&pool).await.unwrap();

    let row = sqlx::query(
        r#"
        SELECT id, public_id, last_used_at, revoked_at
        FROM refresh_token_families
        WHERE id = $1
        "#,
    )
    .bind(family_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.get::<Uuid, _>("id"), family_id);
    assert_ne!(row.get::<Uuid, _>("public_id"), family_id);
    assert!(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("last_used_at")
            .is_some()
    );
    assert!(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("revoked_at")
            .is_none()
    );

    let role = sqlx::query(
        r#"
        SELECT role.rolsuper, role.rolbypassrls
        FROM pg_roles role
        WHERE role.rolname = current_user
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(!role.get::<bool, _>("rolsuper"));
    assert!(!role.get::<bool, _>("rolbypassrls"));
}
