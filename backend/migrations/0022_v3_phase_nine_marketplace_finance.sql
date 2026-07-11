CREATE TABLE IF NOT EXISTS marketplace_purchases (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE RESTRICT,
  version_id UUID NOT NULL REFERENCES marketplace_versions(id) ON DELETE RESTRICT,
  buyer_id UUID REFERENCES users(id) ON DELETE SET NULL,
  pricing_type TEXT NOT NULL,
  currency TEXT NOT NULL DEFAULT 'usd',
  subtotal_cents INTEGER NOT NULL,
  tax_cents INTEGER NOT NULL DEFAULT 0,
  total_cents INTEGER NOT NULL,
  provider TEXT NOT NULL DEFAULT 'none',
  provider_checkout_id TEXT,
  provider_payment_id TEXT,
  status TEXT NOT NULL DEFAULT 'pending',
  receipt_number TEXT NOT NULL,
  provider_metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  refunded_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_purchases_pricing_supported CHECK (pricing_type IN ('free', 'paid')),
  CONSTRAINT marketplace_purchases_currency_format CHECK (currency ~ '^[a-z]{3}$'),
  CONSTRAINT marketplace_purchases_amounts_nonnegative CHECK (subtotal_cents >= 0 AND tax_cents >= 0 AND total_cents = subtotal_cents + tax_cents),
  CONSTRAINT marketplace_purchases_status_supported CHECK (status IN ('pending', 'completed', 'failed', 'refunded', 'canceled')),
  CONSTRAINT marketplace_purchases_provider_metadata_object CHECK (jsonb_typeof(provider_metadata) = 'object'),
  CONSTRAINT marketplace_purchases_receipt_unique UNIQUE (receipt_number),
  CONSTRAINT marketplace_purchases_listing_version_fk FOREIGN KEY (listing_id, version_id) REFERENCES marketplace_versions(listing_id, id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_purchases_provider_checkout
  ON marketplace_purchases(provider, provider_checkout_id)
  WHERE provider_checkout_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_marketplace_purchases_org_status
  ON marketplace_purchases(organization_id, status, created_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_entitlements (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  purchase_id UUID NOT NULL REFERENCES marketplace_purchases(id) ON DELETE RESTRICT,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE RESTRICT,
  version_id UUID NOT NULL REFERENCES marketplace_versions(id) ON DELETE RESTRICT,
  status TEXT NOT NULL DEFAULT 'active',
  granted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  revoked_at TIMESTAMPTZ,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  CONSTRAINT marketplace_entitlements_status_supported CHECK (status IN ('active', 'revoked', 'expired')),
  CONSTRAINT marketplace_entitlements_metadata_object CHECK (jsonb_typeof(metadata) = 'object'),
  CONSTRAINT marketplace_entitlements_purchase_unique UNIQUE (purchase_id),
  CONSTRAINT marketplace_entitlements_listing_version_fk FOREIGN KEY (listing_id, version_id) REFERENCES marketplace_versions(listing_id, id)
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_entitlements_active_listing
  ON marketplace_entitlements(organization_id, listing_id)
  WHERE status = 'active';

CREATE TABLE IF NOT EXISTS marketplace_revenue_ledger (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  purchase_id UUID NOT NULL REFERENCES marketplace_purchases(id) ON DELETE RESTRICT,
  creator_id UUID NOT NULL REFERENCES marketplace_creators(id) ON DELETE RESTRICT,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE RESTRICT,
  entry_type TEXT NOT NULL,
  currency TEXT NOT NULL DEFAULT 'usd',
  gross_cents INTEGER NOT NULL,
  tax_cents INTEGER NOT NULL DEFAULT 0,
  commission_bps INTEGER NOT NULL DEFAULT 2000,
  platform_fee_cents INTEGER NOT NULL,
  creator_share_cents INTEGER NOT NULL,
  adjustment_cents INTEGER NOT NULL DEFAULT 0,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_ledger_entry_type_supported CHECK (entry_type IN ('purchase', 'refund', 'adjustment')),
  CONSTRAINT marketplace_ledger_amounts_valid CHECK (
    gross_cents >= 0
    AND tax_cents >= 0
    AND commission_bps BETWEEN 0 AND 10000
    AND (
      (entry_type = 'purchase' AND platform_fee_cents >= 0 AND creator_share_cents >= 0)
      OR entry_type IN ('refund', 'adjustment')
    )
  ),
  CONSTRAINT marketplace_ledger_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_ledger_purchase_entry
  ON marketplace_revenue_ledger(purchase_id, entry_type);
CREATE INDEX IF NOT EXISTS idx_marketplace_ledger_creator
  ON marketplace_revenue_ledger(creator_id, created_at DESC);

CREATE TABLE IF NOT EXISTS marketplace_payout_accounts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  creator_id UUID NOT NULL REFERENCES marketplace_creators(id) ON DELETE CASCADE,
  provider TEXT NOT NULL DEFAULT 'stripe_connect',
  provider_account_id TEXT,
  status TEXT NOT NULL DEFAULT 'not_configured',
  country TEXT,
  charges_enabled BOOLEAN NOT NULL DEFAULT FALSE,
  payouts_enabled BOOLEAN NOT NULL DEFAULT FALSE,
  details_submitted BOOLEAN NOT NULL DEFAULT FALSE,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_payout_accounts_creator_unique UNIQUE (creator_id),
  CONSTRAINT marketplace_payout_accounts_status_supported CHECK (status IN ('not_configured', 'pending', 'verified', 'restricted')),
  CONSTRAINT marketplace_payout_accounts_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE TABLE IF NOT EXISTS marketplace_payouts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  creator_id UUID NOT NULL REFERENCES marketplace_creators(id) ON DELETE RESTRICT,
  amount_cents INTEGER NOT NULL,
  currency TEXT NOT NULL DEFAULT 'usd',
  status TEXT NOT NULL DEFAULT 'pending',
  provider_transfer_id TEXT,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_payouts_amount_positive CHECK (amount_cents > 0),
  CONSTRAINT marketplace_payouts_status_supported CHECK (status IN ('pending', 'eligible', 'paid', 'failed', 'reversed')),
  CONSTRAINT marketplace_payouts_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

ALTER TABLE marketplace_purchases ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_purchases FORCE ROW LEVEL SECURITY;
ALTER TABLE marketplace_entitlements ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_entitlements FORCE ROW LEVEL SECURITY;
ALTER TABLE marketplace_revenue_ledger ENABLE ROW LEVEL SECURITY;
ALTER TABLE marketplace_revenue_ledger FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS marketplace_purchases_tenant_select ON marketplace_purchases;
DROP POLICY IF EXISTS marketplace_purchases_tenant_insert ON marketplace_purchases;
DROP POLICY IF EXISTS marketplace_purchases_tenant_update ON marketplace_purchases;
CREATE POLICY marketplace_purchases_tenant_select ON marketplace_purchases FOR SELECT USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_purchases_tenant_insert ON marketplace_purchases FOR INSERT WITH CHECK (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_purchases_tenant_update ON marketplace_purchases FOR UPDATE USING (app_rls_tenant_matches(organization_id)) WITH CHECK (app_rls_tenant_matches(organization_id));
DROP POLICY IF EXISTS marketplace_entitlements_tenant_select ON marketplace_entitlements;
DROP POLICY IF EXISTS marketplace_entitlements_tenant_insert ON marketplace_entitlements;
CREATE POLICY marketplace_entitlements_tenant_select ON marketplace_entitlements FOR SELECT USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_entitlements_tenant_insert ON marketplace_entitlements FOR INSERT WITH CHECK (app_rls_tenant_matches(organization_id));
DROP POLICY IF EXISTS marketplace_revenue_ledger_tenant_select ON marketplace_revenue_ledger;
DROP POLICY IF EXISTS marketplace_revenue_ledger_tenant_insert ON marketplace_revenue_ledger;
CREATE POLICY marketplace_revenue_ledger_tenant_select ON marketplace_revenue_ledger FOR SELECT USING (app_rls_tenant_matches(organization_id));
CREATE POLICY marketplace_revenue_ledger_tenant_insert ON marketplace_revenue_ledger FOR INSERT WITH CHECK (app_rls_tenant_matches(organization_id));
