# 11-03 Summary

## Outcome

Completed the operator-surface grounding slice for Phase 11:

- added a new `vel agent inspect` CLI command that reads the backend-owned inspect contract and renders operator-facing grounding, review, and blocker state
- added typed web transport and one backend-driven Settings surface for agent grounding instead of recomputing capability state client-side
- updated operator-facing docs so the inspect flow is discoverable before supervised execution work

This keeps Phase 11 aligned with the target boundary: backend policy decides what the agent can see or do, while CLI and web surfaces render the same typed contract.

## Implementation

### CLI inspect shell

- added [crates/vel-cli/src/commands/agent.rs](../../../../crates/vel-cli/src/commands/agent.rs)
- added `ApiClient::get_agent_inspect` in [crates/vel-cli/src/client.rs](../../../../crates/vel-cli/src/client.rs)
- wired `vel agent inspect [--json]` in:
  - [crates/vel-cli/src/commands/mod.rs](../../../../crates/vel-cli/src/commands/mod.rs)
  - [crates/vel-cli/src/main.rs](../../../../crates/vel-cli/src/main.rs)
- the text formatter surfaces:
  - grounding scope counts
  - pending review obligations
  - capability groups and entries
  - explicit blocker and escalation guidance

### Web grounding card

- added typed `AgentInspectData` decode support in [clients/web/src/types.ts](../../../../clients/web/src/types.ts)
- added API loader in [clients/web/src/data/agent-grounding.ts](../../../../clients/web/src/data/agent-grounding.ts)
- added query key in [clients/web/src/data/operator.ts](../../../../clients/web/src/data/operator.ts)
- updated [clients/web/src/components/SettingsPage.tsx](../../../../clients/web/src/components/SettingsPage.tsx) to render an `Agent grounding` card backed entirely by `/v1/agent/inspect`
- the web card now shows:
  - projects, people, and commitments in scope
  - pending execution review count
  - writeback mode
  - backend-provided capability availability
  - backend-provided blockers and escalation hints

### Docs

- updated [docs/user/daily-use.md](../../../../docs/user/daily-use.md) to include `cargo run -p vel-cli -- agent inspect` in the operator workflow

## Verification

Automated:

- `cargo test -p vel-cli agent_inspect -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts src/data/agent-grounding.test.ts src/components/SettingsPage.test.tsx`

Coverage added:

- CLI formatter and argument parsing coverage in [crates/vel-cli/src/commands/agent.rs](../../../../crates/vel-cli/src/commands/agent.rs) and [crates/vel-cli/src/main.rs](../../../../crates/vel-cli/src/main.rs)
- transport decode coverage in [clients/web/src/types.test.ts](../../../../clients/web/src/types.test.ts)
- API loader coverage in [clients/web/src/data/agent-grounding.test.ts](../../../../clients/web/src/data/agent-grounding.test.ts)
- Settings rendering coverage in [clients/web/src/components/SettingsPage.test.tsx](../../../../clients/web/src/components/SettingsPage.test.tsx)

Manual/UAT:

- skipped per operator instruction

## Notes

- `cargo test -p vel-cli agent_inspect -- --nocapture` passed with pre-existing `dead_code` warnings in `crates/vel-cli/src/client.rs`
- this slice preserved the existing backend-authenticated inspect contract and did not reintroduce client-side policy derivation
