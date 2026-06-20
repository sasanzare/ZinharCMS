CREATE TABLE IF NOT EXISTS webhooks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  events TEXT[] NOT NULL,
  secret TEXT NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT webhooks_name_not_empty CHECK (length(trim(name)) > 0),
  CONSTRAINT webhooks_url_http CHECK (url ~ '^https?://'),
  CONSTRAINT webhooks_events_not_empty CHECK (cardinality(events) > 0)
);

CREATE INDEX IF NOT EXISTS idx_webhooks_active_events
  ON webhooks USING gin(events)
  WHERE is_active = TRUE;

CREATE TABLE IF NOT EXISTS webhook_deliveries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  webhook_id UUID NOT NULL REFERENCES webhooks(id) ON DELETE CASCADE,
  event TEXT NOT NULL,
  payload JSONB NOT NULL,
  status TEXT NOT NULL,
  status_code INTEGER,
  response_body TEXT,
  error TEXT,
  attempted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT webhook_deliveries_payload_object CHECK (jsonb_typeof(payload) = 'object'),
  CONSTRAINT webhook_deliveries_status CHECK (status IN ('delivered', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_webhook_deliveries_webhook_attempted
  ON webhook_deliveries(webhook_id, attempted_at DESC);

CREATE TABLE IF NOT EXISTS public_settings (
  key TEXT PRIMARY KEY,
  value JSONB NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT TRUE,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT public_settings_key_format CHECK (key ~ '^[a-z0-9_]+$')
);

INSERT INTO public_settings (key, value, is_public)
VALUES
  ('site_name', '"ZinharCMS"'::jsonb, TRUE),
  ('site_url', '"http://localhost:5173"'::jsonb, TRUE),
  ('default_locale', '"fa"'::jsonb, TRUE)
ON CONFLICT (key) DO UPDATE
SET value = EXCLUDED.value,
    is_public = EXCLUDED.is_public,
    updated_at = now();

CREATE TABLE IF NOT EXISTS navigation_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  label TEXT NOT NULL,
  url TEXT NOT NULL,
  parent_id UUID REFERENCES navigation_items(id) ON DELETE CASCADE,
  position INTEGER NOT NULL DEFAULT 0,
  locale TEXT NOT NULL DEFAULT 'fa',
  is_public BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT navigation_label_not_empty CHECK (length(trim(label)) > 0),
  CONSTRAINT navigation_url_not_empty CHECK (length(trim(url)) > 0),
  CONSTRAINT navigation_locale_format CHECK (locale ~ '^[a-z]{2}(-[A-Z]{2})?$')
);

CREATE INDEX IF NOT EXISTS idx_navigation_items_public_locale
  ON navigation_items(locale, position)
  WHERE is_public = TRUE;

INSERT INTO navigation_items (label, url, position, locale, is_public)
VALUES
  ('Home', '/', 10, 'fa', TRUE),
  ('Content', '/content', 20, 'fa', TRUE)
ON CONFLICT DO NOTHING;

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'webhooks:create',
          'webhooks:read',
          'webhooks:update',
          'webhooks:delete',
          'webhooks:test'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name = 'admin';
