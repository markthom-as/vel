---
id: SK-010
title: Create self-knowledge maintenance and remediation loop
status: proposed
priority: P2
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Continuously maintain the self-knowledge system as the repo evolves.

# Tasks

1. Trigger incremental reindex on repo changes.
2. Recompute affected claims and drift reports after updates.
3. Generate remediation suggestions for stale docs / missing tests.
4. Optionally open internal tasks or draft patches behind policy gates.
5. Track validation timestamps for docs that were checked against code.

# Acceptance Criteria

- Repo changes invalidate and refresh only affected entities where practical.
- New drift findings appear after relevant changes.
- Suggestions include concrete target artifacts.
- Any automated write path remains policy-gated and disabled by default.

# Notes

This is where Vel stops being a static analyst and becomes a janitor with taste.

