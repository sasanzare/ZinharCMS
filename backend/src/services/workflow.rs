use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    Draft,
    PendingReview,
    Published,
    Archived,
}

impl WorkflowStatus {
    pub fn parse(value: &str) -> Result<Self, AppError> {
        match value {
            "draft" => Ok(Self::Draft),
            "pending_review" => Ok(Self::PendingReview),
            "published" => Ok(Self::Published),
            "archived" => Ok(Self::Archived),
            other => Err(AppError::Validation(format!(
                "workflow status '{other}' is not supported"
            ))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::PendingReview => "pending_review",
            Self::Published => "published",
            Self::Archived => "archived",
        }
    }

    pub fn can_transition_to(self, next: Self, can_bypass_review: bool) -> bool {
        matches!(
            (self, next),
            (Self::Draft, Self::PendingReview)
                | (Self::PendingReview, Self::Published)
                | (Self::PendingReview, Self::Draft)
                | (Self::Published, Self::Archived)
                | (Self::Archived, Self::Draft)
                | (Self::Published, Self::Draft)
        ) || (self == Self::Draft && next == Self::Published && can_bypass_review)
    }
}

pub fn require_transition(
    current: &str,
    next: WorkflowStatus,
    can_bypass_review: bool,
) -> Result<(), AppError> {
    let current = WorkflowStatus::parse(current)?;
    if current.can_transition_to(next, can_bypass_review) {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "cannot transition workflow from '{}' to '{}'",
            current.as_str(),
            next.as_str()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_can_submit_for_review() {
        assert!(WorkflowStatus::Draft.can_transition_to(WorkflowStatus::PendingReview, false));
    }

    #[test]
    fn author_cannot_publish_draft_directly() {
        assert!(!WorkflowStatus::Draft.can_transition_to(WorkflowStatus::Published, false));
    }

    #[test]
    fn reviewer_can_publish_draft_directly() {
        assert!(WorkflowStatus::Draft.can_transition_to(WorkflowStatus::Published, true));
    }
}
