---
title: Documentation Truth Repair & Architecture Mapping
status: in-progress
owner: staff-eng
type: documentation
priority: medium
created: 2026-03-17
labels:
  - docs
  - architecture
  - phase-1
---

Rewrite all internal developer documentation to align with the Phase 1 repository pattern, typed context mandates, and centralized Master Plan.

## Technical Details
- **Concept Anchor**: Maintain one explicit architecture and concept anchor for agentic/runtime principles under `docs/cognitive-agent-architecture/`.
- **Architecture Diagram**: Update the high-level diagrams to show the new repository boundaries.
- **API Documentation**: Ensure all endpoint descriptions match the strictly layered Service/DTO model.
- **Storage Guide**: Document the new `StorageBackend` trait and repository implementation patterns.
- **Master Plan Sync**: Ensure the `README.md` and `docs/README.md` strictly point to the Master Plan as the single source of truth.
- **Queue Alignment**: Ensure ticket queue language matches the concept pack on trust domains, capability mediation, deny-by-default behavior, and execution-backed verification.

## Acceptance Criteria
- No documentation remains that references the old monolithic `db.rs` logic.
- A new developer can understand the storage and service layering solely from the updated docs.
- The architecture pack contains an explicit concept spec and reading order.
- The `docs/` directory is clean and free of legacy speculative specs or broken authority pointers.
