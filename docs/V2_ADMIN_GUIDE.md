# V2 Organization Admin Guide

This guide is for operators and organization administrators managing ZinharCMS V2 after General Availability.

## Organization Management

Each customer workspace is represented by an organization. Organization-owned data includes content types, entries, pages, media, components, settings, navigation, webhooks, workflow comments, subscriptions, usage counters, domains, audit logs, and email delivery records.

Use the organization administration pages or APIs to:

- create and review organizations
- update organization profile details
- manage workspace URLs
- verify custom domains
- review usage and quotas
- inspect audit logs
- triage SaaS alerts

## Memberships And Roles

Recommended role usage:

- `owner`: billing owner and final organization administrator
- `admin`: manages members, content operations, domains, and billing support
- `editor`: manages content, pages, media, workflow, and beta feedback
- `author`: creates and edits assigned content
- `viewer`: read-only operational access

Use least-privilege roles for day-to-day work. Reserve global `super_admin` access for platform-level support and emergency operations.

## Invitations

When a user cannot access an organization:

1. Confirm the invitation was sent to the correct email address.
2. Confirm the invitation has not expired.
3. Confirm the user accepted the invitation using the same email address.
4. Confirm the organization membership is active.
5. Confirm the user selected the correct active organization after login.

Failed invitation emails should be checked in the email delivery logs.

## Domains And Workspace URLs

For workspace URLs:

- keep slugs stable after customer onboarding
- avoid changing workspace URLs during a release window
- confirm routing and CORS configuration before sharing the URL

For custom domains:

- validate ownership before enabling production traffic
- keep DNS and certificate status visible to support
- document any CDN or reverse proxy configuration outside the app

## Audit Logs

Audit logs should be reviewed for:

- role changes
- organization settings changes
- billing changes
- content deletion
- webhook changes
- repeated failed account access attempts
- unexpected cross-organization operations

Treat unexpected cross-organization activity as a tenant isolation incident.

## Rate Limits And Quotas

Organization limits can come from the active plan and organization-specific overrides. When support receives a quota report:

1. Check the active subscription.
2. Check the plan limits.
3. Check usage counters.
4. Rebuild usage counters if the numbers look stale.
5. Apply a temporary override only when approved by the release owner or support policy.

## Beta And Feedback Records

Some V2 installations may retain beta feedback and GA blocker records after GA. Keep these records for release history, but do not use open beta blockers as the primary GA support queue. Convert post-GA production issues into normal operational incidents.