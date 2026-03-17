# Vel Integrations

This section covers the currently shipped integration paths that materially affect user setup and daily operation.

Vel currently uses a mix of:

- dedicated credential-backed integrations,
- local file-backed integrations,
- local snapshot-backed integrations,
- macOS auto-discovery for certain local sources.

Start with:

1. [Google Calendar](google-calendar.md) for OAuth-backed calendar sync.
2. [Todoist](todoist.md) for API-token-backed task sync.
3. [Local sources](local-sources.md) for file and snapshot-backed inputs.
4. [Apple and macOS local sources](apple-macos.md) for the current Apple-linked path.

Important truth:

- the repo has many integration specs, but only some integration paths are actually shipped
- trust [MASTER_PLAN.md](../../MASTER_PLAN.md) for current implementation truth
- use the Settings surface or `vel config show` to verify effective configuration
