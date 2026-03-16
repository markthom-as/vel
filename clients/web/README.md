# Vel Web Client

React + TypeScript + Vite + Tailwind. Chat interface for the Vel agent console.

## Run

```bash
# From repo root
npm run dev:web

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

Base styling and Tailwind are configured. Point the dev server at `veld` (default `http://localhost:3000`) for API and `/ws` for WebSocket.

## Seed data

With veld running, seed a sample conversation and cards:

```bash
npm run seed
# or: VEL_API_URL=http://localhost:3000 bash scripts/seed.sh
```
