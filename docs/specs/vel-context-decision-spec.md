# Vel Context Awareness & Decision Trace System
Version: 1.0
Date: 2026-03-16

## Purpose

This spec describes an extension path for context inspection, decision tracing, and feedback handling in Vel.

It does **not** define a second authoritative user-state system.

## Current Runtime Boundary

Per `docs/status.md`, the current runtime authority for present-tense context is the existing:

- `current_context`
- `context_timeline`
- `explain/*` surfaces
- reducer/inference flow that writes and explains current context

Any work derived from this spec must extend that runtime or its explainability surfaces. It must not introduce a competing belief ledger, a parallel inference engine, or a second independently authoritative context model unless a broader architecture decision explicitly replaces the current stack.

## Current vs Planned

### Current

Vel already has:

- a persistent current-context record
- a context timeline for material transitions
- explain surfaces for context, drift, nudges, and commitments
- signal-, risk-, and policy-informed inference

### Planned Extensions

The ideas in this spec are best interpreted as support for:

1. richer inspection of the existing context runtime
2. structured decision-trace artifacts
3. confidence and uncertainty metadata
4. user feedback loops around existing context decisions

Where this spec uses terms like `belief`, treat them as a possible supporting representation for inspection or explanation, not as a new top-level authority.

## Core Principles

Vel should expose:

- structured summaries of current context decisions
- confidence and uncertainty where useful
- decision reasoning summaries
- correction and suppression mechanisms

Vel should not expose raw chain-of-thought reasoning tokens.

Instead it should store structured reasoning artifacts attached to the current context and explain runtime.

---

# Architecture Direction

Preferred shape:

User Interface
        |
Context Inspector / Explain Surfaces
        |
Existing Current Context Runtime
        |
Optional Supporting Inspection Metadata
        |
Decision Trace Artifacts

If additional stores are introduced, they should support inspection, feedback, or uncertainty handling around the existing reducer/runtime.

---

# Supporting Context Belief Model

This section describes a possible supporting representation for inspectable context assertions.

It should be treated as:

- optional
- subordinate to the existing current-context runtime
- useful for explanation, uncertainty, and feedback

Example assertion:

"You are preparing to leave for a meeting"

Useful metadata includes:

- confidence
- source signals
- temporal scope
- epistemic status

If such entries are stored, they should be sortable and inspectable, but they should not replace the authoritative `current_context` output.

---

# Belief Types

activity
intent
location
constraint
preference
routine
relationship
device_state
affect

---

# Epistemic Status

observed
derived
predicted
preference
policy

---

# Belief Data Model

```ts
type ContextBelief = {
  id: string
  category: string
  label: string
  value: any
  confidence: number
  epistemicStatus: "observed" | "derived" | "predicted" | "preference" | "policy"
  sources: Source[]
  scope: {
    kind: "now" | "session" | "day" | "week" | "persistent"
    expiresAt?: string
  }
  userVerified: boolean
  actionable: boolean
  suppressed: boolean
  createdAt: string
  updatedAt: string
}
```

---

# Decision Trace Model

Every major Vel decision produces a trace record.

Example decision types:

- reminder generation
- suggestion
- task prioritization
- synthesis response

---

## DecisionTrace

```ts
type DecisionTrace = {
  id: string
  decisionType: string
  timestamp: string

  inputs: {
    beliefs: string[]
    retrievedMemories: string[]
    toolOutputs: string[]
  }

  candidates: {
    id: string
    label: string
    summary: string
    selected: boolean
  }[]

  rationale: {
    salientFactors: string[]
    assumptions: string[]
    uncertainties: string[]
  }

  confidence: {
    overall: "high" | "medium" | "low" | "unknown"
    fact: "high" | "medium" | "low" | "unknown"
    inference: "high" | "medium" | "low" | "unknown"
    action: "high" | "medium" | "low" | "unknown"
  }
}
```

---

# Confidence System

Vel uses confidence bands instead of pseudo‑precision.

High
Medium
Low
Unknown

Confidence dimensions:

- fact certainty
- inference certainty
- action suitability
- preference alignment

---

# UI Components

## Context Inspector Panel

Displays Vel's current beliefs.

Each belief shows:

- label
- confidence
- source chips
- last update

Actions:

Confirm  
Correct  
Not Relevant  
Suppress  
Explain  

---

## Decision Explanation

Every Vel action includes:

Why this action  
Confidence level  
Main uncertainties  

---

## Debug Inspector

Expanded view shows:

- retrieved context
- belief inputs
- candidate decisions
- reasoning summary
- decision confidence
- tool calls

---

# Feedback Loop

User actions produce training signals.

Confirm → strengthen inference

Correct → replace value and reduce weight of prior signals

Not Relevant → reduce salience

Suppress → prevent inference type

Manual entry → create explicit belief

---

# Learning Signals

Vel periodically analyzes:

- corrected beliefs
- low confidence decisions
- rejected suggestions

This informs:

- heuristic tuning
- inference calibration
- reminder timing
