use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use ammonia::{Builder, Url, UrlRelative};
use serde_json::Value;

use crate::error::AppError;
use crate::services::entry_validation::FieldSchemaDocument;

pub const MAX_RICH_TEXT_INPUT_BYTES: usize = 128 * 1024;
pub const MAX_PAGE_DOCUMENT_BYTES: usize = 1024 * 1024;
pub const MAX_RICH_TEXT_TAGS: usize = 4_096;
pub const MAX_RICH_TEXT_ATTRIBUTES: usize = 4_096;
pub const MAX_RICH_TEXT_NESTING: usize = 128;
pub const MAX_RICH_TEXT_URL_BYTES: usize = 2_048;

const ALLOWED_TAGS: [&str; 32] = [
    "a",
    "b",
    "blockquote",
    "br",
    "code",
    "del",
    "em",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hr",
    "i",
    "img",
    "li",
    "ol",
    "p",
    "pre",
    "s",
    "span",
    "strong",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    "u",
    "ul",
];

const REMOVE_WITH_CONTENT: [&str; 22] = [
    "applet", "audio", "base", "button", "canvas", "embed", "form", "frame", "frameset", "iframe",
    "input", "link", "math", "meta", "noscript", "object", "portal", "script", "style", "svg",
    "template", "video",
];

pub fn sanitize_entry_data(fields: &FieldSchemaDocument, data: Value) -> Result<Value, AppError> {
    let Some(mut object) = data.as_object().cloned() else {
        return Ok(data);
    };

    for field in &fields.fields {
        let Some(value) = object.get(&field.name).and_then(Value::as_str) else {
            continue;
        };
        match field.field_type.as_str() {
            "richtext" => {
                object.insert(
                    field.name.clone(),
                    Value::String(sanitize_rich_text(value)?),
                );
            }
            "url" => {
                object.insert(
                    field.name.clone(),
                    Value::String(sanitize_rich_text_url(value).unwrap_or_default()),
                );
            }
            _ => {}
        }
    }

    Ok(Value::Object(object))
}

pub fn sanitize_rich_text(value: &str) -> Result<String, AppError> {
    validate_rich_text_complexity(value)?;
    Ok(rich_text_builder().clean(value).to_string())
}

pub fn sanitize_rich_text_url(value: &str) -> Option<String> {
    if value.len() > MAX_RICH_TEXT_URL_BYTES {
        return None;
    }
    let value = value.trim();
    if value.is_empty()
        || value.contains('\\')
        || value.chars().any(|character| character.is_control())
        || value.starts_with("//")
    {
        return None;
    }

    if value.starts_with('/') || value.starts_with("./") || value.starts_with("../") {
        return Some(value.to_owned());
    }
    if value.starts_with('#') && !value[1..].contains('#') {
        return Some(value.to_owned());
    }

    let decoded_prefix = decode_percent_encoded_prefix(value)?;
    let parsed = Url::parse(&decoded_prefix).ok()?;
    if !matches!(parsed.scheme(), "https" | "mailto" | "tel") {
        return None;
    }
    if parsed.scheme() == "https" && (!parsed.username().is_empty() || parsed.password().is_some())
    {
        return None;
    }
    Some(value.to_owned())
}

pub fn sanitize_external_https_url(value: &str) -> Option<String> {
    let safe = sanitize_rich_text_url(value)?;
    let parsed = Url::parse(&safe).ok()?;
    (parsed.scheme() == "https").then_some(safe)
}

pub fn sanitize_page_document(
    page_json: &Value,
    component_schemas: &HashMap<String, Value>,
) -> Result<Value, AppError> {
    if serde_json::to_vec(page_json)
        .map_err(|_| AppError::Validation("page document is invalid".to_owned()))?
        .len()
        > MAX_PAGE_DOCUMENT_BYTES
    {
        return Err(AppError::Validation(
            "page document exceeds the maximum size".to_owned(),
        ));
    }
    let mut sanitized = page_json.clone();
    if let Some(metadata) = sanitized.get_mut("metadata").and_then(Value::as_object_mut)
        && let Some(image) = metadata.get("og_image").and_then(Value::as_str)
    {
        metadata.insert(
            "og_image".to_owned(),
            Value::String(sanitize_same_origin_media_url(image).unwrap_or_default()),
        );
    }
    if let Some(layout) = sanitized.get_mut("layout") {
        sanitize_page_node(layout, component_schemas)?;
    }
    Ok(sanitized)
}

fn sanitize_page_node(
    node: &mut Value,
    component_schemas: &HashMap<String, Value>,
) -> Result<(), AppError> {
    let Some(object) = node.as_object_mut() else {
        return Ok(());
    };
    let component_type = object
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    if component_type != "root"
        && let (Some(schema), Some(props)) = (
            component_schemas.get(&component_type),
            object.get_mut("props").and_then(Value::as_object_mut),
        )
        && let Some(definitions) = component_property_definitions(schema)
    {
        for (name, definition) in definitions {
            let Some(value) = props.get(name).and_then(Value::as_str) else {
                continue;
            };
            if is_rich_text_property(&component_type, name, definition) {
                props.insert(name.clone(), Value::String(sanitize_rich_text(value)?));
            } else if is_url_property(definition) {
                props.insert(
                    name.clone(),
                    Value::String(sanitize_rich_text_url(value).unwrap_or_default()),
                );
            }
        }
    }

    if object.contains_key("styles") {
        object.insert("styles".to_owned(), Value::Object(Default::default()));
    }
    if let Some(children) = object.get_mut("children").and_then(Value::as_array_mut) {
        for child in children {
            sanitize_page_node(child, component_schemas)?;
        }
    }
    Ok(())
}

fn component_property_definitions(schema: &Value) -> Option<&serde_json::Map<String, Value>> {
    let object = schema.as_object()?;
    if object.get("type").and_then(Value::as_str) == Some("object") {
        object.get("properties").and_then(Value::as_object)
    } else {
        Some(object)
    }
}

fn is_rich_text_property(component_type: &str, name: &str, definition: &Value) -> bool {
    definition.get("type").and_then(Value::as_str) == Some("richtext")
        || (component_type == "rich-text" && matches!(name, "html" | "body"))
}

fn is_url_property(definition: &Value) -> bool {
    definition.get("type").and_then(Value::as_str) == Some("url")
        || matches!(
            definition.get("format").and_then(Value::as_str),
            Some("uri" | "uri-reference" | "url")
        )
}

fn rich_text_builder() -> Builder<'static> {
    let mut tag_attributes = HashMap::new();
    tag_attributes.insert(
        "a",
        HashSet::from(["href", "target", "title", "aria-label"]),
    );
    tag_attributes.insert(
        "img",
        HashSet::from(["src", "alt", "title", "width", "height", "aria-label"]),
    );
    tag_attributes.insert("td", HashSet::from(["colspan", "rowspan"]));
    tag_attributes.insert("th", HashSet::from(["colspan", "rowspan", "scope"]));

    let mut builder = Builder::empty();
    builder
        .tags(ALLOWED_TAGS.into_iter().collect())
        .tag_attributes(tag_attributes)
        .clean_content_tags(REMOVE_WITH_CONTENT.into_iter().collect())
        .url_schemes(HashSet::from(["https", "mailto", "tel"]))
        .url_relative(UrlRelative::PassThrough)
        .link_rel(Some("noopener noreferrer"))
        .strip_comments(true)
        .attribute_filter(|element, attribute, value| match (element, attribute) {
            ("a", "href") => sanitize_rich_text_url(value).map(Cow::Owned),
            ("a", "target") if matches!(value, "_blank" | "_self") => Some(value.into()),
            ("a", "target") => None,
            ("img", "src") => sanitize_same_origin_media_url(value).map(Cow::Owned),
            ("img", "width" | "height") if valid_image_dimension(value) => Some(value.into()),
            ("img", "width" | "height") => None,
            ("td" | "th", "colspan" | "rowspan") if valid_table_span(value) => Some(value.into()),
            ("td" | "th", "colspan" | "rowspan") => None,
            _ => Some(value.into()),
        });
    builder
}

fn sanitize_same_origin_media_url(value: &str) -> Option<String> {
    let value = value.trim();
    if value.len() > MAX_RICH_TEXT_URL_BYTES
        || !value.starts_with('/')
        || value.starts_with("//")
        || value.contains('\\')
        || value.chars().any(|character| character.is_control())
    {
        return None;
    }
    Some(value.to_owned())
}

fn valid_image_dimension(value: &str) -> bool {
    value
        .parse::<u16>()
        .is_ok_and(|dimension| (1..=4_096).contains(&dimension))
}

fn valid_table_span(value: &str) -> bool {
    value
        .parse::<u8>()
        .is_ok_and(|span| (1..=100).contains(&span))
}

fn validate_rich_text_complexity(value: &str) -> Result<(), AppError> {
    if value.len() > MAX_RICH_TEXT_INPUT_BYTES {
        return Err(AppError::Validation(
            "rich text exceeds the maximum size".to_owned(),
        ));
    }

    let bytes = value.as_bytes();
    let mut cursor: usize = 0;
    let mut depth: usize = 0;
    let mut tag_count: usize = 0;
    let mut attribute_count: usize = 0;
    while cursor < bytes.len() {
        let Some(relative_start) = value[cursor..].find('<') else {
            break;
        };
        let start = cursor + relative_start;
        let Some(relative_end) = value[start..].find('>') else {
            break;
        };
        let end = start + relative_end;
        let tag = value[start + 1..end].trim();
        cursor = end + 1;
        if tag.is_empty() || tag.starts_with('!') || tag.starts_with('?') {
            continue;
        }

        tag_count += 1;
        attribute_count += tag.bytes().filter(|byte| *byte == b'=').count();
        if tag_count > MAX_RICH_TEXT_TAGS || attribute_count > MAX_RICH_TEXT_ATTRIBUTES {
            return Err(AppError::Validation("rich text is too complex".to_owned()));
        }

        if tag.starts_with('/') {
            depth = depth.saturating_sub(1);
        } else if !tag.ends_with('/') && !is_void_tag(tag) {
            depth += 1;
            if depth > MAX_RICH_TEXT_NESTING {
                return Err(AppError::Validation(
                    "rich text is nested too deeply".to_owned(),
                ));
            }
        }
    }
    Ok(())
}

fn is_void_tag(tag: &str) -> bool {
    let name = tag
        .split_ascii_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    matches!(
        name.as_str(),
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn decode_percent_encoded_prefix(value: &str) -> Option<String> {
    let limit = value.find(['/', '?', '#']).unwrap_or(value.len()).min(64);
    let mut decoded = String::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut cursor = 0;
    while cursor < value.len() {
        if cursor < limit && bytes[cursor] == b'%' {
            if cursor + 2 >= value.len() {
                return None;
            }
            let high = hex_value(bytes[cursor + 1])?;
            let low = hex_value(bytes[cursor + 2])?;
            let byte = high * 16 + low;
            if !byte.is_ascii() {
                return None;
            }
            decoded.push(char::from(byte));
            cursor += 3;
        } else {
            let character = value[cursor..].chars().next()?;
            decoded.push(character);
            cursor += character.len_utf8();
        }
    }
    Some(decoded)
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde_json::{Value, json};

    use super::{
        MAX_PAGE_DOCUMENT_BYTES, MAX_RICH_TEXT_INPUT_BYTES, sanitize_page_document,
        sanitize_rich_text, sanitize_rich_text_url,
    };

    #[derive(Debug, Deserialize)]
    struct Corpus {
        malicious: Vec<MaliciousCase>,
        safe: Vec<SafeCase>,
    }

    #[derive(Debug, Deserialize)]
    struct MaliciousCase {
        name: String,
        html: String,
    }

    #[derive(Debug, Deserialize)]
    struct SafeCase {
        name: String,
        html: String,
        fragments: Vec<String>,
    }

    fn corpus() -> Corpus {
        serde_json::from_str(include_str!("../../../security/phase4-xss-corpus.json")).unwrap()
    }

    #[test]
    fn malicious_corpus_removes_executable_markup() {
        for case in corpus().malicious {
            let output = sanitize_rich_text(&case.html)
                .unwrap_or_else(|error| panic!("{} failed to sanitize: {error}", case.name));
            let lower = output.to_ascii_lowercase();
            for forbidden in [
                "<script",
                "<svg",
                "<math",
                "<iframe",
                "<object",
                "<embed",
                "<form",
                "<input",
                "<base",
                "<meta",
                "javascript:",
                "data:",
                "srcdoc",
                "style=",
                " id=",
                " name=",
            ] {
                assert!(
                    !lower.contains(forbidden),
                    "{} retained forbidden content: {forbidden}",
                    case.name
                );
            }
            assert!(
                !lower
                    .split_ascii_whitespace()
                    .any(|part| part.starts_with("on") && part.contains('=')),
                "{} retained an event handler",
                case.name
            );
        }
    }

    #[test]
    fn safe_corpus_preserves_supported_formatting() {
        for case in corpus().safe {
            let output = sanitize_rich_text(&case.html)
                .unwrap_or_else(|error| panic!("{} failed to sanitize: {error}", case.name));
            for fragment in case.fragments {
                assert!(
                    output.contains(&fragment),
                    "{} did not preserve {fragment}: {output}",
                    case.name
                );
            }
        }
    }

    #[test]
    fn url_policy_rejects_dangerous_and_ambiguous_destinations() {
        for value in [
            "javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            "data:text/html,blocked",
            "vbscript:blocked",
            "file:///tmp/blocked",
            "filesystem:https://example.invalid/temporary/file",
            "chrome-extension://blocked",
            "about:blank",
            "//example.invalid/path",
            "https://user:password@example.invalid/",
            "https:\\\\example.invalid\\path",
            "\0https://example.invalid/",
        ] {
            assert!(
                sanitize_rich_text_url(value).is_none(),
                "dangerous URL was accepted: {value:?}"
            );
        }

        for value in [
            "/guide",
            "./guide",
            "../guide",
            "#section",
            "https://example.invalid/docs",
            "mailto:security@example.invalid",
            "tel:+12025550123",
        ] {
            assert!(
                sanitize_rich_text_url(value).is_some(),
                "safe URL was rejected: {value}"
            );
        }
    }

    #[test]
    fn rich_text_processing_has_size_and_complexity_limits() {
        let oversized = "a".repeat(MAX_RICH_TEXT_INPUT_BYTES + 1);
        assert!(sanitize_rich_text(&oversized).is_err());

        let deeply_nested = format!(
            "{}content{}",
            "<blockquote>".repeat(2_100),
            "</blockquote>".repeat(2_100)
        );
        assert!(sanitize_rich_text(&deeply_nested).is_err());

        let excessive_attributes = format!("<p {}>content</p>", "title=\"x\" ".repeat(4_100));
        assert!(sanitize_rich_text(&excessive_attributes).is_err());
    }

    #[test]
    fn page_documents_sanitize_registered_rich_text_and_url_properties() {
        let schemas = HashMap::from([(
            "text-block".to_owned(),
            json!({
                "body": {"type": "richtext"},
                "cta_url": {"type": "url"}
            }),
        )]);
        let page = json!({
            "version": "1.0",
            "metadata": {},
            "layout": {
                "id": "root",
                "type": "root",
                "children": [{
                    "id": "node",
                    "type": "text-block",
                    "props": {
                        "body": "<p>safe</p><img src=x onerror=alert(1)>",
                        "cta_url": "javascript:alert(1)"
                    },
                    "styles": {"background": "url(javascript:alert(1))"},
                    "children": []
                }]
            }
        });

        let sanitized = sanitize_page_document(&page, &schemas).unwrap();
        assert_eq!(
            sanitized.pointer("/layout/children/0/props/body"),
            Some(&Value::String("<p>safe</p><img>".to_owned()))
        );
        assert_eq!(
            sanitized.pointer("/layout/children/0/props/cta_url"),
            Some(&Value::String(String::new()))
        );
        assert_eq!(
            sanitized.pointer("/layout/children/0/styles"),
            Some(&json!({}))
        );
    }

    #[test]
    fn page_documents_support_legacy_json_schema_rich_text_components() {
        let schemas = HashMap::from([(
            "rich-text".to_owned(),
            json!({
                "type": "object",
                "required": ["html"],
                "properties": {
                    "html": {"type": "string"},
                    "documentation": {"type": "string", "format": "uri"}
                }
            }),
        )]);
        let page = json!({
            "version": "1.0",
            "metadata": {},
            "layout": {
                "id": "root",
                "type": "root",
                "children": [{
                    "id": "legacy",
                    "type": "rich-text",
                    "props": {
                        "html": "<p>safe</p><script>alert(1)</script>",
                        "documentation": "javascript:alert(1)"
                    },
                    "styles": {},
                    "children": []
                }]
            }
        });

        let sanitized = sanitize_page_document(&page, &schemas).unwrap();
        assert_eq!(
            sanitized.pointer("/layout/children/0/props/html"),
            Some(&Value::String("<p>safe</p>".to_owned()))
        );
        assert_eq!(
            sanitized.pointer("/layout/children/0/props/documentation"),
            Some(&Value::String(String::new()))
        );
    }

    #[test]
    fn page_documents_reject_oversized_payloads() {
        let page = json!({
            "version": "1.0",
            "metadata": {
                "description": "a".repeat(MAX_PAGE_DOCUMENT_BYTES)
            },
            "layout": {
                "id": "root",
                "type": "root",
                "children": []
            }
        });

        assert!(sanitize_page_document(&page, &HashMap::new()).is_err());
    }
}
