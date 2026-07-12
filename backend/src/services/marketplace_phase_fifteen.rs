#[cfg(test)]
mod tests {
    const PHASE_FIFTEEN_DOC: &str = include_str!("../../../docs/V3_PHASE_FIFTEEN.md");
    const OPERATIONS_RUNBOOK: &str =
        include_str!("../../../docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md");
    const RELEASE_NOTES: &str = include_str!("../../../docs/V3_MARKETPLACE_RELEASE_NOTES.md");
    const MARKETPLACE_POLICY: &str = include_str!("../../../docs/V3_MARKETPLACE_POLICY.md");
    const GA_CHECK_SCRIPT: &str = include_str!("../../../scripts/marketplace-phase15-ga-check.ps1");
    const PHASE_FIFTEEN_DIAGRAM: &str =
        include_str!("../../../docs/diagrams/42-marketplace-launch-ga.mmd");
    const MARKETPLACE_ROUTES: &str = include_str!("../routes/marketplace.rs");
    const MARKETPLACE_FINANCE_ROUTES: &str = include_str!("../routes/marketplace_finance.rs");
    const MARKETPLACE_ANALYTICS_ROUTES: &str = include_str!("../routes/marketplace_analytics.rs");
    const BETA_ROUTES: &str = include_str!("../routes/beta.rs");

    fn combined_phase_text() -> String {
        [
            PHASE_FIFTEEN_DOC,
            OPERATIONS_RUNBOOK,
            RELEASE_NOTES,
            MARKETPLACE_POLICY,
            GA_CHECK_SCRIPT,
            PHASE_FIFTEEN_DIAGRAM,
        ]
        .join("\n")
        .to_lowercase()
    }

    #[test]
    fn phase_fifteen_launch_readiness_contract_is_documented() {
        let text = combined_phase_text();

        for required in [
            "15.1 launch readiness",
            "runbook",
            "final policy",
            "support workflow",
            "rollback",
            "incident checklist",
            "broken install",
            "malicious product",
            "wrong payment",
            "emergency block",
        ] {
            assert!(
                text.contains(required),
                "missing Phase 15.1 launch readiness contract: {required}"
            );
        }
    }

    #[test]
    fn phase_fifteen_general_availability_contract_is_documented() {
        let text = combined_phase_text();

        for required in [
            "15.2 general availability",
            "release notes",
            "public docs",
            "monitoring dashboard",
            "support plan",
            "production",
            "approved products",
            "marketplace is enabled for installing approved products in production",
        ] {
            assert!(
                text.contains(required),
                "missing Phase 15.2 GA contract: {required}"
            );
        }
    }

    #[test]
    fn phase_fifteen_uses_existing_operational_surfaces_only() {
        for required in [
            "/api/marketplace/installations",
            "/api/marketplace/installations/{installation_id}/rollback",
            "/api/marketplace/reports",
        ] {
            assert!(
                MARKETPLACE_ROUTES.contains(required),
                "missing existing Marketplace route needed by Phase 15 operations: {required}"
            );
        }

        assert!(MARKETPLACE_FINANCE_ROUTES.contains("/api/marketplace/purchases"));
        assert!(
            MARKETPLACE_ANALYTICS_ROUTES.contains("/api/marketplace/analytics/admin"),
            "Phase 15 monitoring dashboard should use existing Marketplace admin analytics"
        );
        assert!(BETA_ROUTES.contains("/api/beta/ga-blockers"));
    }

    #[test]
    fn phase_fifteen_ga_script_runs_expected_checks_without_new_ga_api() {
        for required in [
            "Add-Type -AssemblyName System.Net.Http",
            "$handler.UseProxy = $false",
            "LASTEXITCODE",
            "ReportOnly",
            "marketplace_phase_fifteen",
            "cargo test --manifest-path backend/Cargo.toml marketplace",
            "--prefix frontend run lint",
            "--prefix frontend run build",
            "/health",
            "/ready",
            "/api/marketplace/installations",
            "/api/marketplace/purchases",
            "/api/marketplace/analytics/admin",
            "/api/marketplace/reports",
            "/api/beta/ga-blockers?limit=100",
        ] {
            assert!(
                GA_CHECK_SCRIPT.contains(required),
                "missing Phase 15 GA script behavior: {required}"
            );
        }

        assert!(
            !GA_CHECK_SCRIPT.contains("/api/marketplace/ga"),
            "Phase 15 should not invent a parallel Marketplace GA API"
        );
        assert!(
            !GA_CHECK_SCRIPT.contains("/api/marketplace/launch"),
            "Phase 15 should use existing operational surfaces instead of a launch API"
        );
    }
}
