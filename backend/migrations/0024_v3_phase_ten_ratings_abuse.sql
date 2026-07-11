CREATE TABLE IF NOT EXISTS marketplace_product_reviews (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
  version_id UUID REFERENCES marketplace_versions(id) ON DELETE SET NULL,
  author_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
  rating SMALLINT NOT NULL,
  body TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  moderation_reason TEXT,
  moderated_by UUID REFERENCES users(id) ON DELETE SET NULL,
  moderated_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_product_reviews_rating_range CHECK (rating BETWEEN 1 AND 5),
  CONSTRAINT marketplace_product_reviews_body_nonempty CHECK (length(trim(body)) BETWEEN 3 AND 4000),
  CONSTRAINT marketplace_product_reviews_status_supported CHECK (status IN ('pending', 'published', 'rejected')),
  CONSTRAINT marketplace_product_reviews_listing_version_fk FOREIGN KEY (listing_id, version_id) REFERENCES marketplace_versions(listing_id, id)
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_product_reviews_org_listing
  ON marketplace_product_reviews(organization_id, listing_id);
CREATE INDEX IF NOT EXISTS idx_marketplace_product_reviews_listing_status
  ON marketplace_product_reviews(listing_id, status, created_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_abuse_reports (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
  version_id UUID REFERENCES marketplace_versions(id) ON DELETE SET NULL,
  reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
  report_type TEXT NOT NULL,
  severity TEXT NOT NULL DEFAULT 'medium',
  description TEXT NOT NULL,
  evidence JSONB NOT NULL DEFAULT '{}'::jsonb,
  status TEXT NOT NULL DEFAULT 'open',
  resolution_note TEXT,
  notification_status TEXT NOT NULL DEFAULT 'not_required',
  critical_notified_at TIMESTAMPTZ,
  resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
  resolved_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_abuse_reports_type_supported CHECK (report_type IN ('malware', 'copyright', 'spam', 'fraud', 'privacy', 'other')),
  CONSTRAINT marketplace_abuse_reports_severity_supported CHECK (severity IN ('low', 'medium', 'high', 'critical')),
  CONSTRAINT marketplace_abuse_reports_description_nonempty CHECK (length(trim(description)) BETWEEN 10 AND 4000),
  CONSTRAINT marketplace_abuse_reports_status_supported CHECK (status IN ('open', 'investigating', 'resolved', 'dismissed')),
  CONSTRAINT marketplace_abuse_reports_notification_supported CHECK (notification_status IN ('not_required', 'created', 'acknowledged')),
  CONSTRAINT marketplace_abuse_reports_evidence_object CHECK (jsonb_typeof(evidence) = 'object'),
  CONSTRAINT marketplace_abuse_reports_listing_version_fk FOREIGN KEY (listing_id, version_id) REFERENCES marketplace_versions(listing_id, id)
);
CREATE INDEX IF NOT EXISTS idx_marketplace_abuse_reports_queue
  ON marketplace_abuse_reports(status, severity, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_marketplace_abuse_reports_listing
  ON marketplace_abuse_reports(listing_id, created_at DESC);

ALTER TABLE marketplace_product_reviews ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_product_reviews FORCE ROW LEVEL SECURITY;
ALTER TABLE marketplace_abuse_reports ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_abuse_reports FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS marketplace_product_reviews_tenant_select ON marketplace_product_reviews;
DROP POLICY IF EXISTS marketplace_product_reviews_tenant_insert ON marketplace_product_reviews;
DROP POLICY IF EXISTS marketplace_product_reviews_tenant_update ON marketplace_product_reviews;
CREATE POLICY marketplace_product_reviews_tenant_select ON marketplace_product_reviews FOR SELECT USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_product_reviews_tenant_insert ON marketplace_product_reviews FOR INSERT WITH CHECK (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_product_reviews_tenant_update ON marketplace_product_reviews FOR UPDATE USING (app_rls_tenant_matches(organization_id)) WITH CHECK (app_rls_tenant_matches(organization_id));

DROP POLICY IF EXISTS marketplace_abuse_reports_tenant_select ON marketplace_abuse_reports;
DROP POLICY IF EXISTS marketplace_abuse_reports_tenant_insert ON marketplace_abuse_reports;
DROP POLICY IF EXISTS marketplace_abuse_reports_tenant_update ON marketplace_abuse_reports;
CREATE POLICY marketplace_abuse_reports_tenant_select ON marketplace_abuse_reports FOR SELECT USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_abuse_reports_tenant_insert ON marketplace_abuse_reports FOR INSERT WITH CHECK (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_abuse_reports_tenant_update ON marketplace_abuse_reports FOR UPDATE USING (app_rls_tenant_matches(organization_id)) WITH CHECK (app_rls_tenant_matches(organization_id));
