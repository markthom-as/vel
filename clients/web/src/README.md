# Web client source (`clients/web/src`)

Optimized for **clear navigation** (humans and coding agents): three top-level UI buckets under `src/`, plus shared `data/`, `api/`, and `types/`.

## Layout

| Path | Purpose |
|------|---------|
| **`core/`** | Reusable UI and cross-view building blocks: `Icons/`, `Theme/`, `Cards/`, `SurfaceChips/`, `SurfaceState/`, `MarkdownMessage/`, `MessageComposer/`, `MessageRenderer/`. |
| **`shell/`** | Application chrome: `AppShell/`, `Navbar/`, `MainPanel/` (routes to views). |
| **`views/`** | Feature-aligned screens; each subfolder is one product surface. |

### `views/` map

| Folder | Surface |
|--------|---------|
| `views/now/` | Now |
| `views/inbox/` | Inbox |
| `views/threads/` | Threads shell plus thread-specific UI: `ThreadView/`, `ConversationList/`, `ProvenanceDrawer/` |
| `views/settings/` | Settings |
| `views/projects/` | Projects |
| `views/context/` | Info-column panels: `ContextPanel/`, `DocumentationPanel/` |

### Import conventions

- **Public API** of each package: `index.ts` in that folder (`export * from './MainFile'` is typical).
- **App entry:** `App.tsx` imports shell + `views/context` + `core/Icons`.
- **Shell → views:** `import { NowView } from '../../views/now'` (from `shell/MainPanel/`).
- **Views → core:** `import { SurfaceState } from '../../core/SurfaceState'` (from `views/<feature>/` — two levels down to `src/`, then `core/…`).
- **Threads sub-features** (`ConversationList/`, `ProvenanceDrawer/`): one extra `../` to reach `src/` (three directory levels under `views/threads/`).

### Relative depth rule

From any file under `src/`, count folders up to `src/`:

- `views/now/NowView.tsx` → **2** parents to `src/` → `../../data/…`, `../../core/…`.
- `views/threads/ThreadView.tsx` → **2** parents → same.
- `views/threads/ConversationList/ConversationList.tsx` → **3** parents → `../../../data/…`, `../../../core/…`.

## Where to start (agents)

1. `App.tsx` — wires `AppShell`, `Navbar`, `MainPanel`, info stack.
2. `shell/MainPanel/MainPanel.tsx` — `mainView` → which `views/*` to render.
3. `data/`, `types.ts` — API and domain types.
