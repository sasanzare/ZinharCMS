use std::env;
use std::path::PathBuf;

use sqlx::Row;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[tokio::test]
async fn phase6_upgrade_preserves_sessions_and_adds_fail_closed_mfa_constraints() {
    let Ok(database_url) = env::var("PHASE6_UPGRADE_TEST_DATABASE_URL") else {
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("the disposable Phase 6 upgrade database must be reachable");
    let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut migrations = std::fs::read_dir(&migrations_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    migrations.sort();

    for path in migrations.iter().filter(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name <= "0028_security_phase_five_key_session_recovery.sql")
    }) {
        let sql = std::fs::read_to_string(path).unwrap();
        sqlx::raw_sql(&sql).execute(&pool).await.unwrap();
    }

    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, name, auth_version)
        VALUES ($1, 'phase6-upgrade-password-hash', 'Phase 6 Upgrade', 7)
        RETURNING id
        "#,
    )
    .bind(format!("phase6-upgrade-{}@example.invalid", Uuid::now_v7()))
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

    let phase6_sql =
        std::fs::read_to_string(migrations_dir.join("0029_security_phase_six_mfa_step_up.sql"))
            .unwrap();
    sqlx::raw_sql(&phase6_sql).execute(&pool).await.unwrap();

    let row = sqlx::query(
        r#"
        SELECT assurance_level,
               authentication_methods,
               authenticated_at,
               mfa_authenticated_at,
               auth_version_at_issue
        FROM refresh_token_families
        WHERE id = $1
        "#,
    )
    .bind(family_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.get::<i16, _>("assurance_level"), 1);
    assert_eq!(
        row.get::<Vec<String>, _>("authentication_methods"),
        vec!["pwd".to_owned()]
    );
    assert!(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("authenticated_at")
            .is_some()
    );
    assert!(
        row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("mfa_authenticated_at")
            .is_none()
    );
    assert_eq!(row.get::<i64, _>("auth_version_at_issue"), 7);

    let invalid_aal2 = sqlx::query(
        r#"
        UPDATE refresh_token_families
        SET assurance_level = 2,
            authentication_methods = ARRAY['pwd', 'totp']::TEXT[]
        WHERE id = $1
        "#,
    )
    .bind(family_id)
    .execute(&pool)
    .await;
    assert!(invalid_aal2.is_err());

    let invalid_plaintext_sized_secret = sqlx::query(
        r#"
        INSERT INTO user_mfa (
          user_id,
          status,
          secret_ciphertext,
          secret_nonce,
          encryption_kid,
          encryption_version,
          pending_expires_at
        )
        VALUES ($1, 'pending', $2, $3, 'mfa-test', 1, now() + INTERVAL '10 minutes')
        "#,
    )
    .bind(user_id)
    .bind(b"plaintext-secret".as_slice())
    .bind([0_u8; 12].as_slice())
    .execute(&pool)
    .await;
    assert!(invalid_plaintext_sized_secret.is_err());
}
