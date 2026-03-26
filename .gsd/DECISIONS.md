# Decisions Register

<!-- Append-only. Never edit or remove existing rows.
     To reverse a decision, add a new row that supersedes it.
     Read this file at the start of any planning or research phase. -->

| # | When | Scope | Decision | Choice | Rationale | Revisable? | Made By |
|---|------|-------|----------|--------|-----------|------------|---------|
| D001 | 2026-03-26 | product | Primary UI shape | `Now + Inbox` remain the primary UI | Daily orientation matters more than project dashboards | Yes | collaborative |
| D002 | 2026-03-26 | architecture | Project substrate | Projects are local-first typed records with one primary repo and notes root plus optional secondary links | Matches the current workflow while preserving a clean substrate | Yes | collaborative |
| D003 | 2026-03-26 | integration | Upstream authority | Upstream systems remain authoritative for their own records | Vel should add safe net-new writes and compatible updates, but prompt on conflicts | Yes | collaborative |
| D004 | 2026-03-26 | architecture | Planning-tool migration path | GSD integration should begin through repo-local docs/context that GSD already consumes | Lowest-friction path to coding-first supervised execution | Yes | collaborative |
| D005 | 2026-03-26 | architecture | Daily-loop evolution | Daily-loop value should be built from the shipped `Now`/calendar/Todoist/voice seams instead of inventing a separate planning subsystem | Maximizes value from already-landed foundations and avoids a parallel architecture | Yes | collaborative |
| D006 | 2026-03-26 | architecture | Cross-shell assistant seam | One Rust-owned grounded assistant seam should power text, voice, daily-loop continuity, and thread escalation across shells | Prevents Apple/web/desktop from re-implementing assistant behavior | Yes | collaborative |
| D007 | 2026-03-26 | product | `Now` surface law | `Now` should be an execution-first current-day control surface rather than a dashboard | Keeps the primary operator surface centered on what is happening now and what matters next | Yes | collaborative |
| D008 | 2026-03-26 | planning | Active milestone boundary | `v0.5.8` should stabilize GSD workflow truth before larger feature work resumes | Planning state must be trustworthy before reopening bigger feature delivery | Yes | collaborative |
