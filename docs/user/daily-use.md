# Vel Daily Use

Vel is strongest when used as a repeated local loop, not as a one-shot assistant.

## Morning

Start by orienting:

```bash
cargo run -p vel-cli -- morning
cargo run -p vel-cli -- standup
cargo run -p vel-cli -- nudges
```

Use `vel morning` for the bounded morning overview loop and `vel standup` for the one-to-three commitment pass. `vel context` and `GET /v1/context/morning` still help as legacy context briefs, but they are no longer the primary authority for the repeated morning/standup workflow.

What you are looking for:

- the current mode and morning state,
- what commitments are open,
- whether there are active nudges,
- whether recent source data looks fresh enough to trust.

## During the day

Capture quickly instead of trying to remember everything:

```bash
cargo run -p vel-cli -- capture "remember to send project update"
cargo run -p vel-cli -- capture "book dentist appointment" --type todo --source laptop
echo "snippet from terminal" | cargo run -p vel-cli -- capture -
```

Search or inspect when needed:

```bash
cargo run -p vel-cli -- search dentist
cargo run -p vel-cli -- recent --today
```

For coding-oriented project work, keep the execution handoff explicit and repo-local:

```bash
cargo run -p vel-cli -- exec save <project_id> --objective "ship the next safe slice" --constraint "sidecar only"
cargo run -p vel-cli -- exec preview <project_id>
cargo run -p vel-cli -- exec export <project_id>
cargo run -p vel-cli -- exec review --state pending_review
cargo run -p vel-cli -- exec launch-preview <handoff_id>
cargo run -p vel-cli -- exec approve <handoff_id> --reason "scope and output contract look right"
```

The execution export writes only a small sidecar pack under the project's primary repo root, by default at `.planning/vel/`, so supervised GSD-readable handoffs stay inspectable and bounded instead of mutating arbitrary repo files.
The review queue keeps human-to-agent and agent-to-agent coding handoffs explicit: objective, read scope, write scope, routing reasons, and review gate are persisted before any supervised runtime launch can proceed.

If you use the Apple clients during the day:

- iPhone Voice morning briefing should start or resume the backend daily-loop session, not a client-side morning heuristic.
- Apple Watch quick loops should reflect backend `/v1/now` schedule state plus the bounded Apple behavior summary.
- When offline, treat Apple surfaces as cached-render + queued-safe-action shells. Wait for reconnect before trusting new schedule or explainability answers, and do not treat cached morning/standup state as permission to invent a new local standup.

## Commitments and review

Use commitments as the actionable layer:

```bash
cargo run -p vel-cli -- commitments
cargo run -p vel-cli -- commitment add "finish expense report"
cargo run -p vel-cli -- commitment done <id>
```

Use nudges as prompts to review state, not as unquestioned commands:

```bash
cargo run -p vel-cli -- nudges
cargo run -p vel-cli -- nudge done <id>
cargo run -p vel-cli -- nudge snooze <id> --minutes 10
```

## Sync and refresh

If you know a source changed, sync it and evaluate:

```bash
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- evaluate
```

On macOS, if local source snapshots are already in `~/Library/Application Support/Vel/...`, `veld` may bootstrap them automatically at startup. Manual sync is still useful when you want an immediate refresh.

## Backup and trust check

Before risky local changes, confirm backup trust:

```bash
cargo run -p vel-cli -- doctor
cargo run -p vel-cli -- backup --create
cargo run -p vel-cli -- backup --verify <backup_root>
```

Use `cargo run -p vel-cli -- backup --dry-run-restore <backup_root>` if you need to rehearse recovery without touching live state.

## End of day

Use the end-of-day and review flows:

```bash
cargo run -p vel-cli -- end-of-day
cargo run -p vel-cli -- review today
```

Good uses:

- see what was captured,
- orient in Now before you react to scattered context,
- triage Inbox from the explicit action queue instead of guessing what is urgent,
- check pending writebacks, open conflicts, and people-linked review items from Now or Settings before trusting integration-backed edits,
- inspect what remained unresolved,
- notice response debt or drift,
- prepare a cleaner next start.

## Weekly

Use synthesis for a higher-level pass:

```bash
cargo run -p vel-cli -- review week
cargo run -p vel-cli -- synthesize week
cargo run -p vel-cli -- synthesize project vel
```

Use the weekly pass to review Projects weekly, not just raw capture volume. The JSON review output now includes typed project candidates so you can see which workspaces still have open commitments attached.
The review JSON also reports `pending_writebacks`, `open_conflicts`, and `people_needing_review` so the operator can see whether writeback or contact follow-up is waiting on manual supervision.

## Inspection and trust

When you need to know why Vel said something, inspect the underlying objects:

```bash
cargo run -p vel-cli -- runs --today
cargo run -p vel-cli -- run inspect <id>
cargo run -p vel-cli -- inspect artifact <id>
```

Vel is meant to be inspectable. If a conclusion matters, verify the source freshness and the related run or artifact instead of treating the surface output as magic.
That same rule applies to write lanes: SAFE MODE leaves writeback disabled by default, so the operator should explicitly enable it in Settings before expecting Todoist, GitHub, email, notes, or reminders mutations to apply.
