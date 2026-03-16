---
id: SK-004
title: Detect documentation and implementation drift
status: proposed
priority: P0
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Build a drift engine that flags likely mismatches between docs, code, tests, and config.

# Drift classes

- documented feature missing in code
- code feature missing in docs
- enum / command / API shape mismatch
- doc refers to deleted or renamed module
- examples inconsistent with current interfaces

# Tasks

1. Implement rule-based drift detectors for the highest-signal cases.
2. Compare docs against extracted symbols, commands, and enums.
3. Compare declared interfaces against tests/config where relevant.
4. Persist drift reports with severity and evidence.
5. Add text and machine-readable output.

# Acceptance Criteria

- `vel system drift` returns a structured report.
- At least one detector exists for undocumented command or module.
- At least one detector exists for stale doc references.
- Drift items include severity, evidence, and suggested remediation.

# Notes

Do not start with LLM-only drift detection. Begin with boring deterministic checks. Boring is underrated because it works.

