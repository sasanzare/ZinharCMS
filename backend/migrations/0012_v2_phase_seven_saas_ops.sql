CREATE TABLE IF NOT EXISTS organization_domains (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  domain TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  is_primary BOOLEAN NOT NULL DEFAULT FALSE,
  verification_token TEXT NOT NULL DEFAULT replace(gen_random_uuid()::text, '-', ''),
  verified_at TIMESTAMPTZ,
  created_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT organization_domains_domain_format CHECK (
    domain ~ '^[a-z0-9]+([-.][a-z0-9]+)*\.[a-z]{2,}$'
  ),
  CONSTRAINT organization_domains_status_supported CHECK (
    status IN ('pending', 'verified', 'rejected')
  )
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_organization_domains_domain_lower
  ON organization_domains(lower(domain));

CREATE UNIQUE INDEX IF NOT EXISTS idx_organization_domains_primary
  ON organization_domains(organization_id)
  WHERE is_primary = TRUE;

CREATE INDEX IF NOT EXISTS idx_organization_domains_org
  ON organization_domains(organization_id, created_at DESC);

CREATE TABLE IF NOT EXISTS organization_rate_limits (
  organization_id UUID PRIMARY KEY REFERENCES organizations(id) ON DELETE CASCADE,
  requests_per_minute INTEGER NOT NULL DEFAULT 600,
  user_requests_per_minute INTEGER NOT NULL DEFAULT 120,
  burst INTEGER NOT NULL DEFAULT 120,
  updated_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT organization_rate_limits_positive CHECK (
    requests_per_minute > 0
    AND user_requests_per_minute > 0
    AND burst >= 0
  )
);

INSERT INTO organization_rate_limits (organization_id)
SELECT id FROM organizations
ON CONFLICT (organization_id) DO NOTHING;

CREATE TABLE IF NOT EXISTS audit_logs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
  action TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id UUID,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT audit_logs_action_not_empty CHECK (length(trim(action)) > 0),
  CONSTRAINT audit_logs_entity_type_not_empty CHECK (length(trim(entity_type)) > 0),
  CONSTRAINT audit_logs_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_org_created
  ON audit_logs(organization_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_logs_actor
  ON audit_logs(actor_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_logs_action
  ON audit_logs(organization_id, action, created_at DESC);

CREATE TABLE IF NOT EXISTS email_deliveries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
  recipient_email TEXT NOT NULL,
  template TEXT NOT NULL,
  subject TEXT NOT NULL,
  provider TEXT NOT NULL DEFAULT 'log',
  status TEXT NOT NULL DEFAULT 'pending',
  provider_message_id TEXT,
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  error TEXT,
  sent_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT email_deliveries_recipient_not_empty CHECK (length(trim(recipient_email)) > 0),
  CONSTRAINT email_deliveries_template_not_empty CHECK (length(trim(template)) > 0),
  CONSTRAINT email_deliveries_subject_not_empty CHECK (length(trim(subject)) > 0),
  CONSTRAINT email_deliveries_provider_not_empty CHECK (length(trim(provider)) > 0),
  CONSTRAINT email_deliveries_status_supported CHECK (
    status IN ('pending', 'sent', 'failed', 'skipped')
  ),
  CONSTRAINT email_deliveries_payload_object CHECK (jsonb_typeof(payload) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_email_deliveries_org_created
  ON email_deliveries(organization_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_email_deliveries_status
  ON email_deliveries(status, created_at DESC);

CREATE TABLE IF NOT EXISTS saas_alert_rules (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  alert_key TEXT NOT NULL,
  severity TEXT NOT NULL DEFAULT 'warning',
  is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
  config JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT saas_alert_rules_key_not_empty CHECK (length(trim(alert_key)) > 0),
  CONSTRAINT saas_alert_rules_severity_supported CHECK (
    severity IN ('info', 'warning', 'critical')
  ),
  CONSTRAINT saas_alert_rules_config_object CHECK (jsonb_typeof(config) = 'object'),
  CONSTRAINT saas_alert_rules_org_key_unique UNIQUE (organization_id, alert_key)
);

CREATE INDEX IF NOT EXISTS idx_saas_alert_rules_org
  ON saas_alert_rules(organization_id, alert_key);

INSERT INTO saas_alert_rules (organization_id, alert_key, severity, config)
SELECT organizations.id, rule.alert_key, rule.severity, rule.config
FROM organizations
CROSS JOIN (
  VALUES
    ('billing_event_failed', 'critical', '{"source": "stripe"}'::jsonb),
    ('email_delivery_failed', 'warning', '{"window_minutes": 15}'::jsonb),
    ('rls_error_rate', 'critical', '{"threshold": 1}'::jsonb),
    ('migration_failed', 'critical', '{"runbook": "check migration logs"}'::jsonb)
) AS rule(alert_key, severity, config)
ON CONFLICT (organization_id, alert_key) DO NOTHING;

ALTER TABLE organization_domains ENABLE ROW LEVEL SECURITY;
ALTER TABLE organization_domains FORCE ROW LEVEL SECURITY;
ALTER TABLE organization_rate_limits ENABLE ROW LEVEL SECURITY;
ALTER TABLE organization_rate_limits FORCE ROW LEVEL SECURITY;
ALTER TABLE audit_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit_logs FORCE ROW LEVEL SECURITY;
ALTER TABLE email_deliveries ENABLE ROW LEVEL SECURITY;
ALTER TABLE email_deliveries FORCE ROW LEVEL SECURITY;
ALTER TABLE saas_alert_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE saas_alert_rules FORCE ROW LEVEL SECURITY;

DO $$
DECLARE
  tenant_table TEXT;
BEGIN
  FOREACH tenant_table IN ARRAY ARRAY[
    'organization_domains',
    'organization_rate_limits',
    'audit_logs',
    'saas_alert_rules'
  ]
  LOOP
    EXECUTE format('DROP POLICY IF EXISTS %I_select ON %I', tenant_table || '_tenant', tenant_table);
    EXECUTE format('DROP POLICY IF EXISTS %I_insert ON %I', tenant_table || '_tenant', tenant_table);
    EXECUTE format('DROP POLICY IF EXISTS %I_update ON %I', tenant_table || '_tenant', tenant_table);
    EXECUTE format('DROP POLICY IF EXISTS %I_delete ON %I', tenant_table || '_tenant', tenant_table);

    EXECUTE format(
      'CREATE POLICY %I_select ON %I FOR SELECT USING (app_rls_tenant_matches(organization_id))',
      tenant_table || '_tenant',
      tenant_table
    );
    EXECUTE format(
      'CREATE POLICY %I_insert ON %I FOR INSERT WITH CHECK (app_rls_tenant_matches(organization_id))',
      tenant_table || '_tenant',
      tenant_table
    );
    EXECUTE format(
      'CREATE POLICY %I_update ON %I FOR UPDATE USING (app_rls_tenant_matches(organization_id)) WITH CHECK (app_rls_tenant_matches(organization_id))',
      tenant_table || '_tenant',
      tenant_table
    );
    EXECUTE format(
      'CREATE POLICY %I_delete ON %I FOR DELETE USING (app_rls_tenant_matches(organization_id))',
      tenant_table || '_tenant',
      tenant_table
    );
  END LOOP;
END $$;

DROP POLICY IF EXISTS email_deliveries_tenant_select ON email_deliveries;
DROP POLICY IF EXISTS email_deliveries_tenant_insert ON email_deliveries;
DROP POLICY IF EXISTS email_deliveries_tenant_update ON email_deliveries;
DROP POLICY IF EXISTS email_deliveries_tenant_delete ON email_deliveries;

CREATE POLICY email_deliveries_tenant_select
ON email_deliveries
FOR SELECT
USING (organization_id IS NOT NULL AND app_rls_tenant_matches(organization_id));

CREATE POLICY email_deliveries_tenant_insert
ON email_deliveries
FOR INSERT
WITH CHECK (organization_id IS NULL OR app_rls_tenant_matches(organization_id));

CREATE POLICY email_deliveries_tenant_update
ON email_deliveries
FOR UPDATE
USING (organization_id IS NULL OR app_rls_tenant_matches(organization_id))
WITH CHECK (organization_id IS NULL OR app_rls_tenant_matches(organization_id));

CREATE POLICY email_deliveries_tenant_delete
ON email_deliveries
FOR DELETE
USING (organization_id IS NOT NULL AND app_rls_tenant_matches(organization_id));
