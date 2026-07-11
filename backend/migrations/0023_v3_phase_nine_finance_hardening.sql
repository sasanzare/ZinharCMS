ALTER TABLE marketplace_revenue_ledger
  ADD COLUMN IF NOT EXISTS provider_event_id TEXT;

DROP INDEX IF EXISTS idx_marketplace_ledger_purchase_entry;
CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_ledger_purchase_entry
  ON marketplace_revenue_ledger(purchase_id)
  WHERE entry_type = 'purchase';
CREATE UNIQUE INDEX IF NOT EXISTS idx_marketplace_ledger_provider_event
  ON marketplace_revenue_ledger(purchase_id, entry_type, provider_event_id)
  WHERE provider_event_id IS NOT NULL;

CREATE OR REPLACE FUNCTION marketplace_revenue_ledger_append_only()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
  RAISE EXCEPTION 'Marketplace revenue ledger is append-only';
END $$;

DROP TRIGGER IF EXISTS trg_marketplace_revenue_ledger_append_only ON marketplace_revenue_ledger;
CREATE TRIGGER trg_marketplace_revenue_ledger_append_only
BEFORE UPDATE OR DELETE ON marketplace_revenue_ledger
FOR EACH ROW
EXECUTE FUNCTION marketplace_revenue_ledger_append_only();

ALTER TABLE marketplace_payouts
  ADD COLUMN IF NOT EXISTS settlement_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS provider_account_id TEXT;

CREATE INDEX IF NOT EXISTS idx_marketplace_payouts_creator_status
  ON marketplace_payouts(creator_id, status, created_at DESC);
