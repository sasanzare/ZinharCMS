ALTER TYPE content_status ADD VALUE IF NOT EXISTS 'pending_review';
ALTER TYPE page_status ADD VALUE IF NOT EXISTS 'pending_review';

CREATE TABLE IF NOT EXISTS comments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  entity_type TEXT NOT NULL,
  entity_id UUID NOT NULL,
  body TEXT NOT NULL,
  author_id UUID REFERENCES users(id) ON DELETE SET NULL,
  resolved_at TIMESTAMPTZ,
  resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT comments_entity_type CHECK (entity_type IN ('entry', 'page')),
  CONSTRAINT comments_body_not_empty CHECK (length(trim(body)) > 0)
);

CREATE INDEX IF NOT EXISTS idx_comments_entity
  ON comments(entity_type, entity_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_comments_unresolved
  ON comments(entity_type, entity_id)
  WHERE resolved_at IS NULL;

CREATE TABLE IF NOT EXISTS cms_plugins (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  plugin_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  hooks TEXT[] NOT NULL DEFAULT '{}',
  is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
  is_system BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT cms_plugins_key_format CHECK (plugin_key ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$')
);

INSERT INTO cms_plugins (plugin_key, name, version, description, hooks, is_enabled, is_system)
VALUES (
  'seo-auto',
  'SEO Auto Generator',
  '1.0.0',
  'Generates a data.slug value from title before entry save when slug is empty.',
  ARRAY['entry.before_save'],
  TRUE,
  TRUE
)
ON CONFLICT (plugin_key) DO UPDATE
SET name = EXCLUDED.name,
    version = EXCLUDED.version,
    description = EXCLUDED.description,
    hooks = EXCLUDED.hooks,
    is_system = TRUE,
    updated_at = now();

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'workflow:submit',
          'workflow:review',
          'workflow:archive',
          'comments:create',
          'comments:read',
          'comments:resolve',
          'comments:delete',
          'plugins:read',
          'plugins:update'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name IN ('admin', 'editor');

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'workflow:submit',
          'comments:create',
          'comments:read',
          'plugins:read'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name = 'author';

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'comments:read',
          'plugins:read'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name = 'viewer';
