#[cfg(test)]
mod tests {
    const PHASE_FOURTEEN_DOC: &str = include_str!("../../../docs/V3_PHASE_FOURTEEN.md");
    const BETA_ROUTES: &str = include_str!("../routes/beta.rs");
    const MARKETPLACE_ROUTES: &str = include_str!("../routes/marketplace.rs");
    const MARKETPLACE_FINANCE_ROUTES: &str = include_str!("../routes/marketplace_finance.rs");
    const MARKETPLACE_ANALYTICS_ROUTES: &str = include_str!("../routes/marketplace_analytics.rs");
    const BETA_READINESS_SCRIPT: &str =
        include_str!("../../../scripts/marketplace-phase14-beta-readiness.ps1");
    const PHASE_FOURTEEN_DIAGRAM: &str =
        include_str!("../../../docs/diagrams/41-marketplace-beta.mmd");

    #[test]
    fn phase_fourteen_private_creator_beta_contract_is_documented_and_measurable() {
        for required in [
            "14.1 Private Creator Beta",
            "5 to 10 real products",
            "creator feedback",
            "bug list",
            "CreatorId",
            "/api/marketplace/creators/",
            "/analytics",
            "creator_products",
            "creator_feedback",
        ] {
            assert!(
                PHASE_FOURTEEN_DOC.contains(required)
                    || BETA_READINESS_SCRIPT.contains(required)
                    || PHASE_FOURTEEN_DIAGRAM.contains(required),
                "missing Phase 14.1 creator beta contract: {required}"
            );
        }

        for required in [
            "/api/beta/participants/{organization_id}",
            "/api/beta/feedback",
            "/api/beta/ga-blockers",
        ] {
            assert!(
                BETA_ROUTES.contains(required),
                "missing existing beta route needed by creator beta: {required}"
            );
        }

        assert!(
            MARKETPLACE_ANALYTICS_ROUTES
                .contains("/api/marketplace/creators/{creator_id}/analytics")
        );
    }

    #[test]
    fn phase_fourteen_customer_beta_contract_is_documented_and_measurable() {
        for required in [
            "14.2 Customer Beta",
            "install",
            "uninstall",
            "purchase",
            "support issue",
            "report",
            "/api/marketplace/installations",
            "/api/marketplace/purchases",
            "/api/marketplace/reports",
            "support_issues",
            "customer_reports",
        ] {
            assert!(
                PHASE_FOURTEEN_DOC.contains(required)
                    || BETA_READINESS_SCRIPT.contains(required)
                    || PHASE_FOURTEEN_DIAGRAM.contains(required),
                "missing Phase 14.2 customer beta contract: {required}"
            );
        }

        for required in [
            "/api/marketplace/installations",
            "/api/marketplace/installations/{installation_id}/uninstall",
        ] {
            assert!(
                MARKETPLACE_ROUTES.contains(required),
                "missing existing Marketplace lifecycle route for customer beta: {required}"
            );
        }

        assert!(MARKETPLACE_FINANCE_ROUTES.contains("/api/marketplace/purchases"));
        assert!(MARKETPLACE_ROUTES.contains("/api/marketplace/reports"));
    }

    #[test]
    fn phase_fourteen_readiness_script_uses_real_existing_endpoints_only() {
        for required in [
            "Add-Type -AssemblyName System.Net.Http",
            "$handler.UseProxy = $false",
            "Invoke-Json",
            "ReportOnly",
            "/api/beta/dashboard",
            "/api/beta/feedback?limit=100",
            "/api/beta/ga-blockers?limit=100",
            "/api/beta/product-dashboard",
            "/api/marketplace/analytics/admin",
        ] {
            assert!(
                BETA_READINESS_SCRIPT.contains(required),
                "missing Phase 14 readiness script behavior: {required}"
            );
        }

        assert!(
            !BETA_READINESS_SCRIPT.contains("/api/marketplace/beta"),
            "Phase 14 should use existing beta and Marketplace APIs instead of inventing a parallel beta API"
        );
    }
}
