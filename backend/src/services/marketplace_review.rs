use crate::error::AppError;

pub const REVIEW_DECISION_APPROVE: &str = "approve";
pub const REVIEW_DECISION_REJECT: &str = "reject";
pub const REVIEW_DECISION_REQUEST_CHANGES: &str = "request_changes";

pub const MODERATION_SUSPEND_LISTING: &str = "suspend_listing";
pub const MODERATION_UNPUBLISH_VERSION: &str = "unpublish_version";
pub const MODERATION_EMERGENCY_BLOCK: &str = "emergency_block";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReviewDecisionTransition {
    pub submission_status: &'static str,
    pub version_status: &'static str,
    pub listing_status: &'static str,
}

pub fn validate_review_decision(
    decision: &str,
    version_status: &str,
    validation_status: &str,
    security_risk_level: &str,
) -> Result<ReviewDecisionTransition, AppError> {
    let decision = decision.trim();
    if version_status == "approved" {
        return Err(AppError::Validation(
            "approved versions cannot be reviewed again".to_owned(),
        ));
    }

    match decision {
        REVIEW_DECISION_APPROVE => {
            if version_status == "blocked"
                || validation_status == "failed"
                || matches!(security_risk_level, "high" | "critical")
            {
                return Err(AppError::Validation(
                    "blocked, failed, high-risk, or critical-risk packages cannot be approved"
                        .to_owned(),
                ));
            }
            Ok(ReviewDecisionTransition {
                submission_status: "approved",
                version_status: "approved",
                listing_status: "approved",
            })
        }
        REVIEW_DECISION_REJECT => Ok(ReviewDecisionTransition {
            submission_status: "rejected",
            version_status: "rejected",
            listing_status: "changes_requested",
        }),
        REVIEW_DECISION_REQUEST_CHANGES => Ok(ReviewDecisionTransition {
            submission_status: "changes_requested",
            version_status: "rejected",
            listing_status: "changes_requested",
        }),
        _ => Err(AppError::Validation(
            "decision must be approve, reject, or request_changes".to_owned(),
        )),
    }
}

pub fn validate_moderation_action(
    action: &str,
    reason: &str,
    version_id_present: bool,
) -> Result<(), AppError> {
    if reason.trim().is_empty() {
        return Err(AppError::Validation(
            "moderation reason is required".to_owned(),
        ));
    }

    match action.trim() {
        MODERATION_SUSPEND_LISTING | MODERATION_EMERGENCY_BLOCK => Ok(()),
        MODERATION_UNPUBLISH_VERSION if version_id_present => Ok(()),
        MODERATION_UNPUBLISH_VERSION => Err(AppError::Validation(
            "version_id is required when unpublishing a version".to_owned(),
        )),
        _ => Err(AppError::Validation(
            "moderation action must be suspend_listing, unpublish_version, or emergency_block"
                .to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MODERATION_UNPUBLISH_VERSION, REVIEW_DECISION_APPROVE, REVIEW_DECISION_REJECT,
        REVIEW_DECISION_REQUEST_CHANGES, validate_moderation_action, validate_review_decision,
    };

    const PHASE_FOUR_DOC: &str = include_str!("../../../docs/V3_PHASE_FOUR.md");
    const PHASE_FOUR_MIGRATION: &str =
        include_str!("../../migrations/0018_v3_phase_four_review_moderation.sql");

    #[test]
    fn review_decisions_map_to_publication_statuses() {
        let approved =
            validate_review_decision(REVIEW_DECISION_APPROVE, "submitted", "passed", "low")
                .expect("valid package should be approvable");
        assert_eq!(approved.submission_status, "approved");
        assert_eq!(approved.version_status, "approved");
        assert_eq!(approved.listing_status, "approved");

        let rejected =
            validate_review_decision(REVIEW_DECISION_REJECT, "submitted", "passed", "low")
                .expect("submitted package should be rejectable");
        assert_eq!(rejected.submission_status, "rejected");
        assert_eq!(rejected.version_status, "rejected");
        assert_eq!(rejected.listing_status, "changes_requested");

        let changes = validate_review_decision(
            REVIEW_DECISION_REQUEST_CHANGES,
            "submitted",
            "warning",
            "medium",
        )
        .expect("submitted package can request changes");
        assert_eq!(changes.submission_status, "changes_requested");
    }

    #[test]
    fn blocked_or_high_risk_packages_cannot_be_approved() {
        assert!(
            validate_review_decision(REVIEW_DECISION_APPROVE, "blocked", "passed", "low").is_err()
        );
        assert!(
            validate_review_decision(REVIEW_DECISION_APPROVE, "submitted", "failed", "low")
                .is_err()
        );
        assert!(
            validate_review_decision(REVIEW_DECISION_APPROVE, "submitted", "passed", "high")
                .is_err()
        );
    }

    #[test]
    fn moderation_requires_reason_and_version_for_unpublish() {
        assert!(validate_moderation_action("suspend_listing", "policy issue", false).is_ok());
        assert!(validate_moderation_action("emergency_block", "malware", false).is_ok());
        assert!(
            validate_moderation_action(MODERATION_UNPUBLISH_VERSION, "bad build", true).is_ok()
        );
        assert!(
            validate_moderation_action(MODERATION_UNPUBLISH_VERSION, "bad build", false).is_err()
        );
        assert!(validate_moderation_action("suspend_listing", "", false).is_err());
    }

    #[test]
    fn phase_four_contract_is_documented_and_migrated() {
        for required in [
            "Review Queue",
            "Review Decision Flow",
            "Moderation And Takedown",
            "marketplace_review_events",
            "approve",
            "request_changes",
            "emergency_block",
        ] {
            assert!(
                PHASE_FOUR_DOC.contains(required) || PHASE_FOUR_MIGRATION.contains(required),
                "missing phase 4 contract term: {required}"
            );
        }
    }
}
