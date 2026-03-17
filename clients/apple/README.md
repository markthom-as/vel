# Vel — Apple clients (iOS, watchOS, macOS)

Bootstrap for Vel clients on Apple platforms. All apps talk to the **same Vel daemon (veld)** over HTTP; no business logic in the client.

- **VelAPI** — Swift package (shared): HTTP client and models for the veld API.
- **VeliOS** — iPhone: context, nudges, commitments, quick capture, offline cache + queued actions.
- **VelWatch** — Apple Watch: brief “what matters now”, nudge count, cached fallback.
- **VelMac** — macOS: context, nudges, commitments, quick capture, offline cache + queued actions (sidebar layout), plus local activity/health/messages snapshot export into Vel’s Application Support tree.

Specs: [vel-apple-and-voice-client-spec](../../docs/specs/vel-apple-and-voice-client-spec.md), [vel-rust-swift-boundary-spec](../../docs/specs/vel-rust-swift-boundary-spec.md) (Rust = brain, Swift = body; API-first), [vel-apple-offline-mode-spec](../../docs/specs/vel-apple-offline-mode-spec.md) (cache + action queue + stale-aware offline mode). Repo guidance: [docs/specs/vel-detailed-next-steps-and-ios-repo-guidance.md](../../docs/specs/vel-detailed-next-steps-and-ios-repo-guidance.md) (Apple clients in same repo, under `clients/apple/`).

## Prerequisites

- Xcode 15+ (Swift 5.9)
- **veld** running (e.g. `veld` in the repo root, or `cargo run -p veld`). Default bind address: `127.0.0.1:4130` (base URL `http://127.0.0.1:4130`).

On Linux or any host without Xcode, the repo Nix shell now includes the Swift toolchain for package-level checks:

```bash
make check-apple-swift
```

This is useful for validating the shared `VelAPI` package, but full app builds still require Xcode on macOS.

On a physical device, set the daemon base URL (e.g. `http://<your-mac-ip>:4130`) in app settings/UserDefaults.

## Endpoint resolution

The Apple clients resolve endpoint candidates in this order:

1. `vel_base_url` (manual override)
2. `vel_tailscale_url` (tailnet endpoint, preferred for cross-device)
3. `vel_lan_base_url` (local network fallback)
4. `http://127.0.0.1:4130`
5. `http://localhost:4130`

Use `vel_tailscale_url` for day-to-day multi-device use. Keep `vel_base_url` for explicit temporary overrides.

## Offline cache + queue behavior

When the daemon is unreachable, clients keep rendering cached data and queue low-risk actions:

- cached surfaces: current context, nudges, commitments
- queued actions: nudge `done`, nudge `snooze`, commitment `done`, commitment `create`, capture `create`

Queued actions are drained automatically on next successful reachability/bootstrap check.

Current guardrails:

- queued actions are replayed in insertion order
- on first replay failure, remaining actions stay queued for next retry
- cache is best-effort and can be overwritten by canonical server state after reconnect

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

Current behavior:

- activity writes a lightweight local heartbeat from the running app
- health writes a same-day summary snapshot when HealthKit is available and authorization is granted
- messages writes a recent-thread snapshot from the local Messages database, then asks `veld` to sync it when the daemon is reachable

Current caveats:

- Health export is snapshot-based through HealthKit, not a long-running background sync
- Messages export reads `~/Library/Messages/chat.db` via `sqlite3`; on macOS this may require Full Disk Access depending on host policy
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
- `POST /v1/sync/actions` — queued low-risk client actions (`nudge_done`, `nudge_snooze`, `commitment_done`, `commitment_create`, `capture_create`).

VelAPI can be extended with more endpoints (commitments, captures, explain, risk, etc.) as needed.

## Design notes

- **One brain**: All logic (inference, risk, nudges) lives in veld; clients only display and send actions.
- **Offline mode**: Apple clients now cache context/nudges/commitments and queue user actions locally; `VelMac` also exports local activity/health/messages snapshots for daemon sync. The clients still must not fork local policy or inference.
- **Calm, analytical**: Spec calls for non-chirpy, non-preachy tone; Watch stays ultra-brief.
- **Same repo**: Apple clients live here (not a separate repo) until the core API and release cadence are stable.
