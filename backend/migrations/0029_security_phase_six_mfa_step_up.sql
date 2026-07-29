ALTER TABLE refresh_token_families
  ADD COLUMN assurance_level SMALLINT NOT NULL DEFAULT 1,
  ADD COLUMN authentication_methods TEXT[] NOT NULL DEFAULT ARRAY['pwd']::TEXT[],
  ADD COLUMN authenticated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  ADD COLUMN mfa_authenticated_at TIMESTAMPTZ,
  ADD COLUMN auth_version_at_issue BIGINT NOT NULL DEFAULT 1;

UPDATE refresh_token_families family
SET authenticated_at = family.created_at,
    auth_version_at_issue = users.auth_version
FROM users
WHERE users.id = family.user_id;

ALTER TABLE refresh_token_families
  ADD CONSTRAINT refresh_token_families_assurance_level_valid
    CHECK (assurance_level IN (1, 2)),
  ADD CONSTRAINT refresh_token_families_auth_version_positive
    CHECK (auth_version_at_issue > 0),
  ADD CONSTRAINT refresh_token_families_authentication_methods_valid
    CHECK (
      CASE assurance_level
        WHEN 1 THEN
          authentication_methods = ARRAY['pwd']::TEXT[]
          AND mfa_authenticated_at IS NULL
        WHEN 2 THEN
          cardinality(authentication_methods) = 2
          AND authentication_methods @> ARRAY['pwd']::TEXT[]
          AND (
            authentication_methods @> ARRAY['totp']::TEXT[]
            OR authentication_methods @> ARRAY['recovery']::TEXT[]
          )
          AND authentication_methods <@ ARRAY['pwd', 'totp', 'recovery']::TEXT[]
          AND mfa_authenticated_at IS NOT NULL
          AND mfa_authenticated_at >= authenticated_at
      END
    );

CREATE TABLE user_mfa (
  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  status TEXT NOT NULL,
  enrollment_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
  secret_ciphertext BYTEA NOT NULL,
  secret_nonce BYTEA NOT NULL,
  encryption_kid TEXT NOT NULL,
  encryption_version SMALLINT NOT NULL,
  pending_expires_at TIMESTAMPTZ,
  enabled_at TIMESTAMPTZ,
  last_accepted_totp_step BIGINT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT user_mfa_status_valid
    CHECK (status IN ('pending', 'enabled')),
  CONSTRAINT user_mfa_ciphertext_length
    CHECK (octet_length(secret_ciphertext) = 36),
  CONSTRAINT user_mfa_nonce_length
    CHECK (octet_length(secret_nonce) = 12),
  CONSTRAINT user_mfa_encryption_kid_length
    CHECK (length(encryption_kid) BETWEEN 1 AND 64),
  CONSTRAINT user_mfa_encryption_version_valid
    CHECK (encryption_version = 1),
  CONSTRAINT user_mfa_state_valid
    CHECK (
      (
        status = 'pending'
        AND pending_expires_at IS NOT NULL
        AND pending_expires_at > created_at
        AND enabled_at IS NULL
      )
      OR (
        status = 'enabled'
        AND pending_expires_at IS NULL
        AND enabled_at IS NOT NULL
      )
    )
);

CREATE INDEX idx_user_mfa_pending_expiry
  ON user_mfa(pending_expires_at, user_id)
  WHERE status = 'pending';

CREATE TABLE mfa_recovery_codes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  code_position SMALLINT NOT NULL,
  lookup_hash TEXT NOT NULL UNIQUE,
  verifier_hash TEXT NOT NULL,
  used_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT mfa_recovery_codes_position_valid
    CHECK (code_position BETWEEN 1 AND 10),
  CONSTRAINT mfa_recovery_codes_lookup_hash_length
    CHECK (length(lookup_hash) = 43),
  CONSTRAINT mfa_recovery_codes_verifier_hash_length
    CHECK (length(verifier_hash) BETWEEN 32 AND 255),
  CONSTRAINT mfa_recovery_codes_user_position_unique
    UNIQUE (user_id, code_position)
);

CREATE INDEX idx_mfa_recovery_codes_user_unused
  ON mfa_recovery_codes(user_id, code_position)
  WHERE used_at IS NULL;

CREATE OR REPLACE FUNCTION app_bump_user_auth_version_on_mfa_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
  affected_user_id UUID;
  old_enabled BOOLEAN := TG_OP <> 'INSERT' AND OLD.status = 'enabled';
  new_enabled BOOLEAN := TG_OP <> 'DELETE' AND NEW.status = 'enabled';
BEGIN
  IF old_enabled IS DISTINCT FROM new_enabled THEN
    affected_user_id := CASE WHEN TG_OP = 'DELETE' THEN OLD.user_id ELSE NEW.user_id END;
    UPDATE users
    SET auth_version = auth_version + 1,
        updated_at = now()
    WHERE id = affected_user_id;
  END IF;
  RETURN NULL;
END
$$;

CREATE TRIGGER user_mfa_auth_version
AFTER INSERT OR UPDATE OR DELETE ON user_mfa
FOR EACH ROW
EXECUTE FUNCTION app_bump_user_auth_version_on_mfa_change();
