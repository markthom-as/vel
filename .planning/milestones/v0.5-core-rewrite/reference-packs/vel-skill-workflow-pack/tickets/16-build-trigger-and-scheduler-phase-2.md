# Build trigger and scheduler (phase 2)

## Scope
- Scheduled triggers
- durable pending jobs
- event trigger interface
- dedupe keys and cool-downs
- quiet hours/DND gating

## Acceptance criteria
- schedules survive restart
- event triggers can enqueue runs
- duplicate triggers are suppressed per configured policy
