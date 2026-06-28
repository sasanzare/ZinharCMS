DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'organization_subscription_status') THEN
    CREATE TYPE organization_subscription_status AS ENUM (
      'trialing',
      'active',
      'past_due',
      'canceled',
      'incomplete'
    );
  END IF;
END $$;

CREATE TABLE IF NOT EXISTS plans (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  slug TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  price_monthly_cents INTEGER NOT NULL DEFAULT 0,
  member_limit INTEGER NOT NULL,
  content_limit INTEGER NOT NULL,
  media_limit_mb INTEGER NOT NULL,
  api_requests_limit INTEGER NOT NULL,
  features JSONB NOT NULL DEFAULT '{}'::jsonb,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT plans_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
  CONSTRAINT plans_name_not_empty CHECK (length(trim(name)) > 0),
  CONSTRAINT plans_limits_allow_unlimited CHECK (
    member_limit >= -1
    AND content_limit >= -1
    AND media_limit_mb >= -1
    AND api_requests_limit >= -1
  ),
  CONSTRAINT plans_features_object CHECK (jsonb_typeof(features) = 'object')
);

INSERT INTO plans (
  slug,
  name,
  description,
  price_monthly_cents,
  member_limit,
  content_limit,
  media_limit_mb,
  api_requests_limit,
  features,
  sort_order
)
VALUES
  (
    'free',
    'Free',
    'For trials and small projects',
    0,
    3,
    50,
    1024,
    10000,
    '{"custom_domain": false, "sla": false, "dedicated_support": false}'::jsonb,
    10
  ),
  (
    'pro',
    'Pro',
    'For small and medium teams',
    2900,
    10,
    5000,
    51200,
    250000,
    '{"custom_domain": true, "sla": false, "dedicated_support": false}'::jsonb,
    20
  ),
  (
    'enterprise',
    'Enterprise',
    'For large organizations with custom limits',
    0,
    -1,
    -1,
    -1,
    -1,
    '{"custom_domain": true, "sla": true, "dedicated_support": true, "dedicated_database": true}'::jsonb,
    30
  )
ON CONFLICT (slug) DO UPDATE
SET name = EXCLUDED.name,
    description = EXCLUDED.description,
    price_monthly_cents = EXCLUDED.price_monthly_cents,
    member_limit = EXCLUDED.member_limit,
    content_limit = EXCLUDED.content_limit,
    media_limit_mb = EXCLUDED.media_limit_mb,
    api_requests_limit = EXCLUDED.api_requests_limit,
    features = EXCLUDED.features,
    sort_order = EXCLUDED.sort_order,
    is_active = TRUE,
    updated_at = now();

CREATE TABLE IF NOT EXISTS organization_subscriptions (
  organization_id UUID PRIMARY KEY REFERENCES organizations(id) ON DELETE CASCADE,
  plan_id UUID NOT NULL REFERENCES plans(id),
  status organization_subscription_status NOT NULL DEFAULT 'active',
  provider TEXT NOT NULL DEFAULT 'manual',
  provider_customer_id TEXT,
  provider_subscription_id TEXT,
  current_period_start TIMESTAMPTZ NOT NULL DEFAULT date_trunc('month', now()),
  current_period_end TIMESTAMPTZ NOT NULL DEFAULT (date_trunc('month', now()) + interval '1 month'),
  cancel_at_period_end BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT organization_subscriptions_provider_not_empty CHECK (length(trim(provider)) > 0)
);

CREATE INDEX IF NOT EXISTS idx_organization_subscriptions_plan
  ON organization_subscriptions(plan_id);

INSERT INTO organization_subscriptions (organization_id, plan_id, status, provider)
SELECT organizations.id, plans.id, 'active'::organization_subscription_status, 'manual'
FROM organizations
CROSS JOIN plans
WHERE plans.slug = 'free'
ON CONFLICT (organization_id) DO NOTHING;

CREATE TABLE IF NOT EXISTS usage_counters (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  period_start DATE NOT NULL,
  metric TEXT NOT NULL,
  value BIGINT NOT NULL DEFAULT 0,
  rebuilt_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT usage_counters_value_non_negative CHECK (value >= 0),
  CONSTRAINT usage_counters_metric_supported CHECK (
    metric IN ('members', 'content_records', 'media_bytes', 'api_requests')
  ),
  CONSTRAINT usage_counters_period_month_start CHECK (date_trunc('month', period_start::timestamp)::date = period_start),
  UNIQUE (organization_id, period_start, metric)
);

CREATE INDEX IF NOT EXISTS idx_usage_counters_org_period
  ON usage_counters(organization_id, period_start);

ALTER TABLE organization_subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE organization_subscriptions FORCE ROW LEVEL SECURITY;
ALTER TABLE usage_counters ENABLE ROW LEVEL SECURITY;
ALTER TABLE usage_counters FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS organization_subscriptions_tenant_select ON organization_subscriptions;
DROP POLICY IF EXISTS organization_subscriptions_tenant_insert ON organization_subscriptions;
DROP POLICY IF EXISTS organization_subscriptions_tenant_update ON organization_subscriptions;
DROP POLICY IF EXISTS organization_subscriptions_tenant_delete ON organization_subscriptions;

CREATE POLICY organization_subscriptions_tenant_select
ON organization_subscriptions
FOR SELECT
USING (app_rls_tenant_matches(organization_id));

CREATE POLICY organization_subscriptions_tenant_insert
ON organization_subscriptions
FOR INSERT
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY organization_subscriptions_tenant_update
ON organization_subscriptions
FOR UPDATE
USING (app_rls_tenant_matches(organization_id))
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY organization_subscriptions_tenant_delete
ON organization_subscriptions
FOR DELETE
USING (app_rls_tenant_matches(organization_id));

DROP POLICY IF EXISTS usage_counters_tenant_select ON usage_counters;
DROP POLICY IF EXISTS usage_counters_tenant_insert ON usage_counters;
DROP POLICY IF EXISTS usage_counters_tenant_update ON usage_counters;
DROP POLICY IF EXISTS usage_counters_tenant_delete ON usage_counters;

CREATE POLICY usage_counters_tenant_select
ON usage_counters
FOR SELECT
USING (app_rls_tenant_matches(organization_id));

CREATE POLICY usage_counters_tenant_insert
ON usage_counters
FOR INSERT
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY usage_counters_tenant_update
ON usage_counters
FOR UPDATE
USING (app_rls_tenant_matches(organization_id))
WITH CHECK (app_rls_tenant_matches(organization_id));

CREATE POLICY usage_counters_tenant_delete
ON usage_counters
FOR DELETE
USING (app_rls_tenant_matches(organization_id));
