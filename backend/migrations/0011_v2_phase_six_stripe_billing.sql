ALTER TABLE plans
  ADD COLUMN IF NOT EXISTS stripe_product_id TEXT,
  ADD COLUMN IF NOT EXISTS stripe_price_id TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_plans_stripe_price_id
  ON plans(stripe_price_id)
  WHERE stripe_price_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS billing_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  provider TEXT NOT NULL DEFAULT 'stripe',
  provider_event_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  organization_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
  status TEXT NOT NULL DEFAULT 'processing',
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  error TEXT,
  processed_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT billing_events_provider_not_empty CHECK (length(trim(provider)) > 0),
  CONSTRAINT billing_events_provider_event_unique UNIQUE (provider, provider_event_id),
  CONSTRAINT billing_events_status_supported CHECK (status IN ('processing', 'processed', 'failed', 'ignored')),
  CONSTRAINT billing_events_payload_object CHECK (jsonb_typeof(payload) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_billing_events_organization
  ON billing_events(organization_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_billing_events_type
  ON billing_events(provider, event_type, created_at DESC);

ALTER TABLE billing_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE billing_events FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS billing_events_tenant_select ON billing_events;
DROP POLICY IF EXISTS billing_events_tenant_insert ON billing_events;
DROP POLICY IF EXISTS billing_events_tenant_update ON billing_events;
DROP POLICY IF EXISTS billing_events_tenant_delete ON billing_events;

CREATE POLICY billing_events_tenant_select
ON billing_events
FOR SELECT
USING (organization_id IS NOT NULL AND app_rls_tenant_matches(organization_id));

CREATE POLICY billing_events_tenant_insert
ON billing_events
FOR INSERT
WITH CHECK (organization_id IS NULL OR app_rls_tenant_matches(organization_id));

CREATE POLICY billing_events_tenant_update
ON billing_events
FOR UPDATE
USING (organization_id IS NULL OR app_rls_tenant_matches(organization_id))
WITH CHECK (organization_id IS NULL OR app_rls_tenant_matches(organization_id));

CREATE POLICY billing_events_tenant_delete
ON billing_events
FOR DELETE
USING (organization_id IS NOT NULL AND app_rls_tenant_matches(organization_id));
