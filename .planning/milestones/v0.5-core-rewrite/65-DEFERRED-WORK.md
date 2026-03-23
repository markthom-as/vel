# 65 Deferred Work

## Post-0.5 Deferred Work

The following items are explicitly not included in milestone `0.5` and must not be mistaken for incomplete cutover work.

### Providers outside the `0.5` proving-adapter scope

- Notes writeback migration to canonical `WriteIntent`
- Apple Reminders integration as a future provider of the same canonical `Task`
- GitHub writeback migration to canonical `WriteIntent`
- Email draft/send migration to canonical `WriteIntent`

### Platform and product work

- web and Apple clients rebuilt against `0.5` canonical backend contracts
- trigger/scheduler/background automation over the workflow runtime
- broader connector expansion beyond Todoist and Google Calendar

### Higher-order semantics

- richer behavior analytics beyond the minimum reschedule/rewrite proof set
- post-`0.5` availability policy expansion such as work hours, travel buffers, and soft holds
- broader task ontology expansion beyond the frozen `0.5` task schema

## Not Included

- no provider-first task species
- no provider-first calendar ontology
- no new legacy writeback paths
- no client/UI rebuild bundled into backend cutover
