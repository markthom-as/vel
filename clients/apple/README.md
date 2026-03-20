# Vel — Apple clients (iOS, watchOS, macOS)

Bootstrap for Vel clients on Apple platforms. Current truth remains HTTP-first against the **same Vel daemon (veld)**. Phase 37 introduces an additive iPhone embedded-capable path, but it does not make Apple shell code the owner of product logic.

Cross-surface architecture authority:

- runtime topology and shell boundaries: [cross-surface core and adapters](../../docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md)
- embedded-capable Apple boundary: [apple embedded runtime contract](../../docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md)
- local-first iPhone voice/offline boundary: [apple local-first voice continuity contract](../../docs/cognitive-agent-architecture/apple/apple-local-first-voice-continuity-contract.md)
- command/query/read-model vocabulary: [cross-surface contract vocabulary](../../docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md)
- shipped architecture proof flow: [cross-surface proof flows](../../docs/cognitive-agent-architecture/architecture/cross-surface-proof-flows.md)

Product-mode authority for summary-first disclosure:

- operator surface taxonomy: [operator surface taxonomy](../../docs/product/operator-surface-taxonomy.md)
- operator mode policy: [operator mode policy](../../docs/product/operator-mode-policy.md)

- **VelAPI** — Swift package (shared): HTTP client and models for the veld API.
- **VelAppleModules** — shared Apple scaffold package for domain/application/infrastructure/platform/capability seams.
- **VeliOS** — iPhone/iPad: summary-first `Now`, triage-first `Inbox`, continuity-first `Threads`, secondary `Projects`, and advanced `Settings`, plus quick-entry capture + voice sheets, multimodal capture (photo + note + optional voice transcript), backend-owned Apple voice replies, offline cache + queued actions.
- **VelWatch** — Apple Watch: summary-first `Now` lane from `/v1/now`, bounded advisories and inbox actions, quick capture/check-in/task entry, and cached fallback.
- **VelMac** — macOS: summary-first `Now`, explicit `Inbox` triage, secondary `Projects`, quick entry, settings/docs drill-down, offline cache + queued actions (sidebar layout), plus local activity/health/messages/reminders snapshot export into Vel’s Application Support tree.

## Product surface hierarchy

Apple clients follow the same shell policy as web:

- `Now` is the default summary-first lane for current context, immediate pressure, and quick entry.
- Apple `Now` should inherit the same sleep-relative current-day truth as web and CLI. It should not roll to a new day at midnight if the backend still considers the operator inside the same day-between-sleeps window.
- `Inbox` is the explicit triage lane for urgent nudges, commitments, and other reviewable work.
- `Threads` is the continuity/history lane and should not become a second live queue.
- `Projects` remains real but secondary; it is for project-specific roots and context after the daily-use lanes point there.
- `Settings` is advanced setup, trust, linking, and deeper detail rather than first-contact operator education.

Apple embodiment remains intentionally summary-first. It should reuse typed backend state and keep runtime/internal detail progressively disclosed instead of teaching implementation categories by default.

Current architecture and planning references: [Master Plan](../../docs/MASTER_PLAN.md), [overarching concept spec](../../docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md), [HLC sync implementation ticket](../../docs/tickets/phase-2/005-hlc-sync-implementation.md), and [tester-readiness onboarding ticket](../../docs/tickets/phase-2/012-tester-readiness-onboarding.md).

## Prerequisites

- Xcode 15+ (Swift 5.9)
- **veld** running (e.g. `veld` in the repo root, or `cargo run -p veld`). Default bind address: `127.0.0.1:4130` (base URL `http://127.0.0.1:4130`).
- For Apple Watch builds, ensure watchOS platform/runtime is installed in Xcode (Settings -> Components).

On Linux or any host without Xcode, the repo Nix shell now includes the Swift toolchain for package-level checks:

```bash
make check-apple-swift
```

This is useful for validating the shared `VelAPI` package, but full app builds still require Xcode on macOS.

On a physical device, set the daemon base URL (e.g. `http://<your-mac-ip>:4130`) in app settings/UserDefaults.

If your runtime enables strict HTTP auth, also set `vel_operator_token` in the Apple client's `UserDefaults`. `VelAPI` sends that value as `x-vel-operator-token` on operator-authenticated `/v1/*` requests, including `/v1/now`, `/v1/apple/voice/turn`, and `/v1/apple/behavior-summary`.

## Daily loop authority

Phase 10 morning and standup authority now lives in the shared backend daily-loop surface:

- CLI authority: `vel morning` and `vel standup`
- Runtime API authority: `POST /v1/daily-loop/sessions`, `GET /v1/daily-loop/sessions/active`, `POST /v1/daily-loop/sessions/:id/turn`
- Apple voice entry: `POST /v1/apple/voice/turn` with `MorningBriefing` intent delegates into that same backend session flow after transcript capture

`GET /v1/context/morning` still exists, but it is now the legacy context brief rather than the bounded morning/standup loop authority.

## Quick local setup (simulator)

From repo root:

```bash
make apple-setup-simulator
make apple-build
make apple-build-watch-sim
make apple-run
make apple-run-watch-sim
# or launch both:
make apple-run-all-sim
```

If you want Xcode UI:

```bash
make apple-open
```

The build script selects the newest available iPhone simulator runtime automatically. To pin one device:

```bash
APPLE_SIM_DEVICE_ID=<sim-udid> make apple-build
```

## Personal device distribution path (iPhone, iPad, Apple Watch)

For installing on your own hardware, use signed device builds plus `devicectl` install.

1. Set your signing team (paid or free Apple ID team both work for personal installs):

```bash
export APPLE_DEVELOPMENT_TEAM=<your-team-id>
```

2. List connected devices and copy UDIDs:

```bash
make apple-list-devices
```

Quick install flow (iPhone/iPad first, optional watch):

```bash
export APPLE_DEVELOPMENT_TEAM=<your-team-id>
export APPLE_DEVICE_ID=<iphone-or-ipad-udid>
export APPLE_WATCH_DEVICE_ID=<watch-udid>   # optional
make apple-install-devices
```

3. Build/install on iPhone or iPad (same `VeliOS` target supports both):

```bash
export APPLE_DEVICE_ID=<iphone-or-ipad-udid>
make apple-build-ios-device
make apple-install-ios-device
```

Optional iOS bundle identifier override (useful for personal-team signing uniqueness):

```bash
export APPLE_IOS_BUNDLE_ID=vel.VeliOS.personal
```

4. Build/install on Apple Watch:

```bash
export APPLE_WATCH_DEVICE_ID=<watch-udid>
make apple-build-watch-device
make apple-install-watch-device
```

Optional watch bundle identifier override:

```bash
export APPLE_WATCH_BUNDLE_ID=vel.VeliOS.personal.watchkitapp
```

Optional watch companion bundle identifier override (must match iOS app bundle ID):

```bash
export APPLE_WATCH_COMPANION_BUNDLE_ID=vel.VeliOS.personal
```

If device install fails on first attempt, open `clients/apple/Vel.xcodeproj` in Xcode once, select each target (`VeliOS`, `VelWatch`, `VelWidgetExtension`, `VelIntentsExtension`), and confirm Signing uses your team.

## Endpoint resolution

The Apple clients resolve endpoint candidates in this order:

1. `vel_tailscale_url` (tailnet endpoint, preferred for cross-device)
2. `vel_base_url`
3. `vel_lan_base_url` (local network fallback)
4. `http://127.0.0.1:4130`
5. `http://localhost:4130`

Use `vel_tailscale_url` for day-to-day multi-device use. Keep `vel_base_url` as a fallback target.

Practical recovery order:

1. confirm the same daemon route appears in web Settings
2. prefer `vel_tailscale_url` when the device is off-host
3. fall back to `vel_base_url` or `vel_lan_base_url` only when that route is actually reachable from the device
4. if the app still renders only cached state, treat it as an endpoint or daemon reachability issue before debugging local export

Current setup limit:

- local-source path discovery and validation are still guide-driven rather than fully automatic on Apple clients; use the setup and integration docs before assuming the client can prove every default path for you

## Offline cache + queue behavior

When the daemon is unreachable, clients keep rendering cached backend data and queue low-risk actions:

- cached surfaces: current context, nudges, commitments, `/v1/now`, Apple behavior summary
- queued actions: nudge `done`, nudge `snooze`, commitment `done`, commitment `create`, capture `create`

Queued actions are drained automatically on next successful reachability/bootstrap check.

Current guardrails:

- queued actions are replayed in insertion order
- on first replay failure, remaining actions stay queued for next retry
- cache is best-effort and can be overwritten by canonical server state after reconnect
- cached daily-loop sessions are render-only fallback state; Apple clients do not synthesize new morning prompts, standup commitments, or local reduction logic while offline

Watch quick-action notes:

- quick presets for common check-ins (for example meds taken / prep started)
- quick note capture and quick commitment add directly from watch
- watch refresh reads `/v1/now` for schedule/next-commitment state and `/v1/apple/behavior-summary` for the bounded behavior card
- watch capture/task actions use the same queued sync lane when offline
- current-day schedule rendering should stay backend-owned here too: next event means the next future relevant calendar event, not a routine block or midnight-fragmented carryover artifact

Voice capture notes:

- iOS voice quick entry uses push-to-talk only (no always-listening mode)
- requires microphone + speech recognition permission
- submissions preserve transcript provenance as a `voice_note` capture
- transcript is editable before submit and intent suggestions update live
- supported backend voice query intents: morning briefing, current schedule, next commitment, active nudges, explain-why, and behavior summary
- the iOS voice surface sends supported replies through `POST /v1/apple/voice/turn`; MorningBriefing now starts or resumes the shared backend daily loop, and Swift renders typed session state instead of synthesizing morning/standup policy locally
- when the backend handles a supported Apple voice turn, the transcript is also persisted into shared thread continuity with the same voice provenance shape used by assistant entry; Swift may render that as a follow-up hint, but it should not invent separate thread policy locally
- longer `check_in`, `reflow`, and action follow-through should use that same shared thread continuity lane with typed resolution metadata; Apple may deep-link to the hinted thread, but it should not synthesize local deferred/resolved semantics
- planning-profile edit requests now follow the same rule: Apple voice may stage a typed routine-block or planning-constraint proposal in confirmation mode, but the edit stays thread-backed and backend-owned until a later approval/apply lane handles it
- when Swift receives that proposal, it should treat the returned thread continuity metadata as the follow-up handle and should not imply that the planning profile was already saved
- once that follow-through is applied through the supervised backend lane, Apple summary surfaces should reflect the same pending/applied/failed continuity that web `Settings`, CLI, and `Now` show from the canonical planning-profile response
- same-day schedule proposals now follow the same compact continuity rule: watch/iPhone/macOS summary surfaces may show pending/applied/failed schedule status from `GET /v1/now`, but the actual review/apply lane still lives in backend state plus `Threads`
- commitment creation still uses the existing direct capture/queue shell because it is not part of the current Apple voice backend contract
- supported voice action intents include targeted commitment done and nudge snooze/done when the backend route is reachable; offline fallback is limited to provenance capture plus queued safe actions
- when the daemon is offline, the iOS voice surface may show cached active morning/standup session state, but resume/submit requires reconnect so the backend remains the source of truth
- local voice draft and recovery state should stay compact and shared across Apple surfaces: `Now` may show draft/pending/merged posture, `Threads` may show the latest local continuity entries, and canonical `thread_id` follow-up must win once the backend provides it
- voice responses can be spoken back with built-in TTS playback
- voice transcript handoff can open the capture flow directly for multimodal draft composition

Cross-surface parity note:

- desktop/browser voice uses local browser speech-to-text and then submits through `/api/assistant/entry`
- Apple voice keeps its typed quick-loop route at `/v1/apple/voice/turn` for compatibility, but supported backend-handled turns now preserve the same shared thread continuity substrate
- Apple `Settings` and Mac summary surfaces now read the same backend-owned planning profile that web `Settings` manages; that parity is inspect-only in this phase and should not be treated as a second planner
- Apple `Now` and quick-loop surfaces should keep thread resurfacing equally bounded: one clearly relevant resumable thread is acceptable, but the main Apple surface should not become a thread inbox
- in both cases, shells own permissions, push-to-talk, local STT/TTS, and offline presentation, while the backend owns transcript provenance, daily-loop authority, and continuity routing
- current platform limit: browser voice depends on browser STT availability, and full Apple app-target validation still requires Xcode/macOS even though `make check-apple-swift` validates the shared Swift package on Linux
- current closeout limit: Apple still inherits the compact `Now` contract rather than a paginated schedule browser or contextual-help system; deeper setup/help routing continues to live in docs and the daemon-backed setup surfaces

Multimodal capture notes:

- iOS capture flow supports multimodal drafts with photo + note + optional voice transcript context
- optional inline photo payload encoding (compressed JPEG + base64) can be embedded for richer multimodal processing
- optional current-context snapshot fields can be attached to each multimodal draft
- saved as capture type `multimodal_note` from source `apple_ios_multimodal`
- photo metadata (dimensions/size), transcript context, and payload preview are exposed before save
- when captures are queued offline, requested capture type/source metadata is preserved in queued payload text

Bootstrap metadata now also advertises node execution capabilities:

- `capabilities` for high-level node abilities
- `branch_sync` for repo branch-sync support on that node
- `validation_profiles` for environment-specific checks such as `api`, `web`, `apple`, `repo`, and `runtime`

Current queued sync contracts can also carry structured `branch_sync_request` and `validation_request` payloads for richer clients that need to ask the authority node for repo work without embedding free-form command text.

## VelMac local export

`VelMac` now writes local snapshot files into `~/Library/Application Support/Vel/` during reachability/bootstrap checks so `veld` can auto-discover them on macOS and ingest them into cluster context immediately.

Current snapshot outputs:

- `activity/snapshot.json`
- `health/snapshot.json`
- `messages/snapshot.json`
- `reminders/snapshot.json`

Current behavior:

- activity writes a lightweight local heartbeat from the running app
- health writes a same-day summary snapshot (step count, active energy, sleep hours, heart rate, stand hours, blood pressure) when HealthKit is available and authorization is granted
- messages writes a recent-thread snapshot from the local Messages database, then asks `veld` to sync it when the daemon is reachable
- reminders writes recent reminders from EventKit and asks `veld` to sync when reachable

Current caveats:

- Health export is snapshot-based through HealthKit, not a long-running background sync
- Messages export reads `~/Library/Messages/chat.db` via `sqlite3`; on macOS this may require Full Disk Access depending on host policy
- Reminders export requires EventKit reminders authorization
- when `veld` is unreachable, `VelMac` still writes snapshots locally but cannot trigger daemon sync until a later reachability check

For the operator-facing setup path, the matching docs are:

- `docs/user/setup.md` for endpoint order and daemon-side route expectations
- `docs/user/integrations/apple-macos.md` for snapshot/export behavior
- `docs/user/integrations/local-sources.md` for local path selection and sync expectations

## 1. Open the project in Xcode

From the repo root:

```bash
open clients/apple/Vel.xcodeproj
```

If the project file is not present, create it from the package and app sources (see **Manual project setup** below).

## 2. Build and run

- **iOS**: Select the **VeliOS** scheme, choose a simulator or device, Run.
- **macOS**: Select the **VelMac** scheme, Run.
- **watchOS**: Select the **VelWatch** scheme, choose a watch simulator (or pair with iPhone simulator), Run.

CLI simulator path:

```bash
make apple-build
make apple-run
make apple-build-watch-sim
make apple-run-watch-sim
```

CLI build/install path (physical devices):

```bash
make apple-build-ios-device
make apple-install-ios-device
make apple-build-watch-device
make apple-install-watch-device
```

Ensure veld is running so the apps can reach the API (e.g. `cargo run -p veld` in the repo root).

## 3. Manual project setup (if no `Vel.xcodeproj`)

If you need to create the Xcode project from scratch:

1. **New project**
   - File → New → Project.
   - Choose **App** (iOS), product name **VeliOS**, interface **SwiftUI**, language **Swift**, save in `clients/apple/Apps/VeliOS`.
   - Repeat for **macOS App** → **VelMac** in `clients/apple/Apps/VelMac`, and **watchOS App** → **VelWatch** in `clients/apple/Apps/VelWatch`.
   - Add app extension targets for `VelWidgetExtension` and `VelIntentsExtension`.

2. **Add local packages**
   - File → Add Package Dependencies → **Add Local…**.
   - Select the `clients/apple/VelAPI` folder (the one containing `Package.swift`).
   - Select the `clients/apple/Packages/VelAppleModules` folder.
   - Add **VelAPI**, **VelFeatureFlags**, and **VelApplePlatform** to the app targets.

3. **Replace default app code**
   - For each app, replace the default `*App.swift` and `ContentView.swift` with the files in `clients/apple/Apps/<AppName>/`.

4. **Optional: single workspace**
   - File → New → Workspace, save as `clients/apple/Vel.xcworkspace`.
   - Add the three app projects and the VelAPI package so all share one workspace.

## Package layout

```
clients/apple/
├── README.md
├── Vel.xcodeproj          # Optional: generated or created manually
├── VelAPI/                 # Swift package (shared)
│   ├── Package.swift
│   └── Sources/VelAPI/
│       ├── VelClient.swift
│       └── Models.swift
└── Apps/
    ├── VeliOS/            # iOS app sources
    │   ├── VelApp.swift
    │   └── ContentView.swift
    ├── VelWatch/          # watchOS app sources
    │   ├── VelWatchApp.swift
    │   └── ContentView.swift
    ├── VelMac/            # macOS app sources
    │   ├── VelMacApp.swift
    │   └── ContentView.swift
    ├── VelWidgetExtension/
    │   └── VelWidgetBundle.swift
    └── VelIntentsExtension/
        └── VelIntentsExtension.swift
```

## API surface used

- `GET /v1/health` — reachability.
- `GET /v1/cluster/bootstrap` — node/transport metadata.
- `GET /v1/sync/bootstrap` — cache hydration for current context, nudges, and commitments.
- `GET /v1/signals?limit=` — recent activity feed on iOS.
- `GET /v1/sync/cluster` — cluster/node/worker inspection metadata for transport and capacity-aware clients.
- `POST /v1/sync/actions` — queued low-risk client actions (`nudge_done`, `nudge_snooze`, `commitment_done`, `commitment_create`, `capture_create`).

VelAPI can be extended with more endpoints (commitments, captures, explain, risk, etc.) as needed.

## Design notes

- **One brain**: All logic (inference, risk, nudges) lives in veld; clients only display and send actions.
- **Offline mode**: Apple clients now cache context/nudges/commitments and queue user actions locally; `VelMac` also exports local activity/health/messages/reminders snapshots for daemon sync. The clients still must not fork local policy or inference.
- **Calm, analytical**: Spec calls for non-chirpy, non-preachy tone; Watch stays ultra-brief.
- **Same repo**: Apple clients live here (not a separate repo) until the core API and release cadence are stable.
