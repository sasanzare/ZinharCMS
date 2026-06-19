# Phase Two Implementation

Phase two goal: a Postman-testable backend engine for the visual page builder.

## Completed Deliverables

- Component registry API backed by PostgreSQL.
- Seeded system component catalog for sections, content, layout, media, forms, navigation, and data components.
- Stable `component_key` identifiers such as `hero-banner` and `feature-grid` for Page JSON nodes.
- Page CRUD API with slug lookup, pagination, status filtering, sorting, publish, and unpublish.
- Page JSON validation for metadata, root layout, component nodes, props/styles objects, nesting depth, node count, and registered component types.
- Page version snapshots on create, update, and restore.
- Version history listing and restore endpoint.
- Authenticated live preview WebSocket at `/api/preview/{page_id}` that sends the current draft and broadcasts subsequent page JSON updates.
- RBAC helpers and role permission seed updates for page builder and component registry operations.

## Page JSON Shape

```json
{
  "version": "1.0",
  "metadata": {
    "title": "Home",
    "description": "Landing page"
  },
  "layout": {
    "id": "root",
    "type": "root",
    "children": [
      {
        "id": "hero-1",
        "type": "hero-banner",
        "props": {
          "title": "Welcome",
          "alignment": "center"
        },
        "styles": {
          "padding_top": 80
        },
        "children": []
      }
    ]
  }
}
```

## Endpoint Summary

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/component-registry` | List registered components; optional `category` filter |
| `POST` | `/api/component-registry` | Create custom component; admin-only |
| `GET` | `/api/component-registry/{component_key}` | Get one component |
| `PUT` | `/api/component-registry/{component_key}` | Update component metadata/schema; admin-only |
| `DELETE` | `/api/component-registry/{component_key}?confirm=true` | Delete custom component; admin-only, system components are protected |
| `GET` | `/api/pages` | List pages with `page`, `per_page`, `status`, and `sort` |
| `POST` | `/api/pages` | Create page and version snapshot |
| `GET` | `/api/pages/{id}` | Get page by UUID |
| `GET` | `/api/pages/slug/{slug}` | Get page by slug |
| `PUT` | `/api/pages/{id}` | Update page and create version snapshot |
| `DELETE` | `/api/pages/{id}?confirm=true` | Delete page; editor/admin |
| `POST` | `/api/pages/{id}/publish` | Publish page; editor/admin |
| `POST` | `/api/pages/{id}/unpublish` | Return page to draft; editor/admin |
| `GET` | `/api/pages/{id}/versions` | List page snapshots |
| `POST` | `/api/pages/{id}/versions/{version}/restore` | Restore snapshot as a new draft version |
| `GET` | `/api/preview/{page_id}` | Authenticated WebSocket stream for live preview |

Use the same `Authorization: Bearer <access_token>` header as phase-one protected endpoints. Browser WebSocket clients may pass the access token as `?access_token=...` or `?token=...` because native WebSocket APIs cannot set custom headers.
