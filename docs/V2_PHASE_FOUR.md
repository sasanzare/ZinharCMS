# V2 Phase 4: Organization UI and Member Management

## Scope

Phase 4 adds organization management to the admin workspace. It lets authenticated users switch active organizations, manage organization settings, invite members, accept invitations, update member roles, leave an organization, and transfer ownership.

## Backend

- Added organization routes under authenticated and tenant-aware API groups.
- Added current organization settings read/update endpoints.
- Added organization member listing, role update, removal, leave, and ownership transfer endpoints.
- Added invitation create/list/revoke endpoints and a signed-token invitation acceptance endpoint.
- Enforced organization-level access control for owner/admin/editor boundaries.
- Prevented removing, leaving, or downgrading the last active organization owner.
- Exposed the new organization endpoints in OpenAPI.

## Frontend

- Added an active organization switcher to the top bar.
- Added the Organization page at `/organization`.
- Added organization settings, plan limit, organization creation, invitation acceptance, member invitation, member role, member removal, invitation review, and ownership transfer UI.
- Added typed frontend API methods and TypeScript response contracts for organization workflows.
- Added i18n keys for English and Persian UI text.

## Notes

Invitation email delivery is intentionally not wired to an email provider yet. The API returns a one-time invitation token and accept link so the flow can be tested end-to-end in local development.
