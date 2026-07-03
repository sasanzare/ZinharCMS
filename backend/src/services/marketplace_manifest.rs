use serde_json::{Map, Value};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

pub const MARKETPLACE_MANIFEST_SCHEMA_VERSION: &str = "2026-07";

pub const SUPPORTED_MARKETPLACE_PRODUCT_TYPES: &[&str] = &[
    "component_pack",
    "design_template",
    "integration_plugin",
    "backend_extension",
];

pub const SUPPORTED_MARKETPLACE_PERMISSIONS: &[&str] = &[
    "content.read",
    "content.write",
    "page.read",
    "page.write",
    "media.read",
    "media.write",
    "webhook.send",
    "settings.read",
    "external_network.request",
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ManifestValidationError {
    pub errors: Vec<String>,
}

impl fmt::Display for ManifestValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid marketplace manifest: {}",
            self.errors.join("; ")
        )
    }
}

impl Error for ManifestValidationError {}

pub fn validate_marketplace_manifest(manifest: &Value) -> Result<(), ManifestValidationError> {
    let Some(object) = manifest.as_object() else {
        return Err(ManifestValidationError {
            errors: vec!["manifest must be a JSON object".to_owned()],
        });
    };

    let mut errors = Vec::new();

    require_string(
        object,
        "manifest_version",
        &mut errors,
        Some(MARKETPLACE_MANIFEST_SCHEMA_VERSION),
    );
    require_string(object, "name", &mut errors, None);

    if let Some(version) = require_string(object, "version", &mut errors, None) {
        if !is_semver(version) {
            errors.push("version must use semantic version format".to_owned());
        }
    }

    if let Some(product_type) = require_string(object, "type", &mut errors, None) {
        if !SUPPORTED_MARKETPLACE_PRODUCT_TYPES.contains(&product_type) {
            errors.push(format!("unsupported product type '{product_type}'"));
        }
    }

    validate_permissions(object.get("permissions"), &mut errors);
    validate_compatibility(object.get("compatibility"), &mut errors);
    validate_entry_points(object.get("entry_points"), &mut errors);
    validate_assets(object.get("assets"), &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ManifestValidationError { errors })
    }
}

fn require_string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    errors: &mut Vec<String>,
    expected: Option<&str>,
) -> Option<&'a str> {
    let Some(value) = object.get(field) else {
        errors.push(format!("{field} is required"));
        return None;
    };

    let Some(value) = value
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        errors.push(format!("{field} must be a non-empty string"));
        return None;
    };

    if let Some(expected) = expected {
        if value != expected {
            errors.push(format!("{field} must be '{expected}'"));
        }
    }

    Some(value)
}

fn validate_permissions(value: Option<&Value>, errors: &mut Vec<String>) {
    let Some(value) = value else {
        errors.push("permissions is required".to_owned());
        return;
    };

    let Some(items) = value.as_array() else {
        errors.push("permissions must be an array".to_owned());
        return;
    };

    let mut seen = HashSet::new();
    for item in items {
        let Some(permission) = item
            .as_str()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            errors.push("permissions must contain only non-empty strings".to_owned());
            continue;
        };

        if !SUPPORTED_MARKETPLACE_PERMISSIONS.contains(&permission) {
            errors.push(format!("unsupported permission '{permission}'"));
        }

        if !seen.insert(permission.to_owned()) {
            errors.push(format!("duplicate permission '{permission}'"));
        }
    }
}

fn validate_compatibility(value: Option<&Value>, errors: &mut Vec<String>) {
    let Some(value) = value else {
        errors.push("compatibility is required".to_owned());
        return;
    };

    let Some(object) = value.as_object() else {
        errors.push("compatibility must be an object".to_owned());
        return;
    };

    let Some(min_version) = require_string(object, "min_zinhar_version", errors, None) else {
        return;
    };

    if !is_semver(min_version) {
        errors.push("compatibility.min_zinhar_version must use semantic version format".to_owned());
    }

    if let Some(max_version) = object.get("max_zinhar_version") {
        let Some(max_version) = max_version
            .as_str()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            errors.push("compatibility.max_zinhar_version must be a non-empty string".to_owned());
            return;
        };

        if !is_semver(max_version) {
            errors.push(
                "compatibility.max_zinhar_version must use semantic version format".to_owned(),
            );
        }
    }
}

fn validate_entry_points(value: Option<&Value>, errors: &mut Vec<String>) {
    let Some(value) = value else {
        errors.push("entry_points is required".to_owned());
        return;
    };

    let Some(object) = value.as_object() else {
        errors.push("entry_points must be an object".to_owned());
        return;
    };

    if object.is_empty() {
        errors.push("entry_points must not be empty".to_owned());
    }
}

fn validate_assets(value: Option<&Value>, errors: &mut Vec<String>) {
    let Some(value) = value else {
        errors.push("assets is required".to_owned());
        return;
    };

    if !value.is_array() {
        errors.push("assets must be an array".to_owned());
    }
}

pub fn is_semver(value: &str) -> bool {
    let (core_and_pre, build) = value
        .split_once('+')
        .map_or((value, None), |(left, right)| (left, Some(right)));
    if let Some(build) = build {
        if !is_semver_identifier_list(build, true) {
            return false;
        }
    }

    let (core, prerelease) = core_and_pre
        .split_once('-')
        .map_or((core_and_pre, None), |(left, right)| (left, Some(right)));
    if let Some(prerelease) = prerelease {
        if !is_semver_identifier_list(prerelease, false) {
            return false;
        }
    }

    let parts: Vec<&str> = core.split('.').collect();

    parts.len() == 3 && parts.iter().all(|part| is_semver_numeric_identifier(part))
}

fn is_semver_numeric_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| character.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}

fn is_semver_identifier_list(value: &str, allow_leading_zero_numeric: bool) -> bool {
    !value.is_empty()
        && value.split('.').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
                && (allow_leading_zero_numeric
                    || !part.chars().all(|character| character.is_ascii_digit())
                    || is_semver_numeric_identifier(part))
        })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{is_semver, validate_marketplace_manifest};

    fn valid_manifest() -> serde_json::Value {
        json!({
            "manifest_version": "2026-07",
            "name": "SaaS Hero Pack",
            "version": "1.0.0",
            "type": "component_pack",
            "permissions": ["page.read"],
            "compatibility": {
                "min_zinhar_version": "2.0.0",
                "max_zinhar_version": "3.0.0"
            },
            "entry_points": {
                "components": "components/index.json"
            },
            "assets": ["components/hero.json", "assets/preview.png"]
        })
    }

    #[test]
    fn accepts_valid_marketplace_manifest() {
        assert!(validate_marketplace_manifest(&valid_manifest()).is_ok());
    }

    #[test]
    fn rejects_missing_required_fields() {
        let manifest = json!({
            "manifest_version": "2026-07",
            "name": "Broken"
        });

        let error = validate_marketplace_manifest(&manifest).expect_err("manifest should fail");
        assert!(
            error
                .errors
                .iter()
                .any(|message| message == "version is required")
        );
        assert!(
            error
                .errors
                .iter()
                .any(|message| message == "type is required")
        );
        assert!(
            error
                .errors
                .iter()
                .any(|message| message == "permissions is required")
        );
    }

    #[test]
    fn rejects_unsupported_permissions_and_types() {
        let mut manifest = valid_manifest();
        manifest["type"] = json!("unknown");
        manifest["permissions"] = json!(["database.root"]);

        let error = validate_marketplace_manifest(&manifest).expect_err("manifest should fail");
        assert!(
            error
                .errors
                .iter()
                .any(|message| message.contains("unsupported product type"))
        );
        assert!(
            error
                .errors
                .iter()
                .any(|message| message.contains("unsupported permission"))
        );
    }

    #[test]
    fn validates_semver_shape() {
        assert!(is_semver("1.2.3"));
        assert!(is_semver("1.2.3-beta.1"));
        assert!(is_semver("1.2.3-beta.1+build.7"));
        assert!(!is_semver("1.2"));
        assert!(!is_semver("v1.2.3"));
        assert!(!is_semver("1.2.3-"));
        assert!(!is_semver("1.2.3+"));
        assert!(!is_semver("1.02.3"));
        assert!(!is_semver("1.2.3-01"));
    }
}
