# vel — Architecture Diagram (Signals → Inference → Nudges → Artifacts)

This diagram is intended to accompany `vel_next_phase_instructions.md`.

It gives the coding agent a compact visual model of the **next implementation phase** so the system is built as a coherent pipeline rather than as disconnected features.

---

## High-level flow

```mermaid
flowchart TD
    A[External Systems] --> B[Signal Adapters]
    B --> C[Signals Store]
    C --> D[Inference Engine]
    E[Commitments] --> D
    F[Current Time] --> D
    D --> G[Inferred State]
    G --> H[Nudge Engine]
    E --> H
    H --> I[Notification Adapters]
    I --> J[CLI]
    I --> K[Desktop Toast]
    I --> L[Watch Notification]

    D --> M[Runs]
    H --> M
    H --> N[Nudges Store]
    D --> O[Artifacts]
    H --> O
    P[Weekly Synthesis] --> O
    C --> P
    E --> P
    N --> P
    M --> Q[Inspection]
    N --> Q
    O --> Q
```

---

## Concrete subsystem model

```mermaid
flowchart LR
    subgraph Sources
        CAL[Calendar]
        TOD[Todoist / Reminders]
        ACT[Computer Activity]
        NOTES[Daily Notes / Captures]
    end

    subgraph Adapters
        ACAL[calendar adapter]
        ATOD[todoist adapter]
        AACT[activity adapter]
        ANOTE[note/capture adapter]
    end

    subgraph Core
        SIG[signals]
        COM[commitments]
        INF[inference engine]
        STATE[inferred state]
        NUDGE[nudge engine]
        NSTORE[nudges]
        RUNS[runs / run_events]
        ART[artifacts]
        REFS[refs / provenance]
    end

    subgraph Delivery
        CLI[CLI]
        DESK[Desktop]
        WATCH[Watch]
    end

    subgraph Reflection
        SYN[weekly synthesis]
        INSPECT[inspect / debug]
    end

    CAL --> ACAL --> SIG
    TOD --> ATOD --> SIG
    TOD --> ATOD --> COM
    ACT --> AACT --> SIG
    NOTES --> ANOTE --> SIG

    SIG --> INF
    COM --> INF
    INF --> STATE

    STATE --> NUDGE
    COM --> NUDGE
    NUDGE --> NSTORE

    NUDGE --> CLI
    NUDGE --> DESK
    NUDGE --> WATCH

    INF --> RUNS
    NUDGE --> RUNS
    NUDGE --> ART
    ART --> REFS
    COM --> REFS

    SIG --> SYN
    COM --> SYN
    NSTORE --> SYN
    SYN --> ART

    RUNS --> INSPECT
    NSTORE --> INSPECT
    ART --> INSPECT
    REFS --> INSPECT
```

---

## First implementation slice

The first end-to-end slice should be exactly this:

```mermaid
flowchart TD
    A[Todoist meds task incomplete] --> B[Todoist adapter]
    B --> C[Signals + Commitment]
    C --> D[Inference: meds_pending]
    D --> E[Nudge Engine]
    E --> F[Generate meds_not_logged nudge]
    F --> G[Notification Adapter]
    G --> H[CLI or Desktop]
    H --> I{User action}
    I -->|Done| J[Resolve commitment + nudge]
    I -->|Snooze| K[Set snoozed_until]
```

This slice proves:

- external ingestion
- commitment linkage
- inference
- nudge generation
- done/snooze protocol
- persistence of state changes

If this slice works, the rest of the system can be built by extension rather than reinvention.

---

## Morning-state-focused slice

```mermaid
flowchart TD
    A[Calendar first meeting] --> D[Inference Engine]
    B[Todoist meds state] --> D
    C[Shell login / activity] --> D

    D --> E[Infer morning state]
    E --> F{State}

    F -->|awake_unstarted| G[Possible gentle nudge]
    F -->|underway| H[Lower urgency]
    F -->|at_risk| I[Warning or danger nudge]

    E --> J[State artifact / run trace]
```

The morning state machine should be treated as a **derived model** built from the three agreed signal sources:

- calendar
- task completion
- workstation activity

Do not introduce additional ambient sensors until this version is working.

---

## Nudge escalation ladder

```mermaid
flowchart TD
    A[Condition detected] --> B[Gentle]
    B --> C{Done?}
    C -->|Yes| Z[Resolved]
    C -->|No| D[Snoozed or time passes]
    D --> E[Warning]
    E --> F{Done?}
    F -->|Yes| Z
    F -->|No| G[Snoozed or time runs out]
    G --> H[Danger]
    H --> I{Done?}
    I -->|Yes| Z
    I -->|No| J[Repeat per policy]
```

The escalation ladder should be:

- time-proximity-based
- confidence-aware
- consequence-aware

The system should **not** escalate merely because a timer expired. It should escalate because the cost of inaction is increasing.

---

## Important design boundaries

```mermaid
flowchart LR
    A[Adapters] --> B[Signals]
    B --> C[Inference]
    C --> D[Nudges]
    D --> E[Notification Adapters]

    X[Do NOT put inference in adapters]
    Y[Do NOT put nudge logic in routes]
    Z[Do NOT make notification channel the source of truth]
```

### Translation into engineering rules

- Adapters only normalize external data
- Inference engine owns interpretation
- Nudge engine owns prompting/escalation
- Notification adapters only deliver
- Runs/artifacts/refs preserve observability

---

## Practical engineering rule

If a feature does not fit this pipeline:

```text
source → signal → inference → nudge or artifact → inspection
```

it probably does not belong in this phase.

That rule should help prevent scope creep while the dogfooding version is being built.

---

## Short version for the coding agent

Build this in order:

1. adapters
2. signals store
3. commitments linkage
4. inference engine
5. nudges
6. notification adapters
7. artifacts + synthesis
8. inspection/debugging

That is the whole machine.