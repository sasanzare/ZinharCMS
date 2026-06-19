use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::error::AppError;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct FieldSchemaDocument {
    #[serde(default)]
    pub fields: Vec<FieldSchema>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct FieldSchema {
    pub id: Option<String>,
    pub name: String,
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub required: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub allowed_types: Option<Vec<String>>,
    pub multiple: Option<bool>,
    pub target_type: Option<String>,
    pub cardinality: Option<String>,
    pub source_field: Option<String>,
}

pub fn parse_fields(value: &Value) -> Result<FieldSchemaDocument, AppError> {
    if !value.is_object() {
        return Err(AppError::Validation(
            "fields must be a JSON object".to_owned(),
        ));
    }

    let document: FieldSchemaDocument = serde_json::from_value(value.clone())
        .map_err(|error| AppError::Validation(error.to_string()))?;

    for field in &document.fields {
        validate_field_schema(field)?;
    }

    Ok(document)
}

pub fn validate_entry_data(fields: &FieldSchemaDocument, data: &Value) -> Result<(), AppError> {
    let object = data
        .as_object()
        .ok_or_else(|| AppError::Validation("entry data must be a JSON object".to_owned()))?;

    for field in &fields.fields {
        let value = object.get(&field.name);
        if field.required && value.is_none() {
            return Err(AppError::Validation(format!(
                "required field '{}' is missing",
                field.name
            )));
        }

        if let Some(value) = value {
            validate_value(field, value)?;
        }
    }

    Ok(())
}

fn validate_field_schema(field: &FieldSchema) -> Result<(), AppError> {
    if !is_valid_identifier(&field.name) {
        return Err(AppError::Validation(format!(
            "field '{}' must use snake_case letters, numbers, or underscores",
            field.name
        )));
    }

    match field.field_type.as_str() {
        "text" | "longtext" | "richtext" | "number" | "boolean" | "datetime" | "media"
        | "relation" | "json" | "slug" => Ok(()),
        other => Err(AppError::Validation(format!(
            "field type '{other}' is not supported"
        ))),
    }
}

fn validate_value(field: &FieldSchema, value: &Value) -> Result<(), AppError> {
    match field.field_type.as_str() {
        "text" | "longtext" | "richtext" | "slug" | "datetime" => validate_string(field, value),
        "number" => validate_number(field, value),
        "boolean" => {
            if value.is_boolean() {
                Ok(())
            } else {
                Err(type_error(field, "boolean"))
            }
        }
        "media" => validate_reference(field, value),
        "relation" => validate_reference(field, value),
        "json" => Ok(()),
        _ => Ok(()),
    }
}

fn validate_string(field: &FieldSchema, value: &Value) -> Result<(), AppError> {
    let Some(value) = value.as_str() else {
        return Err(type_error(field, "string"));
    };

    let value_len = value.chars().count();
    if let Some(min) = field.min_length.filter(|min| value_len < *min) {
        return Err(AppError::Validation(format!(
            "field '{}' must be at least {min} characters",
            field.name
        )));
    }

    if let Some(max) = field.max_length.filter(|max| value_len > *max) {
        return Err(AppError::Validation(format!(
            "field '{}' must be at most {max} characters",
            field.name
        )));
    }

    if field.field_type == "slug" && !is_valid_slug(value) {
        return Err(AppError::Validation(format!(
            "field '{}' must be a valid slug",
            field.name
        )));
    }

    Ok(())
}

fn validate_number(field: &FieldSchema, value: &Value) -> Result<(), AppError> {
    let Some(value) = value.as_f64() else {
        return Err(type_error(field, "number"));
    };

    if let Some(min) = field.min.filter(|min| value < *min) {
        return Err(AppError::Validation(format!(
            "field '{}' must be greater than or equal to {min}",
            field.name
        )));
    }

    if let Some(max) = field.max.filter(|max| value > *max) {
        return Err(AppError::Validation(format!(
            "field '{}' must be less than or equal to {max}",
            field.name
        )));
    }

    Ok(())
}

fn validate_reference(field: &FieldSchema, value: &Value) -> Result<(), AppError> {
    let multiple = field.multiple.unwrap_or(false);
    if multiple {
        if value.as_array().is_some() {
            Ok(())
        } else {
            Err(type_error(field, "array"))
        }
    } else if value.as_str().is_some() {
        Ok(())
    } else {
        Err(type_error(field, "string"))
    }
}

fn type_error(field: &FieldSchema, expected: &str) -> AppError {
    AppError::Validation(format!("field '{}' must be {expected}", field.name))
}

fn is_valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() || first == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

pub fn is_valid_slug(value: &str) -> bool {
    let mut previous_dash = false;
    if value.is_empty() || value.starts_with('-') || value.ends_with('-') {
        return false;
    }

    for ch in value.chars() {
        let ok = ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-';
        if !ok || (ch == '-' && previous_dash) {
            return false;
        }
        previous_dash = ch == '-';
    }

    true
}
