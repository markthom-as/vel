# Vel Web Client

React + TypeScript + Vite + Tailwind. Vel's web operator surface for `Now`, `Inbox`, `Threads`, `Suggestions`, `Stats`, and `Settings`, with a truthful `Projects` placeholder until the project workspace API exists.

The `Settings` runtime tab owns implementation-aware controls such as loop toggles, recent-run recovery, daemon-side LLM routing, and the global daemon restart action.

## Run

```bash
# From repo root
npm run dev:web
npm run gallery:web

# Or from this directory
npm run dev
```

`npm run dev` now builds the embedded Rust bridge artifact first and starts Vite with:

```bash
VITE_VEL_EMBEDDED_BRIDGE_WASM_URL=/embedded-bridge/vel-embedded-bridge.js
```

## Build

```bash
npm run build:web   # from root
npm run build       # from clients/web
```

## Embedded bridge WASM

The web client now expects Rust to be the canonical authority for deterministic embedded packet shaping.

Build the browser artifact from `crates/vel-embedded-bridge` with:

```bash
npm run build:embedded-bridge-wasm
```

That emits a browser-targeted module under:

```bash
clients/web/public/embedded-bridge/
```

The default dev path already points Vite at the generated module. The equivalent manual form is:

```bash
VITE_VEL_EMBEDDED_BRIDGE_WASM_URL=/embedded-bridge/vel-embedded-bridge.js npm run dev
```

If that env var is unset, the web client fails closed when embedded packet calls are attempted instead of silently falling back to duplicated TypeScript shaping.

## Stack

- React 19
- TypeScript
- Vite 8
- Tailwind CSS v4 (@tailwindcss/postcss)

Base styling and Tailwind are configured. The app talks to veld via `VITE_API_URL` (default `http://localhost:4130`). From the repo root, `make dev` runs veld and this dev server together.

## Seed data

With veld running, seed a sample conversation and cards:

```bash
npm run seed
# or from repo root: make seed
# Override API: VEL_API_URL=http://localhost:4130 bash scripts/seed.sh
```
