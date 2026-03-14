# AGENTS.md

This document defines how AI coding agents (such as Codex) should interact with the Vel repository.

## Mission

Vel is a **local-first personal executive system** designed to maintain life context, memory continuity, and goal alignment for its user.

AI agents working in this repository should prioritize:

1. clarity of architecture
2. local-first design
3. modular systems
4. long-term maintainability
5. user privacy and data ownership

---

## Development Principles

### Local-First

Vel must operate without reliance on external services whenever possible.

Remote APIs and SaaS models should be optional.

---

### Modular Architecture

Subsystems should be separated into modules such as:

- capture
- memory graph
- alignment engine
- execution layer
- interfaces

Modules must communicate through clearly defined interfaces.

---

### Data Ownership

User data must always remain under user control.

Storage should default to:

- local databases
- filesystem storage
- user-controlled infrastructure

---

### Explainability

Vel decisions should be traceable.

When Vel suggests actions it should be possible to determine:

- which context it used
- which rules triggered the suggestion

---

## Coding Style Expectations

- prefer readable code over clever code
- include documentation for all modules
- avoid unnecessary dependencies
- ensure reproducible builds

---

## AI Agent Workflow

Agents contributing to the repository should:

1. read `docs/vel-product-spec.md`
2. read `docs/vel-architecture.md`
3. read `docs/vel-data-model.md`
4. read `docs/vel-mvp.md`
5. implement minimal viable components first
6. write tests where appropriate
7. document new modules

---

## Priority Order for Implementation

1. capture system
2. memory graph
3. context recall
4. daily alignment engine
5. execution automation

---

## Non-Goals (Early Versions)

Early versions of Vel should avoid:

- complex distributed systems before needed
- unnecessary cloud dependencies
- premature optimization
- excessive UI complexity
- speculative productization features

The priority is **core cognition features**.
