pub const TENANT_RLS_TABLES: &[&str] = &[
    "content_types",
    "content_entries",
    "pages",
    "page_versions",
    "media",
    "media_variants",
    "comments",
    "webhooks",
    "webhook_deliveries",
    "public_settings",
    "navigation_items",
    "component_registry",
    "organization_subscriptions",
    "usage_counters",
    "billing_events",
    "organization_domains",
    "organization_rate_limits",
    "audit_logs",
    "email_deliveries",
    "saas_alert_rules",
    "beta_participants",
    "beta_feedback",
    "beta_ga_blockers",
    "marketplace_installations",
];

pub const PHASE8_LOAD_SMOKE_ENDPOINTS: &[&str] = &[
    "/health",
    "/ready",
    "/api/billing/plans",
    "/api/organizations/current",
    "/api/billing/usage",
    "/api/content-types",
    "/api/pages",
    "/api/media",
];

#[cfg(test)]
mod tests {
    use super::{PHASE8_LOAD_SMOKE_ENDPOINTS, TENANT_RLS_TABLES};

    const RLS_MIGRATION: &str = include_str!("../../migrations/0009_v2_phase_three_rls.sql");
    const BILLING_MIGRATION: &str =
        include_str!("../../migrations/0010_v2_phase_five_billing_quota.sql");
    const STRIPE_MIGRATION: &str =
        include_str!("../../migrations/0011_v2_phase_six_stripe_billing.sql");
    const SAAS_OPS_MIGRATION: &str =
        include_str!("../../migrations/0012_v2_phase_seven_saas_ops.sql");
    const PHASE8_MIGRATION: &str =
        include_str!("../../migrations/0013_v2_phase_eight_hardening.sql");
    const PHASE9_MIGRATION: &str =
        include_str!("../../migrations/0014_v2_phase_nine_beta_release.sql");
    const V3_MARKETPLACE_MIGRATION: &str =
        include_str!("../../migrations/0015_v3_phase_one_marketplace_foundation.sql");

    #[test]
    fn tenant_tables_have_forced_rls_coverage() {
        let migrations = [
            RLS_MIGRATION,
            BILLING_MIGRATION,
            STRIPE_MIGRATION,
            SAAS_OPS_MIGRATION,
            PHASE9_MIGRATION,
            V3_MARKETPLACE_MIGRATION,
        ]
        .join("\n");

        for table in TENANT_RLS_TABLES {
            let direct_force = format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY");
            let looped_force = format!("'{table}'");
            assert!(
                migrations.contains(&direct_force) || migrations.contains(&looped_force),
                "{table} is missing forced RLS coverage"
            );
        }
    }

    #[test]
    fn tenant_policy_migrations_use_tenant_context_helper() {
        let migrations = [
            RLS_MIGRATION,
            BILLING_MIGRATION,
            STRIPE_MIGRATION,
            PHASE9_MIGRATION,
            SAAS_OPS_MIGRATION,
        ]
        .join("\n");

        assert!(migrations.contains("app_rls_tenant_matches(organization_id)"));
        assert!(migrations.contains("app_rls_bypass_enabled()"));
        assert!(migrations.contains("app_current_organization_id()"));
    }

    #[test]
    fn phase8_migration_is_additive_and_idempotent() {
        assert!(PHASE8_MIGRATION.contains("ADD COLUMN IF NOT EXISTS provider_event_created_at"));
        assert!(PHASE8_MIGRATION.contains("CREATE INDEX IF NOT EXISTS"));
        assert!(!PHASE8_MIGRATION.contains("DROP TABLE"));
        assert!(!PHASE8_MIGRATION.contains("DROP COLUMN"));
    }

    #[test]
    fn phase9_migration_adds_beta_release_tables() {
        assert!(PHASE9_MIGRATION.contains("CREATE TABLE IF NOT EXISTS beta_participants"));
        assert!(PHASE9_MIGRATION.contains("CREATE TABLE IF NOT EXISTS beta_feedback"));
        assert!(PHASE9_MIGRATION.contains("CREATE TABLE IF NOT EXISTS beta_ga_blockers"));
        assert!(PHASE9_MIGRATION.contains("ALTER TABLE beta_feedback FORCE ROW LEVEL SECURITY"));
        assert!(!PHASE9_MIGRATION.contains("DROP TABLE"));
        assert!(!PHASE9_MIGRATION.contains("DROP COLUMN"));
    }

    #[test]
    fn load_smoke_manifest_covers_public_and_tenant_paths() {
        assert!(
            PHASE8_LOAD_SMOKE_ENDPOINTS
                .iter()
                .any(|path| *path == "/health")
        );
        assert!(
            PHASE8_LOAD_SMOKE_ENDPOINTS
                .iter()
                .any(|path| path.starts_with("/api/billing"))
        );
        assert!(
            PHASE8_LOAD_SMOKE_ENDPOINTS
                .iter()
                .any(|path| path.starts_with("/api/organizations"))
        );
    }
}
