ALTER TABLE media
  ADD COLUMN IF NOT EXISTS storage_key TEXT,
  ADD COLUMN IF NOT EXISTS source_sha256 TEXT,
  ADD COLUMN IF NOT EXISTS stored_sha256 TEXT,
  ADD COLUMN IF NOT EXISTS source_size BIGINT,
  ADD COLUMN IF NOT EXISTS visibility TEXT NOT NULL DEFAULT 'restricted',
  ADD COLUMN IF NOT EXISTS verification_status TEXT NOT NULL DEFAULT 'legacy_unverified',
  ADD COLUMN IF NOT EXISTS malware_scan_status TEXT NOT NULL DEFAULT 'unavailable',
  ADD COLUMN IF NOT EXISTS lifecycle_status TEXT NOT NULL DEFAULT 'active',
  ADD COLUMN IF NOT EXISTS security_metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS retention_until TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

UPDATE media
SET storage_key = CASE
      WHEN url ~ '^/uploads/[a-f0-9-]+/(variants/)?[A-Za-z0-9._-]+$'
        THEN substring(url FROM 10)
      ELSE 'legacy/unresolved/' || id::text || '/original.bin'
    END,
    source_size = COALESCE(source_size, size),
    visibility = 'restricted',
    verification_status = 'legacy_unverified',
    malware_scan_status = 'unavailable',
    security_metadata = security_metadata || jsonb_build_object(
      'legacy_classification', 'restricted_unverified',
      'phase', 'security_phase_7'
    )
WHERE storage_key IS NULL;

ALTER TABLE media
  ALTER COLUMN storage_key SET NOT NULL,
  ADD CONSTRAINT media_storage_key_safe CHECK (
    storage_key ~ '^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$'
    AND storage_key !~ '(^|/)\.\.?(/|$)'
  ),
  ADD CONSTRAINT media_source_sha256_format CHECK (
    source_sha256 IS NULL OR source_sha256 ~ '^[a-f0-9]{64}$'
  ),
  ADD CONSTRAINT media_stored_sha256_format CHECK (
    stored_sha256 IS NULL OR stored_sha256 ~ '^[a-f0-9]{64}$'
  ),
  ADD CONSTRAINT media_source_size_nonnegative CHECK (
    source_size IS NULL OR source_size >= 0
  ),
  ADD CONSTRAINT media_visibility_supported CHECK (
    visibility IN ('public', 'restricted')
  ),
  ADD CONSTRAINT media_verification_status_supported CHECK (
    verification_status IN ('legacy_unverified', 'verified', 'rejected')
  ),
  ADD CONSTRAINT media_malware_scan_status_supported CHECK (
    malware_scan_status IN ('pending', 'clean', 'infected', 'error', 'unavailable')
  ),
  ADD CONSTRAINT media_lifecycle_status_supported CHECK (
    lifecycle_status IN ('publishing', 'active', 'deletion_pending', 'failed')
  ),
  ADD CONSTRAINT media_security_metadata_object CHECK (
    jsonb_typeof(security_metadata) = 'object'
  ),
  ADD CONSTRAINT media_verified_checksum_required CHECK (
    verification_status <> 'verified'
    OR (source_sha256 IS NOT NULL AND stored_sha256 IS NOT NULL)
  ),
  ADD CONSTRAINT media_public_verified_only CHECK (
    visibility <> 'public'
    OR (
      verification_status = 'verified'
      AND lifecycle_status IN ('publishing', 'active')
      AND mime_type IN ('image/jpeg', 'image/png', 'image/webp')
    )
  );

CREATE UNIQUE INDEX IF NOT EXISTS idx_media_storage_key
  ON media(storage_key);
CREATE INDEX IF NOT EXISTS idx_media_public_delivery
  ON media(organization_id, id, lifecycle_status)
  WHERE visibility = 'public' AND verification_status = 'verified';
CREATE INDEX IF NOT EXISTS idx_media_cleanup_state
  ON media(lifecycle_status, updated_at)
  WHERE lifecycle_status IN ('publishing', 'deletion_pending', 'failed');

ALTER TABLE media_variants
  ADD COLUMN IF NOT EXISTS storage_key TEXT,
  ADD COLUMN IF NOT EXISTS stored_sha256 TEXT,
  ADD COLUMN IF NOT EXISTS verification_status TEXT NOT NULL DEFAULT 'legacy_unverified',
  ADD COLUMN IF NOT EXISTS lifecycle_status TEXT NOT NULL DEFAULT 'active';

UPDATE media_variants
SET storage_key = CASE
      WHEN url ~ '^/uploads/[a-f0-9-]+/variants/[A-Za-z0-9._-]+$'
        THEN substring(url FROM 10)
      ELSE 'legacy/unresolved/' || id::text || '/variant.bin'
    END,
    verification_status = 'legacy_unverified'
WHERE storage_key IS NULL;

ALTER TABLE media_variants
  ALTER COLUMN storage_key SET NOT NULL,
  ADD CONSTRAINT media_variants_storage_key_safe CHECK (
    storage_key ~ '^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$'
    AND storage_key !~ '(^|/)\.\.?(/|$)'
  ),
  ADD CONSTRAINT media_variants_sha256_format CHECK (
    stored_sha256 IS NULL OR stored_sha256 ~ '^[a-f0-9]{64}$'
  ),
  ADD CONSTRAINT media_variants_verification_supported CHECK (
    verification_status IN ('legacy_unverified', 'verified', 'rejected')
  ),
  ADD CONSTRAINT media_variants_lifecycle_supported CHECK (
    lifecycle_status IN ('publishing', 'active', 'deletion_pending', 'failed')
  ),
  ADD CONSTRAINT media_variants_verified_checksum_required CHECK (
    verification_status <> 'verified' OR stored_sha256 IS NOT NULL
  );

CREATE UNIQUE INDEX IF NOT EXISTS idx_media_variants_storage_key
  ON media_variants(storage_key);

CREATE TABLE IF NOT EXISTS file_cleanup_jobs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  media_id UUID,
  storage_key TEXT NOT NULL,
  reason TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  attempts INTEGER NOT NULL DEFAULT 0,
  available_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  completed_at TIMESTAMPTZ,
  last_error_code TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT file_cleanup_jobs_storage_key_safe CHECK (
    storage_key ~ '^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$'
    AND storage_key !~ '(^|/)\.\.?(/|$)'
  ),
  CONSTRAINT file_cleanup_jobs_reason_supported CHECK (
    reason IN ('delete', 'publish_rollback', 'orphan_reconciliation', 'retention')
  ),
  CONSTRAINT file_cleanup_jobs_status_supported CHECK (
    status IN ('pending', 'retry', 'complete', 'failed')
  ),
  CONSTRAINT file_cleanup_jobs_attempts_nonnegative CHECK (attempts >= 0),
  CONSTRAINT file_cleanup_jobs_error_code_safe CHECK (
    last_error_code IS NULL OR last_error_code ~ '^[a-z0-9_]{1,64}$'
  )
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_file_cleanup_jobs_active_key
  ON file_cleanup_jobs(organization_id, storage_key)
  WHERE status IN ('pending', 'retry');
CREATE INDEX IF NOT EXISTS idx_file_cleanup_jobs_ready
  ON file_cleanup_jobs(status, available_at, created_at)
  WHERE status IN ('pending', 'retry');

ALTER TABLE file_cleanup_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE file_cleanup_jobs FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS file_cleanup_jobs_tenant_select ON file_cleanup_jobs;
DROP POLICY IF EXISTS file_cleanup_jobs_tenant_insert ON file_cleanup_jobs;
DROP POLICY IF EXISTS file_cleanup_jobs_tenant_update ON file_cleanup_jobs;
DROP POLICY IF EXISTS file_cleanup_jobs_tenant_delete ON file_cleanup_jobs;

CREATE POLICY file_cleanup_jobs_tenant_select
ON file_cleanup_jobs
FOR SELECT
USING (app_rls_tenant_matches(organization_id));

CREATE POLICY file_cleanup_jobs_tenant_insert
ON file_cleanup_jobs
FOR INSERT
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY file_cleanup_jobs_tenant_update
ON file_cleanup_jobs
FOR UPDATE
USING (app_rls_tenant_matches(organization_id))
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY file_cleanup_jobs_tenant_delete
ON file_cleanup_jobs
FOR DELETE
USING (app_rls_tenant_matches(organization_id));

ALTER TABLE marketplace_versions
  ADD COLUMN IF NOT EXISTS artifact_state TEXT NOT NULL DEFAULT 'legacy_unverified',
  ADD COLUMN IF NOT EXISTS malware_scan_status TEXT NOT NULL DEFAULT 'unavailable',
  ADD COLUMN IF NOT EXISTS archive_inspected_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS artifact_verified_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS artifact_retention_until TIMESTAMPTZ,
  ADD CONSTRAINT marketplace_versions_artifact_state_supported CHECK (
    artifact_state IN (
      'legacy_unverified',
      'quarantined',
      'reviewed',
      'rejected',
      'deletion_pending'
    )
  ),
  ADD CONSTRAINT marketplace_versions_malware_scan_supported CHECK (
    malware_scan_status IN ('pending', 'clean', 'infected', 'error', 'unavailable')
  ),
  ADD CONSTRAINT marketplace_versions_reviewed_verification_required CHECK (
    artifact_state <> 'reviewed' OR artifact_verified_at IS NOT NULL
  );

UPDATE marketplace_versions
SET artifact_state = 'legacy_unverified',
    malware_scan_status = 'unavailable',
    storage_metadata = storage_metadata || jsonb_build_object(
      'legacy_classification', 'restricted_unverified',
      'phase', 'security_phase_7'
    )
WHERE archive_inspected_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_marketplace_versions_artifact_state
  ON marketplace_versions(artifact_state, created_at DESC);
