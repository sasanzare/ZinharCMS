use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;

use serde_json::{Value, json};
use tokio::fs;

use crate::services::marketplace_manifest::{SUPPORTED_MARKETPLACE_PERMISSIONS, is_semver};
use crate::services::marketplace_package::{validate_package_size, verify_package_checksum};

pub const CLEANUP_POLICY_PRESERVE: &str = "preserve_organization_data";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LifecycleAction {
    Enable,
    Disable,
    Uninstall,
    Update,
    Rollback,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InstallationRuleError {
    pub message: String,
}

impl fmt::Display for InstallationRuleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for InstallationRuleError {}

pub fn validate_mvp_product_type(product_type: &str) -> Result<(), InstallationRuleError> {
    if matches!(product_type, "component_pack" | "design_template") {
        Ok(())
    } else {
        Err(rule_error(format!(
            "product type '{product_type}' is not installable in the Phase 6 Marketplace MVP"
        )))
    }
}

pub fn validate_lifecycle_action(
    current_status: &str,
    action: LifecycleAction,
) -> Result<(), InstallationRuleError> {
    let allowed = match action {
        LifecycleAction::Enable => current_status == "disabled",
        LifecycleAction::Disable => current_status == "active",
        LifecycleAction::Uninstall => matches!(current_status, "active" | "disabled" | "blocked"),
        LifecycleAction::Update | LifecycleAction::Rollback => {
            matches!(current_status, "active" | "disabled")
        }
    };

    if allowed {
        Ok(())
    } else {
        Err(rule_error(format!(
            "cannot {action_name} an installation with status '{current_status}'",
            action_name = action_name(action)
        )))
    }
}

pub fn approved_permission_snapshot(
    manifest: &Value,
    approved_permissions: &[String],
) -> Result<Value, InstallationRuleError> {
    let requested = permissions_from_manifest(manifest)?;
    let approved = canonicalize_permission_strings(approved_permissions, "approved_permissions")?;

    if requested != approved {
        return Err(rule_error(format!(
            "approved_permissions must exactly match the manifest permissions: {}",
            requested.join(", ")
        )));
    }

    Ok(json!(requested))
}

pub fn permissions_from_manifest(manifest: &Value) -> Result<Vec<String>, InstallationRuleError> {
    let permissions = manifest
        .get("permissions")
        .ok_or_else(|| rule_error("manifest permissions are missing"))?;
    canonicalize_permission_value(permissions, "manifest permissions")
}

pub fn canonicalize_permission_value(
    permissions: &Value,
    label: &str,
) -> Result<Vec<String>, InstallationRuleError> {
    let values = permissions
        .as_array()
        .ok_or_else(|| rule_error(format!("{label} must be an array")))?;
    let strings = values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| rule_error(format!("{label} must contain only strings")))
        })
        .collect::<Result<Vec<_>, _>>()?;
    canonicalize_permission_strings(&strings, label)
}

pub fn permission_reapproval_required(
    current_snapshot: &Value,
    target_manifest: &Value,
) -> Result<bool, InstallationRuleError> {
    Ok(
        canonicalize_permission_value(current_snapshot, "current permission snapshot")?
            != permissions_from_manifest(target_manifest)?,
    )
}

pub fn permissions_are_subset(candidate: &[String], approved: &[String]) -> bool {
    let approved: BTreeSet<&str> = approved.iter().map(String::as_str).collect();
    candidate
        .iter()
        .all(|permission| approved.contains(permission.as_str()))
}

pub fn compare_semver(left: &str, right: &str) -> Option<Ordering> {
    let left = ParsedSemver::parse(left)?;
    let right = ParsedSemver::parse(right)?;
    Some(left.compare(&right))
}

pub fn validate_newer_version(
    current_version: &str,
    target_version: &str,
) -> Result<(), InstallationRuleError> {
    match compare_semver(target_version, current_version) {
        Some(Ordering::Greater) => Ok(()),
        Some(_) => Err(rule_error(format!(
            "update version '{target_version}' must be newer than installed version '{current_version}'"
        ))),
        None => Err(rule_error(
            "installed or target version is not valid semantic version",
        )),
    }
}

pub async fn verify_stored_artifact(
    upload_dir: &str,
    object_key: &str,
    expected_size_bytes: i64,
    expected_sha256: &str,
) -> Result<(), InstallationRuleError> {
    if expected_size_bytes <= 0 {
        return Err(rule_error("stored package size must be positive"));
    }
    validate_package_size(expected_size_bytes as u64)
        .map_err(|error| rule_error(error.to_string()))?;

    let path = safe_artifact_path(upload_dir, object_key)?;
    let canonical_root = fs::canonicalize(upload_dir).await.map_err(|error| {
        rule_error(format!(
            "Marketplace package storage root is unavailable: {error}"
        ))
    })?;
    let canonical_path = fs::canonicalize(&path).await.map_err(|error| {
        rule_error(format!(
            "Marketplace package artifact is unavailable at '{}': {error}",
            path.display()
        ))
    })?;
    if !canonical_path.starts_with(&canonical_root) {
        return Err(rule_error(
            "Marketplace package artifact resolves outside the configured storage root",
        ));
    }
    let metadata = fs::metadata(&canonical_path).await.map_err(|error| {
        rule_error(format!(
            "Marketplace package artifact metadata is unavailable: {error}"
        ))
    })?;
    if !metadata.is_file() {
        return Err(rule_error(
            "Marketplace package artifact is not a regular file",
        ));
    }
    if metadata.len() != expected_size_bytes as u64 {
        return Err(rule_error(format!(
            "Marketplace package artifact size mismatch: expected {expected_size_bytes}, found {}",
            metadata.len()
        )));
    }

    let bytes = fs::read(&canonical_path).await.map_err(|error| {
        rule_error(format!(
            "failed to read Marketplace package artifact: {error}"
        ))
    })?;
    if bytes.len() as i64 != expected_size_bytes {
        return Err(rule_error(format!(
            "Marketplace package artifact size changed during verification: expected {expected_size_bytes}, found {}",
            bytes.len()
        )));
    }
    if !verify_package_checksum(&bytes, expected_sha256) {
        return Err(rule_error(
            "Marketplace package artifact checksum does not match the approved version",
        ));
    }

    Ok(())
}

fn canonicalize_permission_strings(
    permissions: &[String],
    label: &str,
) -> Result<Vec<String>, InstallationRuleError> {
    let mut canonical = BTreeSet::new();
    for permission in permissions {
        let permission = permission.trim();
        if permission.is_empty() {
            return Err(rule_error(format!("{label} must not contain empty values")));
        }
        if !SUPPORTED_MARKETPLACE_PERMISSIONS.contains(&permission) {
            return Err(rule_error(format!(
                "{label} contains unsupported permission '{permission}'"
            )));
        }
        if !canonical.insert(permission.to_owned()) {
            return Err(rule_error(format!(
                "{label} contains duplicate permission '{permission}'"
            )));
        }
    }
    Ok(canonical.into_iter().collect())
}

fn safe_artifact_path(
    upload_dir: &str,
    object_key: &str,
) -> Result<PathBuf, InstallationRuleError> {
    if !object_key.starts_with("marketplace/packages/") {
        return Err(rule_error(
            "Marketplace package object key has an invalid prefix",
        ));
    }

    let mut path = PathBuf::from(upload_dir);
    for segment in object_key.split('/') {
        let safe = !segment.is_empty()
            && segment != "."
            && segment != ".."
            && !segment.contains(['\\', ':']);
        if !safe {
            return Err(rule_error(
                "Marketplace package object key contains an unsafe path",
            ));
        }
        path.push(segment);
    }
    Ok(path)
}

fn action_name(action: LifecycleAction) -> &'static str {
    match action {
        LifecycleAction::Enable => "enable",
        LifecycleAction::Disable => "disable",
        LifecycleAction::Uninstall => "uninstall",
        LifecycleAction::Update => "update",
        LifecycleAction::Rollback => "rollback",
    }
}

fn rule_error(message: impl Into<String>) -> InstallationRuleError {
    InstallationRuleError {
        message: message.into(),
    }
}

#[derive(Debug)]
struct ParsedSemver<'a> {
    core: [u64; 3],
    prerelease: Option<Vec<&'a str>>,
}

impl<'a> ParsedSemver<'a> {
    fn parse(value: &'a str) -> Option<Self> {
        if !is_semver(value) {
            return None;
        }
        let without_build = value.split_once('+').map_or(value, |(core, _)| core);
        let (core, prerelease) = without_build
            .split_once('-')
            .map_or((without_build, None), |(core, prerelease)| {
                (core, Some(prerelease.split('.').collect()))
            });
        let mut parts = core.split('.').map(|part| part.parse::<u64>().ok());
        Some(Self {
            core: [parts.next()??, parts.next()??, parts.next()??],
            prerelease,
        })
    }

    fn compare(&self, other: &Self) -> Ordering {
        let core = self.core.cmp(&other.core);
        if core != Ordering::Equal {
            return core;
        }

        match (&self.prerelease, &other.prerelease) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(left), Some(right)) => compare_prerelease(left, right),
        }
    }
}

fn compare_prerelease(left: &[&str], right: &[&str]) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        let ordering = match (left.parse::<u64>(), right.parse::<u64>()) {
            (Ok(left), Ok(right)) => left.cmp(&right),
            (Ok(_), Err(_)) => Ordering::Less,
            (Err(_), Ok(_)) => Ordering::Greater,
            (Err(_), Err(_)) => left.cmp(right),
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use serde_json::json;
    use tokio::fs;
    use uuid::Uuid;

    use super::{
        LifecycleAction, approved_permission_snapshot, compare_semver,
        permission_reapproval_required, permissions_are_subset, validate_lifecycle_action,
        validate_mvp_product_type, validate_newer_version, verify_stored_artifact,
    };
    use crate::services::marketplace_package::sha256_hex;

    const PHASE_SIX_MIGRATION: &str =
        include_str!("../../migrations/0019_v3_phase_six_installation_lifecycle.sql");
    const PHASE_SIX_DOC: &str = include_str!("../../../docs/V3_PHASE_SIX.md");
    const MARKETPLACE_ROUTES: &str = include_str!("../routes/marketplace.rs");

    #[test]
    fn permission_approval_must_exactly_match_manifest() {
        let manifest = json!({ "permissions": ["page.write", "content.read"] });
        let approved = vec!["content.read".to_owned(), "page.write".to_owned()];
        assert_eq!(
            approved_permission_snapshot(&manifest, &approved).unwrap(),
            json!(["content.read", "page.write"])
        );
        assert!(approved_permission_snapshot(&manifest, &["content.read".to_owned()]).is_err());
    }

    #[test]
    fn permission_change_requires_reapproval_and_subset_is_safe() {
        let current = json!(["content.read", "page.read"]);
        let unchanged = json!({ "permissions": ["page.read", "content.read"] });
        let changed = json!({ "permissions": ["page.read", "content.write"] });
        assert!(!permission_reapproval_required(&current, &unchanged).unwrap());
        assert!(permission_reapproval_required(&current, &changed).unwrap());
        assert!(permissions_are_subset(
            &["content.read".to_owned()],
            &["content.read".to_owned(), "page.read".to_owned()]
        ));
    }

    #[test]
    fn lifecycle_transitions_preserve_uninstalled_and_blocked_states() {
        assert!(validate_lifecycle_action("disabled", LifecycleAction::Enable).is_ok());
        assert!(validate_lifecycle_action("active", LifecycleAction::Disable).is_ok());
        assert!(validate_lifecycle_action("blocked", LifecycleAction::Uninstall).is_ok());
        assert!(validate_lifecycle_action("uninstalled", LifecycleAction::Enable).is_err());
        assert!(validate_lifecycle_action("blocked", LifecycleAction::Update).is_err());
        assert!(validate_lifecycle_action("uninstalled", LifecycleAction::Rollback).is_err());
    }

    #[test]
    fn semantic_version_comparison_handles_prerelease_precedence() {
        assert_eq!(
            compare_semver("1.0.0", "1.0.0-rc.1"),
            Some(Ordering::Greater)
        );
        assert_eq!(
            compare_semver("1.0.0-rc.2", "1.0.0-rc.10"),
            Some(Ordering::Less)
        );
        assert!(validate_newer_version("1.2.3", "1.3.0").is_ok());
        assert!(validate_newer_version("1.2.3", "1.2.3+build.2").is_err());
    }

    #[test]
    fn phase_six_mvp_limits_installable_product_types() {
        assert!(validate_mvp_product_type("component_pack").is_ok());
        assert!(validate_mvp_product_type("design_template").is_ok());
        assert!(validate_mvp_product_type("integration_plugin").is_err());
        assert!(validate_mvp_product_type("backend_extension").is_err());
    }

    #[test]
    fn phase_six_contract_is_documented_migrated_and_routed() {
        for required in [
            "cleanup_policy",
            "version_pinned",
            "enabled_at",
            "disabled_at",
            "uninstalled_at",
            "marketplace_installations_listing_rollback_version_fk",
        ] {
            assert!(
                PHASE_SIX_MIGRATION.contains(required),
                "missing Phase 6 migration contract: {required}"
            );
        }
        for endpoint in [
            "/api/marketplace/installations",
            "/updates",
            "/enable",
            "/disable",
            "/uninstall",
            "/update",
            "/rollback",
        ] {
            assert!(
                PHASE_SIX_DOC.contains(endpoint) || MARKETPLACE_ROUTES.contains(endpoint),
                "missing Phase 6 endpoint contract: {endpoint}"
            );
        }
        for required in [
            "begin_tenant_transaction",
            "record_in_transaction",
            "verify_stored_artifact",
            "marketplace.installation.install",
            "marketplace.installation.update",
            "marketplace.installation.rollback",
            "preserve_organization_data",
        ] {
            assert!(
                PHASE_SIX_DOC.contains(required) || MARKETPLACE_ROUTES.contains(required),
                "missing Phase 6 lifecycle contract: {required}"
            );
        }
        assert!(!MARKETPLACE_ROUTES.contains("DELETE FROM marketplace_installations"));
    }

    #[tokio::test]
    async fn stored_artifact_gate_accepts_valid_bytes() {
        let root = std::env::temp_dir().join(format!("zinhar-phase6-{}", Uuid::now_v7()));
        let bytes = b"approved marketplace artifact";
        let checksum = sha256_hex(bytes);
        let object_key = format!("marketplace/packages/creator/listing/1.0.0/{checksum}.zip");
        let path = root.join(object_key.replace('/', std::path::MAIN_SEPARATOR_STR));
        fs::create_dir_all(path.parent().unwrap()).await.unwrap();
        fs::write(&path, bytes).await.unwrap();

        assert!(
            verify_stored_artifact(
                root.to_str().unwrap(),
                &object_key,
                bytes.len() as i64,
                &checksum,
            )
            .await
            .is_ok()
        );
        assert!(root.starts_with(std::env::temp_dir()));
        assert!(
            root.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("zinhar-phase6-"))
        );
        fs::remove_dir_all(root).await.unwrap();
    }

    #[tokio::test]
    async fn stored_artifact_gate_rejects_tampered_checksum_and_size() {
        let root = std::env::temp_dir().join(format!("zinhar-phase6-{}", Uuid::now_v7()));
        let bytes = b"approved marketplace artifact";
        let checksum = sha256_hex(bytes);
        let object_key = format!("marketplace/packages/creator/listing/1.0.0/{checksum}.zip");
        let path = root.join(object_key.replace('/', std::path::MAIN_SEPARATOR_STR));
        fs::create_dir_all(path.parent().unwrap()).await.unwrap();
        fs::write(&path, bytes).await.unwrap();

        assert!(
            verify_stored_artifact(
                root.to_str().unwrap(),
                &object_key,
                bytes.len() as i64,
                &sha256_hex(b"tampered"),
            )
            .await
            .is_err()
        );
        assert!(
            verify_stored_artifact(
                root.to_str().unwrap(),
                &object_key,
                bytes.len() as i64 + 1,
                &checksum,
            )
            .await
            .is_err()
        );
        assert!(root.starts_with(std::env::temp_dir()));
        assert!(
            root.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("zinhar-phase6-"))
        );
        fs::remove_dir_all(root).await.unwrap();
    }

    #[tokio::test]
    async fn stored_artifact_gate_rejects_object_key_traversal() {
        let root = std::env::temp_dir().join(format!("zinhar-phase6-{}", Uuid::now_v7()));
        let result = verify_stored_artifact(
            root.to_str().unwrap(),
            "marketplace/packages/../escape.zip",
            1,
            &sha256_hex(b"x"),
        )
        .await;
        assert!(result.is_err());
    }
}
