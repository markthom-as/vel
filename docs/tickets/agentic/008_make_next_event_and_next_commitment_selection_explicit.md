# Ticket 008 — Make next-event and next-commitment selection explicit

## Goal

Replace accidental ordering with explicit urgency selection logic for:

- next relevant event
- next relevant commitment

## Why now

The repo is now operational enough that silent ordering bugs can turn into user-facing nonsense.

The dangerous pattern is:
- “first row wins”
or
- “minimum timestamp of the day wins”
when what the product actually needs is “next relevant thing now”.

## Current starting point

The latest repo feedback already flagged this as a real issue area.

Places to inspect first:
- `crates/veld/src/services/inference.rs`
- any helper logic that derives:
  - first event
  - next commitment
  - `next_commitment_id`
  - `next_event_start_ts`
  - `leave_by_ts`

## Deliverable

Make ordering policy explicit, named, and test-backed.

## Implementation plan

### 1. Define the ordering contract
Recommended event selection:
1. next future event today
2. otherwise currently active event
3. otherwise no next event

Recommended commitment selection:
1. operational commitments attached to near-term external events
2. commitments due soonest
3. highest risk open commitments
4. remaining open commitments

### 2. Encode helpers
Add functions like:
- `select_next_relevant_event(...)`
- `select_next_relevant_commitment(...)`

### 3. Add focused tests
Cover:
- earlier event already passed
- currently active event
- multiple open commitments with different due times / risk
- no due date fallback

## Files likely touched

- `crates/veld/src/services/inference.rs`
- maybe `crates/veld/src/services/risk.rs` only if needed for reading risk order
- tests in inference or app-level route tests

## Tests

Add direct tests for ordering policy.
Do not rely only on indirect integration outcomes.

## Acceptance criteria

- selection policy is explicit and named
- event and commitment ordering are test-backed
- context fields stop depending on incidental database order
- behavior matches operational context, not archival chronology

## Out of scope

- whole new scheduling engine
- speculative prioritization ML
- large schema changes

## Suggested agent prompt

Implement Ticket 008.

Make next-event and next-commitment selection explicit and test-backed.
Prefer simple deterministic ordering over accidental row order.
Keep the rules readable and operational.
