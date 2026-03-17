# Agent Implementation Protocol (ADX)

This protocol defines the standardized, high-signal workflow for autonomous coding agents operating on the Vel codebase.

The goal is not just to produce code quickly. The goal is to produce small, reviewable, executed changes with clear evidence and low cognitive debt.

## 1. Research & Analysis
- **Baseline first**: Run the narrowest relevant existing test or command first so you know whether the area already passes before you change it.
- **Map the territory**: Use fast symbol and file search to identify the exact modules, tests, fixtures, and docs that govern the change.
- **Reuse examples**: Prefer known-good patterns already present in this repository over novel implementations.
- **Context discipline**: Read only the code and docs needed to understand the active seam; avoid loading unrelated files.
- **Invariants**: Identify system invariants (for example immutable signals, run lifecycle rules, route/service boundaries) and ensure they are not violated.
- **Reduce cognitive debt**: If the subsystem is hard to reason about, create or refresh a concise walkthrough while you work.

## 2. Strategy & Planning
- **Internal Reflection**: Synthesize research findings into a concise, technical strategy.
- **Minimum viable slice**: Choose the smallest change that moves the ticket forward while preserving target architecture.
- **Reviewability**: Prefer a sequence of small patches over one large mixed-purpose change.
- **Plan Communication**: Present the plan to the human operator when the task needs coordination or the scope is non-trivial.
- **Execution model**: Default to one orchestrating agent. Split work only when responsibilities and review boundaries are explicit.

## 3. Execution (The "Act" Phase)
- **Surgical Changes**: Make targeted edits that stay tightly aligned with the ticket objective.
- **Red/Green by default**: For logic changes, prefer writing or extending a failing test first, then implement until it passes.
- **Contract-first discipline**: For shared data/config surfaces, update or add schema/manifest and canonical template/fixture artifacts before widening implementation.
- **Transactional Logic**: When modifying `veld` services, ensure that multi-repo writes are wrapped in a single database transaction using the repository pattern.
- **Boundary protection**: Do not normalize current architectural drift by copying it into new code.
- **Error handling discipline**: Let errors propagate to the boundary that can map them correctly; avoid swallowing failures in the middle of the stack.
- **Security defaults**: New routes, tools, or execution paths should be auth-gated or capability-gated by default unless the ticket explicitly defines a public surface.
- **Sandbox discipline**: New code-execution or tool-execution paths should run in the narrowest isolated environment available.
- **Secret mediation**: Prefer brokered capabilities, scoped tokens, or boundary-time injection over handing raw secrets to agents or clients.
- **Fail closed**: Unknown routes, unsupported tool actions, and unmatched external-access requests should reject safely by default.
- **Repo-aware supervision**: Self-modification paths must keep read scope broader than write scope, with explicit writable boundaries and review gates.
- **Style Alignment**: Adhere to existing naming, formatting, typing, and module-boundary conventions.

## 4. Verification & Validation
- **Automated testing**: Run the narrowest meaningful tests for the affected crate, package, module, or client first, then widen only as needed.
- **Manual execution**: Exercise the changed behavior directly. For API and web flows, prefer execution-backed checks such as CLI commands, `curl`, or browser automation.
- **Edge cases**: Use temporary files, fixtures, or one-off manual probes to explore likely failure cases before closing the task.
- **Diagnostic hygiene**: Run relevant linting, doctor, or build checks when they materially improve confidence.
- **Observability**: Confirm that new external calls, tool boundaries, or workflow transitions are visible through logs, traces, or run events.
- **Secret hygiene**: Verify that no raw secrets leak into logs, prompts, traces, snapshots, or returned payloads, and that access remains scoped to the intended host/path/tool boundary.
- **Evidence**: Do not describe a behavior as fixed unless you executed a check that demonstrates it.

## 5. Completion & Documentation
- **Verification summary**: Report what was tested, how it was tested, and any remaining limits or risks.
- **Doc repair**: Update nearby docs or authority pointers when your change affects module boundaries, contracts, or workflows.
- **Walkthroughs when useful**: If the work uncovered a confusing area, leave behind a concise explanation or linear walkthrough.
- **Compound improvement**: Capture reusable prompts, verification techniques, or guardrails in repo docs so future agent runs start from a higher baseline.
- **Review responsibility**: Treat agent output as draft implementation until a human has reviewed it.
- **Commit proposing**: Propose a concise, "why"-focused commit message. Do not commit unless explicitly asked.
