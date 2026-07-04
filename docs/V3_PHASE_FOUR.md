# V3 Phase 4 Review, Decision Flow, And Moderation

Phase 4 turns automated package validation into an operational Marketplace review workflow. It lets internal Marketplace reviewers open submitted package reports, approve safe submissions, reject or request changes, and moderate products after publication.

## Delivered Scope

- Phase 4.1: Review Queue for submitted and blocked Marketplace submissions with creator, risk, validation, version, and listing context.
- Phase 4.2: Review Decision Flow with `approve`, `reject`, and `request_changes` actions.
- Phase 4.3: Moderation And Takedown actions for `suspend_listing`, `unpublish_version`, and `emergency_block`.
- Append-only review event log in `marketplace_review_events`.
- Creator-facing review notes through submission `review_notes`.
- Internal comments, creator messages, action reason, actor, and metadata captured for every decision and moderation action.
- Audit log records for review decisions and moderation actions.
- Admin UI controls for queue decisions and post-publication moderation.

## Backend API Surface

- `GET /api/marketplace/review/queue`
- `GET /api/marketplace/review/reports`
- `GET /api/marketplace/review/events`
- `PATCH /api/marketplace/review/submissions/{submission_id}`
- `POST /api/marketplace/review/listings/{listing_id}/moderation`

All phase 4 endpoints are restricted to platform admins through the existing global RBAC checks.

## Review Queue

The review queue returns Marketplace submissions that still need human attention:

- queued submissions
- validating submissions
- blocked submissions created by phase 3 security or static validation
- submitted listings waiting for a final decision

Each queue item includes the creator, listing, version, validation status, risk level, validation report, compatibility report, and latest submission status.

## Review Decision Flow

Reviewers can make these decisions:

- `approve`: moves the submission, version, and listing to approved publication state.
- `reject`: rejects the submitted package version and returns the listing to changes requested.
- `request_changes`: stores a creator-facing message and returns the listing to changes requested.

Approval is intentionally blocked when the package is already blocked, static validation failed, or the security risk is high or critical. This preserves the phase 3 rule that high-risk packages enter human review as blocked products and cannot be published accidentally.

## Moderation And Takedown

Reviewers can moderate products after publication:

- `suspend_listing`: hides the listing from future publication paths without deleting history.
- `unpublish_version`: marks a selected version as deprecated; if no approved versions remain, the listing returns to changes requested.
- `emergency_block`: blocks the listing, blocks submitted or approved versions, and blocks active installations.

Every moderation action requires a non-empty reason and writes a review event plus audit log entry.

## Database Changes

Migration `0018_v3_phase_four_review_moderation.sql` adds `marketplace_review_events` with:

- submission, listing, version, actor references
- action, previous status, next status
- internal comment
- creator message
- required reason
- JSON metadata
- append-only created timestamp

## Acceptance Coverage

- A reviewer can open review queue items and make a decision.
- Approve, reject, and request changes update submission, version, and listing statuses consistently.
- No listing becomes publicly approved without an explicit approve decision.
- Suspend, unpublish, and emergency block actions are recorded with a reason.
- Blocked products are no longer available for new install paths because their listing or version status is no longer approved.
- Review and moderation actions are represented in both Marketplace event logs and organization audit logs.
