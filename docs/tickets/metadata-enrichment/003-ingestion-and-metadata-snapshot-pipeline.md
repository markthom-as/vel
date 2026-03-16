---
id: VEL-META-003
title: Metadata snapshot ingestion pipeline
status: proposed
priority: P0
estimate: 3-4 days
dependencies: [VEL-META-001, VEL-META-002]
---

# Goal

Create a pipeline that reads source objects and stores normalized metadata snapshots.

# Scope

- Snapshot ingestion jobs per source.
- Normalize initial object sets for:
  - Todoist tasks
  - Calendar events
- Store raw source payload plus normalized projection.
- Add optimistic versioning or etag-like source fingerprinting where possible.

# Deliverables

- snapshot persistence tables
- source readers + normalizers
- job runner integration
- incremental re-scan support

# Acceptance criteria

- Can scan a bounded set of Todoist tasks and calendar events.
- Snapshots are deduplicated/versioned.
- Re-scan updates only changed objects when possible.
- Logs/metrics emitted for scan counts, errors, and latency.

# Notes

Do not wait for every source. Two solid sources beat a sprawling mush pile.
