# V2 Phase Two: Tenant API Context

This phase connects organization membership from the V2 data model to the authenticated API layer.

## Scope

- Added a `TenantContext` request extension for tenant-owned admin endpoints.
- Added tenant middleware that resolves the active organization from `X-Organization-Id`.
- Validated the authenticated user as an active member of the selected organization.
- Scoped content types, entries, media, pages, page versions, comments, webhooks, and custom components by `organization_id`.
- Updated organization-scoped RBAC checks to use organization membership roles instead of global user roles.
- Updated delivery cache invalidation and webhook dispatch so organization-owned events do not cross tenants.
- Updated the frontend session store and API client to persist and send the active organization id.

## API Contract

Authenticated tenant-owned admin requests must include:

```http
Authorization: Bearer <access-token>
X-Organization-Id: <organization-uuid>
```

The auth responses now include organization memberships:

```json
{
  "user": {
    "id": "00000000-0000-0000-0000-000000000000",
    "email": "admin@example.com",
    "name": "Admin",
    "avatar_url": null,
    "role": "super_admin"
  },
  "organizations": [
    {
      "id": "00000000-0000-0000-0000-000000000000",
      "name": "Default Organization",
      "slug": "default",
      "role": "owner",
      "status": "active"
    }
  ],
  "default_organization_id": "00000000-0000-0000-0000-000000000000"
}
```

The frontend selects the default organization after login and stores it in `zinhar.active_organization_id`.

## Protected Surfaces

The following API areas are now tenant-aware:

- `/api/content-types`
- `/api/entries/{type_slug}`
- `/api/media`
- `/api/pages`
- `/api/component-registry`
- `/api/comments`
- `/api/webhooks`
- `/api/preview/{page_id}`

Global auth and system/plugin routes remain protected by authentication without tenant context.

## Access Rules

- A missing organization header returns `400 Bad Request`.
- A malformed organization id returns `400 Bad Request`.
- A valid organization id without active membership returns `403 Forbidden`.
- Tenant-owned records are queried with `organization_id` filters so IDs from another organization are not returned or mutated.
- Not-found records remain `404 Not Found` through the existing database error mapping.
- Uniqueness conflicts remain `409 Conflict` through the existing database error mapping.

## Preview WebSocket

Browsers cannot attach custom headers to a native WebSocket request. For the preview endpoint only, the tenant middleware also accepts:

```text
/api/preview/{page_id}?access_token=<token>&organization_id=<organization-uuid>
```

Regular HTTP admin requests should continue to use the `Authorization` and `X-Organization-Id` headers.

## Delivery Compatibility

The existing public delivery routes remain compatible with V1 URLs. They now read from the default organization and include the organization id in Redis cache keys.

Organization-slugged public delivery URLs are intentionally left for a later routing phase so this phase can stay focused on the admin API tenant boundary.

## Verification

- Backend test suite passes with all features enabled.
- Frontend TypeScript typecheck passes.
- Frontend production build passes.
- Frontend ESLint passes.
