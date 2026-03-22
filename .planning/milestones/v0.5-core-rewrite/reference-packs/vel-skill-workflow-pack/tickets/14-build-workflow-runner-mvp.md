# Build workflow runner MVP

## Scope
- Manual trigger only
- Sequential execution only
- Step types: skill, hook, gate, emit
- Checkpoint after each step
- Resume from checkpoint

## Acceptance criteria
- workflow run survives restart between steps
- gate pauses and resumes correctly
- step outputs stored in run record
