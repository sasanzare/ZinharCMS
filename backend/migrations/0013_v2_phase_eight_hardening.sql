ALTER TABLE organization_subscriptions
  ADD COLUMN IF NOT EXISTS provider_event_created_at TIMESTAMPTZ;

ALTER TABLE billing_events
  ADD COLUMN IF NOT EXISTS provider_event_created_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_billing_events_provider_created
  ON billing_events(provider, provider_event_created_at DESC)
  WHERE provider_event_created_at IS NOT NULL;
