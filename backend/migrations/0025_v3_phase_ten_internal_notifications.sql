CREATE TABLE IF NOT EXISTS marketplace_internal_notifications (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  abuse_report_id UUID NOT NULL UNIQUE REFERENCES marketplace_abuse_reports(id) ON DELETE CASCADE,
  notification_type TEXT NOT NULL,
  recipient_role TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'unread',
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  acknowledged_at TIMESTAMPTZ,
  CONSTRAINT marketplace_internal_notifications_type_supported
    CHECK (notification_type IN ('critical_abuse_report')),
  CONSTRAINT marketplace_internal_notifications_recipient_supported
    CHECK (recipient_role IN ('admin')),
  CONSTRAINT marketplace_internal_notifications_status_supported
    CHECK (status IN ('unread', 'acknowledged')),
  CONSTRAINT marketplace_internal_notifications_payload_object
    CHECK (jsonb_typeof(payload) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_marketplace_internal_notifications_queue
  ON marketplace_internal_notifications(status, created_at ASC);
