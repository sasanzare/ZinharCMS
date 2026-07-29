use axum::http::Method;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::mfa_challenges::{self, StepUpScope};
use crate::state::AppState;

pub const STEP_UP_HEADER: &str = "X-Step-Up-Token";

pub async fn enforce_scope(
    state: &AppState,
    claims: &Claims,
    scope: StepUpScope,
    raw_grant: Option<&str>,
) -> Result<(), AppError> {
    if claims.aal != 2 || claims.mfa_time.is_none() {
        return Err(AppError::Forbidden(
            "MFA enrollment and an MFA-authenticated session are required".to_owned(),
        ));
    }
    let raw_grant = raw_grant
        .ok_or_else(|| AppError::Forbidden("step-up verification is required".to_owned()))?;
    mfa_challenges::consume_step_up_grant(&state.redis, raw_grant, claims, scope).await
}

pub fn required_scope(method: &Method, path: &str) -> Option<StepUpScope> {
    let mutating = matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );
    if !mutating {
        return None;
    }
    if *method == Method::POST && path == "/api/auth/logout-all" {
        return Some(StepUpScope::SessionLogoutAll);
    }
    if *method == Method::DELETE && path.starts_with("/api/auth/sessions/") {
        return Some(StepUpScope::SessionLogoutAll);
    }
    if *method == Method::POST
        && path.starts_with("/api/auth/admin/users/")
        && path.ends_with("/revoke-sessions")
    {
        return Some(StepUpScope::PrivilegedSessionRevocation);
    }
    if *method == Method::DELETE && path == "/api/auth/mfa" {
        return Some(StepUpScope::MfaDisable);
    }
    if *method == Method::POST && path == "/api/auth/mfa/recovery-codes" {
        return Some(StepUpScope::MfaRecoveryRegenerate);
    }
    if path.starts_with("/api/organizations")
        && (path.contains("/members")
            || path.contains("/invitations")
            || path.contains("/domains")
            || path.contains("/transfer-ownership")
            || path.ends_with("/rate-limit"))
    {
        return Some(StepUpScope::OrganizationAdministration);
    }
    if path.starts_with("/api/plugins") {
        return Some(StepUpScope::OrganizationAdministration);
    }
    if path.starts_with("/api/webhooks") {
        return Some(StepUpScope::WebhookAdministration);
    }
    if path.starts_with("/api/billing") {
        return Some(StepUpScope::BillingAdministration);
    }
    if path.contains("marketplace") && path.contains("payout") {
        return Some(StepUpScope::MarketplacePayout);
    }
    if path.contains("marketplace")
        && (path.contains("kill-switch")
            || path.contains("moderate")
            || path.contains("review-queue")
            || path.contains("submissions")
            || path.contains("abuse-reports"))
    {
        return Some(StepUpScope::MarketplaceAdministration);
    }
    None
}

#[cfg(test)]
mod tests {
    use axum::http::Method;

    use super::required_scope;
    use crate::services::mfa_challenges::StepUpScope;

    #[test]
    fn sensitive_action_matrix_is_method_and_path_specific() {
        assert_eq!(
            required_scope(&Method::POST, "/api/auth/logout-all"),
            Some(StepUpScope::SessionLogoutAll)
        );
        assert_eq!(
            required_scope(
                &Method::POST,
                "/api/auth/admin/users/01900000-0000-7000-8000-000000000000/revoke-sessions"
            ),
            Some(StepUpScope::PrivilegedSessionRevocation)
        );
        assert_eq!(
            required_scope(&Method::DELETE, "/api/auth/mfa"),
            Some(StepUpScope::MfaDisable)
        );
        assert_eq!(
            required_scope(
                &Method::PATCH,
                "/api/organizations/01900000-0000-7000-8000-000000000000/members/member"
            ),
            Some(StepUpScope::OrganizationAdministration)
        );
        assert_eq!(
            required_scope(&Method::POST, "/api/webhooks"),
            Some(StepUpScope::WebhookAdministration)
        );
        assert_eq!(required_scope(&Method::GET, "/api/webhooks"), None);
        assert_eq!(required_scope(&Method::POST, "/api/content/entries"), None);
    }
}
