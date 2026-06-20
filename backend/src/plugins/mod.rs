use serde_json::Value;
use uuid::Uuid;

use crate::error::AppError;

mod seo;

pub struct EntryData {
    pub type_slug: String,
    pub data: Value,
}

pub struct PluginContext {
    pub user_id: Uuid,
}

pub trait CmsPlugin: Send + Sync {
    fn key(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn hooks(&self) -> &'static [&'static str];

    fn on_entry_before_save(
        &self,
        _entry: &mut EntryData,
        _ctx: &PluginContext,
    ) -> Result<(), AppError> {
        Ok(())
    }

    fn on_entry_after_publish(
        &self,
        _entry: &EntryData,
        _ctx: &PluginContext,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

pub fn builtin_plugins() -> Vec<Box<dyn CmsPlugin>> {
    vec![Box::new(seo::SeoAutoPlugin)]
}

pub async fn run_entry_before_save(
    state: &crate::state::AppState,
    type_slug: &str,
    data: Value,
    user_id: Uuid,
) -> Result<Value, AppError> {
    let enabled = enabled_plugin_keys(state).await?;
    let mut entry = EntryData {
        type_slug: type_slug.to_owned(),
        data,
    };
    let ctx = PluginContext { user_id };

    for plugin in builtin_plugins() {
        if enabled.iter().any(|key| key == plugin.key()) {
            plugin.on_entry_before_save(&mut entry, &ctx)?;
        }
    }

    Ok(entry.data)
}

pub async fn run_entry_after_publish(
    state: &crate::state::AppState,
    type_slug: &str,
    data: Value,
    user_id: Uuid,
) -> Result<(), AppError> {
    let enabled = enabled_plugin_keys(state).await?;
    let entry = EntryData {
        type_slug: type_slug.to_owned(),
        data,
    };
    let ctx = PluginContext { user_id };

    for plugin in builtin_plugins() {
        if enabled.iter().any(|key| key == plugin.key()) {
            plugin.on_entry_after_publish(&entry, &ctx)?;
        }
    }

    Ok(())
}

async fn enabled_plugin_keys(state: &crate::state::AppState) -> Result<Vec<String>, AppError> {
    sqlx::query_scalar::<_, String>(
        r#"
        SELECT plugin_key
        FROM cms_plugins
        WHERE is_enabled = TRUE
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(AppError::from)
}
