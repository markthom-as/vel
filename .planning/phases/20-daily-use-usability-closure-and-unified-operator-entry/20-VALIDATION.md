# Phase 20 Validation Notes

## Locked Boundaries

- `Now` remains the compact urgent-first orientation and routing surface.
- `Inbox` remains the explicit triage queue over shared action/intervention state.
- `Threads` remains continuity, archive, and searchable follow-up, not a second inbox.
- `Settings` remains advanced setup and trust disclosure; default daily use should not become settings-first.
- The backend owns assistant policy, routing, tool use, and continuity decisions.

## Explicit Scope Guards

- Start with the existing `ThreadView` placeholder/copy drift as the first stabilization slice.
- Reuse existing conversation/thread records; do not invent a separate assistant archive.
- Preserve optional configured remote routing, including localhost `openai_oauth`, but keep basic entry useful when no model is configured.
- Do not plan Apple voice parity or assistant-capable daily-loop/closeout closure beyond the minimum seam needed for later phases.

## Execution Checks

- Every plan must keep shell behavior thin and route through Rust-owned seams.
- Any new assistant entry contract must return enough typed continuity data for web shells to land in `Threads` without client-local routing guesses.
- Web tests for `ThreadView`, `MessageComposer`, `NowView`, and `InboxView` should be updated intentionally, not implicitly through copy drift.
- Docs must continue to distinguish implemented Phase 20 closure from later voice or daily-loop assistant phases.

## Requirement Coverage Map

- `20-01`: `ENTRY-01`, `THREADS-UX-01`
- `20-02`: `ASSIST-01`, `ASSIST-02`, `ENTRY-01`, `THREADS-02`
- `20-03`: `USABLE-01`, `NOW-UX-01`, `ENTRY-01`, `ASSIST-01`, `THREADS-02`
- `20-04`: `USABLE-02`, `INBOX-UX-01`, `THREADS-UX-01`, `SETTINGS-UX-01`
