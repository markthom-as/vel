# vel_rust_swift_boundary_spec.md

Status: Canonical Rust/Swift boundary specification  
Audience: coding agent, Apple client implementer, Vel core implementer  
Purpose: define how the Rust core and Apple clients should interact across macOS, iPhone, iPad, Apple Watch, and desktop voice surfaces

---

# 1. Purpose

Vel should use:

- **Rust for the brain**
- **Swift / SwiftUI for the body**

This specification defines the boundary between:

## Rust side
- signals
- commitments
- current context
- risk
- policies
- nudges
- suggestions
- threads
- artifacts
- synthesis
- replication
- local storage / canonical runtime behavior

## Apple side
- UI rendering
- notifications
- watch interactions
- App Intents / Shortcuts
- speech input/output
- Health / motion / Apple platform signal acquisition
- device-local UX and caching

The goal is to keep the system coherent while avoiding two bad outcomes:

1. re-implementing core logic in Swift  
2. forcing Apple UX through awkward Rust-native UI approaches

---

# 2. Core Architectural Rule

Apple clients must not own business logic for:

- context inference
- risk scoring
- policy evaluation
- nudge generation
- synthesis
- thread inference

They may:

- display state
- collect local signals
- collect user actions
- cache data locally
- forward commands to Vel core
- render explanations returned by Vel core

This is the most important rule in the whole spec.

---

# 3. Deployment Model

The Apple layer should be treated as a set of clients of Vel core.

## 3.1 Preferred model
Swift/SwiftUI apps talk to Vel through one of:

### A. Local daemon/API
For macOS, maybe iPhone/iPad later if local runtime exists

### B. Remote VELD API
Over Tailscale/private network

### C. Local embedded Rust library
Only where it is truly worth the complexity

For Vel’s current phase, the best default is:

> **network/API boundary first, embedded Rust boundary second**

This keeps Apple clients simple while Rust core is still evolving.

---

# 4. Recommended Boundary Strategy

## 4.1 Near-term default
Use **HTTP/JSON or similar API boundary** between Apple clients and Vel core.

Why:
- easier iteration
- easier debugging
- easier reuse across CLI, desktop voice, iPhone, iPad, Watch companion
- avoids premature FFI complexity
- works with distributed/ambient architecture already planned

## 4.2 Optional later optimization
Introduce embedded Rust libraries only for:
- local macOS assistant mode
- offline-first local processing
- possibly some lightweight on-device context helpers

Do not start there.

---

# 5. What Rust Owns

The Rust core must own these domains.

## 5.1 Signals / events
- normalization
- durable storage
- event log
- replication-ready semantics

## 5.2 Commitments
- CRUD
- dependencies
- source linkage
- resolution semantics

## 5.3 Current context
- context reducer
- morning state
- drift state
- next commitment
- prep/commute windows
- risk summary

## 5.4 Risk engine
- risk scoring
- explanation factors
- snapshots

## 5.5 Policy engine
- nudge creation
- escalation
- resolution
- suppression/cooldown
- suggestion triggers

## 5.6 Thread graph
- thread creation/linking
- relevance
- lifecycle

## 5.7 Synthesis
- weekly synthesis
- project synthesis
- self-review
- future alignment synthesis

## 5.8 Self-model
- feedback ingestion
- nudge effectiveness
- suggestion acceptance/rejection
- reflective tuning signals

---

# 6. What Swift / SwiftUI Owns

Swift should own all Apple-native UX and device integration.

## 6.1 UI
- current context screens
- nudge list / cards
- thread inspection views
- synthesis display
- settings / local diagnostics

## 6.2 Notifications
- watch notifications
- iPhone/macOS local notifications
- action buttons (`Done`, `Snooze`)
- haptics
- sound/escalation presentation

## 6.3 Apple system integrations
- App Intents / Shortcuts
- speech recognition / text-to-speech
- HealthKit
- Watch connectivity
- local motion/activity signals
- reminders/calendar framework integration if Apple-native acquisition is used

## 6.4 Local device cache
- latest current context
- active nudges
- recent commitments
- recent synthesis summaries

## 6.5 Input capture
- voice capture
- quick add / capture UI
- feedback prompts
- watch quick actions

Swift is the correct place for all of this.

---

# 7. Recommended Client Layout

Keep Apple clients in the same repo for now, outside the Rust workspace.

Suggested structure:

```text
crates/
  vel-core/
  vel-storage/
  veld/
  vel-cli/
  vel-signals/
  vel-context/
  vel-risk/
  vel-nudges/
  vel-threads/
  vel-synthesis/

clients/apple/
  VelApp/
  VelWatch/
  VelMac/
  VelShared/
```

## 7.1 VelApp
iPhone + iPad client

## 7.2 VelWatch
Watch app / Watch extension / watch-specific UI

## 7.3 VelMac
Optional native macOS app shell later, if desired

## 7.4 VelShared
Shared Swift models, networking client, action dispatch, notification helpers, voice helpers

This should live in an Xcode workspace / Swift package world, not in Cargo.

---

# 8. Shared Interface Contract

The Apple side should treat Vel core as an external stateful service.

At minimum, the boundary should expose operations like:

## Read
- get current context
- get context explanation
- get active nudges
- get commitments
- get threads
- get latest synthesis artifact
- get risk for commitment(s)

## Write / actions
- create capture
- mark commitment/nudge done
- snooze nudge
- send feedback
- accept/reject suggestion
- request synthesis

## Signal upload
- upload activity signal
- upload transcript chunk
- upload feedback signal
- upload Apple-native calendar/reminder signal if using client-side adapters

These should be explicit request/response operations, not magical side channels.

---

# 9. Canonical Models Shared Across Boundary

The Swift side should not invent its own domain semantics. It should mirror Rust-owned shapes.

At minimum, align on these models:

- `CurrentContext`
- `Commitment`
- `Nudge`
- `Suggestion`
- `Thread`
- `RiskSnapshot`
- `ArtifactSummary`
- `ExplainContext`
- `FeedbackSignal`

Swift can have presentation-layer wrappers, but the semantic model must match Rust.

---

# 10. Watch Interaction Model

Apple Watch is the primary interrupt surface.

## 10.1 Watch MVP responsibilities
- receive nudge notifications
- allow `Done`
- allow `Snooze`
- maybe show “what matters now?” later

## 10.2 Watch must not own
- risk logic
- nudge generation logic
- context inference
- thread logic

## 10.3 Watch action flow
Example:

1. Watch receives `meeting_prep_window` notification
2. User taps `Done`
3. Swift action layer sends canonical `done` action to Vel core
4. Vel core resolves nudge and related state
5. Other clients sync updated state

This is the correct flow.

---

# 11. iPhone / iPad Interaction Model

iPhone and iPad are review/control surfaces.

## 11.1 MVP screens
- current context
- active nudges
- quick capture
- feedback
- maybe recent commitments

## 11.2 Later screens
- explanation view
- thread view
- synthesis reader
- settings / policy suggestions

## 11.3 Important rule
Even if the iPhone client has a chat interface later, it still should not bypass the shared core action model.

---

# 12. Desktop Voice Boundary

Desktop voice should be treated as another client over the same core.

## 12.1 Desktop voice owns
- speech capture
- local TTS
- push-to-talk UX
- voice session state

## 12.2 Rust core owns
- interpretation target actions
- current context retrieval
- explanations
- commitment actions
- nudge acknowledgements

## 12.3 Voice action mapping
Examples:

Voice:
- “mark meds done”
- “what matters now?”
- “snooze that ten minutes”

These map to the same underlying action grammar used by:
- CLI
- iPhone UI
- Watch action buttons

Do not create a separate “voice-only semantics” layer.

---

# 13. Networking Model

Apple clients should assume a Vel endpoint that may be:

- local on-device
- nearby desktop
- NAS
- VPS
- any preferred node reachable over Tailscale

Clients should:
- try preferred endpoints in configured order
- cache latest successful endpoint
- degrade gracefully when offline
- keep a local action queue where necessary

This should align with the distributed/ambient architecture spec.

---

# 14. Offline Behavior

Apple clients should support degraded offline mode.

## 14.1 Must work offline
- create capture
- mark done
- snooze
- read latest cached current context
- read latest cached nudges
- collect feedback

## 14.2 Sync later
When network/canonical node returns:
- queued actions upload
- canonical state overwrites stale derived client state
- local caches refresh

## 14.3 Important rule
Clients should queue **actions**, not try to merge their own independently inferred truths.

---

# 15. Signal Acquisition on Apple Side

Apple clients may become signal sources.

Possible Apple-origin signals:
- watch acknowledgement
- push-to-talk transcript
- speech transcript
- reminder completion
- calendar event data if acquired client-side
- HealthKit / activity signals later
- wake/alarm proxies later

These must be normalized into canonical Vel signals before they enter the core system.

Swift may gather them, but Rust defines the normalized meaning.

---

# 16. API-First Before FFI

This is important enough to state directly.

## 16.1 Recommended first phase
Build Apple clients against:
- explicit API endpoints
- shared JSON models
- explicit action requests

## 16.2 Only later consider
- embedded Rust dynamic library
- UniFFI / custom FFI bridge
- direct local engine embedding

Why:
- API keeps the boundary honest
- easier to debug
- easier to evolve while core architecture still settling
- easier to support distributed Vel node model

FFI too early is how you end up stapling a changing brain directly into an unstable body.

---

# 17. If/When Embedded Rust Becomes Worth It

Embedded Rust may later make sense for:

## macOS
- local offline-first assistant mode
- lower latency local context/risk access
- richer local daemon integration

## maybe iPhone/iPad later
- limited local context operations
- offline local action processing
- local cache validation

## not first priority for Watch
Watch should almost certainly remain a thin action/notification surface routed through iPhone or networked core.

---

# 18. Swift Package / Module Recommendations

Within `clients/apple/`, consider these Swift modules:

## 18.1 VelAPI
- API client
- request/response models
- auth/session
- endpoint routing

## 18.2 VelModels
- Swift representations of core entities
- decoding/encoding
- light presentation adapters

## 18.3 VelNotifications
- local notification categories
- Watch/iPhone/macOS notification actions
- haptic/sound mapping

## 18.4 VelVoice
- speech recognition
- TTS
- push-to-talk UX
- command/session handling

## 18.5 VelAppleSignals
- Apple-specific signal acquisition
- reminders/calendar/health/activity collection
- translation into upload payloads

This keeps the Apple side sane.

---

# 19. Boundary Rules for the Coding Agent

These are hard rules.

## Rule 1
Do not duplicate core logic in Swift.

## Rule 2
Do not couple Apple UI directly to unstable internal Rust storage structs.

## Rule 3
Use stable boundary models and endpoints.

## Rule 4
Keep watch thin.

## Rule 5
Voice uses shared command semantics.

## Rule 6
Offline mode queues actions, not inferred truth.

## Rule 7
API first, FFI later.

---

# 20. Testing Expectations

## 20.1 Shared action semantics
Verify that:
- Watch `Done`
- iPhone `Done`
- desktop voice “done”
- CLI `vel done ...`

all dispatch the same canonical action type.

## 20.2 Cache/offline tests
Verify:
- offline capture queues
- offline done/snooze queues
- reconnect flushes action queue
- stale cache replaced by canonical context

## 20.3 Notification tests
Verify:
- notification categories map to correct actions
- `Done` and `Snooze` dispatch correctly

## 20.4 Voice intent tests
Verify:
- command-first phrases map to canonical actions
- ambiguous high-risk phrases trigger clarification
- low-risk ambiguous phrases fall back to capture or confirm

---

# 21. Phased Implementation Recommendation

## Phase 1
- API boundary only
- iPhone current context + nudges
- Watch Done/Snooze notifications
- desktop voice command/control

## Phase 2
- feedback capture
- explanation views
- better local cache
- signal uploads from Apple clients

## Phase 3
- richer thread/synthesis views
- local queue + offline polish
- maybe embedded local Rust on macOS if worth it

## Phase 4
- wake word / smart speaker / richer ambient behaviors
- optional deeper FFI work if proven necessary

---

# 22. When a Separate Apple Repo Might Make Sense Later

Not now, but later maybe.

Split later only when:
- API has stabilized
- client cadence differs strongly from core
- signing/provisioning/release workflow dominates
- the client team needs independence

Until then, same repo is the right move.

---

# 23. Final Summary

The correct near-term architecture is:

> Rust owns state, inference, risk, policy, and synthesis.
> Swift owns Apple UX, notifications, voice, and platform integrations.
> The boundary is API-first and action-based.
> Apple clients are surfaces over one stateful Vel core, not little independent brains.

In short:

> let Rust think, let Swift feel, and do not make either pretend to be the other.