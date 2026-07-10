use crate::error::AppError;
use crate::middleware::auth::Claims;

pub const SUPER_ADMIN: &str = "super_admin";
pub const ADMIN: &str = "admin";
pub const EDITOR: &str = "editor";
pub const AUTHOR: &str = "author";
pub const VIEWER: &str = "viewer";

pub const ORG_OWNER: &str = "owner";
pub const ORG_ADMIN: &str = "admin";
pub const ORG_EDITOR: &str = "editor";
pub const ORG_AUTHOR: &str = "author";
pub const ORG_VIEWER: &str = "viewer";
pub const ORG_BILLING_MANAGER: &str = "billing_manager";

pub fn require_any(claims: &Claims, roles: &[&str]) -> Result<(), AppError> {
    if claims.role == SUPER_ADMIN || roles.iter().any(|role| *role == claims.role) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "role '{}' cannot access this resource",
            claims.role
        )))
    }
}

pub fn require_org_any(role: &str, roles: &[&str]) -> Result<(), AppError> {
    if role == ORG_OWNER || roles.iter().any(|allowed| *allowed == role) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "organization role '{role}' cannot access this resource"
        )))
    }
}

pub fn require_content_type_manager(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN])
}

pub fn require_entry_writer(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR, AUTHOR])
}

pub fn require_entry_publisher(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR])
}

pub fn require_media_writer(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR, AUTHOR])
}

pub fn require_page_writer(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR, AUTHOR])
}

pub fn require_page_publisher(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR])
}

pub fn require_page_manager(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR])
}

pub fn require_component_registry_manager(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN])
}

pub fn require_webhook_manager(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN])
}

pub fn require_workflow_reviewer(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR])
}

pub fn require_comment_reader(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR, AUTHOR, VIEWER])
}

pub fn require_comment_writer(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR, AUTHOR])
}

pub fn require_comment_manager(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR])
}

pub fn require_plugin_reader(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN, EDITOR, AUTHOR, VIEWER])
}

pub fn require_plugin_manager(claims: &Claims) -> Result<(), AppError> {
    require_any(claims, &[ADMIN])
}

pub fn require_org_content_type_manager(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn require_org_entry_writer(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR])
}

pub fn require_org_entry_publisher(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn require_org_media_writer(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR])
}

pub fn require_org_page_writer(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR])
}

pub fn require_org_page_publisher(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn require_org_page_manager(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn require_org_component_registry_manager(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn require_org_webhook_manager(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN])
}

pub fn require_org_billing_manager(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_BILLING_MANAGER])
}

pub fn require_org_marketplace_installer(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN])
}

pub fn require_org_marketplace_permission_approver(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN])
}

pub fn require_org_workflow_reviewer(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn require_org_comment_reader(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR, ORG_VIEWER])
}

pub fn require_org_comment_writer(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR])
}

pub fn require_org_comment_manager(role: &str) -> Result<(), AppError> {
    require_org_any(role, &[ORG_ADMIN, ORG_EDITOR])
}

pub fn default_registration_role(existing_users: i64) -> &'static str {
    if existing_users == 0 {
        SUPER_ADMIN
    } else {
        AUTHOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Check = fn(&str) -> Result<(), AppError>;

    #[test]
    fn organization_permission_matrix_matches_phase8_contract() {
        let matrix: &[(&str, Check, &[&str])] = &[
            (
                "content_type_manager",
                require_org_content_type_manager,
                &[ORG_OWNER, ORG_ADMIN, ORG_EDITOR],
            ),
            (
                "entry_writer",
                require_org_entry_writer,
                &[ORG_OWNER, ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR],
            ),
            (
                "entry_publisher",
                require_org_entry_publisher,
                &[ORG_OWNER, ORG_ADMIN, ORG_EDITOR],
            ),
            (
                "media_writer",
                require_org_media_writer,
                &[ORG_OWNER, ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR],
            ),
            (
                "page_manager",
                require_org_page_manager,
                &[ORG_OWNER, ORG_ADMIN, ORG_EDITOR],
            ),
            (
                "webhook_manager",
                require_org_webhook_manager,
                &[ORG_OWNER, ORG_ADMIN],
            ),
            (
                "billing_manager",
                require_org_billing_manager,
                &[ORG_OWNER, ORG_ADMIN, ORG_BILLING_MANAGER],
            ),
            (
                "marketplace_installer",
                require_org_marketplace_installer,
                &[ORG_OWNER, ORG_ADMIN],
            ),
            (
                "marketplace_permission_approver",
                require_org_marketplace_permission_approver,
                &[ORG_OWNER, ORG_ADMIN],
            ),
            (
                "comment_reader",
                require_org_comment_reader,
                &[ORG_OWNER, ORG_ADMIN, ORG_EDITOR, ORG_AUTHOR, ORG_VIEWER],
            ),
        ];
        let roles = [
            ORG_OWNER,
            ORG_ADMIN,
            ORG_EDITOR,
            ORG_AUTHOR,
            ORG_VIEWER,
            ORG_BILLING_MANAGER,
        ];

        for (permission, check, allowed_roles) in matrix {
            for role in roles {
                let allowed = allowed_roles.contains(&role);
                assert_eq!(
                    check(role).is_ok(),
                    allowed,
                    "permission {permission} for role {role}"
                );
            }
        }
    }
}
