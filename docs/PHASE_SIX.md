# Phase Six Implementation

Phase six goal: editorial workflow, collaboration comments, and the first plugin hook system.

## Delivered

- Workflow status support for entries and pages: `draft`, `pending_review`, `published`, and `archived`.
- State transition enforcement in backend services before status updates.
- Review endpoints for submitting, publishing, rejecting, archiving, and restoring entries/pages.
- Collaboration comments attached to entries or pages, with resolve/reopen support.
- Plugin registry table plus protected plugin management endpoints.
- Built-in `seo-auto` plugin that fills `data.slug` from `data.title` before entry save when slug is empty.
- Admin Workflow page with pending review queues, review comments, and plugin toggles.
- Entries and Pages primary status action now follows the workflow: submit, publish, archive, restore.

## Workflow

| Current | Action | Next |
| --- | --- | --- |
| `draft` | Submit review | `pending_review` |
| `pending_review` | Publish | `published` |
| `pending_review` | Reject | `draft` |
| `published` | Archive | `archived` |
| `archived` | Restore | `draft` |

Reviewers can still publish a draft directly through existing publish endpoints.

## Collaboration

Comments are stored in `comments` and reference either an `entry` or a `page`.
Resolved comments stay available when `include_resolved=true` is passed to the comments list endpoint.

## Plugins

The first plugin interface supports entry hooks:

- `entry.before_save`
- `entry.after_publish`

Only built-in plugins run in this phase. The `cms_plugins` table controls whether a built-in plugin is enabled.
