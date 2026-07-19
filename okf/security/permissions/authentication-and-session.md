---
okf_document_id: "security-permission-authentication-session"
title: "Authentication and Session Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "authentication-session"
permission_group_name: "Authentication and Session"
resource_domain: "identity and session"
permission_scope: "public and authenticated user"
implementation_status: "verified"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/services/jwt.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../authentication-architecture.md"
  - "../../api/endpoints/authentication-and-session.md"
related_diagrams:
  - "../diagrams/authentication-flow.mmd"
  - "../diagrams/session-token-lifecycle.mmd"
---

# Authentication and Session Permission Group

## Included Permissions and Operations

This is a route-boundary group rather than a named RBAC capability: register, login, refresh, logout, and read current user/memberships. Register/login/refresh are public; logout and current-user lookup require a valid access token.

## Scope and Roles

Any caller may attempt public flows. Successful registration assigns a global role. Any valid recognized access token may use logout and `/me`; no additional role helper is called.

## Backend Enforcement and API

`routes/auth.rs` validates credentials and token state; `auth_middleware` protects logout and `/me`. The endpoints are `/api/auth`, `/register`, `/login`, `/refresh`, `/logout`, and `/me`.

## Frontend Checks

`AuthPage` submits public flows. `RequireAuth` and the API client use local token presence and bearer construction. These are `FRONTEND_ONLY_SECURITY_CHECK FOSC-01`.

## Database Implications

The flows read/write `users`, `roles`, `user_roles`, `refresh_tokens`, `login_attempts`, organizations, and memberships. Access tokens are not persisted.

## Tests and Unclear Semantics

Security/password helper tests exist, but no dedicated route integration suite was found. Recovery, MFA, session enumeration, logout-all, and refresh concurrency remain `AUTHENTICATION_FLOW_UNCLEAR AFU-01` and `TOKEN_LIFECYCLE_UNCLEAR TLU-01`.
