# Diagram Evidence Workspace

This directory is the evidence base for the 20-step Mermaid documentation process.
Step 1 does not create diagrams yet. It records repository inventory, implementation
status, and source-to-domain evidence so future Mermaid diagrams are grounded in
current code, schema, routes, tests, and configuration.

## Files

| File | Purpose |
| --- | --- |
| `REPOSITORY_INVENTORY.md` | Architecture-relevant repository structure with responsibilities, domains, file kind, major symbols, and diagram targets. |
| `ARCHITECTURE_AUDIT.md` | Initial implementation matrix using evidence-first statuses and confidence levels. |
| `FILE_EVIDENCE_INDEX.md` | Searchable domain-to-file matrix connecting migrations, routes, services, frontend pages, APIs, tests, docs, and conflicts. |

## Evidence Rules Used

Evidence was prioritized in this order:

1. Current SQL migrations and database constraints.
2. Backend route composition and middleware.
3. Backend handlers and services.
4. Frontend router, API client, stores, hooks, and pages.
5. Automated tests.
6. Docker, environment, and CI configuration.
7. Current architecture and API documentation.
8. README.
9. Historical phase documentation.

Features mentioned only in documentation are not marked implemented.

## Step 1 Scope

- Repository behavior was not changed.
- Migrations were not modified.
- No packages, browsers, Docker images, or system dependencies were installed.
- Generated dependency/build folders were excluded from the inventory counts.
