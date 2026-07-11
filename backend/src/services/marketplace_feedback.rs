pub const REVIEW_BODY_MAX: usize = 4_000;
pub const REPORT_DESCRIPTION_MAX: usize = 4_000;

pub fn validate_rating(rating: i32) -> Result<(), String> {
    if (1..=5).contains(&rating) {
        Ok(())
    } else {
        Err("rating must be between 1 and 5".to_owned())
    }
}

pub fn validate_text(value: &str, label: &str, min: usize, max: usize) -> Result<(), String> {
    let trimmed = value.trim();
    let character_count = trimmed.chars().count();
    if character_count < min || character_count > max {
        return Err(format!(
            "{label} must be between {min} and {max} characters"
        ));
    }
    Ok(())
}

pub fn validate_report_type(report_type: &str) -> Result<(), String> {
    if matches!(
        report_type,
        "malware" | "copyright" | "spam" | "fraud" | "privacy" | "other"
    ) {
        Ok(())
    } else {
        Err("unsupported abuse report type".to_owned())
    }
}

pub fn validate_severity(severity: &str) -> Result<(), String> {
    if matches!(severity, "low" | "medium" | "high" | "critical") {
        Ok(())
    } else {
        Err("unsupported abuse report severity".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_rating_and_feedback_bounds() {
        assert!(validate_rating(5).is_ok());
        assert!(validate_rating(0).is_err());
        assert!(validate_text("valid review", "body", 3, REVIEW_BODY_MAX).is_ok());
        assert!(validate_text("x", "body", 3, REVIEW_BODY_MAX).is_err());
        assert!(validate_text(&"ژ".repeat(REVIEW_BODY_MAX), "body", 3, REVIEW_BODY_MAX).is_ok());
        assert!(
            validate_text(&"ژ".repeat(REVIEW_BODY_MAX + 1), "body", 3, REVIEW_BODY_MAX).is_err()
        );
    }

    #[test]
    fn validates_abuse_taxonomy() {
        assert!(validate_report_type("malware").is_ok());
        assert!(validate_report_type("unknown").is_err());
        assert!(validate_severity("critical").is_ok());
        assert!(validate_severity("urgent").is_err());
    }

    #[test]
    fn phase_ten_contract_is_migrated_documented_and_routed() {
        let migration = include_str!("../../migrations/0024_v3_phase_ten_ratings_abuse.sql");
        let notification_migration =
            include_str!("../../migrations/0025_v3_phase_ten_internal_notifications.sql");
        let documentation = include_str!("../../../docs/V3_PHASE_TEN.md");
        let routes = include_str!("../routes/marketplace.rs");
        assert!(migration.contains("marketplace_product_reviews"));
        assert!(migration.contains("marketplace_abuse_reports"));
        assert!(migration.contains("FORCE ROW LEVEL SECURITY"));
        assert!(notification_migration.contains("marketplace_internal_notifications"));
        assert!(notification_migration.contains("critical_abuse_report"));
        assert!(documentation.contains("critical_notification"));
        assert!(routes.contains("create_product_review"));
        assert!(routes.contains("create_abuse_report"));
    }
}
