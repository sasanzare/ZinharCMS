ALTER TABLE users
  ADD COLUMN auth_version BIGINT NOT NULL DEFAULT 1;

ALTER TABLE users
  ADD CONSTRAINT users_auth_version_positive CHECK (auth_version > 0);

CREATE TABLE refresh_token_families (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  expires_at TIMESTAMPTZ NOT NULL,
  revoked_at TIMESTAMPTZ,
  compromised_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT refresh_token_families_expiry_after_creation
    CHECK (expires_at > created_at)
);

ALTER TABLE refresh_tokens
  ADD COLUMN family_id UUID,
  ADD COLUMN predecessor_token_id UUID,
  ADD COLUMN successor_token_id UUID,
  ADD COLUMN rotated_at TIMESTAMPTZ;

INSERT INTO refresh_token_families (
  id,
  user_id,
  expires_at,
  revoked_at,
  created_at
)
SELECT
  id,
  user_id,
  GREATEST(expires_at, created_at + INTERVAL '1 microsecond'),
  now(),
  created_at
FROM refresh_tokens;

UPDATE refresh_tokens
SET family_id = id,
    revoked_at = COALESCE(revoked_at, now());

ALTER TABLE refresh_tokens
  ALTER COLUMN family_id SET NOT NULL,
  ADD CONSTRAINT refresh_tokens_family_id_fkey
    FOREIGN KEY (family_id)
    REFERENCES refresh_token_families(id)
    ON DELETE CASCADE,
  ADD CONSTRAINT refresh_tokens_predecessor_token_id_fkey
    FOREIGN KEY (predecessor_token_id)
    REFERENCES refresh_tokens(id)
    ON DELETE SET NULL,
  ADD CONSTRAINT refresh_tokens_successor_token_id_fkey
    FOREIGN KEY (successor_token_id)
    REFERENCES refresh_tokens(id)
    ON DELETE SET NULL,
  ADD CONSTRAINT refresh_tokens_predecessor_not_self
    CHECK (predecessor_token_id IS NULL OR predecessor_token_id <> id),
  ADD CONSTRAINT refresh_tokens_successor_not_self
    CHECK (successor_token_id IS NULL OR successor_token_id <> id);

CREATE UNIQUE INDEX idx_refresh_tokens_predecessor_unique
  ON refresh_tokens(predecessor_token_id)
  WHERE predecessor_token_id IS NOT NULL;

CREATE UNIQUE INDEX idx_refresh_tokens_successor_unique
  ON refresh_tokens(successor_token_id)
  WHERE successor_token_id IS NOT NULL;

CREATE INDEX idx_refresh_tokens_family_id
  ON refresh_tokens(family_id);

CREATE INDEX idx_refresh_tokens_family_active
  ON refresh_tokens(family_id, expires_at)
  WHERE revoked_at IS NULL;

CREATE INDEX idx_refresh_token_families_user_id
  ON refresh_token_families(user_id);

CREATE INDEX idx_refresh_token_families_expiry
  ON refresh_token_families(expires_at);

CREATE OR REPLACE FUNCTION app_bump_user_auth_version_on_sensitive_update()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
  IF NEW.auth_version < OLD.auth_version THEN
    RAISE EXCEPTION 'auth_version cannot be decreased';
  END IF;

  IF NEW.is_active IS DISTINCT FROM OLD.is_active
     OR NEW.password_hash IS DISTINCT FROM OLD.password_hash
     OR NEW.email IS DISTINCT FROM OLD.email THEN
    NEW.auth_version := GREATEST(NEW.auth_version, OLD.auth_version + 1);
  END IF;

  RETURN NEW;
END
$$;

CREATE TRIGGER users_sensitive_auth_version
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION app_bump_user_auth_version_on_sensitive_update();

CREATE OR REPLACE FUNCTION app_bump_user_auth_version_on_role_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
  IF TG_OP = 'DELETE' THEN
    UPDATE users
    SET auth_version = auth_version + 1,
        updated_at = now()
    WHERE id = OLD.user_id;
  ELSIF TG_OP = 'INSERT' THEN
    UPDATE users
    SET auth_version = auth_version + 1,
        updated_at = now()
    WHERE id = NEW.user_id;
  ELSE
    UPDATE users
    SET auth_version = auth_version + 1,
        updated_at = now()
    WHERE id = OLD.user_id;

    IF NEW.user_id IS DISTINCT FROM OLD.user_id THEN
      UPDATE users
      SET auth_version = auth_version + 1,
          updated_at = now()
      WHERE id = NEW.user_id;
    END IF;
  END IF;

  RETURN NULL;
END
$$;

CREATE TRIGGER user_roles_auth_version
AFTER INSERT OR UPDATE OR DELETE ON user_roles
FOR EACH ROW
EXECUTE FUNCTION app_bump_user_auth_version_on_role_change();
