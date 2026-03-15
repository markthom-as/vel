# vel_weekly_synthesis_spec.md

Status: Canonical weekly synthesis specification  
Audience: coding agent implementing Vel  
Purpose: define the first reflective synthesis artifact Vel should generate from commitments, nudges, signals, threads, and self-model data

---

# 1. Purpose

Weekly synthesis is Vel’s first **reflective artifact**.

Its job is not to run real-time decisions.  
Its job is to look across the week and answer:

- what commitments mattered?
- what got done?
- what kept getting deferred?
- what patterns of drift or overcommitment appeared?
- what threads dominated attention?
- what did Vel do well or poorly?
- what should change next week?

This is where Vel starts helping improve Vel.

---

# 2. Design Principles

## 2.1 Reflective, not operational
Weekly synthesis should not drive immediate nudges.  
It should produce an artifact for review and later policy tuning.

## 2.2 Data-grounded
The synthesis should be based on durable records:
- commitments
- nudges
- signals
- context timeline
- threads
- self-model metrics
- transcripts/captures if available

## 2.3 Structured first
The first implementation should produce a structured artifact.
Natural-language polish can come later.

## 2.4 LLM optional at first
A rule-based scaffold is acceptable initially.
LLM enrichment can be layered on later.

---

# 3. Inputs

Weekly synthesis should consume data from the previous 7 days.

Required inputs:
- commitments
- commitment_risk history
- nudges + nudge_events
- signals
- current_context timeline
- threads + thread_links
- self-model metrics / feedback
- recent artifacts if useful

Optional later:
- transcripts
- project planning notes
- intent objects

---

# 4. Output Artifact

The synthesis should produce a durable artifact:

- artifact_type: `weekly_synthesis`
- storage_kind: managed
- canonical format: JSON
- optional secondary format later: markdown

It should also be linked via refs to major input entities.

---

# 5. Canonical Output Shape

Suggested JSON structure:

```json
{
  "week_start": 1700000000,
  "week_end": 1700604800,
  "summary": {
    "commitments_completed": 14,
    "commitments_open": 9,
    "nudges_sent": 22,
    "nudges_resolved": 15,
    "critical_risk_events": 3
  },
  "top_commitment_patterns": [],
  "top_threads": [],
  "drift_patterns": [],
  "alignment_observations": [],
  "vel_self_review": [],
  "suggested_adjustments": []
}
```

Keep the shape explicit and versionable.

---

# 6. Required Sections

Implement these sections first.

## 6.1 Operational recap
Summarize:
- commitments completed
- commitments still open
- high-risk commitments
- active/danger nudges
- major context shifts

## 6.2 Drift patterns
Examples:
- repeated morning drift
- meds delayed on early-meeting days
- repeated snoozes on same commitment kind
- prep windows repeatedly too tight

## 6.3 Thread summary
Examples:
- which project/person/theme threads dominated
- which threads remained unresolved
- which dormant threads resurfaced

## 6.4 Vel self-review
Examples:
- which nudges were helpful
- which nudges were ignored
- where Vel lacked enough context
- where policy timing appears wrong

## 6.5 Suggested adjustments
Examples:
- increase prep default
- increase commute buffer
- add a focus block
- explicitly schedule neglected priorities
- prompt for clarifying commitment consequence

---

# 7. Synthesis Sections in Detail

## 7.1 Operational recap

Suggested fields:
```json
{
  "completed_commitment_ids": [],
  "open_commitment_ids": [],
  "high_risk_commitment_ids": [],
  "danger_nudge_ids": []
}
```

## 7.2 Drift patterns

Each pattern should include:
- pattern name
- evidence ids
- count
- suggested interpretation

Example:
```json
{
  "pattern": "meds_delayed_before_early_meetings",
  "count": 3,
  "evidence": ["com_1", "nud_2", "sig_8"],
  "note": "Medication commitments tended to remain open when meetings started early."
}
```

## 7.3 Thread summary

Each thread summary should include:
- thread id
- title
- thread type
- open commitment count
- recent activity count
- unresolved status

## 7.4 Vel self-review

Use self-model metrics and feedback signals.

Example:
```json
{
  "observation": "gentle nudges were usually snoozed for commute commitments",
  "evidence": ["metric_1", "nud_4", "nud_7"],
  "confidence": "medium"
}
```

## 7.5 Suggested adjustments

These are not automatic changes.

Each suggestion should include:
- suggestion type
- rationale
- evidence ids
- confidence
- whether explicit user confirmation is needed

---

# 8. Weekly Synthesis Modes

Support at least two modes eventually.

## 8.1 personal weekly synthesis
General weekly reflection across all commitments/threads.

## 8.2 project weekly synthesis
Scoped to one project thread, especially `Vel`.

Example:
```bash
vel synthesize week
vel synthesize week --project vel
```

Start with global weekly synthesis, but keep project scoping in mind.

---

# 9. Relation to Intent vs Behavior

Weekly synthesis should eventually compare:

- what you said mattered
- what you actually did
- what got calendar time
- what got ignored
- what generated nudges but no completion

This may initially be sparse if explicit intent capture is limited, but the artifact structure should leave room for:

```json
"alignment_observations": [...]
```

Examples:
- Vel was a recurring discussion thread but had little calendar allocation
- health-related commitments were open most days despite being repeatedly surfaced

---

# 10. Relation to Overcommitment

Weekly synthesis should be the first place Vel surfaces overcommitment analysis.

Possible indicators:
- too many overlapping high-consequence commitments
- repeated danger nudges
- repeated carryover of open commitments
- calendar saturation with unresolved project threads

An initial simple observation is enough.  
Do not build a giant optimizer yet.

---

# 11. Relation to Self-Model

Weekly synthesis should include a section for Vel’s own performance.

Questions to answer:
- Did Vel’s nudges help?
- Which nudges were most effective?
- Which were ignored or annoying?
- Where did Vel lack enough information?
- Which suggestions were accepted or rejected?

This is critical for the dogfooding loop.

---

# 12. Generation Strategy

## 12.1 Initial implementation
Start with deterministic aggregation and pattern rules.

No LLM required initially.

This first version should:
- aggregate counts
- detect repeated conditions
- summarize threads
- surface obvious patterns
- emit suggested adjustments

## 12.2 Later enhancement
Optional LLM layer may:
- improve phrasing
- produce natural-language synthesis
- compare intent vs behavior with more nuance
- cluster themes from transcripts/captures

But the first implementation should already be useful without it.

---

# 13. Artifact + Provenance Requirements

Weekly synthesis must produce:
- one `weekly_synthesis` artifact
- refs linking artifact to major input entities (or at least to the run + key threads/commitments)

At minimum, provenance should support:
- this artifact came from this date range
- these commitments
- these nudges
- these threads
- these metrics

This keeps weekly synthesis inspectable and trustworthy.

---

# 14. CLI / API Surface

## 14.1 CLI
```bash
vel synthesize week
vel synthesize week --project vel
vel artifact latest --type weekly_synthesis
```

## 14.2 API
Suggested later:
- `POST /v1/synthesis/week`
- `GET /v1/artifacts?type=weekly_synthesis`

For now, CLI-first is fine.

---

# 15. Testing Requirements

## 15.1 Unit tests
- counts aggregate correctly
- repeated snoozes detected
- danger nudge patterns detected
- thread summaries computed correctly

## 15.2 Replay tests
Given one week of fixtures:
- weekly synthesis artifact produced
- expected sections populated
- provenance links present

## 15.3 Explainability tests
Ensure suggested adjustments include evidence ids and rationale.

---

# 16. Minimal First Slice

The first useful weekly synthesis slice should do:

1. aggregate completed/open commitments
2. aggregate nudges sent/resolved/snoozed
3. identify top 3 threads by activity
4. detect one drift pattern
5. emit one suggested adjustment
6. store a `weekly_synthesis` artifact

This is enough to make the reflective loop real.

After that, add:
- self-model review
- alignment observations
- project-scoped weekly synthesis
- transcript-aware thematic synthesis

---

# 17. Practical Engineering Rules

1. Keep first synthesis structured.
2. Use real stored data, not vague heuristics.
3. Every suggestion must cite evidence.
4. Weekly synthesis must be an artifact, not just console output.
5. Keep project scoping in mind from the beginning.
6. Do not let LLM phrasing substitute for real aggregation.
7. Reflection is advisory, not authoritative.

---

# 18. Success Criteria

Weekly synthesis is successful when:

- a weekly artifact can be generated reliably
- it summarizes commitments/nudges/threads coherently
- it surfaces at least one meaningful drift or overcommitment pattern
- it includes at least one actionable suggestion
- it includes enough evidence/provenance to be trusted
- it helps generate “Vel on Vel” improvement work

---

# 19. Final Summary

Weekly synthesis is Vel’s first **reflective mirror artifact**.

Operational systems keep the day moving.
Weekly synthesis helps reveal:
- what kept recurring
- what drifted
- what mattered
- what Vel did well or poorly
- what should change next

In short:

> weekly synthesis turns the week’s signals, commitments, and nudges into a pattern-bearing artifact you can actually learn from.