# 65 Milestone Evidence

## Status

`0.5` backend rewrite milestone verified with execution-backed evidence.

## Flows Executed

| Flow | Evidence | Result |
|---|---|---|
| Legacy write-path quarantine | `cargo test -p veld --test phase65_cutover_routes` | Pass |
| Canonical Todoist/Google route reconciliation | `cargo test -p veld --test phase65_contract_reconciliation` | Pass |
| Milestone-spanning backend proof | `cargo test -p veld --test phase65_milestone_e2e` | Pass |
| Policy/audit/explain evidence | `cargo test -p veld --test phase65_policy_audit_evidence` | Pass |
| Governed module/bootstrap seam | `cargo test -p veld --test phase60_provider_modules` | Pass |
| Workflow runtime proof | `cargo test -p veld --test phase61_workflow_black_box` | Pass |
| Todoist black-box adapter proof | `cargo test -p veld --test phase63_todoist_black_box` | Pass |
| Google black-box adapter proof | `cargo test -p veld --test phase64_gcal_black_box` | Pass |

## Verified Outcomes

- canonical backend is the live write authority for Todoist and Google Calendar
- legacy provider write paths no longer execute writes
- canonical write routes return explicit `WriteIntent` plus explain/dispatch payloads
- canonical write routes emit runtime audit evidence
- module bootstrap, workflow runtime, Todoist, and Google Calendar operate on one backend substrate

## Known Deviations

- read/configuration compatibility surfaces remain quarantined for operator continuity
- non-`0.5` provider writes for Notes, Reminders, GitHub, and Email are explicitly deferred rather than migrated in this milestone
- the crate currently emits substantial pre-existing dead-code warnings during test compiles; these do not block the milestone verification line

## Quarantined Legacy Read Paths

- `/api/integrations`
- `/api/integrations/connections`
- `/api/integrations/connections/:id`
- `/api/integrations/connections/:id/events`
- `/api/integrations/:id/logs`
- `/api/integrations/:id/source`
- `/api/integrations/:id/path-dialog`
- `/api/integrations/google-calendar`
- `/api/integrations/google-calendar/disconnect`
- `/api/integrations/google-calendar/auth/start`
- `/api/integrations/todoist`
- `/api/integrations/todoist/disconnect`
- `/v1/sync/todoist`

## Pass / Fail Posture

Pass. No legacy write path survived the cutover.
