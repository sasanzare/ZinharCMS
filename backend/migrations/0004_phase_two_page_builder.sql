ALTER TABLE component_registry
  ADD COLUMN IF NOT EXISTS component_key TEXT;

UPDATE component_registry
SET component_key = COALESCE(
  NULLIF(trim(both '-' from regexp_replace(lower(name), '[^a-z0-9]+', '-', 'g')), ''),
  id::text
)
WHERE component_key IS NULL;

ALTER TABLE component_registry
  ALTER COLUMN component_key SET NOT NULL;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'component_registry_key_unique'
  ) THEN
    ALTER TABLE component_registry
      ADD CONSTRAINT component_registry_key_unique UNIQUE (component_key);
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'component_registry_key_format'
  ) THEN
    ALTER TABLE component_registry
      ADD CONSTRAINT component_registry_key_format
      CHECK (component_key ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$');
  END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_component_registry_key
  ON component_registry(component_key);

INSERT INTO component_registry (component_key, name, category, props_schema, is_system)
VALUES
  ('hero-banner', 'Hero Banner', 'sections', '{
    "title": {"type": "text", "label": "Title", "required": true, "default": "Welcome"},
    "subtitle": {"type": "text", "label": "Subtitle", "required": false},
    "background_image": {"type": "media", "label": "Background image", "allowed_types": ["image/*"]},
    "cta_text": {"type": "text", "label": "CTA text"},
    "cta_url": {"type": "url", "label": "CTA URL"},
    "alignment": {"type": "select", "label": "Alignment", "options": ["left", "center", "right"], "default": "center"}
  }'::jsonb, true),
  ('feature-grid', 'Feature Grid', 'sections', '{
    "columns": {"type": "number", "label": "Columns", "default": 3},
    "features": {"type": "array", "label": "Features"}
  }'::jsonb, true),
  ('testimonial', 'Testimonial', 'sections', '{
    "quote": {"type": "text", "label": "Quote", "required": true},
    "author": {"type": "text", "label": "Author"}
  }'::jsonb, true),
  ('cta-section', 'CTA Section', 'sections', '{
    "title": {"type": "text", "label": "Title", "required": true},
    "button_text": {"type": "text", "label": "Button text"},
    "button_url": {"type": "url", "label": "Button URL"}
  }'::jsonb, true),
  ('about-section', 'About Section', 'sections', '{
    "title": {"type": "text", "label": "Title"},
    "body": {"type": "richtext", "label": "Body"}
  }'::jsonb, true),
  ('text-block', 'Text Block', 'content', '{
    "body": {"type": "richtext", "label": "Body", "required": true}
  }'::jsonb, true),
  ('image-block', 'Image Block', 'content', '{
    "image": {"type": "media", "label": "Image", "allowed_types": ["image/*"], "required": true},
    "alt": {"type": "text", "label": "Alt text"}
  }'::jsonb, true),
  ('video-embed', 'Video Embed', 'content', '{
    "url": {"type": "url", "label": "Video URL", "required": true}
  }'::jsonb, true),
  ('code-block', 'Code Block', 'content', '{
    "language": {"type": "text", "label": "Language"},
    "code": {"type": "text", "label": "Code", "required": true}
  }'::jsonb, true),
  ('quote', 'Quote', 'content', '{
    "quote": {"type": "text", "label": "Quote", "required": true},
    "citation": {"type": "text", "label": "Citation"}
  }'::jsonb, true),
  ('two-column', 'Two Column', 'layout', '{"gap": {"type": "number", "label": "Gap", "default": 24}}'::jsonb, true),
  ('three-column', 'Three Column', 'layout', '{"gap": {"type": "number", "label": "Gap", "default": 24}}'::jsonb, true),
  ('container', 'Container', 'layout', '{"max_width": {"type": "text", "label": "Max width", "default": "1200px"}}'::jsonb, true),
  ('divider', 'Divider', 'layout', '{"style": {"type": "select", "label": "Style", "options": ["solid", "dashed", "dotted"]}}'::jsonb, true),
  ('spacer', 'Spacer', 'layout', '{"height": {"type": "number", "label": "Height", "default": 32}}'::jsonb, true),
  ('image-gallery', 'Image Gallery', 'media', '{"images": {"type": "array", "label": "Images"}}'::jsonb, true),
  ('carousel', 'Carousel', 'media', '{"slides": {"type": "array", "label": "Slides"}}'::jsonb, true),
  ('video-player', 'Video Player', 'media', '{"video": {"type": "media", "label": "Video"}}'::jsonb, true),
  ('contact-form', 'Contact Form', 'forms', '{"recipient": {"type": "email", "label": "Recipient", "required": true}}'::jsonb, true),
  ('newsletter-signup', 'Newsletter Signup', 'forms', '{"list_id": {"type": "text", "label": "List ID"}}'::jsonb, true),
  ('survey', 'Survey', 'forms', '{"questions": {"type": "array", "label": "Questions"}}'::jsonb, true),
  ('header', 'Header', 'navigation', '{"menu": {"type": "array", "label": "Menu items"}}'::jsonb, true),
  ('footer', 'Footer', 'navigation', '{"columns": {"type": "array", "label": "Columns"}}'::jsonb, true),
  ('breadcrumb', 'Breadcrumb', 'navigation', '{"show_home": {"type": "boolean", "label": "Show home", "default": true}}'::jsonb, true),
  ('pricing-table', 'Pricing Table', 'data', '{"plans": {"type": "array", "label": "Plans"}}'::jsonb, true),
  ('comparison-table', 'Comparison Table', 'data', '{"rows": {"type": "array", "label": "Rows"}}'::jsonb, true),
  ('faq-accordion', 'FAQ Accordion', 'data', '{"items": {"type": "array", "label": "Items"}}'::jsonb, true),
  ('team-grid', 'Team Grid', 'data', '{"members": {"type": "array", "label": "Members"}}'::jsonb, true)
ON CONFLICT (component_key) DO UPDATE
SET name = EXCLUDED.name,
    category = EXCLUDED.category,
    props_schema = EXCLUDED.props_schema,
    is_system = true,
    updated_at = now();

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'pages:create',
          'pages:read',
          'pages:update',
          'pages:delete',
          'pages:publish',
          'pages:versions',
          'pages:restore',
          'components:create',
          'components:read',
          'components:update',
          'components:delete'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name = 'admin';

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'pages:create',
          'pages:read',
          'pages:update',
          'pages:publish',
          'pages:versions',
          'pages:restore',
          'components:read'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name = 'editor';

UPDATE roles
SET permissions = (
      SELECT ARRAY(
        SELECT DISTINCT permission
        FROM unnest(permissions || ARRAY[
          'pages:create',
          'pages:read',
          'pages:update_own',
          'pages:versions',
          'components:read'
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
          'pages:read',
          'components:read'
        ]) AS p(permission)
        ORDER BY permission
      )
    ),
    updated_at = now()
WHERE name = 'viewer';
