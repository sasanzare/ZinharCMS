-- ZinharCMS sample data
-- Safe to run multiple times. It upserts deterministic sample records.

BEGIN;
SELECT set_config('zinhar.organization_id', app_default_organization_id()::text, true);
SELECT set_config(
  'zinhar.user_id',
  COALESCE((SELECT id::text FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1), ''),
  true
);
SELECT set_config('zinhar.rls_bypass', 'false', true);

INSERT INTO content_types (id, name, slug, fields, created_by)
VALUES (
  '11111111-1111-4111-8111-111111111111'::uuid,
  'Article',
  'article',
  '{
    "fields": [
      {"id":"title","name":"title","label":"Title","type":"text","required":true,"min_length":3,"max_length":120},
      {"id":"slug","name":"slug","label":"Slug","type":"slug","required":true},
      {"id":"excerpt","name":"excerpt","label":"Excerpt","type":"longtext","required":false,"max_length":280},
      {"id":"body","name":"body","label":"Body","type":"richtext","required":true},
      {"id":"featured","name":"featured","label":"Featured","type":"boolean","required":false},
      {"id":"reading_time","name":"reading_time","label":"Reading time","type":"number","required":false,"min":1,"max":60}
    ]
  }'::jsonb,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1)
)
ON CONFLICT (organization_id, slug) DO UPDATE
SET name = EXCLUDED.name,
    fields = EXCLUDED.fields,
    updated_at = now();

INSERT INTO content_types (id, name, slug, fields, created_by)
VALUES (
  '22222222-2222-4222-8222-222222222222'::uuid,
  'Product',
  'product',
  '{
    "fields": [
      {"id":"name","name":"name","label":"Name","type":"text","required":true,"min_length":2,"max_length":120},
      {"id":"slug","name":"slug","label":"Slug","type":"slug","required":true},
      {"id":"summary","name":"summary","label":"Summary","type":"longtext","required":false,"max_length":280},
      {"id":"price","name":"price","label":"Price","type":"number","required":true,"min":0},
      {"id":"active","name":"active","label":"Active","type":"boolean","required":false},
      {"id":"details","name":"details","label":"Details","type":"richtext","required":false}
    ]
  }'::jsonb,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1)
)
ON CONFLICT (organization_id, slug) DO UPDATE
SET name = EXCLUDED.name,
    fields = EXCLUDED.fields,
    updated_at = now();

INSERT INTO media (id, filename, url, mime_type, size, alt_text, caption, uploader_id)
VALUES (
  '33333333-3333-4333-8333-333333333333'::uuid,
  'sample-dashboard.png',
  '/uploads/sample-dashboard.png',
  'image/png',
  248128,
  'ZinharCMS dashboard sample image',
  'A sample image record for demo entries and pages.',
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1)
)
ON CONFLICT (id) DO UPDATE
SET filename = EXCLUDED.filename,
    url = EXCLUDED.url,
    mime_type = EXCLUDED.mime_type,
    size = EXCLUDED.size,
    alt_text = EXCLUDED.alt_text,
    caption = EXCLUDED.caption,
    updated_at = now();

INSERT INTO content_entries (id, type_id, data, status, version, author_id, published_at)
VALUES (
  '44444444-4444-4444-8444-444444444444'::uuid,
  (SELECT id FROM content_types WHERE slug = 'article' AND organization_id = app_default_organization_id()),
  '{
    "title": "Getting started with ZinharCMS",
    "slug": "getting-started-with-zinharcms",
    "excerpt": "A practical overview of creating content models, entries, pages, and delivery API output.",
    "body": "<p>ZinharCMS lets editors create structured content, build pages, and publish API-ready data from one admin panel.</p><p>This sample article is published and visible through the Delivery API.</p>",
    "featured": true,
    "reading_time": 6
  }'::jsonb,
  'published'::content_status,
  1,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1),
  now()
)
ON CONFLICT (id) DO UPDATE
SET type_id = EXCLUDED.type_id,
    data = EXCLUDED.data,
    status = EXCLUDED.status,
    version = content_entries.version + 1,
    author_id = EXCLUDED.author_id,
    published_at = EXCLUDED.published_at,
    updated_at = now();

INSERT INTO content_entries (id, type_id, data, status, version, author_id, published_at)
VALUES (
  '55555555-5555-4555-8555-555555555555'::uuid,
  (SELECT id FROM content_types WHERE slug = 'article' AND organization_id = app_default_organization_id()),
  '{
    "title": "Editorial workflow sample",
    "slug": "editorial-workflow-sample",
    "excerpt": "A draft article used to test review queues and collaboration comments.",
    "body": "<p>This draft demonstrates how pending content can be reviewed before publication.</p>",
    "featured": false,
    "reading_time": 4
  }'::jsonb,
  'draft'::content_status,
  1,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1),
  NULL
)
ON CONFLICT (id) DO UPDATE
SET type_id = EXCLUDED.type_id,
    data = EXCLUDED.data,
    status = EXCLUDED.status,
    version = content_entries.version + 1,
    author_id = EXCLUDED.author_id,
    published_at = NULL,
    updated_at = now();

INSERT INTO content_entries (id, type_id, data, status, version, author_id, published_at)
VALUES (
  '66666666-6666-4666-8666-666666666666'::uuid,
  (SELECT id FROM content_types WHERE slug = 'product' AND organization_id = app_default_organization_id()),
  '{
    "name": "Starter CMS Plan",
    "slug": "starter-cms-plan",
    "summary": "A sample product entry for testing product-like content.",
    "price": 29,
    "active": true,
    "details": "<p>Includes content modeling, page publishing, media management, and delivery API access.</p>"
  }'::jsonb,
  'published'::content_status,
  1,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1),
  now()
)
ON CONFLICT (id) DO UPDATE
SET type_id = EXCLUDED.type_id,
    data = EXCLUDED.data,
    status = EXCLUDED.status,
    version = content_entries.version + 1,
    author_id = EXCLUDED.author_id,
    published_at = EXCLUDED.published_at,
    updated_at = now();

INSERT INTO pages (id, title, slug, page_json, status, author_id, published_at)
VALUES (
  '77777777-7777-4777-8777-777777777777'::uuid,
  'Home',
  'home',
  '{
    "version": "1.0",
    "metadata": {"title": "ZinharCMS Home", "description": "Sample home page built with the visual page builder."},
    "layout": {
      "id": "root",
      "type": "root",
      "children": [
        {"id":"hero-sample","type":"hero-banner","props":{"title":"Build structured content faster","subtitle":"A sample ZinharCMS page with reusable components and published API output.","cta_text":"Explore articles","cta_url":"/articles","alignment":"center"},"styles":{},"children":[]},
        {"id":"features-sample","type":"feature-grid","props":{"columns":3,"features":[{"title":"Content types","body":"Model reusable structured content."},{"title":"Page builder","body":"Compose pages from registered blocks."},{"title":"Delivery API","body":"Serve published content to any frontend."}]},"styles":{},"children":[]},
        {"id":"cta-sample","type":"cta-section","props":{"title":"Ready to publish?","button_text":"Open dashboard","button_url":"/"},"styles":{},"children":[]}
      ]
    }
  }'::jsonb,
  'published'::page_status,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1),
  now()
)
ON CONFLICT (organization_id, slug) DO UPDATE
SET title = EXCLUDED.title,
    page_json = EXCLUDED.page_json,
    status = EXCLUDED.status,
    author_id = EXCLUDED.author_id,
    published_at = EXCLUDED.published_at,
    updated_at = now();

INSERT INTO pages (id, title, slug, page_json, status, author_id, published_at)
VALUES (
  '88888888-8888-4888-8888-888888888888'::uuid,
  'About',
  'about',
  '{
    "version": "1.0",
    "metadata": {"title": "About ZinharCMS", "description": "Sample about page in draft state."},
    "layout": {
      "id": "root",
      "type": "root",
      "children": [
        {"id":"about-copy","type":"about-section","props":{"title":"About this demo","body":"<p>This draft page is included to test editing, comments, versioning, and workflow transitions.</p>"},"styles":{},"children":[]}
      ]
    }
  }'::jsonb,
  'draft'::page_status,
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1),
  NULL
)
ON CONFLICT (organization_id, slug) DO UPDATE
SET title = EXCLUDED.title,
    page_json = EXCLUDED.page_json,
    status = EXCLUDED.status,
    author_id = EXCLUDED.author_id,
    published_at = NULL,
    updated_at = now();

INSERT INTO page_versions (page_id, version, page_json, created_by)
SELECT id, 1, page_json, author_id FROM pages WHERE slug = 'home' AND organization_id = app_default_organization_id()
ON CONFLICT (page_id, version) DO UPDATE
SET page_json = EXCLUDED.page_json,
    created_by = EXCLUDED.created_by,
    snapshot_at = now();

INSERT INTO page_versions (page_id, version, page_json, created_by)
SELECT id, 1, page_json, author_id FROM pages WHERE slug = 'about' AND organization_id = app_default_organization_id()
ON CONFLICT (page_id, version) DO UPDATE
SET page_json = EXCLUDED.page_json,
    created_by = EXCLUDED.created_by,
    snapshot_at = now();

INSERT INTO comments (id, entity_type, entity_id, body, author_id)
VALUES (
  '99999999-9999-4999-8999-999999999999'::uuid,
  'entry',
  '55555555-5555-4555-8555-555555555555'::uuid,
  'Please review the headline and add one internal link before publishing.',
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1)
)
ON CONFLICT (id) DO UPDATE
SET body = EXCLUDED.body,
    author_id = EXCLUDED.author_id,
    resolved_at = NULL,
    resolved_by = NULL,
    updated_at = now();

INSERT INTO comments (id, entity_type, entity_id, body, author_id)
VALUES (
  'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa'::uuid,
  'page',
  (SELECT id FROM pages WHERE slug = 'about' AND organization_id = app_default_organization_id()),
  'Draft sample: confirm the about section copy before moving to review.',
  (SELECT id FROM users WHERE email = 'admin@example.com' ORDER BY created_at ASC LIMIT 1)
)
ON CONFLICT (id) DO UPDATE
SET entity_id = EXCLUDED.entity_id,
    body = EXCLUDED.body,
    author_id = EXCLUDED.author_id,
    resolved_at = NULL,
    resolved_by = NULL,
    updated_at = now();

INSERT INTO public_settings (key, value, is_public)
VALUES
  ('site_name', '"ZinharCMS Demo"'::jsonb, true),
  ('site_url', '"http://localhost:5173"'::jsonb, true),
  ('default_locale', '"fa"'::jsonb, true),
  ('support_email', '"support@example.com"'::jsonb, true),
  ('social_links', '{"github":"https://github.com/example/zinharcms","docs":"/docs"}'::jsonb, true)
ON CONFLICT (organization_id, key) DO UPDATE
SET value = EXCLUDED.value,
    is_public = EXCLUDED.is_public,
    updated_at = now();

DELETE FROM navigation_items
WHERE organization_id = app_default_organization_id()
  AND (
    label IN ('Home', 'Articles', 'Products', 'About', 'Content')
    OR id IN (
      'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb'::uuid,
      'cccccccc-cccc-4ccc-8ccc-cccccccccccc'::uuid,
      'dddddddd-dddd-4ddd-8ddd-dddddddddddd'::uuid,
      'eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee'::uuid
    )
  );

INSERT INTO navigation_items (id, label, url, position, locale, is_public)
VALUES
  ('bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb'::uuid, 'Home', '/', 10, 'fa', true),
  ('cccccccc-cccc-4ccc-8ccc-cccccccccccc'::uuid, 'Articles', '/articles', 20, 'fa', true),
  ('dddddddd-dddd-4ddd-8ddd-dddddddddddd'::uuid, 'Products', '/products', 30, 'fa', true),
  ('eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee'::uuid, 'About', '/about', 40, 'fa', true)
ON CONFLICT (id) DO UPDATE
SET label = EXCLUDED.label,
    url = EXCLUDED.url,
    position = EXCLUDED.position,
    locale = EXCLUDED.locale,
    is_public = EXCLUDED.is_public,
    updated_at = now();

COMMIT;

SELECT
  (SELECT count(*) FROM content_types WHERE organization_id = app_default_organization_id() AND slug IN ('article', 'product')) AS sample_content_types,
  (SELECT count(*) FROM content_entries WHERE id IN (
    '44444444-4444-4444-8444-444444444444'::uuid,
    '55555555-5555-4555-8555-555555555555'::uuid,
    '66666666-6666-4666-8666-666666666666'::uuid
  )) AS sample_entries,
  (SELECT count(*) FROM pages WHERE organization_id = app_default_organization_id() AND slug IN ('home', 'about')) AS sample_pages,
  (SELECT count(*) FROM comments WHERE id IN (
    '99999999-9999-4999-8999-999999999999'::uuid,
    'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa'::uuid
  )) AS sample_comments,
  (SELECT count(*) FROM navigation_items WHERE organization_id = app_default_organization_id() AND id IN (
    'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb'::uuid,
    'cccccccc-cccc-4ccc-8ccc-cccccccccccc'::uuid,
    'dddddddd-dddd-4ddd-8ddd-dddddddddddd'::uuid,
    'eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee'::uuid
  )) AS sample_navigation_items;