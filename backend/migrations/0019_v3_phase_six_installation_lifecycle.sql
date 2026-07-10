ALTER TABLE marketplace_installations
  ADD COLUMN IF NOT EXISTS cleanup_policy TEXT NOT NULL DEFAULT 'preserve_organization_data',
  ADD COLUMN IF NOT EXISTS version_pinned BOOLEAN NOT NULL DEFAULT TRUE,
  ADD COLUMN IF NOT EXISTS enabled_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  ADD COLUMN IF NOT EXISTS disabled_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS uninstalled_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS version_changed_at TIMESTAMPTZ NOT NULL DEFAULT now();

ALTER TABLE marketplace_installations
  DROP CONSTRAINT IF EXISTS marketplace_installations_cleanup_policy_supported;

ALTER TABLE marketplace_installations
  ADD CONSTRAINT marketplace_installations_cleanup_policy_supported CHECK (
    cleanup_policy = 'preserve_organization_data'
  );

ALTER TABLE marketplace_installations
  DROP CONSTRAINT IF EXISTS marketplace_installations_rollback_version_id_fkey;

ALTER TABLE marketplace_installations
  DROP CONSTRAINT IF EXISTS marketplace_installations_listing_rollback_version_fk;

ALTER TABLE marketplace_installations
  ADD CONSTRAINT marketplace_installations_listing_rollback_version_fk
    FOREIGN KEY (listing_id, rollback_version_id)
    REFERENCES marketplace_versions(listing_id, id)
    ON DELETE RESTRICT;

CREATE INDEX IF NOT EXISTS idx_marketplace_installations_org_updated
  ON marketplace_installations(organization_id, updated_at DESC);
