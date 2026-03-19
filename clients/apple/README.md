# Vel — Apple clients (iOS, watchOS, macOS)

Bootstrap for Vel clients on Apple platforms. All apps talk to the **same Vel daemon (veld)** over HTTP; no business logic in the client.

- **VelAPI** — Swift package (shared): HTTP client and models for the veld API.
- **VeliOS** — iPhone: Today/Nudges/Activity/Capture/Voice/Settings shell, multimodal capture (photo + note + optional voice transcript), backend-owned Apple voice replies, offline cache + queued actions.
- **VelWatch** — Apple Watch: backend-owned quick-loop summary from `/v1/now` plus Apple behavior summary, nudge quick actions (done/snooze), quick capture/check-in/task entry, cached fallback.
- **VelMac** — macOS: context, nudges, commitments, quick capture, offline cache + queued actions (sidebar layout), plus local activity/health/messages/reminders snapshot export into Vel’s Application Support tree.

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

## Quick local setup (simulator)

From repo root:

```bash
make apple-setup-simulator
make apple-build
make apple-build-watch-sim
make apple-run
```

If you want Xcode UI:

```bash
make apple-open
```

The build script selects the first available iPhone simulator automatically. To pin one device:

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
export APPLE_WATCH_BUNDLE_ID=vel.VelWatch.personal
```

If device install fails on first attempt, open `clients/apple/Vel.xcodeproj` in Xcode once, select each target (`VeliOS`, `VelWatch`), and confirm Signing uses your team.

## Endpoint resolution

The Apple clients resolve endpoint candidates in this order:

1. `vel_tailscale_url` (tailnet endpoint, preferred for cross-device)
2. `vel_base_url`
3. `vel_lan_base_url` (local network fallback)
4. `http://127.0.0.1:4130`
5. `http://localhost:4130`

Use `vel_tailscale_url` for day-to-day multi-device use. Keep `vel_base_url` as a fallback target.

## Offline cache + queue behavior

When the daemon is unreachable, clients keep rendering cached backend data and queue low-risk actions:

- cached surfaces: current context, nudges, commitments, `/v1/now`, Apple behavior summary
- queued actions: nudge `done`, nudge `snooze`, commitment `done`, commitment `create`, capture `create`

Queued actions are drained automatically on next successful reachability/bootstrap check.

Current guardrails:

- queued actions are replayed in insertion order
- on first replay failure, remaining actions stay queued for next retry
- cache is best-effort and can be overwritten by canonical server state after reconnect

Watch quick-action notes:

- quick presets for common check-ins (for example meds taken / prep started)
- quick note capture and quick commitment add directly from watch
- watch refresh reads `/v1/now` for schedule/next-commitment state and `/v1/apple/behavior-summary` for the bounded behavior card
- watch capture/task actions use the same queued sync lane when offline

Voice capture notes:

- iOS Voice tab uses push-to-talk only (no always-listening mode)
- requires microphone + speech recognition permission
- submissions preserve transcript provenance as a `voice_note` capture
- transcript is editable before submit and intent suggestions update live
- supported backend voice query intents: morning briefing, current schedule, next commitment, active nudges, explain-why, and behavior summary
- the Voice tab sends supported replies through `POST /v1/apple/voice/turn`; Swift renders the typed response and does not synthesize query answers locally
- commitment creation still uses the existing direct capture/queue shell because it is not part of the current Apple voice backend contract
- supported voice action intents include targeted commitment done and nudge snooze/done when the backend route is reachable; offline fallback is limited to provenance capture plus queued safe actions
- voice responses can be spoken back with built-in TTS playback
- voice transcript handoff can open the Capture tab directly for multimodal draft composition

Multimodal capture notes:

- iOS Capture tab supports multimodal drafts with photo + note + optional voice transcript context
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

CLI build path (iOS simulator):

```bash
make apple-build
make apple-run
make apple-build-watch-sim
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
   - Repeat for **macOS App** → **VelMac** in `clients/apple/Apps/VelMac`, and **watchOS App** → **VelWatch** in `clients/apple/Apps/VelWatch` (or create one workspace with three app targets).

2. **Add VelAPI package**
   - File → Add Package Dependencies → **Add Local…**.
   - Select the `clients/apple/VelAPI` folder (the one containing `Package.swift`).
   - Add the **VelAPI** library to each app target.

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
    └── VelMac/            # macOS app sources
        ├── VelMacApp.swift
        └── ContentView.swift
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
