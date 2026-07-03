ALTER TABLE marketplace_creators
  ADD COLUMN IF NOT EXISTS requested_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  ADD COLUMN IF NOT EXISTS verification_notes TEXT,
  ADD COLUMN IF NOT EXISTS verified_by UUID REFERENCES users(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS verified_at TIMESTAMPTZ;

UPDATE marketplace_creators
SET status = CASE status
  WHEN 'draft' THEN 'pending'
  WHEN 'pending_verification' THEN 'pending'
  WHEN 'verified' THEN 'approved'
  ELSE status
END;

ALTER TABLE marketplace_creators
  ALTER COLUMN status SET DEFAULT 'pending';

ALTER TABLE marketplace_creators
  DROP CONSTRAINT IF EXISTS marketplace_creators_status_supported;

ALTER TABLE marketplace_creators
  ADD CONSTRAINT marketplace_creators_status_supported CHECK (
    status IN ('pending', 'approved', 'suspended', 'rejected')
  );

CREATE INDEX IF NOT EXISTS idx_marketplace_creators_verification_queue
  ON marketplace_creators(status, requested_at ASC);

ALTER TABLE marketplace_listings
  ADD COLUMN IF NOT EXISTS description TEXT NOT NULL DEFAULT '',
  ADD COLUMN IF NOT EXISTS price_cents INTEGER NOT NULL DEFAULT 0,
  ADD COLUMN IF NOT EXISTS license TEXT NOT NULL DEFAULT '',
  ADD COLUMN IF NOT EXISTS screenshots JSONB NOT NULL DEFAULT '[]'::jsonb,
  ADD COLUMN IF NOT EXISTS submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS submitted_at TIMESTAMPTZ;

UPDATE marketplace_listings
SET description = summary
WHERE description = '';

ALTER TABLE marketplace_listings
  DROP CONSTRAINT IF EXISTS marketplace_listings_price_nonnegative;

ALTER TABLE marketplace_listings
  ADD CONSTRAINT marketplace_listings_price_nonnegative CHECK (price_cents >= 0);

ALTER TABLE marketplace_listings
  DROP CONSTRAINT IF EXISTS marketplace_listings_screenshots_array;

ALTER TABLE marketplace_listings
  ADD CONSTRAINT marketplace_listings_screenshots_array CHECK (jsonb_typeof(screenshots) = 'array');

CREATE INDEX IF NOT EXISTS idx_marketplace_listings_submission_queue
  ON marketplace_listings(status, submitted_at ASC)
  WHERE status = 'submitted';

CREATE OR REPLACE FUNCTION marketplace_prevent_version_artifact_mutation()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
  IF OLD.status IN ('submitted', 'validating', 'approved', 'deprecated', 'blocked') AND (
    NEW.version IS DISTINCT FROM OLD.version
    OR NEW.manifest_json IS DISTINCT FROM OLD.manifest_json
    OR NEW.artifact_object_key IS DISTINCT FROM OLD.artifact_object_key
    OR NEW.artifact_sha256 IS DISTINCT FROM OLD.artifact_sha256
    OR NEW.artifact_size_bytes IS DISTINCT FROM OLD.artifact_size_bytes
    OR NEW.artifact_file_name IS DISTINCT FROM OLD.artifact_file_name
    OR NEW.artifact_content_type IS DISTINCT FROM OLD.artifact_content_type
    OR NEW.storage_metadata IS DISTINCT FROM OLD.storage_metadata
  ) THEN
    RAISE EXCEPTION 'submitted marketplace package versions are immutable';
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_marketplace_versions_immutable ON marketplace_versions;
CREATE TRIGGER trg_marketplace_versions_immutable
BEFORE UPDATE ON marketplace_versions
FOR EACH ROW
EXECUTE FUNCTION marketplace_prevent_version_artifact_mutation();