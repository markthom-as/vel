---
title: "Establish build, CI, and repo-truth checks for Rust, web, docs, and status"
status: todo
owner: agent
type: infrastructure
priority: high
created: 2026-03-15
depends_on: []
labels:
  - vel
  - ci
  - testing
  - docs
---
The repo has enough moving parts now that "it probably still works" is no longer an acceptable release process.

Also: this review environment could not run `cargo test` because Rust tooling was not present here, so I am intentionally not pretending the current tree is green. The repo needs machine-enforced truth, not optimistic aura.

## Goal

Make the repository self-reporting about whether it is healthy.

## Tasks

- Add CI for:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings` or a scoped warning policy
  - `cargo test`
  - web install/build/test/lint
- Standardize package manager/workspace expectations at the repo root.
- Add a lightweight `make`, `just`, or equivalent task runner for common dev flows.
- Add a status-truth check so docs claiming "implemented" do not drift too far from code reality.
- Add integration smoke tests for the daemon + CLI happy path.
- Add a fixture/dev script for bootstrapping demo data into a local DB.

## Suggested extra check

A small doc hygiene script that verifies:
- `docs/status.md`
- `README.md`
- API docs
- ticket folders

do not contradict obvious repo structure or removed endpoints.

## Acceptance Criteria

- A fresh clone can run one documented command to validate the repo.
- CI covers both Rust and web surfaces.
- The repo has at least one end-to-end smoke test for daemon/API/CLI.
- Documentation drift is treated as a failing quality issue, not fan fiction.

## Notes for Agent

At this stage, CI is not bureaucracy. It is the only adult in the room.
