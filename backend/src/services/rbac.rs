use crate::error::AppError;
use crate::middleware::auth::Claims;

pub const SUPER_ADMIN: &str = "super_admin";
pub const ADMIN: &str = "admin";
pub const EDITOR: &str = "editor";
pub const AUTHOR: &str = "author";
pub const VIEWER: &str = "viewer";

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
pub fn default_registration_role(existing_users: i64) -> &'static str {
    if existing_users == 0 {
        SUPER_ADMIN
    } else {
        AUTHOR
    }
}
