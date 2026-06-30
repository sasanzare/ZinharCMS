-- Phase 8 hardening fixture for staging/local QA.
-- It creates two organizations with similar content so tenant-isolation tests can
-- attempt direct-ID access across organizations.

WITH orgs(id, name, slug) AS (
  VALUES
    ('10000000-0000-7000-8000-000000000001'::uuid, 'Phase 8 Alpha', 'phase8-alpha'),
    ('10000000-0000-7000-8000-000000000002'::uuid, 'Phase 8 Beta', 'phase8-beta')
),
upsert_orgs AS (
  INSERT INTO organizations (id, name, slug, status, settings)
  SELECT id, name, slug, 'active'::organization_status, '{"phase8_fixture": true}'::jsonb
  FROM orgs
  ON CONFLICT (slug) DO UPDATE
  SET name = EXCLUDED.name,
      status = 'active'::organization_status,
      settings = organizations.settings || EXCLUDED.settings,
      updated_at = now()
  RETURNING id, slug
),
users(id, email, name) AS (
  VALUES
    ('20000000-0000-7000-8000-000000000001'::uuid, 'phase8-owner@example.com', 'Phase 8 Owner'),
    ('20000000-0000-7000-8000-000000000002'::uuid, 'phase8-editor@example.com', 'Phase 8 Editor'),
    ('20000000-0000-7000-8000-000000000003'::uuid, 'phase8-viewer@example.com', 'Phase 8 Viewer'),
    ('20000000-0000-7000-8000-000000000004'::uuid, 'phase8-billing@example.com', 'Phase 8 Billing')
),
upsert_users AS (
  INSERT INTO users (id, email, password_hash, name)
  SELECT id, email, 'phase8-fixture-not-for-login', name
  FROM users
  ON CONFLICT (email) DO UPDATE
  SET name = EXCLUDED.name,
      updated_at = now()
  RETURNING id, email
),
memberships(organization_slug, email, role) AS (
  VALUES
    ('phase8-alpha', 'phase8-owner@example.com', 'owner'),
    ('phase8-alpha', 'phase8-editor@example.com', 'editor'),
    ('phase8-alpha', 'phase8-billing@example.com', 'billing_manager'),
    ('phase8-beta', 'phase8-owner@example.com', 'owner'),
    ('phase8-beta', 'phase8-viewer@example.com', 'viewer')
)
INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
SELECT org.id,
       u.id,
       memberships.role::organization_member_role,
       'active'::organization_member_status,
       now()
FROM memberships
JOIN organizations org ON org.slug = memberships.organization_slug
JOIN users u ON u.email = memberships.email
ON CONFLICT (organization_id, user_id) DO UPDATE
SET role = EXCLUDED.role,
    status = 'active'::organization_member_status,
    joined_at = COALESCE(organization_members.joined_at, now()),
    updated_at = now();

WITH article_types(id, organization_slug, name, slug) AS (
  VALUES
    ('30000000-0000-7000-8000-000000000001'::uuid, 'phase8-alpha', 'Article', 'article'),
    ('30000000-0000-7000-8000-000000000002'::uuid, 'phase8-beta', 'Article', 'article')
)
INSERT INTO content_types (id, organization_id, name, slug, fields, created_by)
SELECT article_types.id,
       org.id,
       article_types.name,
       article_types.slug,
       '{"fields":[{"name":"title","label":"Title","type":"text","required":true}]}'::jsonb,
       owner.id
FROM article_types
JOIN organizations org ON org.slug = article_types.organization_slug
JOIN users owner ON owner.email = 'phase8-owner@example.com'
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    fields = EXCLUDED.fields,
    updated_at = now();

INSERT INTO content_entries (id, organization_id, type_id, data, status, author_id)
VALUES
  (
    '40000000-0000-7000-8000-000000000001'::uuid,
    (SELECT id FROM organizations WHERE slug = 'phase8-alpha'),
    '30000000-0000-7000-8000-000000000001'::uuid,
    '{"title":"Alpha isolated entry"}'::jsonb,
    'draft'::content_status,
    (SELECT id FROM users WHERE email = 'phase8-editor@example.com')
  ),
  (
    '40000000-0000-7000-8000-000000000002'::uuid,
    (SELECT id FROM organizations WHERE slug = 'phase8-beta'),
    '30000000-0000-7000-8000-000000000002'::uuid,
    '{"title":"Beta isolated entry"}'::jsonb,
    'draft'::content_status,
    (SELECT id FROM users WHERE email = 'phase8-owner@example.com')
  )
ON CONFLICT (id) DO UPDATE
SET data = EXCLUDED.data,
    updated_at = now();

INSERT INTO pages (id, organization_id, title, slug, page_json, status, author_id)
VALUES
  (
    '50000000-0000-7000-8000-000000000001'::uuid,
    (SELECT id FROM organizations WHERE slug = 'phase8-alpha'),
    'Alpha Landing',
    'landing',
    '{"layout":{"id":"root","type":"section","props":{"title":"Alpha"}}}'::jsonb,
    'draft'::page_status,
    (SELECT id FROM users WHERE email = 'phase8-editor@example.com')
  ),
  (
    '50000000-0000-7000-8000-000000000002'::uuid,
    (SELECT id FROM organizations WHERE slug = 'phase8-beta'),
    'Beta Landing',
    'landing',
    '{"layout":{"id":"root","type":"section","props":{"title":"Beta"}}}'::jsonb,
    'draft'::page_status,
    (SELECT id FROM users WHERE email = 'phase8-owner@example.com')
  )
ON CONFLICT (id) DO UPDATE
SET title = EXCLUDED.title,
    page_json = EXCLUDED.page_json,
    updated_at = now();

INSERT INTO organization_rate_limits (organization_id, requests_per_minute, user_requests_per_minute, burst)
SELECT id, 120, 60, 30
FROM organizations
WHERE slug IN ('phase8-alpha', 'phase8-beta')
ON CONFLICT (organization_id) DO UPDATE
SET requests_per_minute = EXCLUDED.requests_per_minute,
    user_requests_per_minute = EXCLUDED.user_requests_per_minute,
    burst = EXCLUDED.burst,
    updated_at = now();
