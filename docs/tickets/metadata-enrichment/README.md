# Vel Metadata Enrichment Spec Pack

This pack contains:

- `docs/vel-metadata-enrichment-spec.md` — product + technical spec
- `tickets/` — implementation tickets in agent-ready markdown with status frontmatter

## Intent

Enable Vel to inspect metadata across integrated sources, detect missing or weak fields, propose or request enrichment, and apply enrichment through source-specific adapters when authorized.

## Core examples

- Todoist tasks missing tags, project, priority, due time, duration, labels, location context, or upstream linkage
- Calendar events missing location, conferencing, attendee role, transit buffer, preparation metadata, linked task/project, or semantic category
- Email threads missing project/entity association, follow-up deadline, owner, or commitment extraction
- Files/docs missing project/workstream mapping, artifact type, date semantics, or people/entity references

## Package structure

```text
vel_metadata_enrichment_spec/
├── README.md
├── docs/
│   └── vel-metadata-enrichment-spec.md
└── tickets/
    ├── 001-schema-and-domain-model.md
    ├── 002-source-capabilities-registry.md
    ├── 003-ingestion-and-metadata-snapshot-pipeline.md
    ├── 004-gap-detection-engine.md
    ├── 005-enrichment-suggestion-and-request-flow.md
    ├── 006-source-adapters-todoist-calendar-email.md
    ├── 007-policy-consent-and-risk-controls.md
    ├── 008-review-ui-and-bulk-operations.md
    ├── 009-entity-linking-and-context-propagation.md
    ├── 010-observability-audit-and-learning-loop.md
    ├── 011-background-jobs-retries-and-idempotency.md
    └── 012-tests-fixtures-and-rollout.md
```
