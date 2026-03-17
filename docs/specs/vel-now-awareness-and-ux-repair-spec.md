# Vel Now Page Awareness + UX Repair Spec

Status: proposed  
Owner: product + runtime + web  
Scope: `clients/web`, `crates/veld`, `crates/vel-storage`, `crates/vel-api-types`

## Summary

The current **Now** page is presenting a stale, partially recomputed, and semantically leaky view of Vel’s current awareness. The UI is not just cosmetically off; it is exposing broken contracts between sync, inference, explainability, and rendering.

From the latest screenshot and current codebase, the most obvious failures are:

- time-of-day inference is wrong or at least materially misleading
- `mode` is effectively hard-coded to `morning_mode`
- Todoist backlog is not prioritized in a way that matches “what matters right now”
- several sync paths do not trigger a context recompute, so awareness goes stale after ingestion
- the Now page does not refresh aggressively enough after sync/evaluate events
- backend reasoning is UTC-centric and not user-local, which corrupts day-boundary logic
- “upcoming events” and “why this context” are composed from explanation artifacts rather than a stable Now-oriented view model
- the UI shows raw internal field names and duplicated panels instead of an operator-grade situational display

This spec defines the contract and implementation plan to make the Now page trustworthy, live, and legible.

## Current problems in the codebase

### 1) Mode selection is broken
In `crates/veld/src/services/inference.rs`, `mode` falls through to `morning_mode` in every branch except prep/commute. That means the page can show `morning_mode` late in the day even when the system clearly should not.

### 2) Inference is UTC-based instead of user-local
`OffsetDateTime::now_utc()` and UTC day boundaries are used throughout inference. That means morning/day/evening state and “today” calculations are wrong for the user unless they happen to live in UTC.

### 3) Sync does not consistently trigger evaluate
`/v1/sync/calendar` and `/v1/sync/todoist` run evaluate afterwards, but activity/git/messaging/notes/transcripts sync routes do not. This causes awareness drift: ingestion succeeds, but current context remains stale.

### 4) Now page cache invalidation is incomplete
Settings actions only invalidate integrations + current context. The Now page also depends on:

- `/v1/explain/context`
- `/v1/explain/drift`
- `/v1/commitments`

Those are left stale unless the page is reloaded.

### 5) No live refresh model
The Now page uses `useQuery` but does not poll, subscribe, or auto-refresh on visibility/focus. It can sit there like a handsome corpse while the runtime state changes underneath it.

### 6) Todoist backlog sorting is wrong for a “Now” surface
`list_commitments()` orders by `created_at DESC`, and the UI just slices open Todoist commitments. That is a repository-ish ordering, not an operator ordering. The page should prioritize due-now, overdue, medication, and nearest actionable items.

### 7) “Upcoming events” is derived from explain data, not a purpose-built Now model
The UI filters `signal_summaries` from the explain endpoint and renders them as “Upcoming events.” That is brittle. Explainability is not the same thing as a Now-optimized view model.

### 8) The page leaks implementation jargon
Examples:

- `morning_mode`
- `awake_unstarted`
- `morning_drift`
- CLI instructions like `Run \`vel evaluate\``

This is appropriate in debug mode, not as the primary operator display.

### 9) Sidebar and main pane duplicate context
The right rail and the main body both repeat nearly the same data. It creates a “more panels = more intelligence” illusion while reducing scan quality.

### 10) Source freshness is invisible
The page shows a single computed timestamp, but not whether calendar data, Todoist data, messaging data, and activity data are fresh enough to trust.

## Goals

1. Make the Now page reflect freshly recomputed awareness.
2. Make time-based reasoning user-local and day-boundary correct.
3. Make Todoist and calendar state actually relevant to “right now.”
4. Separate operator-facing labels from internal inference enums.
5. Add explicit freshness and degradation states.
6. Keep explain/debug detail available without polluting the primary surface.
7. Make the page resilient to partial integration failure.

## Non-goals

- a full redesign of all web surfaces
- replacing the entire inference model
- introducing websockets if fast polling is enough for now
- solving location enrichment for commute times in this pass

## Product requirements

## 1) Introduce a dedicated Now snapshot contract
The web client should stop stitching together the page from four semi-related endpoints. Add a dedicated endpoint:

- `GET /v1/now`

This endpoint returns a purpose-built payload for the Now page. It should be assembled from persisted context + explainability + integrations + commitments, but it must present a single coherent contract.

### Proposed response shape

```json
{
  "computed_at": 1773706153,
  "timezone": "America/Denver",
  "staleness": {
    "overall": "fresh",
    "current_context_age_seconds": 11,
    "calendar_age_seconds": 48,
    "todoist_age_seconds": 33,
    "activity_age_seconds": 121,
    "messaging_age_seconds": 400,
    "degraded_sources": ["messaging"]
  },
  "summary": {
    "mode": { "key": "morning", "label": "Morning" },
    "phase": { "key": "not_started", "label": "Not started" },
    "meds": { "key": "pending", "label": "Pending" },
    "risk": { "level": "medium", "score": 0.475, "label": "Medium · 48%" }
  },
  "schedule": {
    "next_event": {
      "title": "Dinner",
      "start_ts": 1773712800,
      "end_ts": 1773716400,
      "location": null,
      "prep_minutes": 15,
      "travel_minutes": 0,
      "leave_by_ts": 1773712800
    },
    "upcoming_events": []
  },
  "tasks": {
    "todoist": [],
    "other_open": [],
    "next_commitment": null
  },
  "attention": {
    "state": { "key": "drifting", "label": "Drifting" },
    "drift": { "key": "morning_drift", "label": "Morning drift" },
    "severity": { "key": "medium", "label": "Medium" },
    "confidence": 0.7,
    "reasons": []
  },
  "freshness": {
    "calendar": { "status": "fresh", "last_sync_at": 1773706120 },
    "todoist": { "status": "fresh", "last_sync_at": 1773706128 },
    "activity": { "status": "stale", "last_sync_at": 1773705000 }
  },
  "debug": {
    "raw_context": {},
    "signals_used": [],
    "commitments_used": [],
    "risk_used": []
  }
}
```

## 2) Add a user-local time zone setting
Vel needs an explicit timezone in settings, persisted server-side and used by inference.

### Requirements

- Add `timezone` to settings API and web settings UI.
- Default to system-local if available, else fallback to UTC.
- Use timezone for:
  - start of day
  - “today” medication checks
  - morning/day/evening mode calculation
  - display formatting for all timestamps

## 3) Replace the current mode/state labels with operator-facing labels
Internal enums may remain internal. The primary UI must render human labels.

### Mapping examples

- `morning_mode` -> `Morning`
- `meeting_mode` -> `Meeting prep`
- `commute_mode` -> `Commute`
- `awake_unstarted` -> `Not started`
- `engaged` -> `Underway`
- `at_risk` -> `At risk`
- `morning_drift` -> `Morning drift`
- `prep_drift` -> `Prep drift`

Debug mode can expose raw keys.

## 4) Define freshness and degraded-state UX
The Now page must explicitly tell the user whether a section is trustworthy.

### Freshness states

- `fresh`
- `aging`
- `stale`
- `error`
- `disconnected`

### UI requirements

- show an overall freshness badge near “Updated …”
- show per-source freshness on calendar and Todoist sections
- if data is stale/error/disconnected, show an inline warning and degraded rendering
- do not pretend stale data is live

## 5) Rework task prioritization for the Now surface
The Now page should not use raw `created_at DESC` ordering.

### Prioritization rules for Todoist-backed tasks

Order by:

1. medication tasks that are still pending today
2. overdue tasks
3. due today with timestamp
4. due today without timestamp
5. no due date, but recently updated or high priority
6. everything else

Secondary sort:

- earlier due_at first
- priority descending
- updated_at descending if available
- created_at descending as final fallback

If Todoist API does not currently provide/update the needed fields, extend the stored metadata.

## 6) Rework the page layout
The Now page should separate:

- **Primary status band**: mode, phase, meds, risk
- **Main work area**: next event + prioritized tasks
- **Secondary situational panels**: attention, waiting state, operational details
- **Debug drawer**: raw keys, reasons, signals, IDs

### Layout rules

- remove duplicated reason panels between main column and right rail
- keep one canonical explanation surface in normal mode
- reserve raw context for a debug toggle
- keep operator-critical data above the fold

## 7) Improve schedule semantics
The schedule section should be about actual upcoming events, not whatever signals happen to be attached to the current explain payload.

### Requirements

- `upcoming_events` must include only future or currently active events, never long-expired events
- events should be sorted ascending by start time
- if selected calendars are empty, show a distinct empty state
- if calendar sync is stale, say so
- `leave_by` must only show when meaningful

## 8) Recompute after every sync that can affect context
After sync for any source that contributes to awareness, run evaluate.

At minimum:

- calendar
n- todoist
- activity
- git
- messaging
- notes
- transcripts

If some sources are intentionally excluded, document the reason in code and in status docs.

## 9) Add auto-refresh semantics to the Now page
Minimum acceptable implementation:

- refresh on mount
- refresh on window focus
- refresh every 30s while visible
- refresh immediately after sync/evaluate actions

## 10) Keep CLI-oriented instructions out of primary UI
Strings like `Run \`vel evaluate\`` belong in a debug/help affordance, not mixed into operator reasoning.

## Backend design

## New endpoint

Add `GET /v1/now`.

### Implementation guidance

- create a new service layer: `crates/veld/src/services/now.rs`
- do not recompute inference inside the route
- route reads persisted current context, explain data, integration sync metadata, and prioritized commitments
- produce a transport DTO in `vel-api-types`

## Settings extension

Add `timezone: Option<String>` to settings DTOs and persistence.

## Inference updates

Refactor inference time handling so it uses a resolved local timezone instead of blindly using UTC for all day-boundary logic.

### Minimum behavior changes

- compute `start_of_today` in the user timezone
- determine meds done “today” in the user timezone
- derive morning/day/evening mode from local time windows
- fix the impossible fallback where mode always becomes `morning_mode`

### Proposed mode policy

This pass does not need a grand ontology. Just stop lying.

Suggested default:

- `meeting_mode` when prep window active
- `commute_mode` when commute window active
- `morning_mode` only during local morning window or while morning-start tasks remain unresolved
- `day_mode` otherwise
- optionally `evening_mode` later if useful

## Prioritized commitments query

Add a storage/service query purpose-built for Now-page task selection.

Example API:

```rust
pub async fn list_commitments_for_now(&self, limit: u32) -> Result<Vec<Commitment>, StorageError>
```

Or implement it in service layer if SQL complexity should stay out of storage.

## Frontend design

## Data loading

Replace the four-query Now page composition with a single `loadNow()` resource.

Retain optional debug expansion that lazy-loads explain data only when opened.

## Refresh behavior

Extend query infrastructure to support:

- `refetchIntervalMs`
- `refetchOnWindowFocus`
- `refetchOnVisibilityChange`

If you do not want to generalize `useQuery` yet, implement this locally for the Now page.

## UI states

The page must support:

- loading
- partial data with degraded source warnings
- empty but healthy
- stale snapshot
- hard error

## Visual changes

- compact the top summary cards
- make timestamps consistently local
- add freshness badges
- demote internal snake_case labels
- move raw debugging into a collapsible section
- remove duplicate explanation stacks

## Data model changes

### Settings

Add:

- `timezone: Option<String>`

### Optional integration metadata enrichment for Todoist

Consider storing:

- `priority`
- `updated_at`
- `project_id`
- `labels`

inside commitment metadata if not already present.

## Acceptance criteria

### Correctness

- After syncing activity/git/messaging/notes/transcripts, current context changes without needing a manual separate evaluate.
- The Now page does not show `morning_mode` at 6 PM unless there is an explicit rule justifying it.
- Morning/day state uses the user timezone.
- Todoist backlog ordering reflects due/action priority rather than creation order.
- “Upcoming events” only shows future/current events.

### UX

- No raw internal enum names are visible in the default Now UI.
- A user can see freshness for context, calendar, and Todoist at a glance.
- The page refreshes on focus and periodically while open.
- Debug details remain available behind an explicit affordance.

### Testing

- backend tests cover timezone/day-boundary logic
- backend tests cover evaluate-after-sync for awareness-affecting sources
- backend tests cover `/v1/now` contract
- frontend tests cover degraded freshness, label mapping, refresh, and empty states

## Rollout plan

### Phase 1

- fix evaluate-after-sync gaps
- fix mode fallback bug
- add complete cache invalidation for Now dependencies
- add focus/poll refresh

### Phase 2

- add timezone setting and local-time inference
- add `/v1/now`
- migrate Now page to single endpoint

### Phase 3

- add freshness UX
- add prioritized tasks query
- add debug drawer

## Risks

- timezone logic can get subtly ugly if scattered; centralize it
- overloading explain endpoints for page composition will keep reintroducing UI bugs
- if freshness is hidden, operator trust will continue to rot quietly

## Implementation notes for Codex

- preserve current read-only boundary: `GET /v1/now` must not call evaluate
- keep route handlers thin
- put prioritization logic in service/storage, not in React
- do not mix operator labels and internal enum keys in transport DTOs without explicit fields for both
- write migration/tests before wiring frontend assumptions
