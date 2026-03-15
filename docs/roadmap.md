# Vel — Roadmap

## Bootstrap (done)

- import the core design docs
- scaffold the Rust workspace
- implement `veld` health and capture endpoints
- implement `vel` health, capture, and config commands
- establish SQLite migrations and local artifact paths
- run-backed context (today/morning/end-of-day)
- operator shell: recent, review, inspect, artifact latest, import, export, backup

## Near-term (dogfooding phase)

- **Commitments layer** — first-class commitments (open/done/cancelled) with due_at, project, source; distinct from raw captures.
- **Calendar integration** — event start/end, title, location; travel/commute awareness; prep duration for first meeting.
- **Todoist / Reminders integration** — task text, due date, completion state; open commitments, meds status, due-today pressure.
- **Workstation activity ingestion** — minimal signals: login time, first shell activity; morning-state inference.
- **Signal / inference / nudge engine** — first signals (calendar, Todoist, activity), inferred states (meds_logged, prep_window_active, morning_started), first nudges (meds not logged, prep window approaching, morning drift); done/snooze protocol.
- **Weekly and project synthesis** — run-backed `vel synthesize week` and `vel synthesize project vel`; recurring themes, unresolved commitments, artifacts and refs.
- **Trust improvements** — stronger export (manifest, refs, artifact metadata); backup as actual operation where possible; document backup as instructions where not.

## Later

- mobile clients
- optional AI sidecar
- sync between local nodes
- richer memory graph and context recall
- default `vel` entrypoint (morning briefing without subcommand)
