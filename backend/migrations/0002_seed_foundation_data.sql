INSERT INTO roles (name, permissions)
VALUES
  ('admin', ARRAY['*']),
  ('editor', ARRAY[
    'content_types:read',
    'entries:create',
    'entries:read',
    'entries:update',
    'entries:publish',
    'pages:create',
    'pages:read',
    'pages:update',
    'pages:publish',
    'media:create',
    'media:read'
  ]),
  ('viewer', ARRAY[
    'content_types:read',
    'entries:read',
    'pages:read',
    'media:read'
  ])
ON CONFLICT (name) DO NOTHING;

INSERT INTO component_registry (name, category, props_schema, is_system)
VALUES
  (
    'Hero Banner',
    'layout',
    '{
      "type":"object",
      "required":["title"],
      "properties":{
        "title":{"type":"string","maxLength":120},
        "subtitle":{"type":"string","maxLength":240},
        "backgroundImage":{"type":"string"},
        "alignment":{"type":"string","enum":["left","center","right"]}
      }
    }'::jsonb,
    TRUE
  ),
  (
    'Feature Grid',
    'layout',
    '{
      "type":"object",
      "properties":{
        "columns":{"type":"integer","minimum":2,"maximum":4},
        "items":{"type":"array"}
      }
    }'::jsonb,
    TRUE
  ),
  (
    'Rich Text',
    'content',
    '{
      "type":"object",
      "required":["html"],
      "properties":{"html":{"type":"string"}}
    }'::jsonb,
    TRUE
  ),
  (
    'Image',
    'media',
    '{
      "type":"object",
      "required":["src"],
      "properties":{
        "src":{"type":"string"},
        "alt":{"type":"string"},
        "caption":{"type":"string"}
      }
    }'::jsonb,
    TRUE
  )
ON CONFLICT (name) DO NOTHING;
