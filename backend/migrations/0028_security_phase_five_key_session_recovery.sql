ALTER TABLE refresh_token_families
  ADD COLUMN public_id UUID NOT NULL DEFAULT gen_random_uuid(),
  ADD COLUMN last_used_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  ADD COLUMN revocation_reason TEXT;

ALTER TABLE refresh_token_families
  ADD CONSTRAINT refresh_token_families_public_id_unique UNIQUE (public_id),
  ADD CONSTRAINT refresh_token_families_revocation_reason_length
    CHECK (revocation_reason IS NULL OR length(revocation_reason) BETWEEN 1 AND 64);

UPDATE refresh_token_families
SET last_used_at = created_at;

CREATE INDEX idx_refresh_token_families_user_inventory
  ON refresh_token_families(user_id, created_at DESC, public_id DESC);

CREATE INDEX idx_refresh_token_families_revoked_cleanup
  ON refresh_token_families(revoked_at, id)
  WHERE revoked_at IS NOT NULL;

CREATE INDEX idx_refresh_token_families_compromised_cleanup
  ON refresh_token_families(compromised_at, id)
  WHERE compromised_at IS NOT NULL;

ALTER TABLE organization_invitations
  ALTER COLUMN token_hash DROP NOT NULL;

UPDATE organization_invitations
SET token_hash = NULL
WHERE status <> 'pending'::organization_invitation_status;

UPDATE email_deliveries
SET payload = jsonb_build_object(
      'sensitive_delivery', true,
      'historical_payload_redacted', true
    ),
    updated_at = now()
WHERE template = 'organization_invitation';

CREATE INDEX idx_organization_invitations_expiry_cleanup
  ON organization_invitations(expires_at, id)
  WHERE token_hash IS NOT NULL;

CREATE TABLE security_tokens (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  purpose TEXT NOT NULL,
  token_hash TEXT NOT NULL UNIQUE,
  binding_hash TEXT,
  expires_at TIMESTAMPTZ NOT NULL,
  consumed_at TIMESTAMPTZ,
  revoked_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT security_tokens_known_purpose
    CHECK (purpose IN ('password_reset', 'email_verification', 'email_change')),
  CONSTRAINT security_tokens_hash_length
    CHECK (length(token_hash) BETWEEN 43 AND 128),
  CONSTRAINT security_tokens_binding_hash_length
    CHECK (binding_hash IS NULL OR length(binding_hash) BETWEEN 43 AND 128),
  CONSTRAINT security_tokens_expiry_after_creation
    CHECK (expires_at > created_at),
  CONSTRAINT security_tokens_one_final_state
    CHECK (consumed_at IS NULL OR revoked_at IS NULL)
);

CREATE INDEX idx_security_tokens_user_purpose
  ON security_tokens(user_id, purpose, created_at DESC);

CREATE INDEX idx_security_tokens_cleanup
  ON security_tokens(expires_at, id);

CREATE INDEX idx_security_tokens_consumed_cleanup
  ON security_tokens(consumed_at, id)
  WHERE consumed_at IS NOT NULL;

CREATE INDEX idx_security_tokens_revoked_cleanup
  ON security_tokens(revoked_at, id)
  WHERE revoked_at IS NOT NULL;

CREATE TABLE security_audit_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  event_type TEXT NOT NULL,
  actor_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  target_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT security_audit_events_type_length
    CHECK (length(event_type) BETWEEN 1 AND 96),
  CONSTRAINT security_audit_events_metadata_object
    CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE INDEX idx_security_audit_events_created
  ON security_audit_events(created_at, id);

CREATE INDEX idx_security_audit_events_target
  ON security_audit_events(target_user_id, created_at DESC);

CREATE INDEX idx_login_attempts_cleanup
  ON login_attempts(attempted_at, id);
