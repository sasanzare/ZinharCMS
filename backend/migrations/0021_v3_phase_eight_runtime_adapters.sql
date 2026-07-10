ALTER TABLE component_registry
  ADD COLUMN IF NOT EXISTS marketplace_installation_id UUID REFERENCES marketplace_installations(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_component_registry_marketplace_installation
  ON component_registry(marketplace_installation_id);

CREATE TABLE IF NOT EXISTS marketplace_template_imports (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  installation_id UUID NOT NULL REFERENCES marketplace_installations(id) ON DELETE RESTRICT,
  page_id UUID NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
  template_key TEXT NOT NULL,
  asset_mapping JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_template_imports_mapping_object CHECK (jsonb_typeof(asset_mapping) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_marketplace_template_imports_org
  ON marketplace_template_imports(organization_id, created_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_plugin_hooks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  installation_id UUID NOT NULL REFERENCES marketplace_installations(id) ON DELETE CASCADE,
  hook_key TEXT NOT NULL,
  hook_type TEXT NOT NULL,
  label TEXT NOT NULL,
  contract_version TEXT NOT NULL DEFAULT '2026-07',
  config JSONB NOT NULL DEFAULT '{}'::jsonb,
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_plugin_hooks_type_supported CHECK (
    hook_type IN ('sidebar.item', 'dashboard.widget', 'form.field', 'webhook.adapter')
  ),
  CONSTRAINT marketplace_plugin_hooks_config_object CHECK (jsonb_typeof(config) = 'object'),
  CONSTRAINT marketplace_plugin_hooks_key_unique UNIQUE (organization_id, installation_id, hook_key)
);

CREATE INDEX IF NOT EXISTS idx_marketplace_plugin_hooks_org_type
  ON marketplace_plugin_hooks(organization_id, hook_type, enabled);

ALTER TABLE marketplace_template_imports ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_template_imports FORCE ROW LEVEL SECURITY;
ALTER TABLE marketplace_plugin_hooks ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_plugin_hooks FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS marketplace_template_imports_tenant_select ON marketplace_template_imports;
DROP POLICY IF EXISTS marketplace_template_imports_tenant_insert ON marketplace_template_imports;
CREATE POLICY marketplace_template_imports_tenant_select
ON marketplace_template_imports FOR SELECT
USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_template_imports_tenant_insert
ON marketplace_template_imports FOR INSERT
WITH CHECK (app_rls_tenant_matches(organization_id));

DROP POLICY IF EXISTS marketplace_plugin_hooks_tenant_select ON marketplace_plugin_hooks;
DROP POLICY IF EXISTS marketplace_plugin_hooks_tenant_insert ON marketplace_plugin_hooks;
DROP POLICY IF EXISTS marketplace_plugin_hooks_tenant_update ON marketplace_plugin_hooks;
CREATE POLICY marketplace_plugin_hooks_tenant_select
ON marketplace_plugin_hooks FOR SELECT
USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_plugin_hooks_tenant_insert
ON marketplace_plugin_hooks FOR INSERT
WITH CHECK (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_plugin_hooks_tenant_update
ON marketplace_plugin_hooks FOR UPDATE
USING (app_rls_tenant_matches(organization_id))
WITH CHECK (app_rls_tenant_matches(organization_id));
