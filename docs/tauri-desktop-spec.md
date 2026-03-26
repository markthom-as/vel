# Tauri 2 Desktop Surface — Spec

**Status**: Proposal
**Goal**: Ship a double-click-to-run desktop app so non-technical testers can use Vel without touching a terminal.

---

## Motivation

The web client requires users to run `veld` separately and navigate to `localhost:5173`. For non-technical testers, this is a barrier. A Tauri 2 wrapper bundles `veld` as a sidecar and opens the web UI in a native window — single binary, zero setup.

---

## Architecture

```
┌─────────────────────────────────────┐
│         Tauri 2 Desktop App         │
│                                     │
│  ┌───────────────┐  ┌────────────┐  │
│  │  Native Shell │  │  Sidecar   │  │
│  │  (webview)    │  │  (veld)    │  │
│  │               │  │            │  │
│  │  loads React  │──│  :4130     │  │
│  │  from dist/   │  │  HTTP + WS │  │
│  └───────────────┘  └────────────┘  │
└─────────────────────────────────────┘
```

**Key decision: Sidecar, not embedded.**

`veld` runs as a child process managed by Tauri's shell plugin. This preserves the existing multi-surface architecture — CLI, Apple clients, and the browser all keep working against the same daemon. No refactoring of `veld/main.rs` required.

---

## Directory Layout

```
clients/
├── web/                  # existing React app (unchanged)
├── apple/                # existing Swift clients (unchanged)
└── desktop/              # NEW — Tauri 2 app
    ├── src-tauri/
    │   ├── Cargo.toml    # tauri v2, tauri-plugin-shell
    │   ├── tauri.conf.json
    │   ├── capabilities/
    │   │   └── default.json
    │   ├── src/
    │   │   └── lib.rs    # sidecar lifecycle + Tauri commands
    │   └── binaries/
    │       └── veld-*    # sidecar binary (per-platform)
    ├── package.json      # dev scripts, @tauri-apps/cli
    ├── vite.config.ts    # extends ../web config or points at ../web/dist
    └── index.html        # entry (may re-export from ../web)
```

The desktop surface reuses `clients/web` as-is. No fork, no copy. Either:
- **Option A**: Symlink/import — `desktop/vite.config.ts` points `root` at `../web`
- **Option B**: Build `clients/web` first, point Tauri's `frontendDist` at `../web/dist`

Option B is simpler and avoids path headaches. Recommended.

---

## Sidecar Lifecycle

### Startup
1. Tauri `setup()` hook spawns `veld` sidecar via `tauri-plugin-shell`
2. Sidecar binds to `127.0.0.1:{port}` (default 4130, configurable)
3. App waits for a health check (`GET /api/health`) before loading the webview
4. Webview loads the React app, pointed at `http://127.0.0.1:{port}`

### Shutdown
1. Tauri `on_exit` or window close event
2. Send SIGTERM to sidecar process
3. Sidecar gracefully drains connections and shuts down
4. If sidecar doesn't exit within 5s, SIGKILL

### Error Handling
- If sidecar fails to start: show native error dialog with log path
- If sidecar crashes mid-session: show reconnection banner, attempt restart
- Health check timeout: show "Starting Vel..." splash with spinner

---

## Tauri Configuration Sketch

```jsonc
// src-tauri/tauri.conf.json
{
  "productName": "Vel",
  "identifier": "com.vel.desktop",
  "build": {
    "frontendDist": "../../web/dist",
    "devUrl": "http://localhost:5173"
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "appimage", "nsis"],
    "icon": ["icons/icon.png"],
    "resources": ["binaries/*"]
  },
  "app": {
    "windows": [
      {
        "title": "Vel",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600
      }
    ],
    "security": {
      "dangerousRemoteUrlAccess": [
        { "url": "http://127.0.0.1:*" }
      ]
    }
  },
  "plugins": {
    "shell": {
      "sidecar": true
    }
  }
}
```

---

## Web Client Changes

Minimal. The React app already talks to `veld` via relative URLs (`/api/*`, `/v1/*`, `/ws`). In the Tauri context:

- **Dev mode**: Vite proxy continues to work as-is (Tauri opens `devUrl`)
- **Production**: The webview loads from `dist/`, API calls need to target the sidecar's absolute URL (`http://127.0.0.1:4130`)

**One change needed**: Make the API base URL configurable. Options:
1. Tauri injects `window.__VEL_API_URL__` at load time via IPC
2. The React app reads `import.meta.env.VITE_API_URL` with a fallback to `''` (relative, current behavior)

Option 2 is lower friction — set the env var at build time for the Tauri bundle, no runtime injection needed.

---

## Data & Storage

- SQLite database lives at a platform-appropriate path:
  - macOS: `~/Library/Application Support/com.vel.desktop/`
  - Linux: `~/.local/share/com.vel.desktop/`
  - Windows: `%APPDATA%/com.vel.desktop/`
- Sidecar receives `--db-path` and `--artifact-root` flags pointing to the app data dir
- Migrations run automatically on sidecar startup (existing behavior)

---

## Build & Distribution

### Dev Workflow
```sh
# Terminal 1: build web client in watch mode
cd clients/web && npm run dev

# Terminal 2: run Tauri dev (opens native window, spawns sidecar)
cd clients/desktop && cargo tauri dev
```

### Production Build
```sh
# 1. Build web frontend
cd clients/web && npm run build

# 2. Build veld sidecar for target platform
cargo build --release -p veld
cp target/release/veld clients/desktop/src-tauri/binaries/veld-{target-triple}

# 3. Build Tauri bundle
cd clients/desktop && cargo tauri build
```

### Makefile Targets (proposed)
```makefile
build-desktop:   build-web build-desktop-sidecar build-desktop-bundle
dev-desktop:     ## Tauri dev mode (requires dev-web running)
build-desktop-sidecar:  cargo build --release -p veld
build-desktop-bundle:   cd clients/desktop && cargo tauri build
```

---

## Platform Targets

| Platform | Bundle Format | Priority |
|----------|--------------|----------|
| macOS (ARM) | `.dmg` | P0 — primary dev/test platform |
| macOS (Intel) | `.dmg` | P1 |
| Linux (x86_64) | `.AppImage` | P1 |
| Windows (x86_64) | `.msi` / NSIS | P2 |

---

## Tauri Plugins (Start Minimal)

| Plugin | Purpose | Phase |
|--------|---------|-------|
| `tauri-plugin-shell` | Sidecar management | Day 1 |
| `tauri-plugin-notification` | OS notifications | Future |
| `tauri-plugin-autostart` | Launch on login | Future |
| `tauri-plugin-updater` | Auto-update | Future |
| `tauri-plugin-single-instance` | Prevent duplicate launches | Day 1 |

---

## Scope Boundaries

**In scope:**
- Tauri scaffold wrapping existing web client
- Sidecar lifecycle (start, health check, stop)
- Platform bundles (dmg, AppImage)
- Configurable API base URL in React app

**Out of scope (future):**
- System tray / menu bar mode
- Auto-update mechanism
- Native OS notifications
- Deep linking / protocol handler (`vel://`)
- Embedding veld in the Tauri Rust process
- CI/CD pipeline for desktop releases

---

## Decisions

1. **Sidecar binary naming**: **Makefile automation.** A Makefile target detects the current platform triple via `rustc -vV` and copies the built `veld` binary into `src-tauri/binaries/veld-{target_triple}` automatically.

2. **Port conflicts**: **Smart fallback with sensible defaults.** On startup:
   - Try to bind on the default port (4130)
   - If 4130 is in use, probe whether it's an existing `veld` instance (`GET /api/health`)
     - If yes: skip spawning a sidecar, connect to the running instance
     - If no: pick a random available port, pass it to the webview via IPC
   - Webview receives the resolved port before loading the React app

3. **First-run experience**: **Empty state.** Testers see the real onboarding experience. No auto-seeding.

4. **Code signing**: **Deferred.** Testers can right-click → Open to bypass macOS Gatekeeper. Signing will be set up before wider distribution.
