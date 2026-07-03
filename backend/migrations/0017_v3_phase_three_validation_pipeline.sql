ALTER TABLE marketplace_versions
  ADD COLUMN IF NOT EXISTS validation_status TEXT NOT NULL DEFAULT 'pending',
  ADD COLUMN IF NOT EXISTS validation_report JSONB NOT NULL DEFAULT '{}'::jsonb,
  ADD COLUMN IF NOT EXISTS security_risk_level TEXT NOT NULL DEFAULT 'unreviewed',
  ADD COLUMN IF NOT EXISTS compatibility_report JSONB NOT NULL DEFAULT '{}'::jsonb;

UPDATE marketplace_versions
SET validation_status = CASE
    WHEN status IN ('submitted', 'validating', 'approved') THEN 'passed'
    WHEN status = 'blocked' THEN 'failed'
    ELSE validation_status
  END,
  security_risk_level = CASE
    WHEN status = 'blocked' THEN 'high'
    WHEN status IN ('submitted', 'validating', 'approved') THEN 'low'
    ELSE security_risk_level
  END
WHERE validation_status = 'pending'
  AND status IN ('submitted', 'validating', 'approved', 'blocked');

ALTER TABLE marketplace_versions
  DROP CONSTRAINT IF EXISTS marketplace_versions_validation_status_supported;

ALTER TABLE marketplace_versions
  ADD CONSTRAINT marketplace_versions_validation_status_supported CHECK (
    validation_status IN ('pending', 'passed', 'warning', 'failed')
  );

ALTER TABLE marketplace_versions
  DROP CONSTRAINT IF EXISTS marketplace_versions_security_risk_supported;

ALTER TABLE marketplace_versions
  ADD CONSTRAINT marketplace_versions_security_risk_supported CHECK (
    security_risk_level IN ('unreviewed', 'low', 'medium', 'high', 'critical')
  );

ALTER TABLE marketplace_versions
  DROP CONSTRAINT IF EXISTS marketplace_versions_validation_report_object;

ALTER TABLE marketplace_versions
  ADD CONSTRAINT marketplace_versions_validation_report_object CHECK (jsonb_typeof(validation_report) = 'object');

ALTER TABLE marketplace_versions
  DROP CONSTRAINT IF EXISTS marketplace_versions_compatibility_report_object;

ALTER TABLE marketplace_versions
  ADD CONSTRAINT marketplace_versions_compatibility_report_object CHECK (jsonb_typeof(compatibility_report) = 'object');

CREATE INDEX IF NOT EXISTS idx_marketplace_versions_validation_queue
  ON marketplace_versions(validation_status, security_risk_level, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_versions_compatibility_gin
  ON marketplace_versions USING GIN (compatibility_report);