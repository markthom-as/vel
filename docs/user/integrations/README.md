# Vel Integrations

This section covers the currently shipped integration paths that materially affect user setup and daily operation.

Vel currently uses a mix of:

- dedicated credential-backed integrations,
- bounded brokered-tool write lanes,
- local file-backed integrations,
- local snapshot-backed integrations,
- macOS auto-discovery for certain local sources.

Start with:

1. [Google Calendar](google-calendar.md) for OAuth-backed calendar sync.
2. [Todoist](todoist.md) for API-token-backed task sync.
3. [Local sources](local-sources.md) for file and snapshot-backed inputs.
4. [Apple and macOS local sources](apple-macos.md) for the current Apple-linked path.

Use the guide that matches the shell problem you are actually seeing:

- `Now` shows stale or missing Todoist-backed work:
  open [Todoist](todoist.md).
- `Settings` is asking you for a local path, notes root, snapshot file, or vault location:
  open [Local sources](local-sources.md).
- an Apple client cannot reach the daemon or macOS snapshot export/discovery is unclear:
  open [Apple and macOS local sources](apple-macos.md) and then [Vel Setup](../setup.md).
- the shell is healthy but you need implementation truth:
  use [MASTER_PLAN.md](../../MASTER_PLAN.md) first, then [runtime API](../../api/runtime.md) or the specific integration guide.

Phase 12 contract note:

- the web shell should route operators to these guides from existing typed shell and settings state
- there is no separate contextual-help API payload yet

Current bounded write lanes:

- Writeback starts disabled by default in SAFE MODE. Enable it from Settings only when you want Vel to move from read-only review into applying bounded external mutations.
- GitHub is limited to `github_create_issue`, `github_add_comment`, `github_close_issue`, and `github_reopen_issue`. Those writes carry typed `project_id` and person-alias linkage when Vel can resolve them.
- Email is draft-first. `email_create_draft_reply` is the safe default and `email_send_draft` is confirm-required before the runtime marks it applied.
- Now and Settings surface pending writebacks, open conflicts, and people-linked review status so you can inspect the queue before enabling or trusting a write lane.

Important truth:

- the repo has many integration specs, but only some integration paths are actually shipped
- trust [MASTER_PLAN.md](../../MASTER_PLAN.md) for current implementation truth
- use the Settings surface or `vel config show` to verify effective configuration
