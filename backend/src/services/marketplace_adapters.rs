use serde_json::{Map, Value};

pub const PHASE_EIGHT_CONTRACT_VERSION: &str = "2026-07";
pub const PUBLIC_HOOK_TYPES: &[&str] = &[
    "sidebar.item",
    "dashboard.widget",
    "form.field",
    "webhook.adapter",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition {
    pub key: String,
    pub name: String,
    pub category: String,
    pub props_schema: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookDefinition {
    pub key: String,
    pub hook_type: String,
    pub label: String,
    pub contract_version: String,
    pub config: Value,
}

pub fn component_definitions(
    manifest: &Value,
    namespace: &str,
) -> Result<Vec<ComponentDefinition>, String> {
    let Some(items) = manifest.get("components").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    let mut definitions = Vec::with_capacity(items.len());
    for item in items {
        let object = item
            .as_object()
            .ok_or_else(|| "manifest components must be objects".to_owned())?;
        let source_key = required_slug(object, "key")?;
        let key = format!("mp-{namespace}-{source_key}");
        let name = required_text(object, "name")?;
        let category = required_slug(object, "category")?;
        let props_schema = object
            .get("props_schema")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new()));
        if !props_schema.is_object() {
            return Err(format!(
                "component '{source_key}' props_schema must be an object"
            ));
        }
        definitions.push(ComponentDefinition {
            key,
            name,
            category,
            props_schema,
        });
    }
    Ok(definitions)
}

pub fn hook_definitions(manifest: &Value) -> Result<Vec<HookDefinition>, String> {
    let Some(items) = manifest.get("hooks").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    let mut definitions = Vec::with_capacity(items.len());
    for item in items {
        let object = item
            .as_object()
            .ok_or_else(|| "manifest hooks must be objects".to_owned())?;
        let key = required_slug(object, "key")?;
        let hook_type = required_text(object, "type")?;
        if !PUBLIC_HOOK_TYPES.contains(&hook_type.as_str()) {
            return Err(format!(
                "hook '{key}' uses unsupported public hook type '{hook_type}'"
            ));
        }
        let label = required_text(object, "label")?;
        let config = object
            .get("config")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new()));
        if !config.is_object() {
            return Err(format!("hook '{key}' config must be an object"));
        }
        definitions.push(HookDefinition {
            key,
            hook_type,
            label,
            contract_version: object
                .get("contract_version")
                .and_then(Value::as_str)
                .unwrap_or(PHASE_EIGHT_CONTRACT_VERSION)
                .to_owned(),
            config,
        });
    }
    Ok(definitions)
}

pub fn template_page_json(
    manifest: &Value,
    template_key: Option<&str>,
) -> Result<(String, Value, Vec<String>), String> {
    let template = manifest
        .get("template")
        .ok_or_else(|| "manifest template is missing".to_owned())?;
    let object = template
        .as_object()
        .ok_or_else(|| "manifest template must be an object".to_owned())?;
    let key = object
        .get("key")
        .and_then(Value::as_str)
        .unwrap_or("default");
    if template_key.is_some_and(|requested| requested != key) {
        return Err("requested template_key is not present in the installed package".to_owned());
    }
    let page_json = object
        .get("page_json")
        .cloned()
        .ok_or_else(|| "manifest template.page_json is missing".to_owned())?;
    if !page_json.is_object() {
        return Err("manifest template.page_json must be an object".to_owned());
    }
    let assets = object
        .get("assets")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();
    Ok((key.to_owned(), page_json, assets))
}

pub fn apply_asset_mapping(value: &mut Value, mapping: &Map<String, Value>) -> Result<(), String> {
    match value {
        Value::String(text) if text.starts_with("asset:") => {
            let key = text.trim_start_matches("asset:");
            let replacement = mapping
                .get(key)
                .ok_or_else(|| format!("asset mapping is missing for '{key}'"))?;
            *value = replacement.clone();
        }
        Value::Array(items) => {
            for item in items {
                apply_asset_mapping(item, mapping)?;
            }
        }
        Value::Object(object) => {
            for item in object.values_mut() {
                apply_asset_mapping(item, mapping)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn required_text(object: &Map<String, Value>, field: &str) -> Result<String, String> {
    let value = object
        .get(field)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("manifest field '{field}' is required"))?;
    Ok(value.to_owned())
}

fn required_slug(object: &Map<String, Value>, field: &str) -> Result<String, String> {
    let value = required_text(object, field)?;
    let valid = value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--");
    if valid {
        Ok(value)
    } else {
        Err(format!("manifest field '{field}' must be a lowercase slug"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_namespaced_components_and_public_hooks() {
        let manifest = json!({
            "components": [{"key": "hero", "name": "Hero", "category": "sections", "props_schema": {}}],
            "hooks": [{"key": "nav", "type": "sidebar.item", "label": "Navigation"}]
        });
        assert_eq!(
            component_definitions(&manifest, "demo").unwrap()[0].key,
            "mp-demo-hero"
        );
        assert_eq!(
            hook_definitions(&manifest).unwrap()[0].hook_type,
            "sidebar.item"
        );
    }

    #[test]
    fn rejects_private_hook_types_and_requires_asset_mapping() {
        let manifest =
            json!({"hooks": [{"key": "root", "type": "database.root", "label": "Root"}]});
        assert!(hook_definitions(&manifest).is_err());
        let mut template = json!({"value": "asset:hero"});
        assert!(apply_asset_mapping(&mut template, &Map::new()).is_err());
    }
}
