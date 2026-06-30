CREATE TABLE IF NOT EXISTS beta_participants (
  organization_id UUID PRIMARY KEY REFERENCES organizations(id) ON DELETE CASCADE,
  cohort_label TEXT NOT NULL DEFAULT 'private-beta',
  contact_name TEXT,
  contact_email TEXT,
  status TEXT NOT NULL DEFAULT 'candidate',
  onboarded_at TIMESTAMPTZ,
  last_check_in_at TIMESTAMPTZ,
  notes TEXT,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT beta_participants_cohort_not_empty CHECK (length(trim(cohort_label)) > 0),
  CONSTRAINT beta_participants_status_supported CHECK (
    status IN ('candidate', 'invited', 'onboarding', 'active', 'paused', 'graduated', 'rejected')
  ),
  CONSTRAINT beta_participants_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_beta_participants_status
  ON beta_participants(status, updated_at DESC);

CREATE TABLE IF NOT EXISTS beta_feedback (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
  category TEXT NOT NULL DEFAULT 'ux',
  severity TEXT NOT NULL DEFAULT 'medium',
  status TEXT NOT NULL DEFAULT 'open',
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  page_url TEXT,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT beta_feedback_category_supported CHECK (
    category IN ('bug', 'ux', 'billing', 'performance', 'tenant_isolation', 'onboarding', 'other')
  ),
  CONSTRAINT beta_feedback_severity_supported CHECK (
    severity IN ('low', 'medium', 'high', 'critical')
  ),
  CONSTRAINT beta_feedback_status_supported CHECK (
    status IN ('open', 'triaged', 'planned', 'fixed', 'closed')
  ),
  CONSTRAINT beta_feedback_title_not_empty CHECK (length(trim(title)) > 0),
  CONSTRAINT beta_feedback_description_not_empty CHECK (length(trim(description)) > 0),
  CONSTRAINT beta_feedback_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_beta_feedback_org_created
  ON beta_feedback(organization_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_beta_feedback_org_status
  ON beta_feedback(organization_id, status, severity, created_at DESC);

CREATE TABLE IF NOT EXISTS beta_ga_blockers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  feedback_id UUID REFERENCES beta_feedback(id) ON DELETE SET NULL,
  priority TEXT NOT NULL DEFAULT 'p2',
  area TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'open',
  owner TEXT,
  due_at DATE,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT beta_ga_blockers_priority_supported CHECK (
    priority IN ('p0', 'p1', 'p2', 'p3')
  ),
  CONSTRAINT beta_ga_blockers_status_supported CHECK (
    status IN ('open', 'in_progress', 'blocked', 'resolved', 'deferred')
  ),
  CONSTRAINT beta_ga_blockers_area_not_empty CHECK (length(trim(area)) > 0),
  CONSTRAINT beta_ga_blockers_title_not_empty CHECK (length(trim(title)) > 0),
  CONSTRAINT beta_ga_blockers_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_beta_ga_blockers_org_status
  ON beta_ga_blockers(organization_id, status, priority, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_beta_ga_blockers_feedback
  ON beta_ga_blockers(feedback_id)
  WHERE feedback_id IS NOT NULL;

ALTER TABLE beta_participants ENABLE ROW LEVEL SECURITY;
ALTER TABLE beta_participants FORCE ROW LEVEL SECURITY;
ALTER TABLE beta_feedback ENABLE ROW LEVEL SECURITY;
ALTER TABLE beta_feedback FORCE ROW LEVEL SECURITY;
ALTER TABLE beta_ga_blockers ENABLE ROW LEVEL SECURITY;
ALTER TABLE beta_ga_blockers FORCE ROW LEVEL SECURITY;

DO $$
DECLARE
  tenant_table TEXT;
BEGIN
  FOREACH tenant_table IN ARRAY ARRAY[
    'beta_participants',
    'beta_feedback',
    'beta_ga_blockers'
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
