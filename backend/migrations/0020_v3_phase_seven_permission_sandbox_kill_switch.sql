CREATE TABLE IF NOT EXISTS marketplace_permission_catalog (
  permission_key TEXT PRIMARY KEY,
  description TEXT NOT NULL,
  category TEXT NOT NULL,
  risk_level TEXT NOT NULL DEFAULT 'low',
  product_types JSONB NOT NULL DEFAULT '[]'::jsonb,
  runtime_operations JSONB NOT NULL DEFAULT '[]'::jsonb,
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_permission_catalog_key_format CHECK (
    permission_key ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'
  ),
  CONSTRAINT marketplace_permission_catalog_category_supported CHECK (
    category IN ('content', 'page', 'media', 'integration', 'settings', 'runtime')
  ),
  CONSTRAINT marketplace_permission_catalog_risk_supported CHECK (
    risk_level IN ('low', 'medium', 'high', 'critical')
  ),
  CONSTRAINT marketplace_permission_catalog_product_types_array CHECK (
    jsonb_typeof(product_types) = 'array'
  ),
  CONSTRAINT marketplace_permission_catalog_runtime_operations_array CHECK (
    jsonb_typeof(runtime_operations) = 'array'
  )
);

INSERT INTO marketplace_permission_catalog (
  permission_key, description, category, risk_level, product_types, runtime_operations
)
VALUES
  ('content.read', 'Read organization content entries through the host API', 'content', 'low',
   '["component_pack", "design_template", "integration_plugin"]'::jsonb,
   '["content.read"]'::jsonb),
  ('content.write', 'Create or update organization content through the host API', 'content', 'high',
   '["integration_plugin"]'::jsonb,
   '["content.write"]'::jsonb),
  ('page.read', 'Read organization pages and page metadata through the host API', 'page', 'low',
   '["component_pack", "design_template", "integration_plugin"]'::jsonb,
   '["component.render", "page.read"]'::jsonb),
  ('page.write', 'Create or update organization pages through the host API', 'page', 'high',
   '["integration_plugin"]'::jsonb,
   '["page.write"]'::jsonb),
  ('media.read', 'Read organization media metadata through the host API', 'media', 'low',
   '["component_pack", "design_template", "integration_plugin"]'::jsonb,
   '["media.read"]'::jsonb),
  ('media.write', 'Create or update organization media through the host API', 'media', 'high',
   '["integration_plugin"]'::jsonb,
   '["media.write"]'::jsonb),
  ('webhook.send', 'Dispatch an approved integration webhook through the host API', 'integration', 'medium',
   '["integration_plugin"]'::jsonb,
   '["integration.invoke", "webhook.send"]'::jsonb),
  ('settings.read', 'Read the public organization settings surface', 'settings', 'medium',
   '["component_pack", "design_template", "integration_plugin"]'::jsonb,
   '["settings.read"]'::jsonb),
  ('external_network.request', 'Request an allowlisted external integration operation', 'integration', 'high',
   '["integration_plugin"]'::jsonb,
   '["external_network.request"]'::jsonb)
ON CONFLICT (permission_key) DO UPDATE
SET description = EXCLUDED.description,
    category = EXCLUDED.category,
    risk_level = EXCLUDED.risk_level,
    product_types = EXCLUDED.product_types,
    runtime_operations = EXCLUDED.runtime_operations,
    enabled = EXCLUDED.enabled,
    updated_at = now();

ALTER TABLE marketplace_installations
  ADD COLUMN IF NOT EXISTS runtime_status TEXT NOT NULL DEFAULT 'ready',
  ADD COLUMN IF NOT EXISTS runtime_block_reason TEXT,
  ADD COLUMN IF NOT EXISTS runtime_blocked_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS runtime_checked_at TIMESTAMPTZ NOT NULL DEFAULT now();

ALTER TABLE marketplace_installations
  DROP CONSTRAINT IF EXISTS marketplace_installations_runtime_status_supported;

ALTER TABLE marketplace_installations
  ADD CONSTRAINT marketplace_installations_runtime_status_supported CHECK (
    runtime_status IN ('ready', 'blocked')
  );

CREATE INDEX IF NOT EXISTS idx_marketplace_installations_runtime_status
  ON marketplace_installations(organization_id, runtime_status, updated_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_kill_switches (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  scope TEXT NOT NULL,
  organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
  reason TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  lifted_by UUID REFERENCES users(id) ON DELETE SET NULL,
  lifted_at TIMESTAMPTZ,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  CONSTRAINT marketplace_kill_switches_scope_supported CHECK (
    scope IN ('global', 'organization')
  ),
  CONSTRAINT marketplace_kill_switches_scope_owner CHECK (
    (scope = 'global' AND organization_id IS NULL)
    OR (scope = 'organization' AND organization_id IS NOT NULL)
  ),
  CONSTRAINT marketplace_kill_switches_reason_not_empty CHECK (length(trim(reason)) > 0),
  CONSTRAINT marketplace_kill_switches_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_kill_switches_global_active
  ON marketplace_kill_switches(scope)
  WHERE scope = 'global' AND active = TRUE;

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_kill_switches_org_active
  ON marketplace_kill_switches(organization_id)
  WHERE scope = 'organization' AND active = TRUE;

CREATE INDEX IF NOT EXISTS idx_marketplace_kill_switches_scope_created
  ON marketplace_kill_switches(scope, organization_id, created_at DESC);

ALTER TABLE marketplace_kill_switches ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_kill_switches FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS marketplace_kill_switches_tenant_select ON marketplace_kill_switches;
DROP POLICY IF EXISTS marketplace_kill_switches_tenant_insert ON marketplace_kill_switches;
DROP POLICY IF EXISTS marketplace_kill_switches_tenant_update ON marketplace_kill_switches;
DROP POLICY IF EXISTS marketplace_kill_switches_tenant_delete ON marketplace_kill_switches;

CREATE POLICY marketplace_kill_switches_tenant_select
ON marketplace_kill_switches
FOR SELECT
USING (scope = 'global' OR app_rls_tenant_matches(organization_id));

CREATE POLICY marketplace_kill_switches_tenant_insert
ON marketplace_kill_switches
FOR INSERT
WITH CHECK (scope = 'organization' AND app_rls_tenant_matches(organization_id));

CREATE POLICY marketplace_kill_switches_tenant_update
ON marketplace_kill_switches
FOR UPDATE
USING (scope = 'organization' AND app_rls_tenant_matches(organization_id))
WITH CHECK (scope = 'organization' AND app_rls_tenant_matches(organization_id));

CREATE POLICY marketplace_kill_switches_tenant_delete
ON marketplace_kill_switches
FOR DELETE
USING (scope = 'organization' AND app_rls_tenant_matches(organization_id));
