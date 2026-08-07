---
type: Authentication Flow
title: Authentication and Sessions
description: Current authentication, refresh-family, MFA, session, and step-up flows implemented by the backend.
status: draft
sources:
  - id: source-auth-route
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/auth.rs
    title: backend/src/routes/auth.rs at construction commit
  - id: source-auth-middleware
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/middleware/auth.rs
    title: backend/src/middleware/auth.rs at construction commit
  - id: source-sessions
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/sessions.rs
    title: backend/src/services/sessions.rs at construction commit
  - id: source-mfa-challenges
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/mfa_challenges.rs
    title: backend/src/services/mfa_challenges.rs at construction commit
---

# Authentication flow

Protected requests require a bearer access token. The authentication middleware
verifies the token, validates the current session and authentication version,
checks a required step-up scope when a route requests it, and inserts claims
for downstream handlers.

Refresh uses an opaque cookie and a rotating refresh-token family. The session
service supports current-session listing, individual revocation, logout-all,
and access-claim validation. The route layer clears the cookie after logout.

MFA enrollment, TOTP verification, recovery-code use, disablement, and session
revocation are implemented. Redis-backed pre-auth and step-up challenges use
hashed records, single-use locks, TTLs, and rate limits. The repository
configuration supplies key-ring and challenge settings; it does not prove a
particular production secret-management deployment.

The access/session contract is summarized in [authentication and session contract](/api/authentication-and-session-contract.md). Authorization and organization context are separate controls described in [authorization and RBAC](/security/authorization-and-rbac.md) and [tenant isolation](/security/tenant-isolation.md).

## Preserved visualizations

### authentication-flow

```mermaid
sequenceDiagram
    participant Client as Client
    participant Auth as Auth routes
    participant MFA as MFA challenge service
    participant Session as Session service
    Client->>Auth: Login credentials
    alt MFA is required
        Auth->>MFA: Create pre-auth challenge
        MFA-->>Client: Challenge result
        Client->>Auth: MFA verification
    end
    Auth->>Session: Issue access and refresh family
    Session-->>Client: Access token and refresh cookie
```

### session-token-lifecycle

```mermaid
flowchart LR
    Login["Login"] --> Access["Bearer access token"]
    Login --> Refresh["Opaque refresh cookie family"]
    Access --> Validate["Session and auth-version validation"]
    Refresh --> Rotate["Rotate refresh family"]
    Logout["Logout"] --> Revoke["Revoke family and clear cookie"]
```
