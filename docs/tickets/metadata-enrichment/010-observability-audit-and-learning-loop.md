---
id: VEL-META-010
title: Observability audit trails and preference learning loop
status: proposed
priority: P1
estimate: 3-5 days
dependencies: [VEL-META-006, VEL-META-007]
---

# Goal

Instrument the system and learn from user responses without becoming inscrutable.

# Scope

- Metrics for gap volume, candidate precision, apply/reject/revert rates.
- Audit event log for scan, detect, propose, decide, apply, revert.
- Preference updates from accepts/edits/rejects.
- Confidence calibration tracking.

# Deliverables

- audit event schema
- metrics hooks/dashboards
- preference learner service
- retention/redaction policy for provenance details

# Acceptance criteria

- Every candidate/action has an auditable lineage.
- User response updates preference store.
- Metrics available per source and field.
- Learning cannot bypass explicit policy constraints.

# Notes

Teach the system, yes. Let it become a private religion, no.
