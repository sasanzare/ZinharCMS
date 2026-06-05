# Phase One Implementation

Phase one goal: a Postman-testable backend CMS core.

## Completed Deliverables

- Auth endpoints: register, login, refresh, logout, current user.
- Argon2id password hashing.
- HMAC-SHA256 JWT access tokens without the problematic `jsonwebtoken` dependency.
- Refresh token persistence with hashed token storage and rotation.
- RBAC helpers for `super_admin`, `admin`, `editor`, `author`, and `viewer`.
- Content type CRUD with `JSONB` field schema validation.
- Entry CRUD with field-level validation, pagination, sorting, publish, and unpublish.
- Media upload/list/detail/update/delete endpoints.
- Automatic WebP image variants for supported image uploads.
- Static serving for `/uploads`.

## Auth Request Examples

Register:

```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "admin@example.com",
  "password": "password123",
  "name": "Admin User"
}
```

Login:

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "admin@example.com",
  "password": "password123"
}
```

## Content Type Example

```http
POST /api/content-types
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "name": "Article",
  "slug": "article",
  "fields": {
    "fields": [
      {
        "id": "title",
        "name": "title",
        "label": "Title",
        "type": "text",
        "required": true,
        "max_length": 255
      },
      {
        "id": "body",
        "name": "body",
        "label": "Body",
        "type": "richtext",
        "required": false
      }
    ]
  }
}
```

## Entry Example

```http
POST /api/entries/article
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "data": {
    "title": "First article",
    "body": "<p>Hello CMS</p>"
  }
}
```

## Media Upload Example

```http
POST /api/media/upload
Authorization: Bearer <access_token>
Content-Type: multipart/form-data

file=<binary>
alt_text=Hero image
caption=Homepage visual
```
