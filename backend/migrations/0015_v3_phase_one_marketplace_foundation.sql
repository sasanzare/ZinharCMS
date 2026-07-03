CREATE TABLE IF NOT EXISTS marketplace_creators (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  slug TEXT NOT NULL,
  display_name TEXT NOT NULL,
  bio TEXT,
  status TEXT NOT NULL DEFAULT 'draft',
  payout_status TEXT NOT NULL DEFAULT 'not_configured',
  support_email TEXT,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_creators_user_unique UNIQUE (user_id),
  CONSTRAINT marketplace_creators_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
  CONSTRAINT marketplace_creators_display_name_not_empty CHECK (length(trim(display_name)) > 0),
  CONSTRAINT marketplace_creators_status_supported CHECK (
    status IN ('draft', 'pending_verification', 'verified', 'suspended', 'rejected')
  ),
  CONSTRAINT marketplace_creators_payout_status_supported CHECK (
    payout_status IN ('not_configured', 'pending', 'verified', 'restricted')
  ),
  CONSTRAINT marketplace_creators_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_creators_slug_lower
  ON marketplace_creators(lower(slug));

CREATE INDEX IF NOT EXISTS idx_marketplace_creators_status
  ON marketplace_creators(status, created_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_listings (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  creator_id UUID NOT NULL REFERENCES marketplace_creators(id) ON DELETE CASCADE,
  product_type TEXT NOT NULL,
  title TEXT NOT NULL,
  slug TEXT NOT NULL,
  summary TEXT NOT NULL,
  category TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'draft',
  pricing_type TEXT NOT NULL DEFAULT 'free',
  support_url TEXT,
  icon_asset_key TEXT,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_listings_product_type_supported CHECK (
    product_type IN ('component_pack', 'design_template', 'integration_plugin', 'backend_extension')
  ),
  CONSTRAINT marketplace_listings_title_not_empty CHECK (length(trim(title)) > 0),
  CONSTRAINT marketplace_listings_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
  CONSTRAINT marketplace_listings_summary_not_empty CHECK (length(trim(summary)) > 0),
  CONSTRAINT marketplace_listings_category_not_empty CHECK (length(trim(category)) > 0),
  CONSTRAINT marketplace_listings_status_supported CHECK (
    status IN ('draft', 'submitted', 'approved', 'changes_requested', 'suspended', 'blocked', 'archived')
  ),
  CONSTRAINT marketplace_listings_pricing_type_supported CHECK (
    pricing_type IN ('free', 'paid', 'custom')
  ),
  CONSTRAINT marketplace_listings_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_listings_slug_lower
  ON marketplace_listings(lower(slug));

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_creator
  ON marketplace_listings(creator_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_catalog
  ON marketplace_listings(status, product_type, category, pricing_type, created_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_versions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
  version TEXT NOT NULL,
  manifest_schema_version TEXT NOT NULL DEFAULT '2026-07',
  manifest_json JSONB NOT NULL,
  artifact_object_key TEXT NOT NULL,
  artifact_sha256 TEXT NOT NULL,
  artifact_size_bytes BIGINT NOT NULL,
  artifact_file_name TEXT NOT NULL,
  artifact_content_type TEXT NOT NULL DEFAULT 'application/zip',
  storage_metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  status TEXT NOT NULL DEFAULT 'draft',
  created_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_versions_listing_version_unique UNIQUE (listing_id, version),
  CONSTRAINT marketplace_versions_listing_id_unique UNIQUE (listing_id, id),
  CONSTRAINT marketplace_versions_semver CHECK (
    version ~ '^(0|[1-9][0-9]*)[.](0|[1-9][0-9]*)[.](0|[1-9][0-9]*)(-((0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)([.](0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*))*))?([+]([0-9A-Za-z-]+([.][0-9A-Za-z-]+)*))?$'
  ),
  CONSTRAINT marketplace_versions_manifest_object CHECK (jsonb_typeof(manifest_json) = 'object'),
  CONSTRAINT marketplace_versions_manifest_schema_version_supported CHECK (manifest_schema_version = '2026-07'),
  CONSTRAINT marketplace_versions_manifest_required_fields CHECK (
    manifest_json ? 'manifest_version'
    AND manifest_json ? 'name'
    AND manifest_json ? 'version'
    AND manifest_json ? 'type'
    AND manifest_json ? 'permissions'
    AND manifest_json ? 'compatibility'
    AND manifest_json ? 'entry_points'
    AND manifest_json ? 'assets'
    AND manifest_json->>'manifest_version' = manifest_schema_version
  ),  CONSTRAINT marketplace_versions_object_key_not_empty CHECK (length(trim(artifact_object_key)) > 0),
  CONSTRAINT marketplace_versions_sha256_format CHECK (artifact_sha256 ~ '^[a-f0-9]{64}$'),
  CONSTRAINT marketplace_versions_size_limit CHECK (artifact_size_bytes > 0 AND artifact_size_bytes <= 52428800),
  CONSTRAINT marketplace_versions_file_name_not_empty CHECK (length(trim(artifact_file_name)) > 0),
  CONSTRAINT marketplace_versions_storage_metadata_object CHECK (jsonb_typeof(storage_metadata) = 'object'),
  CONSTRAINT marketplace_versions_status_supported CHECK (
    status IN ('draft', 'submitted', 'validating', 'approved', 'rejected', 'blocked', 'deprecated')
  )
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_versions_object_key
  ON marketplace_versions(artifact_object_key);

CREATE INDEX IF NOT EXISTS idx_marketplace_versions_listing_status
  ON marketplace_versions(listing_id, status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_versions_checksum
  ON marketplace_versions(artifact_sha256);

CREATE OR REPLACE FUNCTION marketplace_prevent_version_artifact_mutation()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
  IF OLD.status IN ('approved', 'deprecated', 'blocked') AND (
    NEW.version IS DISTINCT FROM OLD.version
    OR NEW.manifest_json IS DISTINCT FROM OLD.manifest_json
    OR NEW.artifact_object_key IS DISTINCT FROM OLD.artifact_object_key
    OR NEW.artifact_sha256 IS DISTINCT FROM OLD.artifact_sha256
    OR NEW.artifact_size_bytes IS DISTINCT FROM OLD.artifact_size_bytes
    OR NEW.artifact_file_name IS DISTINCT FROM OLD.artifact_file_name
    OR NEW.artifact_content_type IS DISTINCT FROM OLD.artifact_content_type
  ) THEN
    RAISE EXCEPTION 'approved marketplace package versions are immutable';
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_marketplace_versions_immutable ON marketplace_versions;
CREATE TRIGGER trg_marketplace_versions_immutable
BEFORE UPDATE ON marketplace_versions
FOR EACH ROW
EXECUTE FUNCTION marketplace_prevent_version_artifact_mutation();

CREATE TABLE IF NOT EXISTS marketplace_submissions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  version_id UUID NOT NULL REFERENCES marketplace_versions(id) ON DELETE CASCADE,
  submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
  review_status TEXT NOT NULL DEFAULT 'queued',
  risk_level TEXT NOT NULL DEFAULT 'unreviewed',
  review_notes TEXT,
  validation_report JSONB NOT NULL DEFAULT '{}'::jsonb,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  reviewed_by UUID REFERENCES users(id) ON DELETE SET NULL,
  reviewed_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_submissions_status_supported CHECK (
    review_status IN ('queued', 'validating', 'changes_requested', 'approved', 'rejected', 'blocked', 'canceled')
  ),
  CONSTRAINT marketplace_submissions_risk_supported CHECK (
    risk_level IN ('unreviewed', 'low', 'medium', 'high', 'critical')
  ),
  CONSTRAINT marketplace_submissions_validation_report_object CHECK (jsonb_typeof(validation_report) = 'object'),
  CONSTRAINT marketplace_submissions_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_marketplace_submissions_version
  ON marketplace_submissions(version_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_submissions_queue
  ON marketplace_submissions(review_status, risk_level, created_at ASC);

CREATE TABLE IF NOT EXISTS marketplace_installations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
  version_id UUID NOT NULL REFERENCES marketplace_versions(id) ON DELETE RESTRICT,
  installed_by UUID REFERENCES users(id) ON DELETE SET NULL,
  status TEXT NOT NULL DEFAULT 'active',
  permissions_json JSONB NOT NULL DEFAULT '[]'::jsonb,
  permission_approved_by UUID REFERENCES users(id) ON DELETE SET NULL,
  permission_approved_at TIMESTAMPTZ,
  rollback_version_id UUID REFERENCES marketplace_versions(id) ON DELETE SET NULL,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  installed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_installations_listing_version_fk
    FOREIGN KEY (listing_id, version_id)
    REFERENCES marketplace_versions(listing_id, id),
  CONSTRAINT marketplace_installations_status_supported CHECK (
    status IN ('active', 'disabled', 'uninstalled', 'rollback_pending', 'blocked')
  ),
  CONSTRAINT marketplace_installations_permissions_shape CHECK (jsonb_typeof(permissions_json) = 'array'),
  CONSTRAINT marketplace_installations_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_installations_active_listing
  ON marketplace_installations(organization_id, listing_id)
  WHERE status <> 'uninstalled';

CREATE INDEX IF NOT EXISTS idx_marketplace_installations_org_status
  ON marketplace_installations(organization_id, status, installed_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_installations_version
  ON marketplace_installations(version_id);

ALTER TABLE marketplace_installations ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_installations FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS marketplace_installations_tenant_select ON marketplace_installations;
DROP POLICY IF EXISTS marketplace_installations_tenant_insert ON marketplace_installations;
DROP POLICY IF EXISTS marketplace_installations_tenant_update ON marketplace_installations;
DROP POLICY IF EXISTS marketplace_installations_tenant_delete ON marketplace_installations;

CREATE POLICY marketplace_installations_tenant_select
ON marketplace_installations
FOR SELECT
USING (app_rls_tenant_matches(organization_id));

CREATE POLICY marketplace_installations_tenant_insert
ON marketplace_installations
FOR INSERT
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY marketplace_installations_tenant_update
ON marketplace_installations
FOR UPDATE
USING (app_rls_tenant_matches(organization_id))
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY marketplace_installations_tenant_delete
ON marketplace_installations
FOR DELETE
USING (app_rls_tenant_matches(organization_id));