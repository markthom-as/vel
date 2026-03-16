---
title: Vel Metadata Enrichment and Gap-Repair Spec
status: proposed
owner: vel
updated: 2026-03-16
---

# Vel Metadata Enrichment and Gap-Repair Spec

## 1. Why this exists

Vel is already converging on a chief-of-staff posture: it should not merely read user systems, but maintain the semantic integrity of the user's operational field. In practice, integrated tools are messy. Calendar events arrive without locations. Todoist tasks lack tags or project assignments. Email threads imply commitments that never become structured objects. Files exist as free-floating signifiers without stable attachment to projects, people, or timelines.

This feature makes Vel capable of scanning integrated-source metadata, identifying absence, ambiguity, and drift, and then either:

1. auto-enriching when policy allows,
2. proposing enrichment for approval,
3. requesting clarification from the user or another agent,
4. learning from accepted/rejected enrichment decisions.

The result is a tighter symbolic order across the user's systems: fewer orphaned objects, better retrieval, stronger automation, and less administrative entropy.

---

## 2. Product goals

### Primary goals

- Detect incomplete, low-quality, contradictory, or stale metadata across connected sources.
- Normalize heterogeneous metadata into a common Vel metadata graph.
- Recommend high-confidence enrichment actions with explicit rationale.
- Apply enrichment back to source systems through scoped adapters.
- Build user-trust via auditability, consent controls, reversibility, and confidence display.
- Learn enrichment preferences and source-specific conventions over time.

### Secondary goals

- Improve downstream task routing, search, reminders, nudges, and orchestration.
- Support agent-to-agent requests for disambiguation when confidence is low.
- Surface “metadata debt” as a first-class operational concept.

### Non-goals

- Full semantic rewriting of source content bodies.
- Unbounded autonomous editing of every connected system.
- Irreversible mutation without audit trail or rollback strategy.
- Perfect ontology from day one.

---

## 3. Core product concept

Vel maintains a **metadata enrichment loop**:

1. **Observe** source objects and capture normalized snapshots.
2. **Assess** metadata completeness, quality, consistency, and utility.
3. **Infer** candidate enrichments from context, patterns, linked entities, and user history.
4. **Decide** whether to auto-apply, propose, batch-review, or defer.
5. **Write back** using source-specific adapters.
6. **Learn** from outcome signals.

This loop should work per-object, in batch, and as a continuous hygiene pass.

---

## 4. Key user stories

### Todoist

- As a user, I want Vel to notice uncategorized tasks and suggest tags, projects, priority, duration, or labels.
- As a user, I want tasks inferred from emails or meetings to inherit likely project/workstream metadata.
- As a user, I want repetitive low-risk enrichments to happen automatically.

### Calendar

- As a user, I want Vel to detect meetings with missing location or conferencing metadata and propose likely values.
- As a user, I want events to inherit project tags and prep/follow-up semantics.
- As a user, I want travel/transit buffers derived when event location implies movement.

### Cross-source

- As a user, I want Vel to infer that a task, event, email thread, and document belong to the same project.
- As a user, I want Vel to ask me when uncertainty is real instead of hallucinatorily flattening ambiguity.
- As a user, I want a review queue for enrichment proposals with reasons and confidence.

---

## 5. Terminology

### Source object
A concrete item from an integration, such as a Todoist task, Google Calendar event, email thread, document, note, or contact.

### Metadata snapshot
Normalized representation of the object’s source metadata at a point in time.

### Gap
Missing, weak, conflicting, stale, or policy-required metadata that is absent or low quality.

### Enrichment candidate
A proposed metadata addition or update with provenance, confidence, and writeback semantics.

### Enrichment action
A concrete mutation against a source system, such as adding Todoist labels, setting a calendar location, or attaching a linked entity.

### Metadata debt
Aggregate measure of unresolved metadata gaps across objects, sources, and workstreams.

---

## 6. Architecture overview

Vel should implement metadata enrichment as a dedicated subsystem with the following components:

1. **Source capability registry**
2. **Metadata normalization schema**
3. **Snapshot ingestion pipeline**
4. **Gap detection engine**
5. **Candidate inference engine**
6. **Policy/consent/risk evaluator**
7. **Review and execution queue**
8. **Source writeback adapters**
9. **Audit/event log**
10. **Preference learning loop**

### Proposed subsystem name
`metadata_enrichment`

### Proposed major modules

```text
crates/
  vel-core/
    metadata/
      schema.rs
      quality.rs
      gaps.rs
      candidate.rs
      policy.rs
  vel-integrations/
    capabilities/
    adapters/
      todoist.rs
      gcal.rs
      gmail.rs
      drive.rs
  vel-jobs/
    metadata_scan.rs
    enrichment_apply.rs
    enrichment_retry.rs
  vel-api/
    routes/
      enrichment.rs
  vel-ui/
    review_queue/
    source_object_detail/
    metadata_debt/
```

---

## 7. Common metadata model

Vel needs a normalized, source-agnostic schema. Keep it opinionated but extensible.

### 7.1 Canonical object envelope

```json
{
  "object_ref": {
    "source": "todoist",
    "external_id": "task_123",
    "object_type": "task"
  },
  "title": "Draft Mimesis budget",
  "body_excerpt": "...",
  "status": "open",
  "created_at": "2026-03-16T10:30:00Z",
  "updated_at": "2026-03-16T11:02:00Z",
  "participants": ["person:cornelius-o"],
  "entities": ["project:mimesis-institute"],
  "links": ["calendar:event_456"],
  "time": {
    "start": null,
    "end": null,
    "due": "2026-03-18T18:00:00Z",
    "timezone": "America/Denver"
  },
  "location": {
    "raw": null,
    "normalized": null,
    "confidence": null
  },
  "tags": ["grant", "budget"],
  "priority": 3,
  "source_metadata": {"...": "verbatim-ish source fields"}
}
```

### 7.2 Canonical enrichable fields

- classification/category
- tags/labels
- project/workstream
- person/entity linkage
- location
- conferencing/join info
- due/start/end/timezone completeness
- preparation/follow-up requirements
- transit/travel buffer metadata
- priority/urgency
- effort/duration estimate
- ownership/assignee
- dependency/blocker markers
- commitment extraction linkage
- artifact type/document genre
- confidentiality/sensitivity level

### 7.3 Field quality states

Each canonical field should carry:

- `presence`: absent | present
- `quality`: low | medium | high
- `source_of_truth`: source | inferred | user_confirmed | agent_confirmed
- `last_evaluated_at`
- `confidence`
- `writeback_supported`

---

## 8. Source capability registry

Every integration should declare what it can:

- read
- normalize
- enrich
- partially enrich
- bulk enrich
- revert
- audit

### Example capability declaration

```json
{
  "source": "todoist",
  "object_types": ["task"],
  "fields": {
    "tags": {"read": true, "write": true, "bulk": true},
    "project": {"read": true, "write": true, "bulk": true},
    "priority": {"read": true, "write": true, "bulk": true},
    "location": {"read": false, "write": false, "bulk": false},
    "duration": {"read": true, "write": false, "bulk": false}
  },
  "supports_revert": true,
  "requires_user_approval_for": ["project_reassignment"]
}
```

This registry prevents Vel from fantasizing capabilities the integration does not actually possess. That way madness lies.

---

## 9. Gap taxonomy

Metadata gaps should be typed, not just hand-wavy “missing field” blobs.

### 9.1 Gap categories

- `missing_required`
- `missing_recommended`
- `low_quality`
- `conflicting_values`
- `stale_value`
- `dangling_linkage`
- `orphaned_object`
- `schema_violation`
- `policy_required`
- `retrieval_impairing`
- `automation_blocking`

### 9.2 Examples

- Calendar event with no location but title suggests in-person meeting.
- Todoist task has no project, no tags, and was created from an email thread already linked to a project.
- Email thread contains explicit commitment but no task linkage.
- Document name references “budget” and “Mimesis,” but has no project classification.

---

## 10. Gap detection engine

Gap detection should combine:

- source rules,
- user rules/preferences,
- learned patterns,
- ontology/entity graph context,
- temporal logic.

### 10.1 Detection sources

1. **Hard rules**
   - Example: event with attendees and no conference or location is incomplete.
2. **Heuristics**
   - Example: task title starts with “Call” or “Meet” and due time exists, but no linked event.
3. **Contextual inference**
   - Example: thread participants and document project imply likely tag set.
4. **User preference profiles**
   - Example: user prefers every grant task tagged with entity + status + deliverable type.
5. **Cross-object consistency checks**
   - Example: event belongs to Project A, linked task belongs to Project B.

### 10.2 Gap score

Each gap gets a score:

`gap_score = severity × confidence × downstream_impact × recency_weight`

Where downstream impact captures whether the missing metadata breaks reminders, search, routing, mobility planning, or agent orchestration.

---

## 11. Candidate inference engine

Given a gap, Vel proposes one or more enrichment candidates.

### 11.1 Candidate inputs

- source object content/title/body
- linked objects
- user ontology/entities/projects
- historical accepted enrichments
- source-local conventions
- calendar/time/location context
- contacts and meeting patterns
- commitment extraction results

### 11.2 Candidate shape

```json
{
  "candidate_id": "cand_001",
  "object_ref": {"source": "gcal", "external_id": "evt_42", "object_type": "event"},
  "field": "location",
  "proposed_value": "145 4th Ave, New York, NY",
  "confidence": 0.82,
  "reasons": [
    "Event title matches recurring meeting pattern",
    "Same attendees met at this location 3 previous times",
    "Linked email signature contains venue address"
  ],
  "provenance": [
    {"kind": "historical_pattern", "ref": "evt_38"},
    {"kind": "email_thread", "ref": "gmail:thread_991"}
  ],
  "risk_level": "medium",
  "approval_mode": "user_review",
  "expires_at": "2026-03-17T00:00:00Z"
}
```

### 11.3 Inference classes

- deterministic
- pattern-based
- probabilistic/LLM-assisted
- user-rule-driven
- cross-source-joined

Deterministic and user-rule-driven candidates can be auto-applied more aggressively. Probabilistic candidates should be review-biased unless trust and historical accuracy are high.

---

## 12. Policy, consent, and risk model

This is where Vel stops being a bureaucratic raccoon and starts being a trustworthy one.

### 12.1 Approval modes

- `auto_apply`
- `apply_if_rule_matches`
- `queue_for_review`
- `ask_inline_when_contextually_relevant`
- `defer`
- `forbidden`

### 12.2 Risk dimensions

- source sensitivity
- user visibility/embarrassment risk
- reversibility
- ambiguity level
- blast radius
- downstream automation impact

### 12.3 Example policies

- Auto-add known tags to Todoist if confidence > 0.9 and prior accept rate > 95%.
- Never auto-change calendar location if attendees are external unless deterministic.
- Never auto-assign confidentiality labels without explicit rule.
- Auto-link task to project when email thread and document both point to same project with high confidence.

### 12.4 Consent levels

- global source-level permissions
- field-level write permissions
- rule packs defined by user
- one-shot approval
- approval remembered per source+field+confidence band

---

## 13. Review UX

Vel should expose enrichment in at least four UI surfaces.

### 13.1 Review queue

A queue of pending enrichment proposals sorted by:

- gap score
- confidence
- time sensitivity
- source
- project/workstream

Each row should show:

- object title + source icon
- missing/weak field
- proposed value
- confidence
- reasons/provenance summary
- action buttons: apply / edit / reject / always allow / never auto-do this

### 13.2 Object detail pane

Show current metadata state, gaps, recent changes, candidate proposals, and linked objects.

### 13.3 Metadata debt dashboard

Aggregate by:

- source
- project
- field type
- severity
- automation blocked vs cosmetic

### 13.4 Conversational inline prompts

Examples:

- “This event likely needs a location; want me to set it to your usual office?”
- “Five new Todoist tasks look like grant work. Apply the `grant`, `mimesis`, and `writing` tags?”

---

## 14. Writeback model

Writeback should be explicit, idempotent, and reversible when source allows.

### 14.1 Writeback transaction record

Each applied enrichment stores:

- object ref
- field mutated
- old value
- new value
- source adapter action
- trigger origin
- actor (`user`, `vel_auto`, `vel_review_apply`, `agent_x`)
- confidence at apply time
- rollback token if available
- timestamps

### 14.2 Idempotency

Every enrichment action must use a deterministic idempotency key derived from:

`object_ref + field + normalized_value + policy_version`

### 14.3 Reversion

Where supported:

- single action revert
- batch rollback by job id
- rollback preview

---

## 15. Learning loop

Vel should learn from the user’s enrichment choices.

### 15.1 Signals

- accepted as-is
- accepted after edit
- rejected
- reverted later
- auto-applied and kept
- auto-applied and manually changed

### 15.2 Learned preference examples

- user prefers `mimesis` + `grant` + `budget` on budget-related nonprofit tasks
- calendar events with “studio” default to a known location
- project mappings are source-dependent
- some categories should remain suggestion-only

### 15.3 Guardrail

Learning modifies proposal ranking and approval defaults, not raw truth. Vel should not silently convert habit into law without policy traceability.

---

## 16. Data model proposal

### Tables / collections

#### `metadata_snapshots`
- `id`
- `source`
- `external_id`
- `object_type`
- `snapshot_version`
- `normalized_payload_json`
- `source_payload_json`
- `captured_at`

#### `metadata_gaps`
- `id`
- `source`
- `external_id`
- `object_type`
- `field`
- `gap_type`
- `severity`
- `confidence`
- `downstream_impact`
- `status` (`open`, `suppressed`, `resolved`)
- `detected_at`
- `resolved_at`

#### `enrichment_candidates`
- `id`
- `gap_id`
- `field`
- `proposed_value_json`
- `confidence`
- `reason_json`
- `provenance_json`
- `risk_level`
- `approval_mode`
- `status`
- `created_at`
- `expires_at`

#### `enrichment_actions`
- `id`
- `candidate_id`
- `source`
- `external_id`
- `field`
- `old_value_json`
- `new_value_json`
- `action_status`
- `adapter_result_json`
- `idempotency_key`
- `rollback_token`
- `applied_at`

#### `enrichment_preferences`
- `id`
- `scope_type` (`global`, `source`, `field`, `project`, `rule`)
- `scope_ref`
- `preference_key`
- `preference_value_json`
- `source_confidence`
- `updated_at`

#### `enrichment_jobs`
- `id`
- `job_type`
- `source`
- `scope_json`
- `status`
- `started_at`
- `finished_at`
- `metrics_json`

---

## 17. API proposal

### Read endpoints

- `GET /v1/enrichment/queue`
- `GET /v1/enrichment/objects/:source/:id`
- `GET /v1/enrichment/debt`
- `GET /v1/enrichment/candidates/:id`
- `GET /v1/enrichment/policies`

### Write endpoints

- `POST /v1/enrichment/scan`
- `POST /v1/enrichment/candidates/:id/apply`
- `POST /v1/enrichment/candidates/:id/reject`
- `POST /v1/enrichment/candidates/:id/edit-and-apply`
- `POST /v1/enrichment/actions/:id/revert`
- `POST /v1/enrichment/policies`
- `POST /v1/enrichment/preferences`

### Batch endpoints

- `POST /v1/enrichment/queue/apply-batch`
- `POST /v1/enrichment/queue/reject-batch`
- `POST /v1/enrichment/scan/source/:source`

---

## 18. Source-specific behavior examples

### 18.1 Todoist

Potential enrichments:
- tags/labels
- project assignment
- section assignment
- priority normalization
- due-time precision suggestions
- semantic links to calendar/email/docs

Gap examples:
- uncategorized task
- orphaned task with no project/entity
- task created from email but missing follow-up date

### 18.2 Google Calendar

Potential enrichments:
- location
- conferencing info
- event type/classification
- project/workstream tag linkage
- transit buffer recommendation
- prep/follow-up task generation linkage

Gap examples:
- event with attendees but no location/conference
- recurring event drift in location/title semantics
- in-person event without travel considerations

### 18.3 Gmail / email

Potential enrichments:
- project mapping
- commitment extraction linkage
- owner/follow-up deadline
- related contact/entity tags

Gap examples:
- thread contains commitment but no structured task
- thread linked to wrong project

### 18.4 Files / docs

Potential enrichments:
- artifact type
- project/workstream
- date semantics
- linked entities
- meeting note / proposal / budget / contract classification

---

## 19. Execution modes

### Passive mode
Only scans and proposes.

### Guided mode
Asks inline when it detects contextually relevant enrichments.

### Hygiene mode
Periodically scans selected sources and fills low-risk gaps automatically.

### Migration mode
Runs bulk metadata backfill for a source or project.

---

## 20. Observability and audit

Required metrics:

- gaps detected by type/source
- candidate generation rate
- apply rate
- reject rate
- revert rate
- precision by field/source
- average confidence vs actual acceptance
- metadata debt over time
- blocked automations attributable to metadata debt

Audit views must answer:

- Why did Vel suggest this?
- Why did Vel change this?
- What evidence was used?
- What policy allowed it?
- How can I undo it?

---

## 21. Failure modes

- hallucinated source capability
- destructive overwrite of user-authored metadata
- repeated low-quality nagging
- hidden auto-apply behavior
- stale snapshots causing wrong writeback
- race conditions with external edits
- policy drift after learning
- batch operations with excessive blast radius

### Mitigations

- capability registry gating
- optimistic concurrency/version checks
- idempotency keys
- diff preview
- approval thresholds
- bounded batch size
- cool-down/suppression rules
- revert support

---

## 22. Security and privacy

- Respect least-privilege scopes per integration.
- Separate read scopes from write scopes.
- Store provenance sparingly; redact body-level content when field-level evidence suffices.
- Allow per-source disablement.
- Allow per-field sensitivity rules.
- Never expose hidden chain-of-thought; expose structured reasons instead.

---

## 23. Rollout plan

### Phase 1
- Canonical metadata schema
- Snapshot ingestion
- Gap detection for Todoist + Calendar
- Review queue only
- Manual apply only

### Phase 2
- Candidate inference engine
- Policy engine
- Auto-apply for low-risk Todoist tag/project enrichments
- Audit log and rollback

### Phase 3
- Gmail/files cross-source linkage
- Learning loop
- Metadata debt dashboard
- Bulk hygiene jobs

### Phase 4
- Agent-to-agent enrichment requests
- Advanced rule authoring
- Project-specific ontology packs

---

## 24. Open questions

- Should enrichment preferences live in user memory, config, database policy, or all three?
- What is the exact trust threshold for auto-changing external-facing calendar events?
- Do we treat inferred project linkage as metadata or as a graph edge with separate semantics?
- How much LLM involvement is acceptable for high-volume hygiene passes?
- Which sources support proper field-level rollback versus only overwrite?

---

## 25. Recommended initial implementation stance

Start narrower than the fantasy, but structure for the fantasy.

Specifically:

1. build the normalized schema first,
2. implement source capability declarations,
3. support Todoist tags/project/priority and Calendar location/conference/project-link as first-class enrichments,
4. keep first release review-centric,
5. instrument everything,
6. only then allow policy-driven auto-apply.

In other words: let Vel become a meticulous clerk before it tries to become a sorcerer.
