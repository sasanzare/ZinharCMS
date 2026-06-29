# V2 Phase 7: SaaS Operations and Production Readiness

Phase 7 prepares the V2 multi-tenant CMS for real SaaS operation by adding workspace access, organization-level controls, transactional email tracking, sensitive-operation auditing, request rate limits, and alert definitions.

## Delivered Scope

- Stable path-based workspace URLs are generated from the organization slug: `/workspace/{slug}`.
- Custom organization domains can be listed, added, marked primary, and deleted.
- Transactional email delivery records are stored for invitations and billing notifications.
- Sensitive actions are written to `audit_logs`, including member role changes, invitations, content deletion, page deletion, media deletion, billing operations, and organization settings changes.
- Organization and user request rate limits are enforced in the tenant middleware through Redis.
- SaaS alert rules are seeded for billing failures, email delivery failures, RLS errors, and migration failures.
- The Organization UI exposes workspace URL, domains, rate limits, audit log, email deliveries, and alert rules.

## Environment

```env
APP_BASE_URL=http://localhost:5173
EMAIL_PROVIDER=log
EMAIL_FROM=ZinharCMS <noreply@localhost>
EMAIL_WEBHOOK_URL=
EMAIL_FAILURE_MODE=log
ORG_RATE_LIMIT_PER_MINUTE=600
ORG_USER_RATE_LIMIT_PER_MINUTE=120
ORG_RATE_LIMIT_BURST=120
```

`EMAIL_PROVIDER=log` records email deliveries without contacting an external provider. Use `EMAIL_PROVIDER=webhook` with `EMAIL_WEBHOOK_URL` to send JSON payloads to an HTTP email bridge. Set `EMAIL_FAILURE_MODE=strict` when provider failures should fail the API operation instead of only recording a failed delivery.

## Database Objects

- `organization_domains`
- `organization_rate_limits`
- `audit_logs`
- `email_deliveries`
- `saas_alert_rules`

All new organization-owned tables have row-level security policies that use the existing tenant context helpers.

## API Surface

- `GET /api/organizations/current/workspace`
- `GET /api/organizations/current/domains`
- `POST /api/organizations/current/domains`
- `DELETE /api/organizations/current/domains/{domain_id}`
- `GET /api/organizations/current/rate-limit`
- `PUT /api/organizations/current/rate-limit`
- `GET /api/organizations/current/audit-logs`
- `GET /api/organizations/current/email-deliveries`
- `GET /api/organizations/current/alerts`

Admin-level organization roles are required for domain management, rate-limit configuration, audit log access, email delivery inspection, and alert rule inspection.
