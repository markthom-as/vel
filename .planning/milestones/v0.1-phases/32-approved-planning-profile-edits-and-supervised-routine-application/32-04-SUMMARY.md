# 32-04 Summary

## What shipped

`32-04` closed Phase 32 by aligning the repo’s docs and contract story with the supervised planning-profile apply path that is now shipped.

The main documentation and closeout updates landed in:

- `docs/api/runtime.md`
- `docs/user/daily-use.md`
- `docs/user/setup.md`
- `docs/product/operator-mode-policy.md`
- `clients/apple/README.md`
- `docs/cognitive-agent-architecture/architecture/planning-profile-application-contract.md`

The repo now teaches one consistent rule:

- `/v1/planning-profile` is still the canonical backend-owned routine/constraint seam
- `POST /v1/planning-profile/proposals/:id/apply` is the supervised apply path for staged planning-profile proposals
- assistant entry and Apple voice can stage bounded planning-profile edits, but they still do not silently mutate the saved profile
- `Threads` remains the durable follow-through lane
- `Now`, web `Settings`, CLI, and Apple summary surfaces only show compact backend-owned continuity about what is pending, applied, or failed

I also kept the checked-in contract assets honest by re-verifying the planning-profile proposal example, schema, and manifest after the lifecycle/read-model changes.

## Verification

Passed:

- `rg -n "planning-profile|planning profile|proposal continuity|pending/applied/failed|proposal-apply|Threads continuity" docs/api/runtime.md docs/user/daily-use.md docs/user/setup.md docs/product/operator-mode-policy.md clients/apple/README.md docs/cognitive-agent-architecture/architecture/planning-profile-application-contract.md`
- `node -e "JSON.parse(require('fs').readFileSync('config/examples/planning-profile-edit-proposal.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/planning-profile-edit-proposal.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`
- `cargo test -p veld --test planning_profile_api -- --nocapture`

Previously verified in Phase 32 execution and still relevant:

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts src/components/NowView.test.tsx src/components/SettingsPage.test.tsx`
- `cargo test -p veld planning_profile -- --nocapture`
- `cargo test -p vel-cli planning_profile -- --nocapture`
- `make check-apple-swift`

Not performed:

- UAT

## Result

Phase 32 is now complete. Approved planning-profile edits have a real backend-owned apply lane, durable thread continuity, and one cross-surface explanation of what was proposed versus what actually changed.
