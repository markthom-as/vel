# Vel Apple Offline Mode Spec

Status: Planned Apple offline-mode implementation specification  
Audience: coding agent, Apple client implementer, Vel core implementer  
Purpose: define the concrete offline-mode contract for Vel Apple clients without violating the Rust/Swift boundary

---

# 1. Purpose

Vel Apple clients need an offline mode that is:

- genuinely useful
- architecturally safe
- narrow enough to ship

This spec defines the first concrete offline-mode slice for:

- iPhone
- iPad
- Apple Watch via phone companion
- macOS later if needed

This is not a separate on-device Vel brain.

The core rule remains:

> **Rust owns truth and derived state. Apple clients own cache, capture, actions, and presentation.**

This spec refines the existing architectural guidance in:

- `docs/specs/vel-rust-swift-boundary-spec.md`
- `docs/specs/vel-distributed-and-ambient-architecture-spec.md`
- `docs/specs/vel-apple-and-voice-client-spec.md`

---

# 2. Product Goal

Offline Apple mode should let the user do the minimum useful daily loop while disconnected:

- see what matters now from cached state
- capture something quickly
- acknowledge or snooze an existing nudge
- mark a commitment done
- inspect a recent local slice of context

It should not attempt to provide full Vel reasoning.

If the client cannot reach a canonical or temporary VELD node, it should degrade into:

> **cache + action queue + small local assistive model**

not:

> **forked policy engine + local truth reconstruction**

---

# 3. Non-Goals

Do not implement these in early offline mode:

- local context inference over full raw signals
- local risk engine
- local nudge generation policy
- authoritative thread inference
- full historical search over uncached data
- weekly or project synthesis as canonical output
- parallel business logic in Swift

If a feature requires those, it must wait for connectivity or use cached canonical output.

---

# 4. Offline Capability Tiers

## 4.1 Required in v1

These must work offline on iPhone/iPad:

- read latest cached current context
- read cached active nudges
- read cached recent commitments
- create capture
- mark nudge done
- snooze nudge
- mark commitment done
- add a simple commitment
- view sync freshness / degraded state

## 4.2 Allowed if simple

- limited local search over cached hot data
- limited local explanation over cached context
- capture cleanup using a local model
- one-thread or one-day summary over cached local data

## 4.3 Explicitly deferred

- local acceptance/rejection of server-generated suggestions if suggestion state is not cached yet
- local risk explanations
- local generation of new nudges
- local synthesis artifact creation

---

# 5. Architectural Rule

Apple clients must remain edge nodes.

They may own:

- local cache
- local action queue
- local signal/capture acquisition
- stale-aware rendering
- optional local assistive inference on tiny contexts

They must not own:

- canonical context computation
- risk scoring
- policy evaluation
- nudge creation/escalation logic
- synthesis truth

If a local model is present, it is an assistant to the client UX, not an authority over Vel state.

---

# 6. Client Storage Model

Use three local storage groups.

## 6.1 Cache store

Best-effort replicated canonical state for rendering.

Required cached objects:

- current context snapshot
- active nudges
- recent commitments
- recent captures
- minimal sync metadata

Optional later cached objects:

- basic thread summaries
- latest synthesis summary references
- recent artifact excerpts

## 6.2 Queue store

Durable append-only local actions created while offline or before sync confirmation.

This is the most important offline write path.

## 6.3 Attachment store

Optional local files for:

- voice memo placeholders
- image capture references later
- larger pending capture payloads

Attachments must be referenced by queued actions, not treated as separate truth.

---

# 7. Recommended Local Tables

Apple implementation may use SwiftData, SQLite, or another local persistence layer, but the logical schema should follow this shape.

## 7.1 `cached_context`

One row.

Fields:

- `id` = singleton key
- `payload_json`
- `computed_at`
- `fetched_at`
- `source_node`
- `schema_version`

## 7.2 `cached_nudges`

Fields:

- `nudge_id`
- `payload_json`
- `state`
- `updated_at`
- `fetched_at`

## 7.3 `cached_commitments`

Fields:

- `commitment_id`
- `payload_json`
- `status`
- `due_at`
- `updated_at`
- `fetched_at`

## 7.4 `cached_captures`

Fields:

- `capture_id`
- `payload_json`
- `created_at`
- `fetched_at`

Keep this bounded by retention policy.

## 7.5 `sync_state`

Fields:

- `last_successful_sync_at`
- `last_attempted_sync_at`
- `last_error`
- `connected_node`
- `degraded`
- `cache_freshness_class`

## 7.6 `queued_actions`

Fields:

- `action_id`
- `action_type`
- `payload_json`
- `created_at`
- `status` (`pending`, `sending`, `acked`, `failed`, `blocked`)
- `attempt_count`
- `last_attempt_at`
- `error_message`
- `idempotency_key`
- `supersedes_action_id` nullable

This table is append-first. Avoid destructive rewrites.

---

# 8. Queue Action Contract

All offline writes must become explicit queued actions.

## 8.1 Required action types

- `capture.create`
- `nudge.done`
- `nudge.snooze`
- `commitment.done`
- `commitment.create`
- `feedback.submit` later when feedback exists on Apple surfaces

## 8.2 Recommended common envelope

```json
{
  "action_id": "uuid",
  "action_type": "nudge.snooze",
  "created_at": 1773715200,
  "client_id": "device-stable-id",
  "idempotency_key": "uuid-or-derived-key",
  "payload": {
    "nudge_id": "nud_123",
    "minutes": 10
  }
}
```

## 8.3 Domain rules

- `done` dominates `snooze`
- stale queued actions must not reactivate resolved objects
- duplicate sends must be safe through idempotency
- client must preserve action ordering by `created_at`, then local sequence number

---

# 9. Cache Scope

Offline mode should only rely on a bounded hot set.

## 9.1 Required hot cache

- latest current context
- active nudges
- open commitments due soon
- most recent 50 captures
- latest sync status

## 9.2 Warm cache later

- recent 7-30 day captures
- recent artifact summaries
- limited thread summaries

## 9.3 Retention principle

The client is not a full archive.

Cache for responsiveness and degraded usefulness, not for replacing canonical storage.

---

# 10. Freshness Semantics

Every Apple surface must show stale/degraded state explicitly.

Use three classes:

- `fresh` — synced recently and node reachable
- `stale` — cached data shown, sync overdue
- `offline` — node unreachable; rendering cache only, writes queued

The UI must not present cached state as if it were live truth.

Minimum visible indicators:

- last sync time
- queue depth
- degraded banner when offline or stale

---

# 11. Endpoint Contract

The Apple client should prefer a small set of canonical endpoints and avoid stitching business logic client-side.

## 11.1 Required existing endpoints

These are enough for the first offline-capable shell:

- `GET /v1/health`
- `GET /v1/context/current`
- `GET /v1/nudges`
- `POST /v1/nudges/:id/done`
- `POST /v1/nudges/:id/snooze`
- `GET /v1/commitments?status=open&limit=...`
- `POST /v1/commitments`
- `PATCH /v1/commitments/:id`
- `POST /v1/captures`

## 11.2 Client behavior with current endpoints

When online:

- fetch context
- fetch nudges
- fetch commitments
- refresh caches
- drain queued actions in order

When offline:

- never invent missing server responses
- update UI optimistically only for the affected local object
- mark those optimistic changes as pending sync

## 11.3 Recommended near-term additions

These would make offline sync cleaner, but are not required to start:

- `GET /v1/captures/recent?limit=...`
- `POST /v1/actions/batch`
- `GET /v1/sync/bootstrap`
- `GET /v1/sync/changes?since=cursor`

If introduced, they should move Apple from endpoint-by-endpoint refresh toward explicit edge sync semantics.

---

# 12. UI Surface Contract

## 12.1 Now screen

Must work offline from cache.

Show:

- current mode
- morning state
- meds status
- next commitment summary if cached
- active nudges
- freshness banner

Do not recompute these locally from raw signals.

## 12.2 Nudges screen

Must allow:

- inspect cached nudges
- done
- snooze

Actions become queued when offline.

## 12.3 Commitments screen

Must allow:

- inspect recent open commitments
- mark done
- create simple commitment

## 12.4 Capture surface

Must always allow:

- quick text capture
- voice capture placeholder or local attachment

Capture should be the most reliable offline write path.

---

# 13. Optional Local Model Role

If the device supports a usable local model, it may be used for:

- cleaning up dictated capture text
- classifying a capture as `note` vs `todo` vs `question`
- extracting a short title from a long capture
- ranking a tiny cached set of recent items for recall
- compressing one cached thread into a short summary

It must not be used for:

- generating authoritative current context
- generating canonical nudges
- resolving commitment graph semantics
- performing broad corpus retrieval beyond local cache

The local model is a convenience layer, not a policy engine.

---

# 14. Watch Relationship

Apple Watch should not maintain a separate offline truth model.

Preferred shape:

- watch reads from phone companion cache
- watch actions enqueue through phone when possible
- if watch independently records an acknowledgement, the phone reconciles and forwards it as a normal queued action

Watch responsibilities in v1:

- brief current-state glance
- active nudge count
- done/snooze acknowledgement
- quick capture later if low-friction

Do not make watch the source of derived state.

---

# 15. Conflict Rules

Use the same simple deterministic rules defined for distributed Vel:

- `done` beats `snooze`
- later timestamp beats stale replay unless domain rule says otherwise
- duplicate actions must be idempotent
- resolved state must not be reactivated by stale offline replay

If canonical state disagrees with optimistic local cache, canonical wins and the client updates visibly.

---

# 16. Implementation Phases

## Phase 1: Offline-safe Apple shell

Implement:

- local cache persistence
- queued actions
- freshness banners
- offline capture
- offline nudge done/snooze
- offline commitment done/create

No local model required.

## Phase 2: Cached recall improvements

Implement:

- recent cached captures view
- bounded cached search
- better pending-sync indicators

## Phase 3: Small local model assist

Implement only if device/runtime quality is acceptable:

- capture cleanup
- capture classification
- one-thread summary

Keep all outputs explicitly non-canonical until synced.

---

# 17. Guardrails

The following are hard rules:

- Do not reimplement Vel core business logic in Swift.
- Do not let cache become a forked ontology.
- Do not let local-model output silently overwrite canonical fields.
- Do not make offline mode depend on a local model.
- Do not block quick capture on connectivity.

If there is a choice between “smart but divergent” and “simple but coherent,” choose coherent.

---

# 18. Initial API Use In `clients/apple`

For the current repo layout, `VelAPI` should grow toward three responsibilities:

- canonical API client
- local cache read/write helpers
- queued action submission and replay

The app targets should depend on that shared package for all state access paths.

Suggested first additions to `clients/apple/VelAPI`:

- local cache repository types
- queued action model
- sync coordinator
- freshness calculator

Do not add inference, risk, or policy logic there.

---

# 19. Acceptance Criteria

The first acceptable offline Apple implementation satisfies all of these:

1. Phone can launch with no network and still show cached current context, nudges, and commitments.
2. User can create a capture offline without failure.
3. User can mark a nudge done or snooze it offline and see pending state immediately.
4. User can mark a commitment done offline and the action syncs later.
5. Reconnect drains the queue without creating duplicate server-side effects.
6. Canonical sync visibly corrects any optimistic local mismatch.
7. No Swift code computes canonical context, risk, or nudge policy.

That is the correct first offline slice for Vel on Apple platforms.
