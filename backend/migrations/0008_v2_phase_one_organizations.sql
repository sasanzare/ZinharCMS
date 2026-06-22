DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'organization_status') THEN
    CREATE TYPE organization_status AS ENUM ('active', 'suspended', 'deleted');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'organization_member_role') THEN
    CREATE TYPE organization_member_role AS ENUM (
      'owner',
      'admin',
      'editor',
      'author',
      'viewer',
      'billing_manager'
    );
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'organization_member_status') THEN
    CREATE TYPE organization_member_status AS ENUM ('active', 'invited', 'suspended');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'organization_invitation_status') THEN
    CREATE TYPE organization_invitation_status AS ENUM ('pending', 'accepted', 'revoked', 'expired');
  END IF;
END $$;

CREATE TABLE IF NOT EXISTS organizations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  status organization_status NOT NULL DEFAULT 'active',
  owner_id UUID REFERENCES users(id) ON DELETE SET NULL,
  settings JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT organizations_name_not_empty CHECK (length(trim(name)) > 0),
  CONSTRAINT organizations_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
  CONSTRAINT organizations_settings_object CHECK (jsonb_typeof(settings) = 'object')
);

INSERT INTO organizations (name, slug, status)
VALUES ('Default Organization', 'default', 'active')
ON CONFLICT (slug) DO NOTHING;

CREATE OR REPLACE FUNCTION app_default_organization_id()
RETURNS UUID
LANGUAGE sql
STABLE
AS $$
  SELECT id
  FROM organizations
  WHERE slug = 'default'
  LIMIT 1
$$;

CREATE TABLE IF NOT EXISTS organization_members (
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role organization_member_role NOT NULL DEFAULT 'viewer',
  status organization_member_status NOT NULL DEFAULT 'active',
  invited_by UUID REFERENCES users(id) ON DELETE SET NULL,
  joined_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (organization_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_organization_members_user
  ON organization_members(user_id, organization_id);

CREATE INDEX IF NOT EXISTS idx_organization_members_org_role
  ON organization_members(organization_id, role);

CREATE TABLE IF NOT EXISTS organization_invitations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  email CITEXT NOT NULL,
  role organization_member_role NOT NULL DEFAULT 'viewer',
  token_hash TEXT NOT NULL UNIQUE,
  invited_by UUID REFERENCES users(id) ON DELETE SET NULL,
  status organization_invitation_status NOT NULL DEFAULT 'pending',
  expires_at TIMESTAMPTZ NOT NULL,
  accepted_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT organization_invitations_email_not_empty CHECK (length(trim(email::text)) > 0)
);

CREATE INDEX IF NOT EXISTS idx_organization_invitations_org
  ON organization_invitations(organization_id, created_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS idx_organization_invitations_pending_email
  ON organization_invitations(organization_id, email)
  WHERE status = 'pending';

WITH default_org AS (
  SELECT id FROM organizations WHERE slug = 'default'
),
best_user_roles AS (
  SELECT
    u.id AS user_id,
    COALESCE(
      (
        ARRAY_AGG(
          r.name
          ORDER BY CASE r.name
            WHEN 'super_admin' THEN 1
            WHEN 'admin' THEN 2
            WHEN 'editor' THEN 3
            WHEN 'author' THEN 4
            WHEN 'viewer' THEN 5
            ELSE 99
          END
        ) FILTER (WHERE r.name IS NOT NULL)
      )[1],
      'author'
    ) AS global_role
  FROM users u
  LEFT JOIN user_roles ur ON ur.user_id = u.id
  LEFT JOIN roles r ON r.id = ur.role_id
  GROUP BY u.id
)
INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
SELECT
  default_org.id,
  best_user_roles.user_id,
  CASE best_user_roles.global_role
    WHEN 'super_admin' THEN 'owner'::organization_member_role
    WHEN 'admin' THEN 'admin'::organization_member_role
    WHEN 'editor' THEN 'editor'::organization_member_role
    WHEN 'viewer' THEN 'viewer'::organization_member_role
    ELSE 'author'::organization_member_role
  END,
  'active'::organization_member_status,
  now()
FROM default_org
CROSS JOIN best_user_roles
ON CONFLICT (organization_id, user_id) DO UPDATE
SET role = EXCLUDED.role,
    status = 'active'::organization_member_status,
    updated_at = now();

WITH owner_candidate AS (
  SELECT user_id
  FROM organization_members
  WHERE organization_id = app_default_organization_id()
  ORDER BY CASE role
    WHEN 'owner' THEN 1
    WHEN 'admin' THEN 2
    ELSE 99
  END, created_at ASC
  LIMIT 1
)
UPDATE organizations
SET owner_id = owner_candidate.user_id,
    updated_at = now()
FROM owner_candidate
WHERE organizations.slug = 'default'
  AND organizations.owner_id IS NULL;

ALTER TABLE content_types
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE content_types
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE content_types
  ALTER COLUMN organization_id SET DEFAULT app_default_organization_id(),
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT content_types_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

ALTER TABLE content_types
  DROP CONSTRAINT IF EXISTS content_types_slug_key;

ALTER TABLE content_types
  ADD CONSTRAINT content_types_organization_slug_unique UNIQUE (organization_id, slug);

CREATE INDEX IF NOT EXISTS idx_content_types_organization_created
  ON content_types(organization_id, created_at DESC);

ALTER TABLE content_entries
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE content_entries AS entry
SET organization_id = content_types.organization_id
FROM content_types
WHERE entry.type_id = content_types.id
  AND entry.organization_id IS NULL;

UPDATE content_entries
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE content_entries
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT content_entries_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_content_entries_org_type_status
  ON content_entries(organization_id, type_id, status);

CREATE INDEX IF NOT EXISTS idx_content_entries_org_updated
  ON content_entries(organization_id, updated_at DESC);

CREATE OR REPLACE FUNCTION set_content_entry_organization_id()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  IF NEW.organization_id IS NULL THEN
    SELECT organization_id
    INTO NEW.organization_id
    FROM content_types
    WHERE id = NEW.type_id;
  ELSIF TG_OP = 'UPDATE' THEN
    IF NEW.type_id IS DISTINCT FROM OLD.type_id THEN
      SELECT organization_id
      INTO NEW.organization_id
      FROM content_types
      WHERE id = NEW.type_id;
    END IF;
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_content_entries_set_organization_id ON content_entries;
CREATE TRIGGER trg_content_entries_set_organization_id
BEFORE INSERT OR UPDATE OF type_id, organization_id ON content_entries
FOR EACH ROW
EXECUTE FUNCTION set_content_entry_organization_id();

ALTER TABLE pages
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE pages
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE pages
  ALTER COLUMN organization_id SET DEFAULT app_default_organization_id(),
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT pages_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

ALTER TABLE pages
  DROP CONSTRAINT IF EXISTS pages_slug_key;

ALTER TABLE pages
  ADD CONSTRAINT pages_organization_slug_unique UNIQUE (organization_id, slug);

CREATE INDEX IF NOT EXISTS idx_pages_org_status
  ON pages(organization_id, status);

CREATE INDEX IF NOT EXISTS idx_pages_org_updated
  ON pages(organization_id, updated_at DESC);

ALTER TABLE page_versions
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE page_versions AS version
SET organization_id = pages.organization_id
FROM pages
WHERE version.page_id = pages.id
  AND version.organization_id IS NULL;

UPDATE page_versions
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE page_versions
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT page_versions_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_page_versions_org_page
  ON page_versions(organization_id, page_id, version DESC);

CREATE OR REPLACE FUNCTION set_page_version_organization_id()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  IF NEW.organization_id IS NULL THEN
    SELECT organization_id
    INTO NEW.organization_id
    FROM pages
    WHERE id = NEW.page_id;
  ELSIF TG_OP = 'UPDATE' THEN
    IF NEW.page_id IS DISTINCT FROM OLD.page_id THEN
      SELECT organization_id
      INTO NEW.organization_id
      FROM pages
      WHERE id = NEW.page_id;
    END IF;
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_page_versions_set_organization_id ON page_versions;
CREATE TRIGGER trg_page_versions_set_organization_id
BEFORE INSERT OR UPDATE OF page_id, organization_id ON page_versions
FOR EACH ROW
EXECUTE FUNCTION set_page_version_organization_id();

ALTER TABLE component_registry
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE component_registry
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL
  AND is_system = FALSE;

ALTER TABLE component_registry
  ADD CONSTRAINT component_registry_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE,
  ADD CONSTRAINT component_registry_custom_org_required
    CHECK (is_system = TRUE OR organization_id IS NOT NULL);

CREATE INDEX IF NOT EXISTS idx_component_registry_organization_key
  ON component_registry(organization_id, component_key);

ALTER TABLE media
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE media
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE media
  ALTER COLUMN organization_id SET DEFAULT app_default_organization_id(),
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT media_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_media_org_created
  ON media(organization_id, created_at DESC);

ALTER TABLE media_variants
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE media_variants AS variant
SET organization_id = media.organization_id
FROM media
WHERE variant.media_id = media.id
  AND variant.organization_id IS NULL;

UPDATE media_variants
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE media_variants
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT media_variants_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_media_variants_org_media
  ON media_variants(organization_id, media_id);

CREATE OR REPLACE FUNCTION set_media_variant_organization_id()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  IF NEW.organization_id IS NULL THEN
    SELECT organization_id
    INTO NEW.organization_id
    FROM media
    WHERE id = NEW.media_id;
  ELSIF TG_OP = 'UPDATE' THEN
    IF NEW.media_id IS DISTINCT FROM OLD.media_id THEN
      SELECT organization_id
      INTO NEW.organization_id
      FROM media
      WHERE id = NEW.media_id;
    END IF;
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_media_variants_set_organization_id ON media_variants;
CREATE TRIGGER trg_media_variants_set_organization_id
BEFORE INSERT OR UPDATE OF media_id, organization_id ON media_variants
FOR EACH ROW
EXECUTE FUNCTION set_media_variant_organization_id();

ALTER TABLE comments
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE comments AS comment
SET organization_id = COALESCE(
  CASE
    WHEN comment.entity_type = 'entry' THEN (
      SELECT organization_id FROM content_entries WHERE id = comment.entity_id
    )
    WHEN comment.entity_type = 'page' THEN (
      SELECT organization_id FROM pages WHERE id = comment.entity_id
    )
  END,
  app_default_organization_id()
)
WHERE comment.organization_id IS NULL;

ALTER TABLE comments
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT comments_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_comments_org_entity
  ON comments(organization_id, entity_type, entity_id, created_at DESC);

CREATE OR REPLACE FUNCTION set_comment_organization_id()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  IF NEW.organization_id IS NULL THEN
    IF NEW.entity_type = 'entry' THEN
      SELECT organization_id
      INTO NEW.organization_id
      FROM content_entries
      WHERE id = NEW.entity_id;
    ELSIF NEW.entity_type = 'page' THEN
      SELECT organization_id
      INTO NEW.organization_id
      FROM pages
      WHERE id = NEW.entity_id;
    END IF;
  ELSIF TG_OP = 'UPDATE' THEN
    IF NEW.entity_type IS DISTINCT FROM OLD.entity_type
       OR NEW.entity_id IS DISTINCT FROM OLD.entity_id THEN
      IF NEW.entity_type = 'entry' THEN
        SELECT organization_id
        INTO NEW.organization_id
        FROM content_entries
        WHERE id = NEW.entity_id;
      ELSIF NEW.entity_type = 'page' THEN
        SELECT organization_id
        INTO NEW.organization_id
        FROM pages
        WHERE id = NEW.entity_id;
      END IF;
    END IF;
  END IF;

  IF NEW.organization_id IS NULL THEN
    NEW.organization_id := app_default_organization_id();
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_comments_set_organization_id ON comments;
CREATE TRIGGER trg_comments_set_organization_id
BEFORE INSERT OR UPDATE OF entity_type, entity_id, organization_id ON comments
FOR EACH ROW
EXECUTE FUNCTION set_comment_organization_id();

ALTER TABLE webhooks
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE webhooks
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE webhooks
  ALTER COLUMN organization_id SET DEFAULT app_default_organization_id(),
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT webhooks_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_webhooks_org_active
  ON webhooks(organization_id)
  WHERE is_active = TRUE;

CREATE INDEX IF NOT EXISTS idx_webhooks_org_created
  ON webhooks(organization_id, created_at DESC);

ALTER TABLE webhook_deliveries
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE webhook_deliveries AS delivery
SET organization_id = webhooks.organization_id
FROM webhooks
WHERE delivery.webhook_id = webhooks.id
  AND delivery.organization_id IS NULL;

UPDATE webhook_deliveries
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE webhook_deliveries
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT webhook_deliveries_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_webhook_deliveries_org_webhook_attempted
  ON webhook_deliveries(organization_id, webhook_id, attempted_at DESC);

CREATE OR REPLACE FUNCTION set_webhook_delivery_organization_id()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  IF NEW.organization_id IS NULL THEN
    SELECT organization_id
    INTO NEW.organization_id
    FROM webhooks
    WHERE id = NEW.webhook_id;
  ELSIF TG_OP = 'UPDATE' THEN
    IF NEW.webhook_id IS DISTINCT FROM OLD.webhook_id THEN
      SELECT organization_id
      INTO NEW.organization_id
      FROM webhooks
      WHERE id = NEW.webhook_id;
    END IF;
  END IF;

  RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_webhook_deliveries_set_organization_id ON webhook_deliveries;
CREATE TRIGGER trg_webhook_deliveries_set_organization_id
BEFORE INSERT OR UPDATE OF webhook_id, organization_id ON webhook_deliveries
FOR EACH ROW
EXECUTE FUNCTION set_webhook_delivery_organization_id();

ALTER TABLE public_settings
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE public_settings
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE public_settings
  ALTER COLUMN organization_id SET DEFAULT app_default_organization_id(),
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT public_settings_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

ALTER TABLE public_settings
  DROP CONSTRAINT IF EXISTS public_settings_pkey;

ALTER TABLE public_settings
  ADD CONSTRAINT public_settings_pkey PRIMARY KEY (organization_id, key);

ALTER TABLE navigation_items
  ADD COLUMN IF NOT EXISTS organization_id UUID;

UPDATE navigation_items
SET organization_id = app_default_organization_id()
WHERE organization_id IS NULL;

ALTER TABLE navigation_items
  ALTER COLUMN organization_id SET DEFAULT app_default_organization_id(),
  ALTER COLUMN organization_id SET NOT NULL,
  ADD CONSTRAINT navigation_items_organization_id_fkey
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_navigation_items_org_public_locale
  ON navigation_items(organization_id, locale, position)
  WHERE is_public = TRUE;
