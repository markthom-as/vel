---
id: VSM-019
title: Metrics Dashboard
status: proposed
priority: P2
owner: fullstack
labels: [metrics, dashboard, observability]
---

## Summary
Instrument and display proposal counts, approval rates, rollback rates, repeat-failure suppression, and subsystem hotspots.

## Why
If you can’t tell whether self-modification is helping, you are running an ideology, not an engineering practice.

## Scope
- Time-series metrics for proposals and outcomes.
- Breakdown by class, subsystem, owner, and environment.
- Highlight anti-metrics such as protected-core touch attempts.

## Implementation tasks
1. Define metrics schema.
2. Emit metrics from lifecycle events.
3. Build dashboard widgets/charts.
4. Add drill-down links into proposals and ledger events.
5. Add alerts for rollback spikes and protected-path attempts.

## Acceptance criteria
- Dashboard shows core operational health of self-improvement.
- Metrics can answer whether same failures recur after patches.
- Anti-metrics are visible, not buried.
- Data backs operator trust instead of demanding it on vibes.

## Dependencies
- VSM-004, VSM-015.

