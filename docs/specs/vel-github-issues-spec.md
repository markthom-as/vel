# Vel GitHub Issues Spec

Status: Proposed  
Audience: self-knowledge implementer, orchestration implementer, client implementer  
Purpose: define how GitHub issues become first-class evidence and work objects inside Vel's self-knowledge and introspection loops

---

## Purpose

Vel's self-awareness should not stop at source files and docs. If Vel is reasoning about its own codebase, backlog, drift, and maintenance work, it also needs structured awareness of GitHub issues.

GitHub issues are not truth about implementation. They are truth about:

- reported gaps
- intended work
- known failures
- review follow-ups
- deferred decisions
- user-reported pain

That makes them part of Vel's **intent layer** and, when linked to runs or failures, part of the **runtime layer** too.

## Goals

Vel should support:

- indexing GitHub issues as durable self-knowledge artifacts
- linking issues to code, docs, tests, runs, and validation results
- distinguishing implemented behavior from planned or unresolved work
- using issues as inputs to introspection, maintenance, and self-work loops
- exposing issue-aware work requests across clients with different execution capabilities

## Non-Goals

Do not treat GitHub issues as:

- canonical implementation truth
- permission for direct self-modification
- a replacement for `docs/status.md`
- a generic project management suite inside Vel

## Issue Model

Vel should normalize GitHub issues into a first-class `github_issue` record with:

- `issue_id`
- `repository`
- `number`
- `title`
- `state`
- `labels`
- `assignees`
- `author`
- `body_markdown`
- `html_url`
- `created_at`
- `updated_at`
- `closed_at`
- `milestone`
- `linked_pr_numbers`
- `metadata_json`

Representative derived fields:

- `issue_kind` such as `bug`, `feature`, `docs`, `infra`, `followup`, `self_work`
- `intent_strength` such as `proposed`, `accepted`, `in_progress`, `blocked`, `superseded`
- `scope_tags` such as `api`, `storage`, `client`, `docs`, `tests`, `cluster`, `self_knowledge`

## Self-Knowledge Role

GitHub issues belong primarily to the **intent layer**, but they should connect to all four self-knowledge layers:

- source-of-truth layer: linked files, tests, migrations, commands, or changed modules
- intent layer: the issue itself, linked docs/specs/tickets, labels, milestones
- runtime layer: failing runs, smoke checks, validation failures, regression evidence
- belief layer: Vel's synthesized understanding of what is broken, planned, stale, or contradicted

This allows questions like:

- what open issues touch the current module?
- which planned capabilities are only documented in issues/specs?
- which issue claims are contradicted by current tests or code?
- which runtime failures already map to an issue?

## Ingestion

Initial ingestion can be polling-based or connector-driven, but the normalized contract should be stable.

Minimum ingest steps:

1. fetch issue metadata and body
2. normalize labels, state, and timestamps
3. persist a durable issue record
4. extract repo-local references:
   - file paths
   - doc paths
   - ticket IDs
   - PR numbers
   - commands or test names when explicit
5. create edges into the self-knowledge graph

## Linking Rules

Vel should create durable links when an issue references:

- a doc path under `docs/`
- a source file under `crates/`, `clients/`, `packages/`, `scripts/`, or `migrations/`
- a ticket ID
- a PR number
- a validation profile or build/test command

Example edges:

- `github_issue -> documents -> docs/specs/vel-cluster-sync-spec.md`
- `github_issue -> affects -> crates/veld/src/services/client_sync.rs`
- `github_issue -> validated_by -> repo-verify`
- `github_issue -> tracked_by -> SK-011`

## Capability-Aware Client Work

Issue-driven self-work often needs different clients or nodes for different actions:

- lightweight clients can read issue state, queue low-risk actions, and request validation
- repo-capable clients can sync branches
- build-capable clients can run build/test profiles
- environment-specific clients can run profiles that depend on local tooling such as web or Apple checks

The canonical authority must remain responsible for:

- deciding what work is requested
- recording the request durably
- integrating outcomes into the self-knowledge graph

Workers or clients may differ in execution power, but they must advertise that explicitly rather than improvising capability.

## Cluster Bootstrap Contract

Nodes that participate in issue-driven self-work should expose capability metadata in bootstrap:

- `capabilities`
- `branch_sync`
- `validation_profiles`

These fields tell clients whether a node can:

- accept branch sync requests for the repo
- advertise supported validation/build/test profiles
- distinguish environment-specific checks such as `api`, `web`, `apple`, `repo`, or `runtime`

## Sync Action Contract

Issue-driven execution requests should flow through structured sync actions rather than free-form chat text.

Minimum queued actions:

- `branch_sync_request`
- `validation_request`

Each request should include:

- repo root
- requested branch or validation profile
- optional base branch / environment
- requester identity when available

The first shipped behavior may queue these requests as durable signals before a full executor exists. That is acceptable as long as the request contract is explicit and replay-safe.

## Guardrails

- `docs/status.md` remains canonical for shipped behavior.
- Open issues do not imply implementation.
- Closed issues do not imply correctness unless supported by code/tests/runtime evidence.
- Validation and branch sync requests must remain capability-scoped.
- Environment-specific checks must be declared, not guessed.

## Suggested First Implementation Slice

1. add normalized GitHub issue schema or signal/artifact representation
2. ingest issue metadata with durable provenance
3. expose issue links in self-knowledge queries
4. expose capability metadata in cluster bootstrap
5. allow clients to queue branch sync and validation requests through structured sync actions
