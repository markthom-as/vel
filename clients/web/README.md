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

## Build

```bash
npm run build:web   # from root
npm run build       # from clients/web
```

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
