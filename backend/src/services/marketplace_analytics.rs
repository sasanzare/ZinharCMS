pub fn conversion_rate(completed_purchases: i64, purchase_attempts: i64) -> f64 {
    if purchase_attempts <= 0 {
        return 0.0;
    }

    (completed_purchases.max(0) as f64 / purchase_attempts as f64).clamp(0.0, 1.0)
}

pub fn error_count(validation_failures: i64, failed_purchases: i64, abuse_reports: i64) -> i64 {
    validation_failures
        .max(0)
        .saturating_add(failed_purchases.max(0))
        .saturating_add(abuse_reports.max(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_rate_is_bounded_and_zero_safe() {
        assert_eq!(conversion_rate(0, 0), 0.0);
        assert_eq!(conversion_rate(3, 6), 0.5);
        assert_eq!(conversion_rate(8, 4), 1.0);
        assert_eq!(conversion_rate(-1, 4), 0.0);
    }

    #[test]
    fn error_count_ignores_negative_inputs() {
        assert_eq!(error_count(2, 3, 5), 10);
        assert_eq!(error_count(-2, 3, -5), 3);
    }

    #[test]
    fn phase_eleven_contract_documents_creator_and_admin_analytics() {
        let routes = include_str!("../routes/marketplace_analytics.rs");
        let docs = include_str!("../../../docs/V3_PHASE_ELEVEN.md");

        for required in [
            "/api/marketplace/creators/{creator_id}/analytics",
            "/api/marketplace/analytics/admin",
            "only the creator owner can view analytics",
            "Submission rate, approval time, installs, refunds, reports, and blocked packages",
        ] {
            assert!(
                routes.contains(required) || docs.contains(required),
                "Phase 11 analytics contract missing: {required}"
            );
        }
    }
}
