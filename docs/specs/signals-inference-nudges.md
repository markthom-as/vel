# Signals, inference, and nudges — spec

Vel’s proactive behavior should be grounded in **explicit signals** and expressed as **reversible, inspectable nudges**. No invisible magic; every nudge explainable (what triggered it, which signals, what state was inferred, snoozed or completed).

## Design rule

> **Every proactive behavior should be grounded in explicit signals and expressed as a reversible, inspectable nudge.**

---

## First signal sources

1. **Calendar** — event start/end, title, location; travel/commute awareness; manual prep duration. Use: first meeting prep window, commute awareness, morning planning, lateness risk.
2. **Todoist / Reminders** — task text, due date/time, completion state, source identity. Use: open commitments, meds status, response obligations, due-today pressure.
3. **Computer activity** — minimal set: login time, first shell activity; optionally last activity heartbeat later. Use: workstation engagement, morning started, drift from wake / first meeting prep.

---

## First inferred states

- `meds_logged` / `not_logged`
- `first_commitment_upcoming`
- `prep_window_active`
- `morning_started` / `not_started`
- `behind_schedule` (carefully inferred)

---

## First nudges

- meds not logged
- first meeting prep window approaching
- morning routine drift

Do not expand beyond this set initially.

---

## Nudge state machine

Every nudge has only two meaningful responses: **done** and **snooze**.

States:

- `pending`
- `active`
- `snoozed`
- `resolved`

That is enough. Do not bloat. Maps across watch, desktop, CLI, toasts, speaker later and minimizes attentional tax.

---

## Done / snooze protocol

- **Done** — nudge is resolved; record resolution and contributing signals.
- **Snooze** — nudge is deferred; record snooze time and optionally next surface time.

Nudges should be explainable: what triggered, which signals contributed, what state was inferred, whether snoozed or completed.

---

## Escalation channels

(To be defined: how nudges surface on CLI, desktop, watch, toasts. Keep channels explicit and reversible.)
