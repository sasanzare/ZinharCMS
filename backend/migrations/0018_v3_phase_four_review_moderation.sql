CREATE TABLE IF NOT EXISTS marketplace_review_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  submission_id UUID REFERENCES marketplace_submissions(id) ON DELETE SET NULL,
  listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
  version_id UUID REFERENCES marketplace_versions(id) ON DELETE SET NULL,
  actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
  action TEXT NOT NULL,
  previous_status TEXT,
  next_status TEXT NOT NULL,
  internal_comment TEXT,
  creator_message TEXT,
  reason TEXT NOT NULL,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT marketplace_review_events_action_supported CHECK (
    action IN (
      'approve',
      'reject',
      'request_changes',
      'suspend_listing',
      'unpublish_version',
      'emergency_block'
    )
  ),
  CONSTRAINT marketplace_review_events_reason_not_empty CHECK (length(trim(reason)) > 0),
  CONSTRAINT marketplace_review_events_metadata_object CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_marketplace_review_events_submission
  ON marketplace_review_events(submission_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_review_events_listing
  ON marketplace_review_events(listing_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_review_events_action
  ON marketplace_review_events(action, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_marketplace_review_events_actor
  ON marketplace_review_events(actor_id, created_at DESC);
