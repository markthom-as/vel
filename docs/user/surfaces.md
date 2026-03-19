# Vel Surfaces

This guide explains the main user-visible Vel surfaces and how to interpret them.

These descriptions are intentionally tied to shipped behavior. For implementation truth, see [MASTER_PLAN.md](../MASTER_PLAN.md).

## `veld`

`veld` is the daemon. It owns:

- the local API,
- persistence,
- signal ingestion,
- inference,
- nudges,
- synthesis,
- current context.

If `veld` is down, the rest of the system degrades.

## `vel`

`vel` is the main operator CLI.

Use it for:

- captures,
- search,
- commitments,
- nudges,
- sync and evaluate,
- reviews,
- run and artifact inspection,
- diagnostics.

If you want the most direct view of what Vel is doing, the CLI is still the strongest surface.

## Current context

Current context is Vel’s continuously maintained model of the present.

It is where Vel records things like:

- current mode,
- morning state,
- meds status,
- risk summary,
- source-derived summaries,
- attention and drift,
- what seems most relevant right now.

Important rule:

- current context is derived state, not a manual notes field

If it looks wrong, fix the upstream source or rerun `evaluate` rather than trying to patch the context by hand.

## `Now`

The web `Now` view is the main “what matters right now” surface.

Today it shows:

- top-level summary cards for mode, phase, meds, and risk
- upcoming events from persisted calendar signals
- Todoist-backed task backlog
- freshness status across the current state
- direct recovery actions like `Re-run evaluate`, `Sync calendar`, `Sync Todoist`, `Sync activity`, and `Sync messaging` when appropriate

Treat it as a decision surface, not as magic truth. If freshness is degraded, use the recovery actions before trusting it.

## Projects

Projects are a supporting coordination surface, not the home screen.

They are backed by typed project records in the runtime and currently exist to anchor:

- stable project IDs,
- family grouping,
- primary repo and notes-root context,
- pending upstream creation intent.

The stable project families are:

- `Personal`
- `Creative`
- `Work`

Use Projects to inspect or create a local-first workspace record. Keep `Now` and `Inbox` as the primary operating loop.

## Nudges

Nudges are lightweight prompts generated from context and policy.

Current shipped nudge types include:

- meds-related reminders
- meeting prep reminders
- morning drift
- response debt

Use nudges as review prompts. They are meant to surface risk or neglected state, not to replace judgment.

## Commitments

Commitments are the actionable layer.

They represent open or resolved things that matter operationally. Some are created directly, and some arrive through typed captures or integrations.

Use commitments when the question is:

- what am I on the hook for,
- what is still open,
- what changed from open to done,
- what depends on what.

## Captures

Captures are the fast intake surface.

Use captures for:

- thoughts,
- reminders,
- fragments,
- tasks,
- raw notes you do not want to lose.

Vel can later promote or connect captures into more structured state, but capture first remains the right default.

## Context panel

The web Context panel is the explain-oriented inspection surface for current context.

Today it shows:

- computed time,
- top-level context fields,
- attention/drift state when available,
- “why this context” reasons,
- source summaries,
- signal summaries used

Use it when you need to understand why the current state looks the way it does.

## Suggestions

Suggestions are evidence-backed steering proposals.

They are not just reminders. They represent candidate actions or adjustments that the system believes may be useful enough to surface explicitly.

Use the Suggestions surface when you want:

- a more explicit steering layer than nudges,
- detail on a specific suggestion,
- accept/reject control with evidence.

If there are no pending suggestions, that is a normal state.

## Settings and integrations

The Settings surface is where you manage source configuration and integration recovery.

Today it includes:

- Google Calendar connection and sync
- Todoist token save/disconnect/sync
- local path-backed integrations for activity, git, messaging, reminders, notes, and transcripts
- integration history, last sync state, and guidance

Use Settings when the issue is configuration, credentials, or source path, not when the issue is just stale derived context.

## Runs and artifacts

Runs and artifacts are the strongest inspectability surfaces.

Use them when you need to answer:

- what happened,
- when it happened,
- what was generated,
- whether a context or synthesis output is durable and inspectable.

They are especially useful when a high-level surface seems wrong and you need the underlying execution record.
