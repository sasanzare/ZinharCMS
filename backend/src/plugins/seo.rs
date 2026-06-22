use serde_json::Value;

use crate::error::AppError;
use crate::plugins::{CmsPlugin, EntryData, PluginContext};

pub struct SeoAutoPlugin;

impl CmsPlugin for SeoAutoPlugin {
    fn key(&self) -> &'static str {
        "seo-auto"
    }

    fn name(&self) -> &'static str {
        "SEO Auto Generator"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn description(&self) -> &'static str {
        "Generates a data.slug value from title before entry save when slug is empty."
    }

    fn hooks(&self) -> &'static [&'static str] {
        &["entry.before_save"]
    }

    fn on_entry_before_save(
        &self,
        entry: &mut EntryData,
        _ctx: &PluginContext,
    ) -> Result<(), AppError> {
        let Some(object) = entry.data.as_object_mut() else {
            return Ok(());
        };
        let slug_missing = object
            .get("slug")
            .and_then(Value::as_str)
            .is_none_or(|slug| slug.trim().is_empty());
        if !slug_missing {
            return Ok(());
        }
        let Some(title) = object.get("title").and_then(Value::as_str) else {
            return Ok(());
        };

        object.insert("slug".to_owned(), Value::String(slugify(title)));
        Ok(())
    }
}

fn slugify(value: &str) -> String {
    let mut output = String::new();
    let mut last_dash = false;

    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if !last_dash && !output.is_empty() {
            output.push('-');
            last_dash = true;
        }
    }

    let output = output.trim_matches('-').to_owned();
    if output.is_empty() {
        "untitled".to_owned()
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes_ascii_title() {
        assert_eq!(slugify("Hello, Phase 6!"), "hello-phase-6");
    }

    #[test]
    fn plugin_adds_missing_slug() {
        let mut entry = EntryData {
            type_slug: "articles".to_owned(),
            data: serde_json::json!({ "title": "Hello World" }),
        };
        let ctx = PluginContext {
            user_id: uuid::Uuid::nil(),
            organization_id: uuid::Uuid::nil(),
        };
        SeoAutoPlugin
            .on_entry_before_save(&mut entry, &ctx)
            .unwrap();
        assert_eq!(entry.data["slug"], "hello-world");
    }
}
