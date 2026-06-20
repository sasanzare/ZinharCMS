CREATE TABLE IF NOT EXISTS login_attempts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email CITEXT NOT NULL,
  ip_address TEXT NOT NULL,
  succeeded BOOLEAN NOT NULL DEFAULT FALSE,
  attempted_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_login_attempts_ip_attempted
  ON login_attempts(ip_address, attempted_at DESC);

CREATE INDEX IF NOT EXISTS idx_login_attempts_failed_window
  ON login_attempts(ip_address, attempted_at DESC)
  WHERE succeeded = FALSE;
