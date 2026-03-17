# Agent Implementation Protocol (ADX)

This protocol defines the standardized, high-signal workflow for autonomous coding agents operating on the Vel codebase. 

## 1. Research & Analysis
- **Tooling**: Use `glob` and `grep_search` to identify symbols, file paths, and existing patterns.
- **Context Depth**: Read files with `read_file` only after identifying specific line ranges to minimize context token usage.
- **Invariants**: Identify existing system invariants (e.g., "Signals are immutable") and ensure they are not violated.

## 2. Strategy & Planning
- **Internal Reflection**: Synthesize research findings into a concise, technical strategy.
- **Plan Communication**: Present the plan to the human operator if the task is a **Directive**.
- **Change Scope**: Keep changes surgical and strictly related to the ticket objectives.

## 3. Execution (The "Act" Phase)
- **Surgical Changes**: Use `replace` for targeted edits and `write_file` for new or small files.
- **Transactional Logic**: When modifying `veld` services, ensure that multi-repo writes are wrapped in a single database transaction using the repository pattern.
- **Style Alignment**: Adhere to existing naming, formatting (Rust `fmt`), and typing conventions.

## 4. Verification & Validation
- **Unit Testing**: Run `cargo test` for the specific crate and module affected by the change.
- **Integration Testing**: Use the `crates/veld/tests/` suite for end-to-end API and logic verification.
- **Smoke Check**: Execute `vel cli` commands to verify the change from an operator's perspective.
- **Diagnostic Hygiene**: Run `vel doctor` and `cargo clippy` to ensure no new warnings or regressions were introduced.

## 5. Completion & Documentation
- **Status Updates**: Update the `status` of the active ticket in `docs/tickets/`.
- **Commit Proposing**: Propose a concise, "why"-focused commit message. Do not commit unless explicitly asked.
