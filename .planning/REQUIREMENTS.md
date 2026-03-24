# Requirements: Vel Milestone State

## Current State

`0.5.5` is closed.

`0.6.0` is queued as the next milestone and uses [TODO.md](/home/jove/code/vel/TODO.md) as its source backlog for the single-node MVP and polished web UI line.

## Closed `0.5.5` Requirement Buckets

| ID | Requirement |
|----|-------------|
| API-55-01 | API/DTO seams needed by the accepted web UI were added without inventing parallel client-local truth. |
| FUNC-55-01 | Accepted operator behaviors were rebound to truthful runtime state instead of UI-local approximation. |
| POLISH-55-01 | Remaining shell/surface/browser fidelity issues from the earlier UI line were reduced to a narrow deferred list. |
| VERIFY-55-01 | Browser-proof artifacts and final audit closed `0.5.5` honestly. |

## Queued `0.6.0` Requirement Buckets

| ID | Requirement |
|----|-------------|
| MVP-60-01 | Single-node setup, settings, integrations, and runtime behavior are operational enough for real operator use. |
| NOW-60-01 | `Now` reflects truthful current-day task/calendar state, including Todoist-backed inbox/today semantics and durable task-lane behavior. |
| CHAT-60-01 | Multimodal assistant/chat works across the supported local and hosted providers with bounded persisted configuration. |
| SYSTEM-60-01 | `System` exposes the operator-facing single-node configuration and status needed for the MVP while hiding deeper technical surfaces behind developer mode. |
| POLISH-60-01 | Navbar, composer, nudges, `Now`, `Threads`, and `System` reach accepted polished-web-ui quality. |
| VERIFY-60-01 | Browser proof, focused execution-backed checks, and an honest deferred list close the line. |

## Governing Packet

- `.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/ROADMAP.md`
- `.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/REQUIREMENTS.md`
- `.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md`
- `.planning/v0.5.5-MILESTONE-AUDIT.md`

## Non-Negotiable Constraints

- `Now` remains bounded and non-inbox-like
- shell chrome remains instrument-like and spatially stable
- state color outranks brand/provider accent
- provider identity remains recognizable but subdued
- critical actions never hide on hover
- color never stands alone as the only state signal
- this milestone stays single-node and local-first
- `TODO.md` bullets prefixed with `!` are explicitly ignored for this milestone
- future work after this line should come from new feedback rather than pre-expanding the roadmap

## Next Step

- begin Phase 97 planning for `0.6.0` by mapping the copied verbatim `TODO.md` feedback into explicit acceptance checks for single-node MVP closure
