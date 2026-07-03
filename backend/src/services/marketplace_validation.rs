use std::collections::{BTreeSet, HashSet};

use serde_json::{Map, Value, json};

use crate::services::marketplace_manifest::{
    MARKETPLACE_MANIFEST_SCHEMA_VERSION, is_semver, validate_marketplace_manifest,
};
use crate::services::marketplace_package::MAX_MARKETPLACE_PACKAGE_BYTES;

pub const CURRENT_ZINHAR_VERSION: &str = "3.0.0";

const MAX_PACKAGE_FILES: usize = 500;
const MAX_DECLARED_ASSETS: usize = 200;
const MAX_UNCOMPRESSED_PACKAGE_BYTES: u64 = MAX_MARKETPLACE_PACKAGE_BYTES * 2;
const MAX_MANIFEST_DEPENDENCIES: usize = 100;
const SUPPORTED_MARKETPLACE_FEATURES: &[&str] = &[
    "component_packs",
    "design_templates",
    "integration_plugins",
    "media_assets",
    "page_builder",
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MarketplaceValidationDecision {
    pub validation_status: String,
    pub security_risk_level: String,
    pub version_status: String,
    pub submission_review_status: String,
    pub validation_report: Value,
    pub compatibility_report: Value,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ZipEntry {
    path: String,
    uncompressed_size: u64,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone)]
struct SecurityFinding {
    severity: RiskLevel,
    code: &'static str,
    message: String,
    path: Option<String>,
}

pub fn evaluate_marketplace_package(
    manifest: &Value,
    package_bytes: &[u8],
    checksum: &str,
    artifact_file_name: &str,
    listing_product_type: &str,
    organization_plan_slug: &str,
) -> MarketplaceValidationDecision {
    let static_report = static_validation_report(manifest, package_bytes, artifact_file_name);
    let validation_errors = string_array(&static_report, "errors");
    let validation_warnings = string_array(&static_report, "warnings");
    let validation_status = if !validation_errors.is_empty() {
        "failed"
    } else if !validation_warnings.is_empty() {
        "warning"
    } else {
        "passed"
    };

    let entries = parse_zip_entries(package_bytes).unwrap_or_default();
    let security_report = security_scan_report(manifest, &entries, listing_product_type);
    let security_risk_level = security_report
        .get("risk_level")
        .and_then(Value::as_str)
        .unwrap_or("low");

    let compatibility_report = compatibility_report(manifest, organization_plan_slug);

    let blocked =
        validation_status == "failed" || matches!(security_risk_level, "high" | "critical");
    let version_status = if blocked { "blocked" } else { "submitted" };
    let submission_review_status = if blocked { "blocked" } else { "queued" };

    let validation_report = json!({
        "phase": "v3.3",
        "schema_version": MARKETPLACE_MANIFEST_SCHEMA_VERSION,
        "checksum": checksum,
        "artifact_file_name": artifact_file_name,
        "artifact_size_bytes": package_bytes.len(),
        "validation_status": validation_status,
        "security_risk_level": security_risk_level,
        "static": static_report,
        "security": security_report,
        "compatibility": compatibility_report,
    });

    MarketplaceValidationDecision {
        validation_status: validation_status.to_owned(),
        security_risk_level: security_risk_level.to_owned(),
        version_status: version_status.to_owned(),
        submission_review_status: submission_review_status.to_owned(),
        validation_report,
        compatibility_report,
    }
}

fn static_validation_report(
    manifest: &Value,
    package_bytes: &[u8],
    artifact_file_name: &str,
) -> Value {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if !artifact_file_name.to_ascii_lowercase().ends_with(".zip") {
        warnings.push("artifact file name should use the .zip extension".to_owned());
    }

    if let Err(error) = validate_marketplace_manifest(manifest) {
        errors.extend(error.errors);
    }

    let entries = match parse_zip_entries(package_bytes) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(error);
            Vec::new()
        }
    };

    if entries.is_empty() {
        errors.push("package must contain at least one file".to_owned());
    }

    if entries.len() > MAX_PACKAGE_FILES {
        errors.push(format!(
            "package contains more than {MAX_PACKAGE_FILES} files"
        ));
    }

    let total_uncompressed_size: u64 = entries.iter().map(|entry| entry.uncompressed_size).sum();
    if total_uncompressed_size > MAX_UNCOMPRESSED_PACKAGE_BYTES {
        errors.push(format!(
            "package uncompressed size exceeds {MAX_UNCOMPRESSED_PACKAGE_BYTES} bytes"
        ));
    }

    let mut paths = BTreeSet::new();
    for entry in &entries {
        match normalize_manifest_path(&entry.path) {
            Ok(path) => {
                if !paths.insert(path.clone()) {
                    errors.push(format!("duplicate package path '{path}'"));
                }
            }
            Err(error) => errors.push(error),
        }
    }

    validate_manifest_assets(manifest, &paths, &mut errors);
    validate_manifest_entry_points(manifest, &paths, &mut errors);
    validate_manifest_dependencies(manifest, &mut errors, &mut warnings);

    json!({
        "ok": errors.is_empty(),
        "errors": errors,
        "warnings": warnings,
        "package": {
            "file_count": entries.len(),
            "uncompressed_size_bytes": total_uncompressed_size,
            "max_files": MAX_PACKAGE_FILES,
            "max_declared_assets": MAX_DECLARED_ASSETS,
            "max_uncompressed_size_bytes": MAX_UNCOMPRESSED_PACKAGE_BYTES,
        },
        "file_tree": paths.into_iter().collect::<Vec<_>>(),
    })
}

fn validate_manifest_assets(
    manifest: &Value,
    package_paths: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let Some(assets) = manifest.get("assets").and_then(Value::as_array) else {
        return;
    };

    if assets.len() > MAX_DECLARED_ASSETS {
        errors.push(format!(
            "manifest declares more than {MAX_DECLARED_ASSETS} assets"
        ));
    }

    for asset in assets {
        let Some(asset_path) = asset.as_str() else {
            errors.push("manifest assets must contain only strings".to_owned());
            continue;
        };
        match normalize_manifest_path(asset_path) {
            Ok(asset_path) => {
                if !package_paths.contains(&asset_path) {
                    errors.push(format!(
                        "manifest asset '{asset_path}' is missing from package"
                    ));
                }
            }
            Err(error) => errors.push(error),
        }
    }
}

fn validate_manifest_entry_points(
    manifest: &Value,
    package_paths: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let Some(entry_points) = manifest.get("entry_points").and_then(Value::as_object) else {
        return;
    };

    for (key, value) in entry_points {
        let Some(path) = value.as_str() else {
            errors.push(format!("entry point '{key}' must be a package path string"));
            continue;
        };
        match normalize_manifest_path(path) {
            Ok(path) => {
                if !package_paths.contains(&path) {
                    errors.push(format!(
                        "entry point '{key}' path '{path}' is missing from package"
                    ));
                }
            }
            Err(error) => errors.push(error),
        }
    }
}

fn validate_manifest_dependencies(
    manifest: &Value,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let Some(dependencies) = manifest.get("dependencies") else {
        return;
    };

    let Some(dependencies) = dependencies.as_object() else {
        errors.push("dependencies must be an object when provided".to_owned());
        return;
    };

    if dependencies.len() > MAX_MANIFEST_DEPENDENCIES {
        errors.push(format!(
            "manifest declares more than {MAX_MANIFEST_DEPENDENCIES} dependencies"
        ));
    }

    for (name, declaration) in dependencies {
        if name.trim().is_empty() {
            errors.push("dependency names must not be empty".to_owned());
        }

        match declaration {
            Value::String(version) => {
                let version = version.trim();
                if version.is_empty() {
                    errors.push(format!("dependency '{name}' version must not be empty"));
                } else if version == "*" || version.eq_ignore_ascii_case("latest") {
                    warnings.push(format!("dependency '{name}' uses an unpinned version"));
                }
            }
            Value::Object(object) => {
                if !object.contains_key("version") && !object.contains_key("source") {
                    warnings.push(format!(
                        "dependency '{name}' should declare a version or source"
                    ));
                }
            }
            _ => errors.push(format!(
                "dependency '{name}' must be a string or metadata object"
            )),
        }
    }
}

fn security_scan_report(
    manifest: &Value,
    entries: &[ZipEntry],
    listing_product_type: &str,
) -> Value {
    let mut findings = Vec::new();

    if listing_product_type == "backend_extension" {
        findings.push(SecurityFinding {
            severity: RiskLevel::High,
            code: "backend-extension-sandbox-missing",
            message: "backend extensions are blocked until sandbox execution is available"
                .to_owned(),
            path: None,
        });
    }

    for entry in entries {
        scan_path_for_security_findings(&entry.path, &mut findings);
    }

    scan_permissions(manifest, &mut findings);
    scan_dependencies_for_security_findings(manifest, &mut findings);
    scan_manifest_strings(manifest, "$", &mut findings);

    let risk_level = findings
        .iter()
        .map(|finding| finding.severity)
        .max()
        .unwrap_or(RiskLevel::Low);

    json!({
        "risk_level": risk_level.as_str(),
        "blocked": matches!(risk_level, RiskLevel::High | RiskLevel::Critical),
        "findings": findings
            .into_iter()
            .map(|finding| json!({
                "severity": finding.severity.as_str(),
                "code": finding.code,
                "message": finding.message,
                "path": finding.path,
            }))
            .collect::<Vec<_>>(),
    })
}

fn scan_path_for_security_findings(path: &str, findings: &mut Vec<SecurityFinding>) {
    let lower_path = path.to_ascii_lowercase();
    let file_name = lower_path.rsplit('/').next().unwrap_or(lower_path.as_str());

    let forbidden_names = [
        ".env",
        ".env.local",
        ".npmrc",
        ".pypirc",
        "id_rsa",
        "id_dsa",
        "id_ed25519",
        "credentials",
        "credentials.json",
        "service-account.json",
    ];

    if forbidden_names.contains(&file_name)
        || lower_path.ends_with(".pem")
        || lower_path.ends_with(".key")
        || lower_path.ends_with(".p12")
        || lower_path.ends_with(".pfx")
        || lower_path.ends_with(".kube/config")
        || lower_path.ends_with(".aws/credentials")
    {
        findings.push(SecurityFinding {
            severity: RiskLevel::Critical,
            code: "forbidden-secret-file",
            message: "package contains a forbidden secret or credential file".to_owned(),
            path: Some(path.to_owned()),
        });
    }

    let executable_extensions = [
        ".exe", ".dll", ".dylib", ".so", ".jar", ".bat", ".cmd", ".ps1", ".sh",
    ];
    if executable_extensions
        .iter()
        .any(|extension| lower_path.ends_with(extension))
    {
        findings.push(SecurityFinding {
            severity: RiskLevel::High,
            code: "executable-artifact",
            message: "package contains executable or shell-script artifacts".to_owned(),
            path: Some(path.to_owned()),
        });
    }
}

fn scan_permissions(manifest: &Value, findings: &mut Vec<SecurityFinding>) {
    let Some(permissions) = manifest.get("permissions").and_then(Value::as_array) else {
        return;
    };

    for permission in permissions.iter().filter_map(Value::as_str) {
        match permission {
            "external_network.request" => findings.push(SecurityFinding {
                severity: RiskLevel::High,
                code: "external-network-permission",
                message: "manifest requests external network access".to_owned(),
                path: None,
            }),
            "webhook.send" | "settings.read" => findings.push(SecurityFinding {
                severity: RiskLevel::Medium,
                code: "sensitive-permission",
                message: format!("manifest requests sensitive permission '{permission}'"),
                path: None,
            }),
            value if value.ends_with(".write") => findings.push(SecurityFinding {
                severity: RiskLevel::Medium,
                code: "write-permission",
                message: format!("manifest requests write permission '{permission}'"),
                path: None,
            }),
            _ => {}
        }
    }
}

fn scan_dependencies_for_security_findings(manifest: &Value, findings: &mut Vec<SecurityFinding>) {
    let Some(dependencies) = manifest.get("dependencies").and_then(Value::as_object) else {
        return;
    };

    for (name, declaration) in dependencies {
        let Some(raw) = dependency_declaration_string(declaration) else {
            continue;
        };
        let lower = raw.to_ascii_lowercase();
        if lower == "*" || lower == "latest" || lower.contains(" latest") {
            findings.push(SecurityFinding {
                severity: RiskLevel::Medium,
                code: "unpinned-dependency",
                message: format!("dependency '{name}' is not pinned"),
                path: None,
            });
        }

        if lower.starts_with("http://")
            || lower.starts_with("https://")
            || lower.starts_with("git://")
            || lower.starts_with("ssh://")
            || lower.contains("github:")
            || lower.contains("git+")
        {
            findings.push(SecurityFinding {
                severity: RiskLevel::High,
                code: "remote-dependency-source",
                message: format!("dependency '{name}' uses a remote source"),
                path: None,
            });
        }
    }
}

fn dependency_declaration_string(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.trim().to_owned()),
        Value::Object(object) => {
            let version = object.get("version").and_then(Value::as_str);
            let source = object.get("source").and_then(Value::as_str);
            Some(
                [version, source]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        }
        _ => None,
    }
}

fn scan_manifest_strings(value: &Value, path: &str, findings: &mut Vec<SecurityFinding>) {
    match value {
        Value::String(raw) => {
            let lower = raw.to_ascii_lowercase();
            if lower.starts_with("http://")
                || lower.starts_with("https://")
                || lower.contains("://cdn.")
            {
                let severity = if lower.ends_with(".js") || path.contains("script") {
                    RiskLevel::High
                } else {
                    RiskLevel::Medium
                };
                findings.push(SecurityFinding {
                    severity,
                    code: "external-reference",
                    message: format!("manifest contains external reference at {path}"),
                    path: None,
                });
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                scan_manifest_strings(item, &format!("{path}[{index}]"), findings);
            }
        }
        Value::Object(object) => {
            for (key, item) in object {
                scan_manifest_strings(item, &format!("{path}.{key}"), findings);
            }
        }
        _ => {}
    }
}

fn compatibility_report(manifest: &Value, organization_plan_slug: &str) -> Value {
    let compatibility = manifest
        .get("compatibility")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    let min_version = string_from_object(&compatibility, "min_zinhar_version");
    let max_version = string_from_object(&compatibility, "max_zinhar_version");
    let required_plan =
        string_from_object(&compatibility, "required_plan").unwrap_or_else(|| "free".to_owned());
    let required_features = array_of_strings(&compatibility, "required_features");

    let mut reasons = Vec::new();
    if let Some(min_version) = min_version.as_deref() {
        if is_semver(min_version) && compare_semver(CURRENT_ZINHAR_VERSION, min_version).is_lt() {
            reasons.push(format!(
                "requires ZinharCMS {min_version} or newer; current is {CURRENT_ZINHAR_VERSION}"
            ));
        }
    }

    if let Some(max_version) = max_version.as_deref() {
        if is_semver(max_version) && compare_semver(CURRENT_ZINHAR_VERSION, max_version).is_gt() {
            reasons.push(format!(
                "supports ZinharCMS up to {max_version}; current is {CURRENT_ZINHAR_VERSION}"
            ));
        }
    }

    let supported_features: HashSet<&str> =
        SUPPORTED_MARKETPLACE_FEATURES.iter().copied().collect();
    let missing_features: Vec<String> = required_features
        .iter()
        .filter(|feature| !supported_features.contains(feature.as_str()))
        .cloned()
        .collect();
    for feature in &missing_features {
        reasons.push(format!("requires unsupported feature '{feature}'"));
    }

    let plan_ok = plan_rank(organization_plan_slug) >= plan_rank(&required_plan);
    if !plan_ok {
        reasons.push(format!(
            "requires plan '{required_plan}', active organization plan is '{organization_plan_slug}'"
        ));
    }

    let compatible = reasons.is_empty();

    json!({
        "current_zinhar_version": CURRENT_ZINHAR_VERSION,
        "min_zinhar_version": min_version,
        "max_zinhar_version": max_version,
        "required_features": required_features,
        "supported_features": SUPPORTED_MARKETPLACE_FEATURES,
        "missing_features": missing_features,
        "required_plan": required_plan,
        "organization_plan": organization_plan_slug,
        "compatible": compatible,
        "install_eligible": compatible,
        "reasons": reasons,
    })
}

fn parse_zip_entries(bytes: &[u8]) -> Result<Vec<ZipEntry>, String> {
    if bytes.len() < 22 {
        return Err("package is not a readable ZIP archive".to_owned());
    }

    let eocd_offset =
        find_eocd(bytes).ok_or_else(|| "package ZIP central directory was not found".to_owned())?;
    if read_u32(bytes, eocd_offset) != Some(0x0605_4b50) {
        return Err("package ZIP end of central directory is invalid".to_owned());
    }

    let entry_count = read_u16(bytes, eocd_offset + 10)
        .ok_or_else(|| "package ZIP central directory count is invalid".to_owned())?
        as usize;
    let central_directory_size = read_u32(bytes, eocd_offset + 12)
        .ok_or_else(|| "package ZIP central directory size is invalid".to_owned())?
        as usize;
    let central_directory_offset = read_u32(bytes, eocd_offset + 16)
        .ok_or_else(|| "package ZIP central directory offset is invalid".to_owned())?
        as usize;

    if central_directory_offset
        .checked_add(central_directory_size)
        .is_none_or(|end| end > bytes.len())
    {
        return Err("package ZIP central directory is out of bounds".to_owned());
    }

    let mut entries = Vec::with_capacity(entry_count);
    let mut cursor = central_directory_offset;
    for _ in 0..entry_count {
        if read_u32(bytes, cursor) != Some(0x0201_4b50) {
            return Err("package ZIP central directory entry is invalid".to_owned());
        }

        let uncompressed_size = read_u32(bytes, cursor + 24)
            .ok_or_else(|| "package ZIP entry size is invalid".to_owned())?
            as u64;
        let name_len = read_u16(bytes, cursor + 28)
            .ok_or_else(|| "package ZIP entry file name length is invalid".to_owned())?
            as usize;
        let extra_len = read_u16(bytes, cursor + 30)
            .ok_or_else(|| "package ZIP entry extra length is invalid".to_owned())?
            as usize;
        let comment_len = read_u16(bytes, cursor + 32)
            .ok_or_else(|| "package ZIP entry comment length is invalid".to_owned())?
            as usize;
        let name_start = cursor + 46;
        let name_end = name_start
            .checked_add(name_len)
            .ok_or_else(|| "package ZIP entry name is out of bounds".to_owned())?;
        if name_end > bytes.len() {
            return Err("package ZIP entry name is out of bounds".to_owned());
        }

        let path = std::str::from_utf8(&bytes[name_start..name_end])
            .map_err(|_| "package ZIP entry path must be UTF-8".to_owned())?
            .to_owned();
        if !path.ends_with('/') {
            entries.push(ZipEntry {
                path,
                uncompressed_size,
            });
        }

        cursor = name_end
            .checked_add(extra_len)
            .and_then(|value| value.checked_add(comment_len))
            .ok_or_else(|| "package ZIP central directory cursor overflowed".to_owned())?;
    }

    Ok(entries)
}

fn find_eocd(bytes: &[u8]) -> Option<usize> {
    let search_floor = bytes.len().saturating_sub(65_557);
    (search_floor..=bytes.len().saturating_sub(22))
        .rev()
        .find(|offset| read_u32(bytes, *offset) == Some(0x0605_4b50))
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let value = bytes.get(offset..offset + 2)?;
    Some(u16::from_le_bytes([value[0], value[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let value = bytes.get(offset..offset + 4)?;
    Some(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn normalize_manifest_path(path: &str) -> Result<String, String> {
    let trimmed = path.trim().trim_start_matches("./").replace('\\', "/");
    if trimmed.is_empty() {
        return Err("package paths must not be empty".to_owned());
    }
    if trimmed.starts_with('/') || trimmed.contains(':') {
        return Err(format!("package path '{trimmed}' must be relative"));
    }
    if trimmed
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(format!("package path '{trimmed}' contains unsafe segments"));
    }
    if trimmed.chars().any(char::is_control) {
        return Err(format!(
            "package path '{trimmed}' contains control characters"
        ));
    }

    Ok(trimmed)
}

fn string_array(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn string_from_object(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn array_of_strings(object: &Map<String, Value>, key: &str) -> Vec<String> {
    object
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn plan_rank(plan_slug: &str) -> usize {
    match plan_slug {
        "free" => 0,
        "pro" => 1,
        "enterprise" | "custom" => 2,
        _ => 0,
    }
}

fn compare_semver(left: &str, right: &str) -> std::cmp::Ordering {
    let left = semver_core(left);
    let right = semver_core(right);
    left.cmp(&right)
}

fn semver_core(value: &str) -> [u64; 3] {
    let core = value.split(['-', '+']).next().unwrap_or(value);
    let mut parts = core.split('.').map(|part| part.parse::<u64>().unwrap_or(0));
    [
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    ]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{evaluate_marketplace_package, parse_zip_entries};

    fn valid_manifest() -> serde_json::Value {
        json!({
            "manifest_version": "2026-07",
            "name": "SaaS Hero Pack",
            "version": "1.0.0",
            "type": "component_pack",
            "permissions": ["page.read"],
            "compatibility": {
                "min_zinhar_version": "2.0.0",
                "max_zinhar_version": "3.5.0",
                "required_features": ["component_packs"],
                "required_plan": "free"
            },
            "entry_points": {
                "components": "components/index.json"
            },
            "assets": ["components/index.json", "assets/preview.png"],
            "dependencies": {
                "@zinhar/ui": "1.2.3"
            }
        })
    }

    fn fake_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut central = Vec::new();
        let mut local_offset = 0u32;

        for (path, content) in entries {
            let path_bytes = path.as_bytes();
            bytes.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
            bytes.extend_from_slice(&20u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u32.to_le_bytes());
            bytes.extend_from_slice(&(content.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&(content.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(path_bytes);
            bytes.extend_from_slice(content);

            central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
            central.extend_from_slice(&20u16.to_le_bytes());
            central.extend_from_slice(&20u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u32.to_le_bytes());
            central.extend_from_slice(&(content.len() as u32).to_le_bytes());
            central.extend_from_slice(&(content.len() as u32).to_le_bytes());
            central.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u32.to_le_bytes());
            central.extend_from_slice(&local_offset.to_le_bytes());
            central.extend_from_slice(path_bytes);

            local_offset = bytes.len() as u32;
        }

        let central_offset = bytes.len() as u32;
        let central_size = central.len() as u32;
        bytes.extend_from_slice(&central);
        bytes.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&central_size.to_le_bytes());
        bytes.extend_from_slice(&central_offset.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes
    }
    #[test]
    fn reads_zip_central_directory() {
        let bytes = fake_zip(&[
            ("components/index.json", b"{}"),
            ("assets/preview.png", b"png"),
        ]);
        let entries = parse_zip_entries(&bytes).expect("zip entries should parse");

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].path, "components/index.json");
    }

    #[test]
    fn accepts_valid_package_for_review_queue() {
        let bytes = fake_zip(&[
            ("components/index.json", b"{}"),
            ("assets/preview.png", b"png"),
        ]);
        let decision = evaluate_marketplace_package(
            &valid_manifest(),
            &bytes,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "package.zip",
            "component_pack",
            "free",
        );

        assert_eq!(decision.validation_status, "passed");
        assert_eq!(decision.security_risk_level, "low");
        assert_eq!(decision.version_status, "submitted");
        assert_eq!(decision.submission_review_status, "queued");
    }

    #[test]
    fn blocks_missing_declared_assets() {
        let bytes = fake_zip(&[("components/index.json", b"{}")]);
        let decision = evaluate_marketplace_package(
            &valid_manifest(),
            &bytes,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "package.zip",
            "component_pack",
            "free",
        );

        assert_eq!(decision.validation_status, "failed");
        assert_eq!(decision.version_status, "blocked");
    }

    #[test]
    fn blocks_high_risk_packages() {
        let mut manifest = valid_manifest();
        manifest["permissions"] = json!(["page.read", "external_network.request"]);
        let bytes = fake_zip(&[
            ("components/index.json", b"{}"),
            ("assets/preview.png", b"png"),
            (".env", b"SECRET=1"),
        ]);
        let decision = evaluate_marketplace_package(
            &manifest,
            &bytes,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "package.zip",
            "component_pack",
            "free",
        );

        assert_eq!(decision.security_risk_level, "critical");
        assert_eq!(decision.submission_review_status, "blocked");
    }

    #[test]
    fn marks_incompatible_plan_as_not_install_eligible() {
        let mut manifest = valid_manifest();
        manifest["compatibility"]["required_plan"] = json!("enterprise");
        let bytes = fake_zip(&[
            ("components/index.json", b"{}"),
            ("assets/preview.png", b"png"),
        ]);
        let decision = evaluate_marketplace_package(
            &manifest,
            &bytes,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "package.zip",
            "component_pack",
            "free",
        );

        assert_eq!(
            decision
                .compatibility_report
                .get("install_eligible")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_eq!(decision.version_status, "submitted");
    }
}
