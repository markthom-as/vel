# Requirements: Vel Milestone State

## Current State

`0.5.5` is closed.

`0.5.6` is closed and archived.

`0.5.7` is deferred as future duplex voice work.

`0.5.8` is now active as the GSD migration and phase-reset line.

## Closed `0.5.5` Requirement Buckets

| ID | Requirement |
|----|-------------|
| API-55-01 | API/DTO seams needed by the accepted web UI were added without inventing parallel client-local truth. |
| FUNC-55-01 | Accepted operator behaviors were rebound to truthful runtime state instead of UI-local approximation. |
| POLISH-55-01 | Remaining shell/surface/browser fidelity issues from the earlier UI line were reduced to a narrow deferred list. |
| VERIFY-55-01 | Browser-proof artifacts and final audit closed `0.5.5` honestly. |

## Closed `0.5.6` Requirement Buckets

| ID | Requirement |
|----|-------------|
| MVP-56-01 | Single-node setup, settings, integrations, and runtime behavior are operational enough for real operator use. |
| NOW-56-01 | `Now` reflects truthful current-day task/calendar state, including Todoist-backed inbox/today semantics and durable task-lane behavior. |
| CHAT-56-01 | Multimodal assistant/chat works across local `llama.ccp` and OpenAI with bounded persisted configuration, universal operator-profile injection, and thread-level call mode. |
| SYSTEM-56-01 | `System` exposes Core onboarding, operator-facing single-node configuration, and major provider/system lifecycles while hiding deeper technical surfaces behind developer mode. |
| POLISH-56-01 | Navbar, composer, nudges, `Now`, `Threads`, and `System` reach accepted polished-web-ui quality. |
| VERIFY-56-01 | Desktop-Chrome manual QA, focused execution-backed checks, and an honest deferred list close the line. |

## Deferred `0.5.7` Requirement Buckets

| ID | Requirement |
|----|-------------|
| ARCH-57-01 | Native Apple audio/session ownership and portable Rust engine ownership remain future work and did not ship in `0.5.7`. |
| CORE-57-01 | The Rust speech engine seam remains future work and did not ship in `0.5.7`. |
| ADAPTER-57-01 | Platform adapters for typed PCM and device events remain future work and did not ship in `0.5.7`. |
| CALL-57-01 | Duplex thread call mode behavior remains future work and did not ship in `0.5.7`. |
| VERIFY-57-01 | Formal validation and real-device proof remain future work and did not ship in `0.5.7`. |

## Active `0.5.8` Requirement Buckets

| ID | Requirement |
|----|-------------|
| AUDIT-58-01 | Current `get-shit-done` v1 dependencies, assumptions, and risks are inventoried before cutover. |
| MIGRATE-58-01 | The chosen migration, compatibility, or explicit defer path is implemented honestly. |
| STATE-58-01 | Active planning state remains milestone-local and no archived packet is treated as live work. |
| VERIFY-58-01 | Direct workflow checks prove the repo is not left in a speculative planning-tool state. |

## Governing Packet

- `.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/ROADMAP.md`
- `.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/REQUIREMENTS.md`
- `.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/13-NEXT-STEPS.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/ROADMAP.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/REQUIREMENTS.md`
- `docs/future/hybrid-duplex-voice-runtime-spec.md`

## Non-Negotiable Constraints

- `Now` remains bounded and non-inbox-like
- shell chrome remains instrument-like and spatially stable
- state color outranks brand/provider accent
- provider identity remains recognizable but subdued
- critical actions never hide on hover
- color never stands alone as the only state signal
- active planning state must stay milestone-local under `.planning/phases/`
- future duplex voice work must preserve the hybrid ownership split: native session shell, Rust speech engine
- no future voice milestone may normalize Rust ownership of privileged Apple session-policy machinery

## Next Step

- begin Phase `01` planning for `0.5.8` by auditing the current local GSD install and repo-specific workflow assumptions
- decide whether this repo should migrate to `GSD 2`, add a compatibility bridge, or defer with explicit rationale
