# Phase 46 Subsystem Inventory

This inventory records every supporting subsystem the canonical `Now` contract requires and assigns each one to a downstream owner phase.

The source contract for this inventory is:

- `/home/jove/Downloads/vel-now-surface-contract-codex-final.md`
- `docs/product/now-surface-canonical-contract.md`
- `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md`

## Ownership Rule

- Phase 46 defines the inventory and ownership map.
- Phase 47 defines shared Rust-owned `Now` core models and transport seams.
- Phase 48 defines shared client-mesh, sync, governed config, and recovery support.
- Phase 49 embodies the canonical `Now` surface on web.
- Phase 50 embodies parity on Apple, including reduced watch.
- Phase 51 verifies the full loop, parity, and contract truth across shipped surfaces.

## Inventory

| Subsystem | Contract responsibility | Owner phase |
| --- | --- | --- |
| `Now` header title and display policy | Resolve `{FirstName}'s Now`, fallback title, governed title policy inputs | Phase 48 |
| Header bucket model | Canonical bucket identities, counts, urgency markers, filtered-thread routing posture | Phase 47 |
| Count-display policy | `always_show` / `show_nonzero` / `hidden_until_active` support as governed behavior | Phase 48 |
| Sync/offline header summary | Sync state, last sync, queued-write count, compact recovery route target | Phase 48 |
| Status row model | Non-collapsing date/time/context/elapsed inputs and fallback states | Phase 47 |
| Context resolution | Current context priority across task, event, upcoming event, and no-context fallback | Phase 47 |
| Context one-liner | Backend summary plus deterministic fallback, neutral fallback behavior, update triggers | Phase 47 |
| Nudge stack | Types, ordering, severity, allowed actions, lifecycle, escalation, reappearance | Phase 47 |
| Ranking model | Deterministic-enough shared ranking that avoids UI thrash for the same effective state | Phase 47 |
| Canonical task model | One `task` model with `task_kind`, active selection, pending items, recent completed items, metadata, overflow posture | Phase 47 |
| Task completion posture | Optimistic complete, visible undo, reversible metadata where possible, urgent completion chips | Phase 49 |
| Day object | `day_id`, timezone, boundary timestamps, summary/status metadata, linked artifacts | Phase 47 |
| Day rollover and carry-forward | Automatic carry-forward into new day with day-start review and user-adjustable boundary | Phase 47 |
| Thread continuity model | `primary_thread_id`, open-target priority, multi-thread linkage allowance, continuity chips | Phase 47 |
| `day thread` lane | Canonical day-level continuity lane for summaries, context, and fallback routing | Phase 47 |
| `raw capture` thread lane | Thread-first docked capture continuity before inbox promotion | Phase 47 |
| Thread metadata filters | Project, tags, and continuation categories as shared filter surfaces | Phase 47 |
| Docked input routing | Inline/inbox/thread outcomes, stable bubble behavior, thread-artifact guarantee | Phase 47 |
| Intent taxonomy | Closed v1 intent enum with future extension posture and optional multi-label internals | Phase 47 |
| Approval policy surfaces | Per-action versus batch confirmation posture for durable actions | Phase 48 |
| Config mutation policy | Agents may propose governed config changes, but user approval is required before apply in v1 | Phase 48 |
| Offline write queue | Inspectable queued writes, failed-action retry posture, local-first durable behavior | Phase 48 |
| Conflict posture | Latest-user-input rules for atomic state, merge-plus-review posture for ambiguous edits | Phase 48 |
| Linking and recovery guidance | Guided connection, shared authority hints, repair route targets | Phase 48 |
| Web embodiment | Compact execution-first `Now`, task lane, nudge bars, docked input, escalation chips | Phase 49 |
| Apple embodiment | iPhone, iPad, Mac parity over the same contracts | Phase 50 |
| Reduced watch embodiment | Reduced-but-not-divergent watch surface, same contract with lower density | Phase 50 |
| Cross-platform verification | Execution-backed proof that web and Apple consume the same contract truth | Phase 51 |

## Explicit Non-Owners

- Phase 46 does not implement DTOs, services, adapters, or UI code.
- Web does not own ranking, task selection, nudge ordering, sync truth, or approval policy.
- Apple does not own divergent watch logic, shell-local continuity rules, or separate connection truth.

## Validation Targets

When Phase 46 closes, verification must prove:

- every subsystem above has an owner phase
- no required support lane is left “to be figured out later”
- no item in the local source contract depends on shell-local product policy
