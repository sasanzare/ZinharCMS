use axum::http::{HeaderMap, HeaderValue, header::CACHE_CONTROL};

pub const MARKETPLACE_CATALOG_CACHE_CONTROL: &str =
    "private, max-age=60, stale-while-revalidate=30";
pub const MARKETPLACE_MUTATION_CACHE_CONTROL: &str = "no-store";
pub const MARKETPLACE_CATALOG_P95_TARGET_MS: u64 = 300;
pub const MARKETPLACE_LISTING_P95_TARGET_MS: u64 = 250;
pub const MARKETPLACE_INSTALL_P95_TARGET_MS: u64 = 750;

pub fn marketplace_catalog_cache_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_static(MARKETPLACE_CATALOG_CACHE_CONTROL),
    );
    headers
}

#[cfg(test)]
mod tests {
    use axum::http::header::CACHE_CONTROL;

    use super::{
        MARKETPLACE_CATALOG_CACHE_CONTROL, MARKETPLACE_CATALOG_P95_TARGET_MS,
        MARKETPLACE_INSTALL_P95_TARGET_MS, MARKETPLACE_LISTING_P95_TARGET_MS,
        MARKETPLACE_MUTATION_CACHE_CONTROL, marketplace_catalog_cache_headers,
    };

    const PHASE_THIRTEEN_MIGRATION: &str =
        include_str!("../../migrations/0026_v3_phase_thirteen_marketplace_qa_performance.sql");
    const PHASE_THIRTEEN_DOC: &str = include_str!("../../../docs/V3_PHASE_THIRTEEN.md");
    const LOAD_SMOKE_SCRIPT: &str =
        include_str!("../../../scripts/marketplace-phase13-load-smoke.ps1");
    const MARKETPLACE_ROUTES: &str = include_str!("../routes/marketplace.rs");

    #[test]
    fn catalog_cache_policy_is_private_and_bounded() {
        let headers = marketplace_catalog_cache_headers();
        assert_eq!(
            headers
                .get(CACHE_CONTROL)
                .and_then(|value| value.to_str().ok()),
            Some(MARKETPLACE_CATALOG_CACHE_CONTROL)
        );
        assert!(MARKETPLACE_CATALOG_CACHE_CONTROL.contains("private"));
        assert!(MARKETPLACE_CATALOG_CACHE_CONTROL.contains("max-age=60"));
        assert_eq!(MARKETPLACE_MUTATION_CACHE_CONTROL, "no-store");
    }

    #[test]
    fn phase_thirteen_performance_contract_is_indexed_documented_and_scripted() {
        for required in [
            "CREATE EXTENSION IF NOT EXISTS pg_trgm",
            "idx_marketplace_listings_title_trgm",
            "idx_marketplace_listings_summary_trgm",
            "idx_marketplace_creators_display_name_trgm",
            "idx_marketplace_versions_catalog_latest",
            "idx_marketplace_installations_listing_active",
            "idx_marketplace_entitlements_purchase_gate",
            "idx_marketplace_purchases_existing_checkout",
        ] {
            assert!(
                PHASE_THIRTEEN_MIGRATION.contains(required),
                "missing Phase 13 performance migration contract: {required}"
            );
        }

        assert!(MARKETPLACE_ROUTES.contains("marketplace_catalog_cache_headers"));
        assert!(PHASE_THIRTEEN_DOC.contains("baseline latency"));
        assert!(PHASE_THIRTEEN_DOC.contains("index tuning"));
        assert!(PHASE_THIRTEEN_DOC.contains("cache policy"));
        assert!(LOAD_SMOKE_SCRIPT.contains("/api/marketplace/catalog"));
        assert!(LOAD_SMOKE_SCRIPT.contains("/api/marketplace/installations"));
        assert!(LOAD_SMOKE_SCRIPT.contains("Add-Type -AssemblyName System.Net.Http"));
        assert!(LOAD_SMOKE_SCRIPT.contains("$handler.UseProxy = $false"));
        assert!(LOAD_SMOKE_SCRIPT.contains("$_.Status -lt 200 -or $_.Status -ge 400"));
        assert!(LOAD_SMOKE_SCRIPT.contains("P95Ms"));
        assert!(LOAD_SMOKE_SCRIPT.contains("AllowInstallMutation"));
        assert!(MARKETPLACE_CATALOG_P95_TARGET_MS <= 300);
        assert!(MARKETPLACE_LISTING_P95_TARGET_MS <= 250);
        assert!(MARKETPLACE_INSTALL_P95_TARGET_MS <= 750);
    }
}
