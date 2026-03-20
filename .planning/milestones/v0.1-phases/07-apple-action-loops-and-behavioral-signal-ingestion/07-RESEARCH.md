# Phase 7: Apple action loops and behavioral signal ingestion - Research

**Researched:** 2026-03-19
**Domain:** Apple client action loops, backend-owned voice/query flows, lightweight HealthKit behavioral signal ingestion
**Confidence:** HIGH

## Summary

Phase 7 should extend the existing backend-owned Apple seam instead of adding new client-owned logic. The right foundation is already present: `sync/bootstrap` for cache hydration, `sync/actions` for low-risk queued mutations, `now` for schedule/orientation, and snapshot-based health ingestion. Planning should treat Apple as a thin input/output shell over Rust services.

The highest-value slice order is: first publish typed contracts for Apple loops and behavior summaries; then move voice/query reasoning into Rust; then add watch/iPhone quick-response flows on the shared queue path; then narrow health ingestion to steps/stand/exercise-style signals with explainable summaries. Do not deepen the current Swift-side query synthesis pattern.

**Primary recommendation:** Plan Phase 7 around backend-owned `voice -> typed command/query -> persisted capture + explainable response`, plus narrow daily behavior rollups for `steps`, `stand`, and `exercise`, with Apple clients only handling capture, permissions, caching, and queued low-risk actions.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| IOS-01 | Fast iPhone capture loops | Reuse `VelAPI`, `VelOfflineStore`, push-to-talk capture, and `sync/actions`; keep capture provenance persisted server-side |
| IOS-02 | Current schedule retrieval | Use backend `GET /v1/now` as authority; optionally expose a smaller Apple-focused schedule snapshot derived from the same backend data |
| IOS-03 | Nudge response from Apple surfaces | Keep watch/iPhone nudge `done` and `snooze` on the existing queued low-risk action lane |
| HEALTH-01 | Ingest lightweight behavioral signals | Keep snapshot ingestion but narrow Phase 7 scope to steps, stand, and exercise-oriented signals |
| HEALTH-02 | Explainable behavior summaries | Produce typed backend summaries tied to persisted signals and freshness, not client heuristics |
| APPLE-01 | Apple clients remain thin shells | Swift must not own policy, nudge ranking, query reasoning, or durable behavior synthesis |

## User Constraints

- There is no `07-CONTEXT.md` yet. Use the roadmap, repo docs, Apple subtree guide, current Apple code, and recent Phase 05/06 outcomes to determine the correct slice boundaries.
- The phase should prioritize voice capture, current schedule retrieval, nudge response, and step/stand/exercise signal ingestion with explainable summaries.
- Swift must not own policy. Apple surfaces stay thin over Rust backend rules.

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust `veld` routes/services + `vel-api-types` DTOs | repo current | Owns policy, query/response logic, schedule projection, behavior summaries | Matches repo architecture and Apple subtree rules |
| `VelAPI` + `VelOfflineStore` | repo current | Shared Apple transport, cache hydration, queued low-risk actions | Existing shared Apple seam across iPhone/watch/macOS |
| Apple `Speech` + `AVFAudio` / `AVSpeechSynthesizer` | platform | Push-to-talk transcription and spoken responses | Standard Apple stack for short voice loops |
| Apple `HealthKit` | platform | Read step/stand/exercise-style metrics | Standard Apple health access layer |

### Supporting

| Library / Surface | Purpose | When to Use |
|---------|---------|-------------|
| `GET /v1/now` | Current schedule/orientation snapshot | iPhone/watch schedule retrieval and explainable summaries |
| `GET /v1/sync/bootstrap` | Cached context/nudges/commitments/continuity hydration | Fast Apple refresh and offline fallback |
| `POST /v1/sync/actions` | Queued low-risk action replay | Nudge done/snooze, capture create, commitment create/done |
| Apple `AppIntents` | Optional quick entry points | Use for watch/iPhone shortcut-style actions only after backend contracts exist |

**Version verification:** `make check-apple-swift` passed on 2026-03-19 and reported Swift 5.10.1. Apple frameworks are platform SDK surfaces, not separately versioned packages.

## Recommended Plan Slices

### Slice 1: Publish typed Apple loop contracts

- Add explicit backend DTOs for:
  - voice command/query request and response
  - Apple schedule snapshot if `NowData` is too broad for watch use
  - behavior summary payload for daily steps/stand/exercise signals
- Update `vel-api-types` and `clients/apple/VelAPI` in the same slice.
- Keep capture provenance explicit in the contract.

### Slice 2: Move voice/query reasoning into Rust

- Replace Swift-side query synthesis with a backend-owned typed endpoint or reuse a typed command surface.
- Backend should:
  - persist transcript capture provenance
  - answer query intents from persisted state
  - resolve low-risk action intents through existing safe mutation paths
  - return explainable reasons from `Now`/nudge evidence
- Swift should only record/transcribe, submit, render response, and queue capture if offline.

### Slice 3: Close iPhone/watch action loops on shared queue paths

- Keep watch and iPhone quick actions on `sync/actions` and existing capture/create endpoints.
- If adding App Intents, route them through `VelAPI` plus shared queue/caching, not direct client policy.
- Prefer one shared Apple action abstraction in `VelAPI`; do not fork iPhone/watch semantics.

### Slice 4: Narrow behavioral signal ingestion to useful daily rollups

- Keep Phase 7 scope to:
  - `step_count`
  - `stand_hours` or equivalent stand progress
  - `exercise_minutes` or equivalent exercise rollup
- Do not make broad health ingestion, sleep programs, heart analytics, or astrology-like interpretation part of the core loop.
- Persist raw/summary signals in the backend and derive an explainable daily behavior summary there.

### Slice 5: Operator/docs/verification closure

- Update Apple README and user docs to describe real permissions, offline behavior, and explainability.
- Add focused backend endpoint tests and at least package-level Apple verification.
- Manually exercise iPhone voice capture and watch quick actions before phase closure.

## Required Backend and Client Seams

### Backend must own

- Voice query interpretation and response generation
- Action targeting rules for commitments and nudges
- Schedule projection and freshness semantics
- Behavioral signal rollup rules and explainable summaries
- Persisted provenance for transcript, response, and signal-derived summaries

### Swift may own

- Microphone / speech / HealthKit permission handling
- Audio recording and local transcript editing
- Rendering, local cache hydration, and offline queueing
- Watch/iPhone quick-entry affordances

### Swift must not own

- Nudge ranking or response policy
- Query reasoning like "what matters now?" or "why now?"
- Behavioral interpretation of steps/stand/exercise signals
- Conflict resolution or durable writeback policy

## Architecture Patterns

### Pattern 1: Thin Apple shell over backend truth

Use the existing Apple pattern from Phase 05/06:

- `VelAPI` decodes transport DTOs
- `VelOfflineStore` hydrates/cache-reads shared payloads
- app targets render summaries and queue low-risk actions
- Rust remains the authority for logic and durable state

### Pattern 2: Persist before respond for voice loops

For voice requests:

1. Persist transcript capture with Apple provenance
2. Execute backend query/action against persisted state
3. Return typed response with reasons/evidence
4. Let the client speak/render the result

This preserves explainability and keeps voice from becoming an untraceable side channel.

### Pattern 3: Daily behavior rollups, not raw health platform sprawl

- Ingest daily Apple behavior signals as bounded summaries
- Derive one typed backend summary for orientation
- Surface freshness and source timestamps alongside the summary

## Common Risks

### Risk 1: Swift policy drift

- The current iOS voice tab synthesizes query answers locally from cached state in `clients/apple/Apps/VeliOS/ContentView.swift`.
- Planning should explicitly remove or replace this path.
- If left in place, Phase 7 will deepen a pattern the repo rules forbid.

### Risk 2: Over-scoping health

- Existing macOS export already emits more than Phase 7 needs.
- Planning should explicitly bound the phase to steps/stand/exercise-style usefulness and explainability.
- Avoid broad medical or wellness interpretation.

### Risk 3: Schedule authority split

- Do not fetch calendar truth directly in iPhone/watch clients for this phase.
- Schedule retrieval should stay backend-owned via `Now` so Apple and non-Apple surfaces remain consistent.

### Risk 4: Offline queue widening

- Keep queued actions low-risk and replay-safe.
- Do not add high-risk writebacks or ambiguous mutations to the Apple queue without explicit policy review.

### Risk 5: Health dedupe and stale summaries

- Health snapshots are currently latest-sample oriented.
- Phase 7 should define freshness and dedupe behavior clearly, especially for daily rollups and repeated exports.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Speech transcription | custom audio-to-text pipeline | Apple `Speech` framework | Standard permissions, platform support, lower complexity |
| Spoken response | custom TTS stack | `AVSpeechSynthesizer` | Already standard and adequate for short loops |
| Health access | custom file/import formats on iPhone/watch | `HealthKit` queries and bounded snapshot export | Correct permissions and Apple-native data access |
| Watch quick entry routing | watch-only mutation logic | shared `VelAPI` + queue path | Preserves one action contract across Apple surfaces |

## Verification Suggestions

### Automated

| Area | Command | Notes |
|------|---------|-------|
| Sync bootstrap contract | `cargo test -p veld sync_bootstrap_endpoint_returns_ok -- --nocapture` | Passed on 2026-03-19 |
| Health ingestion baseline | `cargo test -p veld sync_health_ingests_snapshot_and_triggers_evaluation -- --nocapture` | Passed on 2026-03-19 |
| Now schedule/orientation baseline | `cargo test -p veld now_endpoint_returns_consolidated_snapshot -- --nocapture` | Passed on 2026-03-19 |
| Apple shared package build | `make check-apple-swift` | Passed on 2026-03-19 |

### Add in Phase 7

- Backend endpoint tests for typed Apple voice/query contracts
- Backend tests for behavior-summary derivation and freshness
- Fixture-backed `VelAPI` decode tests if `SyncBootstrapData` or `NowData` grows
- Focused Apple-side tests if new `VelAPI` mappers or queue types are added

### Manual

- iPhone: push-to-talk voice query, voice action, offline capture queue, replay after reconnect
- Apple Watch: top nudge done/snooze, quick capture, quick commitment, cached fallback
- Verify that the same backend state explains responses across Apple, web, and CLI

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test`; Swift package build check |
| Config file | none for Swift package; Rust workspace defaults |
| Quick run command | `cargo test -p veld <targeted_test> -- --nocapture` |
| Full suite command | `make verify` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| IOS-01 | Voice/capture loop persists and returns typed response | integration | `cargo test -p veld apple_voice_* -- --nocapture` | ❌ Wave 0 |
| IOS-02 | Apple schedule retrieval matches backend `Now` | integration | `cargo test -p veld now_* -- --nocapture` | ✅ extend existing |
| IOS-03 | Nudge response stays replay-safe via queue path | integration | `cargo test -p veld sync_actions_* -- --nocapture` | ✅ extend existing |
| HEALTH-01 | Steps/stand/exercise ingestion persists expected signals | integration | `cargo test -p veld apple_behavior_* -- --nocapture` | ❌ Wave 0 |
| HEALTH-02 | Behavior summary is explainable and fresh | integration | `cargo test -p veld apple_behavior_summary_* -- --nocapture` | ❌ Wave 0 |
| APPLE-01 | Swift remains transport/render only | package/build + review gate | `make check-apple-swift` | ✅ |

### Wave 0 Gaps

- Add targeted backend tests for Apple voice/query endpoint behavior
- Add targeted backend tests for bounded behavior-summary derivation
- Add fixture-backed `VelAPI` decoding tests if new DTOs are introduced

## Sources

### Primary

- Repo authority and Apple subtree guides:
  - `docs/MASTER_PLAN.md`
  - `AGENTS.md`
  - `clients/apple/AGENTS.md`
  - `clients/apple/README.md`
  - `.planning/ROADMAP.md`
  - `.planning/phases/05-now-inbox-core-and-project-substrate/05-08-SUMMARY.md`
  - `.planning/phases/06-high-value-write-back-integrations-and-lightweight-people-graph/06-07-SUMMARY.md`
- Backend and Apple implementation seams:
  - `crates/veld/src/routes/sync.rs`
  - `crates/veld/src/services/client_sync.rs`
  - `crates/veld/src/services/now.rs`
  - `crates/veld/src/adapters/health.rs`
  - `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`
  - `clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift`
  - `clients/apple/VelAPI/Sources/VelAPI/VelLocalSourceExporter.swift`
  - `clients/apple/Apps/VeliOS/ContentView.swift`
  - `clients/apple/Apps/VelWatch/VelWatchApp.swift`
- Official Apple docs:
  - https://developer.apple.com/documentation/speech/sfspeechrecognizer
  - https://developer.apple.com/documentation/avfaudio/avspeechsynthesizer
  - https://developer.apple.com/documentation/healthkit/hkstatisticscollectionquery
  - https://developer.apple.com/documentation/healthkit/hkobserverquery
  - https://developer.apple.com/documentation/appintents
  - https://developer.apple.com/documentation/eventkit

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - matches current repo seams and official Apple frameworks
- Architecture: HIGH - strongly constrained by repo rules and current Phase 05/06 patterns
- Risks/pitfalls: HIGH - directly observed in current code and testable existing seams

**Research date:** 2026-03-19
**Valid until:** 2026-04-18
