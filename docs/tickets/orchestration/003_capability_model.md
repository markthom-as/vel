# Ticket 003 — Capability Model

## Purpose
Vel must match tasks to Navs using declared capabilities.

## Deliverables
Capability enum including:
- WebResearch
- CodeGeneration
- FileIO
- ShellExecution
- MemoryRead
- MemoryWrite
- CalendarRead
- CalendarWrite
- DocumentSynthesis

## Acceptance Criteria
Delegation engine can query NavRegistry::find_by_capability()

