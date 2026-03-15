# Using Vel Daily

A simple workflow for day-to-day use. Vel should optimize for **repeated personal use before broad generality**.

## Morning

- `vel morning` — orientation snapshot (run-backed; creates context artifact).
- `vel recent` — see what you captured recently (optionally `vel recent --today`).

## Throughout the day

- `vel capture "remember to test context refs"` — quick capture.
- `vel capture "check IRS mailing address" --type todo --source laptop` — typed capture.
- `echo "snippet from terminal" | vel capture -` or `vel capture --stdin` — capture from stdin.
- `vel search <query>` — lexical search over captures.

## End of day

- `vel end-of-day` — end-of-day context (run-backed).
- `vel review today` — count of captures today, recent list, latest context artifact.

## Weekly

- `vel review week` — recent captures and latest context.
- `vel synthesize week` — (planned) run-backed weekly synthesis artifact.
- `vel synthesize project vel` — (planned) run-backed synthesis for Vel-related captures.

## Inspection

- `vel recent --limit 20 --today` — recent captures with optional filters.
- `vel inspect capture <id>` — full capture detail.
- `vel inspect artifact <id>` — artifact detail (type, storage, size, hash).
- `vel artifact latest --type context_brief` — latest context artifact.
- `vel runs` / `vel run inspect <id>` — run list and detail (events, linked artifacts).

## Commitments (planned)

Vel will add a **commitments** layer: actionable, reviewable items (open/done/cancelled) with optional due date and project, sourced from typed captures, Todoist/Reminders, and eventually review/synthesis. “What matters today?” and “what’s unresolved?” will be answerable from commitments, not only from raw captures. See `docs/data-model.md` and `docs/roadmap.md`.

## Principle

> **Vel should optimize for repeated personal use before broad generality.**

That means: better daily loops before fancy automation; better capture/review ergonomics before agent complexity; better trust/export before speculative integrations.
