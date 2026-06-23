CREATE OR REPLACE FUNCTION app_current_organization_id()
RETURNS UUID
LANGUAGE sql
STABLE
AS $$
  SELECT NULLIF(current_setting('zinhar.organization_id', true), '')::uuid
$$;

CREATE OR REPLACE FUNCTION app_current_user_id()
RETURNS UUID
LANGUAGE sql
STABLE
AS $$
  SELECT NULLIF(current_setting('zinhar.user_id', true), '')::uuid
$$;

CREATE OR REPLACE FUNCTION app_rls_bypass_enabled()
RETURNS BOOLEAN
LANGUAGE sql
STABLE
AS $$
  SELECT COALESCE(NULLIF(current_setting('zinhar.rls_bypass', true), '')::boolean, false)
$$;

CREATE OR REPLACE FUNCTION app_rls_tenant_matches(row_organization_id UUID)
RETURNS BOOLEAN
LANGUAGE sql
STABLE
AS $$
  SELECT app_rls_bypass_enabled()
      OR row_organization_id = app_current_organization_id()
$$;

CREATE OR REPLACE FUNCTION app_rls_component_select(row_organization_id UUID, row_is_system BOOLEAN)
RETURNS BOOLEAN
LANGUAGE sql
STABLE
AS $$
  SELECT app_rls_bypass_enabled()
      OR row_is_system
      OR row_organization_id = app_current_organization_id()
$$;

CREATE OR REPLACE FUNCTION app_rls_component_write(row_organization_id UUID, row_is_system BOOLEAN)
RETURNS BOOLEAN
LANGUAGE sql
STABLE
AS $$
  SELECT app_rls_bypass_enabled()
      OR (row_is_system = FALSE AND row_organization_id = app_current_organization_id())
$$;

CREATE OR REPLACE PROCEDURE app_enable_tenant_rls(table_name REGCLASS)
LANGUAGE plpgsql
AS $$
BEGIN
  EXECUTE format('ALTER TABLE %s ENABLE ROW LEVEL SECURITY', table_name);
  EXECUTE format('ALTER TABLE %s FORCE ROW LEVEL SECURITY', table_name);
END $$;

CALL app_enable_tenant_rls('content_types');
CALL app_enable_tenant_rls('content_entries');
CALL app_enable_tenant_rls('pages');
CALL app_enable_tenant_rls('page_versions');
CALL app_enable_tenant_rls('media');
CALL app_enable_tenant_rls('media_variants');
CALL app_enable_tenant_rls('comments');
CALL app_enable_tenant_rls('webhooks');
CALL app_enable_tenant_rls('webhook_deliveries');
CALL app_enable_tenant_rls('public_settings');
CALL app_enable_tenant_rls('navigation_items');
CALL app_enable_tenant_rls('component_registry');

DROP PROCEDURE app_enable_tenant_rls(REGCLASS);

DO $$
DECLARE
  tenant_table TEXT;
BEGIN
  FOREACH tenant_table IN ARRAY ARRAY[
    'content_types',
    'content_entries',
    'pages',
    'page_versions',
    'media',
    'media_variants',
    'comments',
    'webhooks',
    'webhook_deliveries',
    'public_settings',
    'navigation_items'
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

DROP POLICY IF EXISTS component_registry_tenant_select ON component_registry;
DROP POLICY IF EXISTS component_registry_tenant_insert ON component_registry;
DROP POLICY IF EXISTS component_registry_tenant_update ON component_registry;
DROP POLICY IF EXISTS component_registry_tenant_delete ON component_registry;

CREATE POLICY component_registry_tenant_select
ON component_registry
FOR SELECT
USING (app_rls_component_select(organization_id, is_system));

CREATE POLICY component_registry_tenant_insert
ON component_registry
FOR INSERT
WITH CHECK (app_rls_component_write(organization_id, is_system));

CREATE POLICY component_registry_tenant_update
ON component_registry
FOR UPDATE
USING (app_rls_component_write(organization_id, is_system))
WITH CHECK (app_rls_component_write(organization_id, is_system));

CREATE POLICY component_registry_tenant_delete
ON component_registry
FOR DELETE
USING (app_rls_component_write(organization_id, is_system));
