use std::collections::HashSet;

use serde_json::{Map, Value, json};

use crate::services::marketplace_manifest::is_semver;
use crate::services::marketplace_validation::CURRENT_ZINHAR_VERSION;

const SUPPORTED_MARKETPLACE_FEATURES: &[&str] = &[
    "component_packs",
    "design_templates",
    "integration_plugins",
    "media_assets",
    "page_builder",
];

pub fn catalog_compatibility_report(manifest: &Value, organization_plan_slug: &str) -> Value {
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

pub fn is_catalog_compatible(report: &Value) -> bool {
    report
        .get("install_eligible")
        .and_then(Value::as_bool)
        .unwrap_or(false)
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

    use super::{catalog_compatibility_report, is_catalog_compatible};

    const PHASE_FIVE_DOC: &str = include_str!("../../../docs/V3_PHASE_FIVE.md");
    const MARKETPLACE_ROUTES: &str = include_str!("../routes/marketplace.rs");

    fn manifest(required_plan: &str) -> serde_json::Value {
        json!({
            "manifest_version": "2026-07",
            "name": "SaaS Hero Pack",
            "version": "1.0.0",
            "type": "component_pack",
            "permissions": ["page.read"],
            "compatibility": {
                "min_zinhar_version": "2.0.0",
                "max_zinhar_version": "3.0.0",
                "required_plan": required_plan
            },
            "entry_points": { "components": "components/index.json" },
            "assets": ["components/hero.json"]
        })
    }

    #[test]
    fn catalog_compatibility_uses_active_organization_plan() {
        let blocked = catalog_compatibility_report(&manifest("pro"), "free");
        assert!(!is_catalog_compatible(&blocked));
        assert_eq!(blocked["required_plan"], "pro");
        assert_eq!(blocked["organization_plan"], "free");

        let eligible = catalog_compatibility_report(&manifest("pro"), "pro");
        assert!(is_catalog_compatible(&eligible));
    }

    #[test]
    fn phase_five_contract_is_documented_and_routed() {
        for required in [
            "Public Catalog",
            "Search And Filter",
            "Listing Detail Page",
            "/api/marketplace/catalog",
            "listing.status = 'approved'",
            "version.status = 'approved'",
            "install_eligible",
            "permissions",
        ] {
            assert!(
                PHASE_FIVE_DOC.contains(required) || MARKETPLACE_ROUTES.contains(required),
                "missing phase 5 contract term: {required}"
            );
        }
    }
}
