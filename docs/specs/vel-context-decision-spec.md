# Vel Context Awareness & Decision Trace System
Version: 1.0
Date: 2026-03-16

## Overview

This system provides three capabilities:

1. **Inspectable Context Model**
2. **Decision Trace Logging**
3. **User Feedback Training Loop**

Vel maintains a structured model of the user's current context and uses it to guide LLM-driven decisions.
Users can inspect and correct Vel's understanding in real time.

## Core Principles

Vel exposes:

• Beliefs about the user's current context  
• Confidence and uncertainty  
• Decision reasoning summaries  
• Correction mechanisms

Vel does NOT expose raw chain-of-thought reasoning tokens.

Instead it stores structured reasoning artifacts.

---

# System Architecture

User Interface
        |
Context Inspector
        |
Belief Store
        |
Decision Engine
        |
LLM + Tools
        |
Decision Trace Store

---

# Context Belief Model

Vel maintains a dynamic set of beliefs about the user's environment and intentions.

Example belief:

"You are preparing to leave for a meeting"

Each belief contains metadata:

- confidence
- source signals
- temporal scope
- epistemic status

Beliefs are sorted by confidence and surfaced in the UI.

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
