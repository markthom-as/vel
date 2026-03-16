---
title: Vel Task HUD system spec
status: ready
owner: product+engineering
priority: P0
---

# Vel Task HUD

## Summary
Vel should expose a task HUD that behaves less like a generic todo list and more like a live operational register of commitments under attention constraints.

The HUD should answer:
- What matters now?
- What is becoming urgent?
- What is blocked?
- What is drifting?
- What ritual or prep action is due?
- What risk is emerging?

## Product goals
- Make active commitments glanceable.
- Reduce executive-function friction.
- Expose task pressure, not just task existence.
- Support cross-surface rendering: desktop full panel, desktop compact HUD, mobile/watch glance mode, future AR overlay.
- Integrate with context, reminders, and the risk engine.

## Non-goals
- Replacing the entire note system.
- Building a full project management product before basic HUD semantics exist.
- Rendering dense task trees in compact or AR surfaces.

## Core model
Tasks are stateful commitments with provenance, timing, decay, and policy.

### Canonical task states
- Pending
- Active
- Blocked
- Waiting
- Completed
- Cancelled

### Canonical task kinds
- Task
- Ritual
- Reminder
- Followup
- Checklist

### Canonical HUD groups
- Now
- Soon
- Waiting
- Drifting
- Ritual
- Threats

## Suggested schema
```rust
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub kind: TaskKind,
    pub source: TaskSource,
    pub priority: Priority,
    pub urgency: Urgency,
    pub estimated_duration: Option<Duration>,
    pub energy_required: Option<EnergyLevel>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub blocked_by: Vec<TaskId>,
    pub depends_on: Vec<TaskId>,
    pub last_touched_at: DateTime<Utc>,
    pub context_refs: Vec<ContextRef>,
    pub attention_score: f32,
    pub commitment_score: f32,
    pub lateness_risk: f32,
    pub decay_state: DecayState,
    pub visibility_mode: VisibilityMode,
}
```

## Ranking
Attention score should be derived, not manually curated in most cases.

Base factors:
- urgency
- deadline proximity
- dependency pressure
- lateness risk
- decay pressure
- priority
- contextual fit

Negative factors:
- snoozed
- blocked with no action possible
- recently completed
- hidden by policy
- redundant sibling tasks

## HUD policy
Only glanceworthy tasks should show in compact or ambient modes.

Allow if:
- high attention score
- deadline soon
- ritual due
- risk detected
- user pinned task

Disallow if:
- completed
- snoozed
- blocked with no user-actionable next step
- low urgency backlog
- explicitly hidden

## Surfaces

### Full panel
Canonical UI. Supports grouping, search/filter later, full metadata, and quick actions.

### Compact HUD
Always-on-top desktop surface. Show top 1-3 active items, one risk, one ritual, and fast actions.

### Ambient mode
Minimal pressure field, not a dense list. Resolve into text only on interaction.

### Watch/mobile glance
Single compressed payload:
- current task
- next task
- top risk
- next ritual

### AR
Do not port full desktop UI. Use the same core semantics with far tighter compression.

## Fast actions
- Complete
- Snooze
- Start
- Break down
- Block
- Defer
- Pin to HUD
- Hide

## Inference sources
- Manual entry
- Calendar
- Email
- Agent suggestion
- System inference

## Notifications
Must be policy-bound, sparse, and escalation-aware.
- no repeated spam
- strict snooze respect
- escalate only if risk increases or context changes materially

## Suggested crate layout
```text
crates/
  vel-task-core/
  vel-task-actions/
  vel-task-ranking/
  vel-task-policy/
  vel-task-hud/
  vel-task-inference/
```

## Rollout plan
1. Task core + migrations
2. Ranking + policy + actions
3. Full task panel + compact desktop HUD
4. Risk/ritual/inference integration
5. Voice + watch/mobile glance
6. Ambient mode
7. AR protocol spec

## Acceptance criteria
- Task model is persisted and test-covered.
- HUD grouping and ranking are deterministic enough to test.
- Desktop HUD exists behind a feature flag.
- Risk engine fields can influence ranking and display.
- At least one ritual flow and one inferred task flow exist.

## Implementation tickets

See [docs/tickets/task-hud/](../tickets/task-hud/README.md) for the ticket pack.

