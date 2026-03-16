
# Vel Predicate / Assertion Cognition Layer

Date: 2026-03-16

## Purpose

Define a **predicate-based cognition layer** for Vel that allows the system to track and reason about:

- Vel's own internal state
- The user's contextual state
- Commitments, obligations, and deadlines
- World state (tasks, documents, events, tools)

This layer enables Vel to move beyond conversational memory into **structured, inspectable cognition**.

The system stores **revisable symbolic commitments** rather than immutable truths.

Core properties:

- provenance
- temporal validity
- confidence scores
- conflict detection
- derivations from rules

---

## Key Concepts

Vel operates across three knowledge tiers:

### Observations

Raw events recorded from external inputs.

Examples:

- user messages
- tool responses
- document scans
- calendar events

Observations are append‑only.

### Assertions

Normalized statements Vel currently believes about the world.

Examples:

- user_prefers(morning_meetings)
- commitment_due(grant_application, 2026‑04‑15)
- task_blocked(api_key_rotation)

Assertions include:

- confidence
- source
- timestamp
- validity window

### Derivations

Rule‑generated interpretations.

Examples:

- risk_of_missing_deadline(task_7)
- user_overloaded(today)
- needs_clarification(spec_12)

These are recalculated whenever the assertion graph changes.

---

## Core State Domains

Vel tracks four primary domains.

### Self State

Represents Vel's internal operational status.

Examples:

- active_goal
- unresolved_ambiguity
- context_pressure
- tool_availability

### User State

Represents inferred or declared conditions about the user.

Examples:

- user_energy
- user_mode
- user_preferences
- user_constraints

### Commitment State

Tracks obligations and goals.

Examples:

- commitments
- deadlines
- dependencies
- stakeholders

### World State

Facts about the external environment.

Examples:

- tasks
- documents
- agents
- calendar events
