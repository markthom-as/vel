# 27-04 Summary

## Completed

- aligned architecture, runtime, product, and user docs around the shipped canonical scheduler-rule model
- added a checked-in machine-readable schema and example for canonical scheduler rules
- documented that persisted commitment `scheduler_rules` are now the backend-owned scheduling semantics used by reflow, assistant context, and grounding
- closed the phase with focused truth checks and narrow live verification instead of broad planner claims

## Main files

- `config/schemas/canonical-scheduler-rules.schema.json`
- `config/examples/canonical-scheduler-rules.example.json`
- `config/README.md`
- `docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md`
- `docs/api/runtime.md`
- `docs/user/daily-use.md`
- `docs/product/operator-mode-policy.md`

## Verification

- `node -e "JSON.parse(require('fs').readFileSync('config/schemas/canonical-scheduler-rules.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/examples/canonical-scheduler-rules.example.json','utf8')); console.log('json-ok')"`
- `cargo test -p vel-core scheduler -- --nocapture`
- `cargo test -p vel-storage commitments_repo -- --nocapture`
- `cargo test -p veld reflow -- --nocapture`
- `cargo test -p veld chat::tools -- --nocapture`
- `rg -n "scheduler_rules|canonical scheduler|block:\\*|cal:free|time:\\*|assistant context|same-day remaining-day" docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md docs/api/runtime.md docs/user/daily-use.md docs/product/operator-mode-policy.md config/README.md`

## Notes

- this closes the canonical rule model for the current bounded scope: same-day recovery, assistant grounding, and commitment-aware recall
- it does not claim a broad autonomous planner or universal provider-tag normalization story yet
