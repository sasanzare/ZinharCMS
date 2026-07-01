pub const GA_REQUIRED_DOCS: &[&str] = &[
    "docs/V2_RELEASE_NOTES.md",
    "docs/V2_MIGRATION_GUIDE.md",
    "docs/V2_ADMIN_GUIDE.md",
    "docs/V2_BILLING_GUIDE.md",
    "docs/V2_OPERATIONS_RUNBOOK.md",
    "docs/V2_PHASE_TEN.md",
];

pub const GA_RELEASE_CHECKS: &[&str] = &[
    "freeze high-risk changes",
    "run final migration",
    "post-release monitoring",
    "enable paid plans",
    "billing and account access support",
    "rollback plan",
];

#[cfg(test)]
mod tests {
    use super::{GA_RELEASE_CHECKS, GA_REQUIRED_DOCS};

    const PHASE_TEN_DOC: &str = include_str!("../../../docs/V2_PHASE_TEN.md");
    const RELEASE_NOTES: &str = include_str!("../../../docs/V2_RELEASE_NOTES.md");
    const MIGRATION_GUIDE: &str = include_str!("../../../docs/V2_MIGRATION_GUIDE.md");
    const ADMIN_GUIDE: &str = include_str!("../../../docs/V2_ADMIN_GUIDE.md");
    const BILLING_GUIDE: &str = include_str!("../../../docs/V2_BILLING_GUIDE.md");
    const OPERATIONS_RUNBOOK: &str = include_str!("../../../docs/V2_OPERATIONS_RUNBOOK.md");
    const GA_CHECK_SCRIPT: &str = include_str!("../../../scripts/v2-ga-check.ps1");

    #[test]
    fn ga_required_docs_manifest_matches_release_outputs() {
        for required_doc in [
            "docs/V2_RELEASE_NOTES.md",
            "docs/V2_MIGRATION_GUIDE.md",
            "docs/V2_ADMIN_GUIDE.md",
            "docs/V2_BILLING_GUIDE.md",
            "docs/V2_OPERATIONS_RUNBOOK.md",
            "docs/V2_PHASE_TEN.md",
        ] {
            assert!(
                GA_REQUIRED_DOCS.contains(&required_doc),
                "{required_doc} is missing from the GA docs manifest"
            );
        }
    }

    #[test]
    fn ga_phase_ten_doc_covers_proposal_acceptance() {
        let lower = PHASE_TEN_DOC.to_lowercase();

        for phrase in GA_RELEASE_CHECKS {
            assert!(
                lower.contains(phrase),
                "phase ten doc is missing GA check phrase: {phrase}"
            );
        }

        assert!(lower.contains("without an internal feature flag"));
        assert!(lower.contains("core documentation"));
        assert!(lower.contains("runbook"));
    }

    #[test]
    fn ga_docs_cover_release_migration_admin_billing_and_runbook() {
        assert!(RELEASE_NOTES.contains("Breaking Changes From V1"));
        assert!(RELEASE_NOTES.contains("Support Policy"));
        assert!(MIGRATION_GUIDE.contains("Migration Order"));
        assert!(MIGRATION_GUIDE.contains("Rollback"));
        assert!(ADMIN_GUIDE.contains("Memberships And Roles"));
        assert!(ADMIN_GUIDE.contains("Audit Logs"));
        assert!(BILLING_GUIDE.contains("Stripe Webhooks"));
        assert!(BILLING_GUIDE.contains("Quota Enforcement"));
        assert!(OPERATIONS_RUNBOOK.contains("Incident Response"));
        assert!(OPERATIONS_RUNBOOK.contains("Rollback Plan"));
    }

    #[test]
    fn operations_runbook_covers_common_support_incidents() {
        let lower = OPERATIONS_RUNBOOK.to_lowercase();

        for phrase in [
            "billing state is wrong",
            "account access fails",
            "tenant isolation concern",
            "migration failure",
            "quota counters look wrong",
        ] {
            assert!(
                lower.contains(phrase),
                "runbook missing support incident: {phrase}"
            );
        }
    }

    #[test]
    fn ga_check_script_runs_release_candidate_checks() {
        assert!(
            GA_CHECK_SCRIPT
                .contains("cargo test --manifest-path backend/Cargo.toml --all-features")
        );
        assert!(GA_CHECK_SCRIPT.contains("--prefix frontend run lint"));
        assert!(GA_CHECK_SCRIPT.contains("--prefix frontend run build"));
        assert!(GA_CHECK_SCRIPT.contains("/health"));
        assert!(GA_CHECK_SCRIPT.contains("/ready"));
        assert!(GA_CHECK_SCRIPT.contains("LASTEXITCODE"));
        assert!(GA_CHECK_SCRIPT.contains("Resolve-NpmCommand"));
    }
}
