---
title: Cross-Cutting Trait Baseline & Subsystem Audit
status: complete
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 011-documentation-truth-repair
labels:
  - architecture
  - repo-wide
  - quality-attributes
  - phase-1
---

# Context & Objectives

Vel now has explicit cross-cutting system traits: modularity, accessibility, configurability, data logging, rewind/replay, and composability.

This ticket makes those traits operational across the repository by auditing the major subsystems, identifying gaps, and formalizing the abstractions or follow-on tickets required to close them.

# Impacted Files & Symbols

- **Docs**: `docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md`
  - **Symbols**: subsystem application matrix
- **Docs**: `docs/templates/*.md`
  - **Symbols**: spec and ticket trait sections
- **Repository Surfaces**: `crates/*`, `clients/*`, `packages/*`
  - **Symbols**: subsystem ownership docs, manifests, queue references

# Technical Requirements

- **Trait Matrix**: Audit each major subsystem against all six traits.
- **Gap Classification**: Distinguish documentation-only gaps from architecture or implementation gaps.
- **Abstraction First**: Where a trait is missing, define the seam or contract that should own it before proposing large implementation work.
- **Queue Hygiene**: Add or patch tickets when gaps are material and not already covered.
- **N/A Discipline**: A trait may be marked not applicable only with an explicit reason.

# Cross-Cutting Trait Impact
- **Modularity**: required — define subsystem seams and ownership boundaries.
- **Accessibility**: required — account for operator, UI, CLI, docs, and machine-readable access.
- **Configurability**: required — require explicit defaults and effective-config inspection where behavior varies.
- **Data Logging**: required — require structured logs, events, traces, or denial records where behavior matters.
- **Rewind/Replay**: required — identify replay or reconstruction paths for sequence-sensitive workflows.
- **Composability**: required — define reusable contracts, manifests, or package boundaries instead of ad hoc coupling.

# Implementation Steps (The "How")

1. **Inventory**: Enumerate major subsystem groups across runtime, storage, clients, integrations, visual packages, and docs.
2. **Trait Audit**: Record current coverage and missing abstractions for each trait.
3. **Queue Update**: Patch existing tickets or add new ones for uncovered high-priority gaps.
4. **Review Loop**: Make the trait audit part of future architecture and ticket work.

# Acceptance Criteria

1. [ ] Every major subsystem is represented in a trait matrix or equivalent audit artifact.
2. [ ] Material trait gaps have corresponding queue coverage or an explicit deferral rationale.
3. [ ] New specs and tickets use the trait sections in the templates.
4. [ ] The Master Plan and concept pack reference the traits as repo-wide standards.

# Verification & Regression

- **Doc Check**: verify trait references exist in concept docs, templates, and planning docs
- **Queue Check**: verify uncovered high-priority gaps have explicit ticket coverage
- **Invariants**: trait applicability is explicit, not assumed or left implicit

# Agent Guardrails

- **No Checkbox Theater**: Do not mark traits as covered without naming the owning seam or mechanism.
- **Concrete Over Aspirational**: Gaps should produce abstractions or tickets, not vague future intent.
- **Repo-Wide Means Repo-Wide**: Do not limit the audit to Rust runtime code only; include clients, packages, and docs.
