# Memory Model

Vel requires **tiered memory**, not one giant slurry of context.

## Memory Tiers

### 1. Ephemeral Working Context
Short-lived execution context.

Examples:

- current conversation
- active task stack
- current calendar window
- recent notifications

TTL: minutes to hours

### 2. Operational Memory
System state needed for functioning.

Examples:

- active commitments
- unresolved risks
- pending suggestions
- device capabilities

TTL: days to weeks

### 3. Personal Pattern Memory
User-specific preferences and patterns.

Examples:

- user prefers gentle reminders for medication
- commute prep usually needs 20 extra minutes
- user tends to delay low-affect admin tasks

TTL: durable but revisable

### 4. Reflective Memory
Findings about system performance.

Examples:

- reminder threshold too late for morning routines
- risk engine overpredicts commute danger on weekends
- chat explanations are accepted more often than terse nudges

TTL: durable, versioned

## Memory Rules

- every stored memory must have provenance
- reflective memories must be falsifiable
- preferences can override learned heuristics
- memory writes should be sparse and justified

## Durable Recall Entities

Phase 6 widens semantic recall only across durable typed entities the runtime already owns.

- projects: local-first project records with stable local IDs and optional upstream IDs
- notes: note documents rooted in configured notes paths or project notes roots, with note-path provenance
- transcripts: assistant transcript rows carried under notes as `transcript_note` records
- threads: durable thread records and their typed local IDs
- people: practical person records with explicit aliases and source-linked identity references

The graph is intentionally narrow. Retrieval should surface provenance-bearing hits for these entities, not speculative merge products or opaque inferred identities.
