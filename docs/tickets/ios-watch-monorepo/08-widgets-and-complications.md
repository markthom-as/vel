---
id: APPLE-008
title: Implement iOS widgets and watch complications
status: proposed
owner: agent
priority: p1
area: widgets
depends_on: [APPLE-003]
---

# Goal

Expose just-enough ambient state via widgets/complications so Vel can haunt the edge of attention without becoming a nag daemon.

# Widget candidates

- Next due action
- Med status today
- Meeting preparation warning
- Risk badge / urgency band

# Watch complication candidates

- next due time
- simple status ring or count
- overdue badge

# Requirements

- widget extension under `Apps/VelWidgets`
- timeline generation sourced from local synced store
- refresh policy documented
- privacy-safe lock-screen behavior considered

# Acceptance criteria

- at least one iOS widget family implemented
- at least one watch complication implemented
- widget state degrades gracefully offline
- no direct network fetch inside widget timeline path unless explicitly justified
