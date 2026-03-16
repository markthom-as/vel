---
title: Vel Uncertainty & Clarification Architecture Spec
status: proposed
owner: nav/core
created: 2026-03-15
priority: high
---

# Vel Uncertainty & Clarification Architecture Spec

## Why this exists

Vel should not treat uncertainty as an incidental side effect of LLM output. It should model uncertainty as a first-class runtime concern that can be observed, persisted, routed, resolved, and learned from.

Without this, Vel will fall into the two classic agent failure modes:

1. confident guessing under ambiguity
2. brittle over-escalation that pesters the user instead of thinking

The goal is a middle path: **Vel should proceed aggressively when safe, escalate precisely when necessary, and make its assumptions legible.**

## Product goals

- Let Vel ask the user for clarification only when the expected information gain justifies the interruption.
- Let Vel consult other agents before bothering the user when another resolver has better domain authority.
- Make uncertainty inspectable in UI and durable in task history.
- Separate model-generated uncertainty descriptions from deterministic routing policy.
- Build a learning loop so Vel gets better at calibration over time.

## Non-goals

- Perfect probabilistic calibration in v1.
- Full Bayesian belief tracking across every subsystem.
- Purely model-native confidence handling with no policy code.
- Solving every ambiguity through user interaction.

## Design principles

### 1. Uncertainty is an object, not a vibe

Every meaningful agent step should be able to emit structured uncertainty records.

### 2. Confidence is decomposed

Do not rely on a single scalar invented by the model. Track a confidence vector and derive policy decisions from its components.

### 3. Routing is deterministic

The model may identify ambiguity, propose assumptions, and suggest questions. Policy code decides whether to proceed, ask, defer, simulate, or block.

### 4. Escalation should be information-efficient

Vel should ask the smallest question that collapses the most uncertainty.

### 5. Resolution should prefer cheaper resolvers first

Typical preference order:

1. self-resolution via retrieval or local inspection
2. dry-run / simulation / validation
3. specialist agent
4. user

This order can be overridden when the uncertainty is inherently normative or preference-dependent.

### 6. Uncertainty should survive handoffs

All open assumptions, unresolved ambiguities, and past clarifications should persist with the task so agents do not repeatedly rediscover the same confusion.

## Uncertainty taxonomy

Vel should support at least these categories in v1:

### Epistemic uncertainty
Unknown or incomplete facts.

Examples:
- missing requirements
- weak retrieval evidence
- unclear repo conventions
- stale assumptions about external systems

### Procedural uncertainty
Known goal, unclear best way to reach it.

Examples:
- multiple plausible implementation strategies
- unclear migration order
- uncertain rollback path

### Normative uncertainty
Unclear optimization target or value judgment.

Examples:
- elegance vs minimal diff
- UX smoothness vs explicit approval
- performance vs readability

### Predictive uncertainty
Unclear whether the proposed action will work.

Examples:
- likely flaky tests
- suspicious patch application
- dependency mismatch risk

### Social / authority uncertainty
Unclear who should decide.

Examples:
- product decision vs engineering decision
- user preference vs org convention
- destructive action needing explicit approval

## Core runtime entities

### AgentAssessment

```ts
export type AgentAssessment = {
  overallConfidence: number; // 0..1
  actionability: number; // 0..1, can we still proceed safely?
  confidenceVector: ConfidenceVector;
  uncertainties: UncertaintyItem[];
  recommendedAction:
    | "proceed"
    | "proceed_with_annotation"
    | "ask_user"
    | "ask_agent"
    | "retrieve_more"
    | "simulate"
    | "block";
  rationale: string;
};
```

### ConfidenceVector

```ts
export type ConfidenceVector = {
  intentClarity: number;
  contextCompleteness: number;
  evidenceQuality: number;
  patternMatchStrength: number;
  executionSafety: number;
  reversibility: number;
  validationCoverage: number;
};
```

### UncertaintyItem

```ts
export type UncertaintyItem = {
  id: string;
  kind:
    | "missing_context"
    | "ambiguous_intent"
    | "conflicting_evidence"
    | "weak_pattern_match"
    | "high_change_risk"
    | "ownership_unknown"
    | "preference_unknown"
    | "tool_instability"
    | "external_dependency_unknown";
  category:
    | "epistemic"
    | "procedural"
    | "normative"
    | "predictive"
    | "authority";
  severity: "low" | "medium" | "high" | "blocking";
  confidenceImpact: number; // 0..1
  scope: "local_step" | "task" | "plan" | "repo" | "external";
  evidence: EvidenceRef[];
  candidateResolvers: ResolverCandidate[];
  suggestedQuestion?: string;
  expiresAt?: string;
};
```

### ResolverCandidate

```ts
export type ResolverCandidate = {
  resolverType: "user" | "agent" | "tool" | "retrieval" | "self_reflection";
  resolverId?: string;
  authorityWeight: number; // 0..1
  expectedInformationGain: number; // 0..1
  expectedLatencyMs: number;
  expectedCost: number; // arbitrary normalized cost
};
```

### Assumption

```ts
export type Assumption = {
  id: string;
  statement: string;
  source: "inferred" | "user_stated" | "repo_pattern" | "agent_advice";
  confidence: number;
  reversible: boolean;
  status: "active" | "confirmed" | "rejected" | "expired";
  linkedUncertaintyIds: string[];
};
```

### DecisionRecord

```ts
export type DecisionRecord = {
  id: string;
  taskId: string;
  timestamp: string;
  proposedAction: string;
  selectedAction: string;
  overallConfidence: number;
  confidenceVector: ConfidenceVector;
  uncertaintyIds: string[];
  resolverInvoked?: string;
  outcome?: "success" | "partial" | "failed" | "revised";
  notes?: string;
};
```

### UncertaintyLedger

```ts
export type UncertaintyLedger = {
  taskId: string;
  openItems: UncertaintyItem[];
  resolvedItems: ResolvedUncertainty[];
  assumptions: Assumption[];
  decisions: DecisionRecord[];
};
```

## Decision policy

### Step 1: detect
The active agent produces an `AgentAssessment` after planning and before any action with nontrivial side effects.

### Step 2: normalize
Policy engine normalizes model output into bounded values, validates required fields, and drops malformed records.

### Step 3: score
Compute derived scores:

- `safe_to_proceed_score`
- `interrupt_user_score`
- `ask_agent_score`
- `retrieve_more_score`
- `block_score`

### Step 4: route
The clarification policy picks one action.

### Step 5: persist
Write the decision, assumptions, and unresolved items to the task ledger.

### Step 6: resolve
Invoke the selected resolver.

## Suggested scoring model

Start rule-based, not ML-driven.

```ts
safe_to_proceed_score =
  0.20 * confidenceVector.executionSafety +
  0.20 * confidenceVector.reversibility +
  0.20 * confidenceVector.validationCoverage +
  0.15 * confidenceVector.intentClarity +
  0.15 * confidenceVector.contextCompleteness +
  0.10 * confidenceVector.patternMatchStrength;
```

```ts
interrupt_user_score =
  uncertaintySeverityWeight *
  blastRadiusWeight *
  irreversibilityWeight *
  informationGainWeight *
  preferenceDependenceWeight
  - interruptionCostWeight
  - toolResolvableWeight
  - agentResolvableWeight;
```

Initial routing heuristics:

- Proceed silently when confidence is high, blast radius is low, and validation or reversibility is strong.
- Proceed with annotation when ambiguity is real but non-blocking.
- Ask agent first when another agent has better authority and lower interruption cost.
- Ask user when the uncertainty is fundamentally normative, preference-based, or destructive.
- Block when uncertainty is high and blast radius is large.

## Resolver strategy

### User resolver
Use when:
- preference or intent is underspecified
- destructive action requires approval
- multiple materially different futures are valid

Question format should include:
- the decision point
- why it matters
- recommended default
- the smallest useful choice set
- fallback assumption if unanswered

### Agent resolver
Use when:
- uncertainty is domain-specific
- another agent has higher authority
- user interruption would be noisy or premature

Examples:
- architecture agent
- security agent
- style/convention agent
- product spec agent

### Retrieval resolver
Use when:
- repo context, docs, or prior decisions may answer the question
- uncertainty is likely evidence-based rather than preference-based

### Validation / simulation resolver
Use when:
- uncertainty is predictive
- a dry-run, typecheck, or targeted test can cheaply collapse the uncertainty

## Ask-before-acting thresholds

Vel should support configurable user preferences:

- `hands_off`
- `balanced`
- `high_clarity`
- `delegate_to_agents_first`
- `ask_before_destructive_actions_only`

These preferences tune thresholds but do not override hard safety gates.

## UI surfaces

### Uncertainty panel
For each active task:
- current overall confidence
- confidence vector breakdown
- open uncertainties
- blocked vs proceedable state
- active assumptions

### Clarification inbox
A queue of pending questions:
- awaiting user input
- awaiting specialist agent
- awaiting retrieval / validation
- auto-resolved

### Assumption review
Allows the user to:
- confirm
- reject
- edit
- pin as preference

### Decision / escalation feed
Human-readable event stream such as:
- "Asked architecture agent about migration ordering"
- "Proceeding under reversible assumption: planner uses service object pattern"
- "Blocked on destructive schema change pending approval"

## APIs and integration points

### New packages / modules

- `packages/core/uncertainty/`
- `packages/core/clarification-policy/`
- `packages/core/resolvers/`
- `packages/ui/uncertainty-panel/`

### Suggested file layout

```txt
packages/
  core/
    uncertainty/
      types.ts
      scoring.ts
      ledger.ts
      normalizer.ts
    clarification-policy/
      engine.ts
      rules.ts
      thresholds.ts
    resolvers/
      user-resolver.ts
      agent-resolver.ts
      retrieval-resolver.ts
      validation-resolver.ts
  ui/
    uncertainty-panel/
      UncertaintyPanel.tsx
      ClarificationInbox.tsx
      AssumptionReview.tsx
```

### Execution hook
Every actionable agent step should call something like:

```ts
const assessment = await assessStep(stepContext);
const policyDecision = clarificationPolicy.decide(assessment, runtimeContext);
await ledger.record(policyDecision);
return await resolve(policyDecision);
```

## Telemetry and learning loop

Track at minimum:

- uncertainty kinds by frequency
- which uncertainties led to rework
- which resolvers produced decisive answers
- user interruption rate
- user answer usefulness score
- confidence calibration drift
- false proceed vs false block cases

This will support future improvements such as:
- per-agent calibration tuning
- adaptive thresholds
- resolver ranking updates
- auto-suggested permanent preferences

## Failure modes to watch

- model emits performative uncertainty on everything
- model emits false confidence on weak evidence
- system asks users questions that repo inspection could have answered
- repeated clarifications because assumptions were not persisted
- specialist agents become laundering layers for bad decisions rather than real resolution sources

## Testing strategy

### Unit tests
- normalization bounds malformed model output
- scoring is deterministic
- routing rules behave correctly at thresholds
- assumptions transition correctly through confirm/reject/expire states

### Integration tests
- ambiguous intent routes to user resolver
- style uncertainty routes to architecture/style agent
- predictive uncertainty routes to validation resolver
- high-risk destructive plan blocks without approval

### UX tests
- clarification prompts are concise and discriminative
- uncertainty panel does not feel noisy
- inbox supports fast accept/reject/delegate loops

## Rollout plan

### Phase 1
- domain types
- ledger persistence
- basic scoring
- rule-based policy engine
- terminal/log visibility only

### Phase 2
- user resolver
- agent resolver
- validation resolver
- UI panel and inbox

### Phase 3
- assumption review
- user preference tuning
- calibration analytics
- learning loop

## Implementation tickets

See [docs/tickets/uncertainty/](../tickets/uncertainty/README.md) for the ticket pack.

## Opinionated recommendation

This belongs in Vel core, not as a thin UI feature. It affects orchestration, execution safety, explainability, task continuity, and user trust. If built well, it becomes one of Vel's defining advantages.

Put differently: Vel should not merely have confidence. It should have **metacognitive hygiene**.
