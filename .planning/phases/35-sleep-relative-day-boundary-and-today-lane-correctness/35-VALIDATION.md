# 35 Validation

## Required Outcomes

- `Now` treats the active day as the current sleep-relative working day rather than a strict midnight reset
- past-midnight unfinished work, late-night events, and same-day commitments still appear in the active day until the sleep boundary changes
- next-event and today-lane ordering stay consistent with the same backend-owned day window
- thread resurfacing and continuity remain current-day relevant instead of flipping early at midnight

## Verification

- targeted Rust tests for day-boundary/current-day derivation and `GET /v1/now` schedule selection
- targeted web `NowView` tests for past-midnight continuity and commitment-first lane ordering
- targeted docs truth checks for current-day wording

## Acceptance Cases

- a late-night work session that continues past midnight still keeps unfinished commitments in the active day
- a nighttime calendar event that belongs to the current working day still appears as today until the sleep boundary changes
- if there is no active event after midnight, `Now` can still say “Free until …” inside the same sleep-relative day
- contextual resurfacing does not pull in stale yesterday threads just because they are recent
