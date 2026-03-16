---
title: Vel Self-Knowledge System Spec
status: proposed
owner: nav-core
last_updated: 2026-03-16
related:
  - documentation-awareness
  - source-code-awareness
  - uncertainty-engine
  - suggestion-engine
---

# Vel Self-Knowledge System

## Purpose

Vel must maintain structured awareness of its own implementation: source code, documentation, tests, schemas, migrations, runtime interfaces, and the relationships between them.

This subsystem enables Vel to:

- reason about its own architecture
- answer engineering questions with evidence
- detect drift between docs and implementation
- identify undocumented or stale features
- map affected surfaces before proposing changes
- express confidence and uncertainty honestly
- keep product narrative aligned with code reality

Without this, Vel risks becoming an agent that can search files but cannot actually understand the system it inhabits.

## Product Thesis

Vel should not merely have access to docs and code. It should maintain a living self-model of:

1. what the system is
2. what the system claims to be
3. where those differ

That third layer is where bugs, confusion, stale assumptions, and bad automation decisions breed.

## Non-Goals

This subsystem is not:

- a generic full-repo vector search feature
- autonomous code modification by default
- a replacement for human architectural judgment
- a license to overstate certainty from weak signals

## System Model

Vel should represent four layers of knowledge:

### 1. Source-of-Truth Layer

Artifacts closest to implementation reality:

- source code
- tests
- migrations
- schemas
- configs
- generated API contracts

### 2. Intent Layer

Artifacts describing design intent:

- specs
- ADRs
- tickets
- README files
- architecture docs
- product notes

### 3. Runtime Layer

Operational evidence:

- logs
- telemetry
- execution traces
- task outcomes
- observed failures

### 4. Belief Layer

Vel's synthesized internal understanding of the system, including:

- evidence
- provenance
- confidence
- contradictions
- freshness

The belief layer must always be grounded in the prior three layers and never treated as authoritative on its own.

## Core Requirements

### Requirement A — First-Class Self-Knowledge

Vel must know:

- what repos, apps, packages, and modules exist
- what each subsystem is intended to do
- where canonical docs live
- what is implemented vs planned vs stale
- what interfaces, commands, tables, and schemas exist
- which docs are authoritative, speculative, deprecated, or superseded

### Requirement B — Evidence-Backed Answers

When Vel answers engineering or architecture questions, it must attach:

- evidence sources
- freshness signals
- confidence level
- contradiction indicators when applicable

### Requirement C — Drift Detection

Vel must detect mismatches between:

- docs and code
- docs and tests
- docs and configuration
- code and tests
- architectural intent and current structure

### Requirement D — Queryable Self-Model

Vel must support navigable architectural questions, such as:

- where is feature X implemented?
- what docs explain Y?
- what changed this week?
- what parts of the system lack documentation?
- which files are most likely impacted by changing subsystem Z?
- what code appears to contradict the spec?

### Requirement E — Honest Uncertainty

Vel must support `unknown`, `ambiguous`, and `contradictory` as first-class answer states.

## Architecture

### High-Level Flow

1. Ingest artifacts from repo(s)
2. Parse and normalize entities
3. Build structured graph and evidence index
4. Run freshness and drift analysis
5. Expose results to reasoning, CLI, and UI surfaces
6. Continuously update on repo changes

### Subsystems

#### 1. Knowledge Ingestion Engine

Indexes:

- source files
- tests
- migrations
- schemas
- docs
- ADRs
- tickets
- config files
- git metadata

Supported initial languages / formats:

- Rust
- TypeScript
- Python
- Markdown
- YAML
- JSON
- SQL

Primary responsibilities:

- discover files
- classify artifact type
- extract symbols and references
- capture metadata
- persist parsed entities

#### 2. Knowledge Graph

Stores entities and relationships.

Representative entities:

- Repository
- Package
- Module
- File
- Symbol
- Function
- Struct
- Class
- Interface
- Test
- Command
- DatabaseTable
- Migration
- Document
- ADR
- Ticket
- ConfigEntry

Representative edges:

- imports
- exports
- depends_on
- implements
- documents
- tested_by
- supersedes
- references
- related_to
- affects
- generated_from

#### 3. Evidence Engine

Every system claim should be representable as:

- subject
- predicate
- object/value
- evidence set
- confidence score
- freshness score
- contradiction state

Example:

- Claim: `ReminderPolicyEngine supports snoozing`
- Evidence:
  - `src/policy/reminder.rs`
  - `tests/reminder_snooze.rs`
  - `docs/reminder_policy.md`
- Confidence: `0.82`
- Freshness: `high`
- Contradiction: `none`

#### 4. Drift Engine

Detects likely mismatches such as:

- docs mention nonexistent module
- code exposes undocumented command
- docs state old enum variants
- spec claims feature exists but tests/code absent
- README examples fail against current CLI/API

#### 5. Navigation & Query APIs

APIs should support:

- `find_symbol(name)`
- `find_module(name)`
- `find_documentation(entity)`
- `find_tests(entity)`
- `find_dependencies(entity)`
- `find_reverse_dependencies(entity)`
- `explain_component(name)`

APIs must return:

- entities
- relationships
- evidence
- confidence
- freshness
- contradictions where relevant

#### 6. CLI and UI Surfaces

CLI:

- `vel system map`
- `vel system coverage`
- `vel system explain <component>`
- `vel system drift`
- `vel system changed --since <window>`

UI dashboard:

- system map
- documentation coverage
- drift report
- confidence / contradictions
- canonical docs by subsystem
- change hotspots

#### 7. Maintenance Loop

Responsibilities:

- incremental reindex on repo changes
- recompute affected claims and drift findings
- generate remediation suggestions
- (optionally) draft patches or internal tickets behind strict policy gates

## Implementation Tickets

See [docs/tickets/self-knowledge/](../tickets/self-knowledge/README.md) for the SK-001–SK-010 ticket pack.

