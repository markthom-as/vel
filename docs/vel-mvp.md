# Vel — MVP Definition

## Purpose

This document defines the **minimum viable version of Vel**.

The goal of v0 is not to prove every long-term idea.  
The goal is to produce a version that is **personally useful every day** and establishes the core loop:

**capture → recall → orient → adjust**

Vel v0 should optimize for **personal cognition first**, with agent automation as a supporting capability rather than the center of gravity.

---

## MVP Thesis

Vel becomes valuable the moment it can reliably do the following:

1. capture thoughts and events quickly
2. remember and retrieve them accurately
3. help reconstruct current context
4. provide lightweight daily executive support
5. learn from suggestion feedback

That means the MVP is:

**A + a small slice of B**

Where:

### A — Capture + Recall
Vel answers:
- What did I say about X?
- What was I working on?
- What commitments did I make?
- Where did this idea come from?

### B — Daily Executive Support
Vel provides:
- morning brief
- end-of-day summary
- lightweight project continuity nudges

---

## Primary User Outcomes

Vel v0 succeeds if it helps the user:

- stop losing ideas
- recover project context quickly
- remember conversations and commitments
- decide what to do next with less friction
- get useful summaries of the day
- tolerate interruptions without losing continuity

---

## v0 In Scope

### 1. Explicit Capture
Supported inputs:
- text notes
- voice notes
- manual meeting capture
- imported transcript/audio artifacts
- selected git/project activity
- optionally tasks/calendar import

Not in scope:
- always-on ambient audio
- full passive surveillance
- complex multimodal sensing

---

### 2. Artifact Storage + Metadata
Vel v0 must store:
- raw capture artifacts
- derived summaries/transcripts
- metadata records
- stable IDs
- timestamps and privacy classes

Required resilience features:
- periodic snapshots
- backup path
- artifact-first rebuild possibility
- incomplete capture recovery

---

### 3. Context Recall
Vel v0 should support:
- search across captures/artifacts
- project continuity reconstruction
- commitment recall
- conversation lookup
- recent thread awareness

Examples:
- What did I say about lidar?
- What was I doing on Mimesis?
- What am I waiting on?
- What did Cornelius and I decide?

---

### 4. Daily Orientation
Vel v0 should generate:
- morning brief
- end-of-day summary

Optional but desirable:
- lightweight weekly reflection

Morning brief should include:
- top active threads
- pending commitments
- suggested focus
- key reminders

End-of-day summary should include:
- what was done
- what remains open
- what may matter tomorrow

---

### 5. Suggestions
Vel v0 should surface:
- dormant thread reminders
- commitment reminders
- misalignment nudges
- resurfaced relevant notes/ideas

Suggestions must support:
- dismiss
- correct
- never show again
- train system

Vel v0 should have:
- quiet mode
- configurable suggestion levels
- contextual suppression

---

### 6. Basic Behavior Configuration
Vel v0 should allow configuration of:
- quiet hours
- suggestion tone / strictness
- nag level per category
- health reminder intensity
- project-specific reminder preferences

Behavior feedback should be storable even if not fully automated yet.

---

## v0 Supported Surfaces

### Required
- CLI
- daemon/API
- basic mobile-compatible API

### Desirable
- simple phone UI
- watch capture surface
- voice entrypoint

### Not required for v0
- polished dashboard
- full desktop GUI
- always-on wearable UX
- smart speaker integration

---

## v0 Integrations

### Likely first
- filesystem artifact store
- local DB
- git awareness
- imported notes/transcripts
- possibly Todoist/Obsidian import hooks later

### Not required for v0
- deep bidirectional sync for everything
- full calendar automation
- health data ingestion
- multi-user collaboration

---

## v0 Failure Modes to Handle

### Capture failures
- interrupted recording
- incomplete transcript
- duplicate source artifacts

### Storage failures
- DB corruption fallback path
- missing blob detection
- restore from backup/snapshot

### Suggestion failures
- noisy nags
- incorrect “you didn’t do this” claims
- wrong project association

### Sync failures
- offline local operation
- conflict artifacts instead of silent overwrite
- reconciliation mode later

---

## v0 Privacy Requirements

Must support:
- privacy classes on captured objects
- do-not-record mode
- selective retention
- redaction/review path for sensitive captures
- quiet/silent behavior modes

---

## Explicitly Out of Scope for v0

Do **not** overbuild these into the first release:

- full distributed queue infrastructure
- sophisticated scenario simulation
- ambient passive recording
- generalized agent swarm automation
- client-facing / multi-tenant product features
- astrology-aware planning engines
- deep health analytics
- enterprise provenance requirements

These may become important later. They are not required to prove value.

---

## Success Criteria

Vel v0 is successful if the user says:

- I stop losing ideas.
- I can reconstruct what I was doing quickly.
- I can ask what matters today and get a useful answer.
- It reminds me of important things without becoming unbearable.
- I trust it enough to keep using it daily.

---

## v0 Privacy Requirements

Must support:
- privacy classes on captured objects
- do-not-record mode
- selective retention
- redaction/review path for sensitive captures
- quiet/silent behavior modes

---

## Explicitly Out of Scope for v0

Do **not** overbuild these into the first release:

- full distributed queue infrastructure
- sophisticated scenario simulation
- ambient passive recording
- generalized agent swarm automation
- client-facing / multi-tenant product features
- astrology-aware planning engines
- deep health analytics
- enterprise provenance requirements

These may become important later. They are not required to prove value.

---

## Success Criteria

Vel v0 is successful if the user says:

- I stop losing ideas.
- I can reconstruct what I was doing quickly.
- I can ask what matters today and get a useful answer.
- It reminds me of important things without becoming unbearable.
- I trust it enough to keep using it daily.

---

## Nice-to-Have v0.1 Features

If the core works, the next increment should add:

- weekly reflection
- better project thread continuity
- mobile quick capture
- better imported transcript handling
- suggestion ranking improvements
- more tunable behavior modes

---

## Design Constraint

Vel v0 should be designed so it can evolve into a broader product **without** making productization the primary constraint today.

Personal usefulness comes first.  
Architectural cleanliness comes second.  
Speculative scale comes last.
