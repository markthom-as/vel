# Phase 92 Context

## Why This Phase Exists

Phase 91 exposed a set of UI requests that cannot be made durable or truthful with the current API contracts alone.

The milestone guardrail remains correct: do not renegotiate backend schemas casually. But several accepted UX behaviors now cross a real transport boundary, and the current frontend has been forced to fake or localize too much of that behavior.

This phase exists to inventory and implement only the API/DTO changes that are actually required to support the accepted web UI.

## Evidence

### Assistant entry contract status

Status update:
- explicit `intent` override is already present in the transport and now verified end-to-end
- typed `attachments` are already present in the transport and now verified end-to-end
- durable `follow_up` handle data is now returned from assistant entry and should be treated as the canonical queued-follow-up seam

Remaining gap:
- the web surface still needs deeper behavior binding/polish around that follow-up seam; the transport itself is no longer the blocker

### Assistant entry follow-up is not durably actionable

This seam is now implemented:
- `AssistantEntryResponse` includes typed `follow_up` handle data with intervention id, message id, conversation id, state, and surfaced time

Remaining implication:
- the UI can now route through a durable queue handle, but higher-order affordances such as lane-local action binding still belong to later implementation slices

### `Now` task lane is too narrow for accepted drag/state behavior

- `crates/veld/src/services/now.rs`
  - `build_task_lane(...)` emits:
    - one `active`
    - truncated `pending`
    - `recent_completed`
    - `overflow_count`
- `clients/web/src/types.ts`
  - `NowTaskLaneData` mirrors that limited shape
- `clients/web/src/data/context.ts`
  - `updateCommitment()` only patches an individual commitment

Implications:
- no persisted support for:
  - multiple active tasks
  - explicit lane membership such as `if_time_allows`
  - drag/drop lane reassignment
  - durable per-lane ordering
- `pending` truncation also conflicts with the accepted “show everything in the section” behavior

### `Now` header/location data is still overly implicit

- current `Now` UI derives `CLIENT | LOCATION` and current-event/context lines from mixed fields and fallbacks
- event location exists, but the accepted UI wants a clearer effective header model

Implication:
- the web client is still synthesizing operator-header truth instead of receiving a stable resolved header block

### Thread list data lacks accepted sidebar metadata

Status update:
- conversation list DTOs now carry `message_count`, `last_message_at`, and optional `project_label`

Remaining implication:
- thread row truth is improved, but any richer participant/avatar summary still needs a dedicated seam if the current-thread header is expected to move beyond role-local placeholders

### `System` preferences are local-only

- `clients/web/src/views/system/SystemView.tsx`
  - appearance/accessibility settings are held in local React state
- route/service searches did not show a persisted operator-preferences contract for these settings

Implication:
- accepted “editable values” can be toggled visually, but not persisted as real operator settings

### Existing coverage that does *not* require new API work

- conversation archiving already exists:
  - `crates/veld/src/routes/chat.rs`
  - `crates/veld/src/services/chat/conversations.rs`
- commitment completion/reopen already exists:
  - `docs/api/runtime.md` `PATCH /v1/commitments/:id`
  - `clients/web/src/data/context.ts` `updateCommitment(...)`

This phase should not recreate those seams.

## Required API Change Inventory

1. Assistant entry follow-up behavior binding
   - transport is now present
   - remaining work is to bind queued follow-up affordances more deeply into the accepted `Now` surface behavior

3. Richer `Now` task-lane transport and mutation
   - support multiple active tasks
   - support explicit lane membership for `active`, `next_up`, `if_time_allows`, `completed`
   - support full-list rendering without forced truncation
   - support drag/drop reorder or lane move mutation

4. Resolved `Now` header block
   - expose effective client/location/date/context header fields directly from `/v1/now`
   - remove frontend guesswork around location precedence

4. Thread participant summary data
   - expose stable participant summaries/avatar hints for the current-thread header if the shell is expected to show participant circles beyond local role inference

5. Persisted web/operator preferences
   - add read/write preference DTOs for appearance/accessibility settings used by `System`
   - keep them typed and scoped to actual operator-surface settings

6. Client capability / trust surfacing for missing voice
   - add a truthful seam for browser capability degradation if those states must appear in shared nudges/system trust
   - avoid inventing a purely local fake trust state if the UI is expected to behave as system truth

7. Deferred/snoozed nudge transport
   - expose deferred-count and deferred nudge inventory explicitly if the shell header is expected to separate active vs deferred nudges
   - avoid hardcoding local assumptions about snoozed nudge state

8. Nudge recency fields
   - expose stable nudge timestamps through the decoded web DTO so the shell can show truthful relative age such as `5 min ago`
   - avoid hardcoded “recent” labels without transport support

9. Durable multi-active `Now` task lanes
   - current `/v1/now` transport still privileges a single `active` task plus truncated `pending`
   - accepted UI now supports multiple visually primary active tasks, lane dragging, and a distinct `if_time_allows` lane
   - the current frontend can only localize part of that behavior; durable ordering/lane membership remains API-backed work

10. Persisted system/settings mutations
   - Phase 91 exposed and styled inline editable fields in `System`
   - most of those edits are still local-only because there is no typed persisted operator-settings/settings-mutation seam for the affected rows
   - follow-on API work should separate truly editable values from inspect-only values so the surface stops implying durability where none exists

## Non-Goals

- redesigning the entire `Now` API beyond accepted UI needs
- inventing a second client-owned planning model
- provider expansion or unrelated backend work
- new top-level surfaces
