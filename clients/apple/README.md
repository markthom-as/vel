# Vel — Apple clients (iOS, watchOS, macOS)

Bootstrap for Vel clients on Apple platforms. All apps talk to the **same Vel daemon (veld)** over HTTP; no business logic in the client.

- **VelAPI** — Swift package (shared): HTTP client and models for the veld API.
- **VeliOS** — iPhone: context, nudges, commitments, quick capture, offline cache + queued actions.
- **VelWatch** — Apple Watch: brief “what matters now”, nudge count, cached fallback.
- **VelMac** — macOS: context, nudges, commitments, quick capture, offline cache + queued actions (sidebar layout).

Specs: [vel-apple-and-voice-client-spec](../../docs/specs/vel-apple-and-voice-client-spec.md), [vel-rust-swift-boundary-spec](../../docs/specs/vel-rust-swift-boundary-spec.md) (Rust = brain, Swift = body; API-first), [vel-apple-offline-mode-spec](../../docs/specs/vel-apple-offline-mode-spec.md) (cache + action queue + stale-aware offline mode). Repo guidance: [docs/specs/vel-detailed-next-steps-and-ios-repo-guidance.md](../../docs/specs/vel-detailed-next-steps-and-ios-repo-guidance.md) (Apple clients in same repo, under `clients/apple/`).

## Prerequisites

- Xcode 15+ (Swift 5.9)
- **veld** running (e.g. `veld` in the repo root, or `cargo run -p veld`). Default bind address: `127.0.0.1:4130` (base URL `http://127.0.0.1:4130`).

On Linux or any host without Xcode, the repo Nix shell now includes the Swift toolchain for package-level checks:

```bash
make check-apple-swift
```

This is useful for validating the shared `VelAPI` package, but full app builds still require Xcode on macOS.

On a physical device, set the daemon base URL (e.g. `http://<your-mac-ip>:4130`) in the app or via UserDefaults key `vel_base_url` until a Settings bundle is added.

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
- `GET /v1/context/current` — current context (mode, morning_state, meds_status, etc.).
- `GET /v1/nudges` — list nudges.
- `POST /v1/nudges/:id/done` — mark nudge done.
- `POST /v1/nudges/:id/snooze` — snooze nudge (body: `{ "minutes": 10 }`).

VelAPI can be extended with more endpoints (commitments, captures, explain, risk, etc.) as needed.

## Design notes

- **One brain**: All logic (inference, risk, nudges) lives in veld; clients only display and send actions.
- **Offline mode**: Apple clients now cache context/nudges/commitments and queue user actions locally; they still must not fork local policy or inference.
- **Calm, analytical**: Spec calls for non-chirpy, non-preachy tone; Watch stays ultra-brief.
- **Same repo**: Apple clients live here (not a separate repo) until the core API and release cadence are stable.
