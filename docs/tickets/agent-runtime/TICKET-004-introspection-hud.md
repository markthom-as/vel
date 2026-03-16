---
title: Introspection + HUD
status: open
---

# Goal

Expose run introspection (snapshot endpoint, uncertainty events, active task tree, confidence display) for debug UI and HUD.

# Tasks

1. Run snapshot endpoint — return current objective, plan step, confidence, blockers, recent tool calls, pending decisions, uncertainty_flags for a run.
2. Uncertainty events — log uncertainty_events (run_id, class, description, resolution); support classes: missing_data, conflicting_evidence, stale_context, policy_ambiguity, tool_failure, low_model_confidence.
3. Active task tree — expose tree of runs (parent/child) with status and summary for HUD.
4. Confidence display — surface confidence in snapshot and in run list for trust and supervision.

# Acceptance Criteria

- Introspection snapshot available per run.
- Uncertainty events logged and queryable.
- Active task tree available for UI.
- Confidence visible in run metadata.

# Spec reference

[docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md) — Introspection Hooks, Uncertainty Escalation, Observability Schema (uncertainty_events).
