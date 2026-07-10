use std::collections::BTreeSet;
use std::fmt;

use serde::Serialize;
use serde_json::Value;

pub const SANDBOX_POLICY_VERSION: &str = "v3.7-allowlisted-host-api";
pub const MAX_RUNTIME_PAYLOAD_BYTES: usize = 64 * 1024;
pub const RUNTIME_STATUS_READY: &str = "ready";
pub const RUNTIME_STATUS_BLOCKED: &str = "blocked";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RuntimeOperationDefinition {
    pub operation: &'static str,
    pub required_permission: &'static str,
    pub product_types: &'static [&'static str],
    pub entry_points: &'static [&'static str],
}

pub const RUNTIME_OPERATION_DEFINITIONS: &[RuntimeOperationDefinition] = &[
    RuntimeOperationDefinition {
        operation: "component.render",
        required_permission: "page.read",
        product_types: &["component_pack", "design_template"],
        entry_points: &["components"],
    },
    RuntimeOperationDefinition {
        operation: "content.read",
        required_permission: "content.read",
        product_types: &["component_pack", "design_template", "integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "content.write",
        required_permission: "content.write",
        product_types: &["integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "page.read",
        required_permission: "page.read",
        product_types: &["component_pack", "design_template", "integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "page.write",
        required_permission: "page.write",
        product_types: &["integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "media.read",
        required_permission: "media.read",
        product_types: &["component_pack", "design_template", "integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "media.write",
        required_permission: "media.write",
        product_types: &["integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "webhook.send",
        required_permission: "webhook.send",
        product_types: &["integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "settings.read",
        required_permission: "settings.read",
        product_types: &["component_pack", "design_template", "integration_plugin"],
        entry_points: &["hooks", "integration"],
    },
    RuntimeOperationDefinition {
        operation: "external_network.request",
        required_permission: "external_network.request",
        product_types: &["integration_plugin"],
        entry_points: &["integration"],
    },
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RuntimeRuleError {
    pub code: &'static str,
    pub message: String,
}

impl fmt::Display for RuntimeRuleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RuntimeRuleError {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct RuntimeAuthorization {
    pub allowed: bool,
    pub operation: String,
    pub required_permission: String,
    pub entry_point: String,
    pub sandbox_policy: &'static str,
    pub execution: &'static str,
}

#[allow(clippy::too_many_arguments)]
pub fn authorize_runtime_operation(
    installation_status: &str,
    runtime_status: &str,
    product_type: &str,
    manifest: &Value,
    approved_permissions: &Value,
    operation: &str,
    entry_point: &str,
    payload: &Value,
) -> Result<RuntimeAuthorization, RuntimeRuleError> {
    if installation_status != "active" {
        return Err(rule_error(
            "installation_inactive",
            "only active Marketplace installations may use the sandbox host API",
        ));
    }
    if runtime_status != RUNTIME_STATUS_READY {
        return Err(rule_error(
            "runtime_blocked",
            "Marketplace runtime is blocked by a kill switch or safety policy",
        ));
    }

    let definition = RUNTIME_OPERATION_DEFINITIONS
        .iter()
        .find(|definition| definition.operation == operation)
        .ok_or_else(|| {
            rule_error(
                "operation_not_allowlisted",
                "runtime operation is not allowlisted",
            )
        })?;
    if !definition.product_types.contains(&product_type) {
        return Err(rule_error(
            "product_type_not_allowed",
            "product type cannot use this sandbox operation",
        ));
    }

    let entry_points = manifest
        .get("entry_points")
        .and_then(Value::as_object)
        .ok_or_else(|| rule_error("entry_points_missing", "manifest entry_points are missing"))?;
    let entry_value = entry_points
        .get(entry_point)
        .and_then(Value::as_str)
        .ok_or_else(|| {
            rule_error(
                "entry_point_not_declared",
                "entry point is not declared by the manifest",
            )
        })?;
    if !definition.entry_points.contains(&entry_point) {
        return Err(rule_error(
            "entry_point_not_allowed",
            "entry point is not allowed for this runtime operation",
        ));
    }
    validate_entry_point_path(entry_value)?;

    let approved = approved_permissions.as_array().ok_or_else(|| {
        rule_error(
            "permission_snapshot_invalid",
            "approved permission snapshot is invalid",
        )
    })?;
    let approved: BTreeSet<&str> = approved.iter().filter_map(Value::as_str).collect();
    if !approved.contains(definition.required_permission) {
        return Err(rule_error(
            "permission_not_approved",
            "runtime operation requires a permission that was not approved for this installation",
        ));
    }

    let payload_bytes = serde_json::to_vec(payload).map_err(|_| {
        rule_error(
            "payload_invalid",
            "runtime payload is not serializable JSON",
        )
    })?;
    if payload_bytes.len() > MAX_RUNTIME_PAYLOAD_BYTES {
        return Err(rule_error(
            "payload_too_large",
            "runtime payload exceeds the sandbox payload limit",
        ));
    }
    if !payload.is_object() {
        return Err(rule_error(
            "payload_invalid",
            "runtime payload must be a JSON object",
        ));
    }

    Ok(RuntimeAuthorization {
        allowed: true,
        operation: operation.to_owned(),
        required_permission: definition.required_permission.to_owned(),
        entry_point: entry_point.to_owned(),
        sandbox_policy: SANDBOX_POLICY_VERSION,
        execution: "not_executed",
    })
}

pub fn validate_entry_point_path(path: &str) -> Result<(), RuntimeRuleError> {
    if path.trim().is_empty()
        || path.starts_with('/')
        || path.contains("..")
        || path.contains(['\\', ':'])
        || path.contains("://")
    {
        return Err(rule_error(
            "entry_point_path_unsafe",
            "manifest entry point path must remain inside the approved artifact",
        ));
    }
    Ok(())
}

pub fn validate_kill_switch_reason(reason: &str) -> Result<String, RuntimeRuleError> {
    let reason = reason.trim();
    if reason.is_empty() {
        return Err(rule_error(
            "reason_required",
            "kill switch reason is required",
        ));
    }
    if reason.chars().count() > 500 {
        return Err(rule_error(
            "reason_too_long",
            "kill switch reason must be 500 characters or fewer",
        ));
    }
    Ok(reason.to_owned())
}

pub fn operation_definitions() -> &'static [RuntimeOperationDefinition] {
    RUNTIME_OPERATION_DEFINITIONS
}

fn rule_error(code: &'static str, message: &str) -> RuntimeRuleError {
    RuntimeRuleError {
        code,
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        MAX_RUNTIME_PAYLOAD_BYTES, RUNTIME_STATUS_BLOCKED, authorize_runtime_operation,
        validate_entry_point_path, validate_kill_switch_reason,
    };

    fn component_manifest() -> serde_json::Value {
        json!({
            "entry_points": { "components": "components/index.json" }
        })
    }

    #[test]
    fn allows_declared_component_operation_with_approved_permission() {
        let result = authorize_runtime_operation(
            "active",
            "ready",
            "component_pack",
            &component_manifest(),
            &json!(["page.read"]),
            "component.render",
            "components",
            &json!({ "component": "hero" }),
        )
        .expect("component runtime operation should be allowed");
        assert!(result.allowed);
        assert_eq!(result.execution, "not_executed");
    }

    #[test]
    fn denies_permission_escalation_and_unknown_operations() {
        let manifest = component_manifest();
        let permission_error = authorize_runtime_operation(
            "active",
            "ready",
            "component_pack",
            &manifest,
            &json!([]),
            "component.render",
            "components",
            &json!({}),
        )
        .expect_err("unapproved permission must be denied");
        assert_eq!(permission_error.code, "permission_not_approved");

        let operation_error = authorize_runtime_operation(
            "active",
            "ready",
            "component_pack",
            &manifest,
            &json!(["page.read"]),
            "database.root",
            "components",
            &json!({}),
        )
        .expect_err("unknown runtime operation must be denied");
        assert_eq!(operation_error.code, "operation_not_allowlisted");
    }

    #[test]
    fn blocks_inactive_or_kill_switched_installations() {
        let error = authorize_runtime_operation(
            "active",
            RUNTIME_STATUS_BLOCKED,
            "component_pack",
            &component_manifest(),
            &json!(["page.read"]),
            "component.render",
            "components",
            &json!({}),
        )
        .expect_err("kill switched installation must be denied");
        assert_eq!(error.code, "runtime_blocked");
    }

    #[test]
    fn rejects_unsafe_entry_points_and_large_payloads() {
        assert!(validate_entry_point_path("../escape.js").is_err());
        let oversized = "x".repeat(MAX_RUNTIME_PAYLOAD_BYTES);
        let error = authorize_runtime_operation(
            "active",
            "ready",
            "component_pack",
            &component_manifest(),
            &json!(["page.read"]),
            "component.render",
            "components",
            &json!({ "data": oversized }),
        )
        .expect_err("oversized payload must be denied");
        assert_eq!(error.code, "payload_too_large");
    }

    #[test]
    fn kill_switch_reason_is_bounded() {
        assert!(validate_kill_switch_reason("artifact compromise").is_ok());
        assert!(validate_kill_switch_reason(" ").is_err());
        assert!(validate_kill_switch_reason(&"x".repeat(501)).is_err());
    }
}
