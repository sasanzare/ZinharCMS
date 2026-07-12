CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_title_trgm
  ON marketplace_listings USING gin (title gin_trgm_ops)
  WHERE status = 'approved';

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_summary_trgm
  ON marketplace_listings USING gin (summary gin_trgm_ops)
  WHERE status = 'approved';

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_category_trgm
  ON marketplace_listings USING gin (category gin_trgm_ops)
  WHERE status = 'approved';

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_slug_trgm
  ON marketplace_listings USING gin (slug gin_trgm_ops)
  WHERE status = 'approved';

CREATE INDEX IF NOT EXISTS idx_marketplace_creators_display_name_trgm
  ON marketplace_creators USING gin (display_name gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_catalog_updated
  ON marketplace_listings(status, product_type, category, pricing_type, updated_at DESC)
  WHERE status = 'approved';

CREATE INDEX IF NOT EXISTS idx_marketplace_versions_catalog_latest
  ON marketplace_versions(listing_id, created_at DESC)
  WHERE status = 'approved'
    AND validation_status IN ('passed', 'warning')
    AND security_risk_level IN ('low', 'medium');

CREATE INDEX IF NOT EXISTS idx_marketplace_installations_listing_active
  ON marketplace_installations(listing_id)
  WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_marketplace_installations_org_active_updated
  ON marketplace_installations(organization_id, updated_at DESC)
  WHERE status <> 'uninstalled';

CREATE INDEX IF NOT EXISTS idx_marketplace_entitlements_purchase_gate
  ON marketplace_entitlements(organization_id, purchase_id, listing_id, version_id)
  WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_marketplace_purchases_existing_checkout
  ON marketplace_purchases(organization_id, listing_id, version_id, status, created_at DESC)
  WHERE status IN ('pending', 'completed');
