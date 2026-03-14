# Vel — Product Specification

## Vision
Vel is a **local-first personal executive system** that maintains continuous awareness of a user’s life context and helps align goals, actions, and memory over time.

Vel functions as an **executive-function prosthesis and collaborative assistant**. It captures thoughts, remembers context, surfaces insights, and helps guide decisions about how time and energy are used.

Vel is not primarily a chatbot or task manager. Vel is a **context engine** that maintains continuity between intentions, memory, and action.

---

## Core Problems Vel Solves

### 1. Memory Precision
Users often remember the emotional or conceptual shape of events but not the details. Vel stores precise artifacts such as:

- transcripts
- notes
- audio recordings
- documents
- code history
- meeting summaries
- decisions and commitments

Vel acts as **precision recall for life and work**.

### 2. Continuity of Effort
Ideas and projects frequently lose momentum due to interruptions or context switching.

Vel tracks:
- active threads
- paused threads
- dormant threads

Vel surfaces dormant work when appropriate to restore momentum.

### 3. Strategic Alignment
Users often intend to prioritize certain goals but end up spending time elsewhere.

Vel compares:
- stated priorities
- observed behavior

Vel surfaces alignment insights and suggests corrective actions.

---

## Core Loop

Vel operates on a continuous loop:

**capture → remember → connect → resurface → guide**

This loop runs continuously across devices and contexts.

---

## Major Subsystems

### Capture System
Responsible for ingesting information from many sources:

Inputs:
- voice memos
- meeting recordings
- manual notes
- git repositories
- tasks
- calendar events
- wearable data

Capture must be **fast and frictionless**.

Primary capture devices:
- Apple Watch
- iPhone
- CLI
- voice interfaces

### Personal Memory Graph
Vel maintains a structured graph of the user’s life context.

Core entities include:
- containers
- projects
- people
- ideas
- artifacts
- tasks
- commitments
- events

### Alignment Engine
The alignment engine evaluates:

**stated goals vs observed behavior**

Inputs include:
- time allocation
- tasks completed
- captured work
- activity patterns

### Execution Layer
Vel coordinates automated tasks and agent workflows.

Examples:
- summarizing meetings
- generating documents
- running code agents
- scheduling reminders

Autonomy model:
- analysis: autonomous
- execution: approval based
- low-risk tasks: optionally automated

---

## Interaction Model

### Conversational Mode (Primary)
Examples:
- Vel, what should I work on?
- Vel, capture this idea.
- Vel, what did I say about lidar sensors?

Supports:
- voice
- text

### Dashboard Mode
Provides visual overview:
- today’s priorities
- pending commitments
- recent ideas
- project status

### Ambient Mode
Vel surfaces context during activity.

Examples:
- You discussed this topic on Feb 12.
- You previously committed to finishing this task.

---

## Initiative Model

Vel initiates interaction in three scenarios:
1. memory resurfacing
2. commitment tracking
3. goal alignment

---

## Planning Horizons

Vel supports:
- daily
- weekly
- monthly
- quarterly
- yearly

---

## Life Timeline

Vel maintains a long-term timeline of significant events.

Supports:
- year in review
- life reflection
- pattern detection

---

## Personal Diary

Vel supports reflective journaling and personal logging.

Diary entries integrate into the personal memory graph.

---

## Containers

Primary containers:
- Mimesis Institute
- Opertus Systems
- Personal Art Practice
- Writing Projects

Each container contains:
- projects
- collaborators
- artifacts
- tasks
- commitments

Vel also detects cross-container relationships.

---

## Vel v0 Scope

The initial version should focus on:

### Capture
- voice capture
- text capture
- idea storage
- git awareness

### Context Recall
Vel should answer:
- What did I say about X?
- What should I work on?
- What was I working on recently?

### Daily Alignment
Vel provides:
- morning briefing
- evening summary
- weekly reflection

---

## Future Capabilities

- meeting augmentation
- behavioral analytics
- planning simulations
- ambient capture
- multi-agent automation
