ALTER TABLE media
  ADD COLUMN IF NOT EXISTS caption TEXT;

INSERT INTO roles (name, permissions)
VALUES
  ('super_admin', ARRAY['*']),
  ('author', ARRAY[
    'content_types:read',
    'entries:create',
    'entries:read',
    'entries:update_own',
    'entries:submit_review',
    'pages:read',
    'media:create',
    'media:read',
    'media:update_own'
  ])
ON CONFLICT (name) DO UPDATE
SET permissions = EXCLUDED.permissions,
    updated_at = now();

INSERT INTO roles (name, permissions)
VALUES
  ('admin', ARRAY[
    'content_types:create',
    'content_types:read',
    'content_types:update',
    'content_types:delete',
    'entries:create',
    'entries:read',
    'entries:update',
    'entries:delete',
    'entries:publish',
    'media:create',
    'media:read',
    'media:update',
    'media:delete',
    'users:read',
    'users:update'
  ]),
  ('editor', ARRAY[
    'content_types:read',
    'entries:create',
    'entries:read',
    'entries:update',
    'entries:delete',
    'entries:publish',
    'pages:create',
    'pages:read',
    'pages:update',
    'pages:publish',
    'media:create',
    'media:read',
    'media:update'
  ]),
  ('viewer', ARRAY[
    'content_types:read',
    'entries:read',
    'pages:read',
    'media:read'
  ])
ON CONFLICT (name) DO UPDATE
SET permissions = EXCLUDED.permissions,
    updated_at = now();
