---
title: Vel Tester Readiness And Onboarding Spec
status: proposed
owner: product / runtime / clients
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

This spec defines the process for moving Vel from a contributor-oriented codebase into a tester-usable system with a clear first-run path.

The target is not a mass-market installer. The target is a trustworthy dogfooding and external-tester flow where a technically capable tester can install Vel, configure one authority node, link one or more clients, and verify that sync, capture, and daily-orientation flows work without needing to understand the repo internals.

# Problem

Vel currently has enough substrate for contributors and the primary operator:

- `veld` runs locally and exposes HTTP APIs,
- `vel` provides operator commands,
- the web client and Apple bootstrap clients can talk to the same daemon,
- macOS local-source auto-discovery exists for some snapshot-backed inputs,
- sync/bootstrap metadata exists for richer clients,
- Nix and Docker/NAS runtime paths exist.

That is real progress, but it is still repo-operator shaped rather than tester shaped.

Today a new tester still has to piece together:

- which runtime path to use,
- which config values are required versus optional,
- how a client finds the daemon,
- how to link a phone/watch/macOS client to the intended authority node,
- what auto-discovery should happen,
- what “healthy and ready” looks like,
- how to recover from stale or missing local source data.

Without a deliberate tester-readiness contract, every new tester becomes an ad hoc support incident.

# Scope

This spec covers:

- authority-node setup for a tester-safe local or tailnet deployment,
- first-run onboarding and configuration,
- endpoint discovery and client linking,
- local-source auto-discovery and bootstrap sync,
- tester-facing health verification and diagnostics,
- packaging/documentation/rollout requirements needed for real testers.

This spec does not define:

- a public app-store release process,
- anonymous consumer onboarding,
- multi-tenant account systems,
- generalized zero-config LAN discovery,
- fully productized mobile sync beyond the current bootstrap architecture.

# Current Truth

This spec builds on shipped behavior described in `docs/status.md`.

Implemented now:

- local daemon plus CLI,
- web client and Apple bootstrap clients,
- `GET /v1/sync/bootstrap` and cluster/bootstrap metadata,
- endpoint candidate resolution using explicit base URLs and localhost fallbacks,
- macOS local-source auto-discovery under `~/Library/Application Support/Vel/`,
- Docker/NAS and Nix-hosted runtime paths,
- local settings and integration configuration,
- `vel doctor`, `vel health`, and config inspection.

Not yet solved as a coherent tester flow:

- one obvious install path for non-contributors,
- a first-run onboarding contract,
- a durable linking/pairing artifact for clients,
- a guided “required vs optional” config model,
- a tester-grade readiness checklist,
- a canonical recovery flow when discovery or linking fails.

# Product Goal

A tester should be able to complete this journey in under 20 minutes:

1. install or unpack Vel on one authority machine,
2. run one first-run setup command or wizard,
3. confirm the daemon is healthy,
4. optionally let Vel auto-discover local sources,
5. generate a client-link bundle or scan target,
6. open a web or Apple client and connect without manual URL guessing,
7. verify that current context, captures, commitments, and sync freshness are visible,
8. recover using explicit diagnostics if any step fails.

# Design Principles

## 1. One authority first

Tester onboarding should start with one explicit authority node.

Do not force early testers to reason about cluster topology, authority handoff, or distributed role selection. Those remain advanced capabilities layered on top of a clear single-authority setup path.

## 2. Explicit beats magical

Auto-discovery should reduce friction, but it must remain inspectable:

- what was discovered,
- from where,
- whether it is active,
- why it failed.

Vel should not hide configuration behind implicit machine-local behavior that testers cannot debug.

## 3. Link clients with a durable artifact

Client linking should not depend on remembering several URL fields or manually retyping hostnames across devices.

There should be one canonical linking artifact produced by the authority node setup flow.

## 4. Local-first trust must stay visible

The tester flow must state clearly:

- where the database lives,
- where artifacts live,
- which local snapshots are read,
- what network endpoint clients are connecting to,
- which permissions are optional versus required.

## 5. Bootstrap clients stay thin

The daemon remains the brain.

Tester onboarding may improve clients, but it must not move domain logic into Apple or web clients just to compensate for missing daemon-side setup contracts.

# Personas

## Primary: technical external tester

This person can run a packaged binary, Docker compose, or a guided local install, but should not need to edit several config files by hand or infer architecture from source.

## Secondary: internal dogfooding tester

This person may run from the repo, but still benefits from the same first-run flow and diagnostics. The tester path should also reduce friction for contributors.

# Target Tester Journey

## Phase 0: obtain a runnable build

Vel should expose a small set of supported tester distribution paths:

- packaged local runtime for macOS/Linux,
- Docker/NAS authority deployment,
- repo/dev path for contributors.

One path should be marked as the recommended tester path.

## Phase 1: first-run authority setup

The tester runs a first-run setup entrypoint such as:

- `vel setup`,
- a web-first setup screen,
- or an equivalent guided bootstrap command.

This flow should:

- confirm writable data directories,
- initialize config if absent,
- validate bind/base URL choices,
- generate or confirm node identity,
- optionally configure Tailscale/LAN/public endpoint metadata,
- offer opt-in local source discovery,
- write a readiness summary.

## Phase 2: local-source discovery and setup

The authority node should scan known local paths and classify each source:

- discovered and ready,
- discovered but permission-blocked,
- configured manually,
- not found,
- unsupported on this platform.

At minimum this applies to the currently supported macOS-local snapshot paths and repo-local file-backed integrations.

Discovery results should be persisted as structured state, not only printed to logs.

## Phase 3: linking artifact generation

Once the authority node is healthy, Vel should generate a client-link artifact with enough information for clients to connect safely.

Proposed artifact contents:

- authority `node_id`,
- display name,
- preferred endpoint list in priority order,
- transport hints such as `tailscale`, `lan`, `localhost`,
- optional validation hash or short-lived linking token,
- protocol/API version,
- timestamp / expiry if tokenized,
- human-readable troubleshooting hints.

Proposed forms:

- file export,
- copyable text blob,
- QR code for phone/watch setup,
- web settings page reveal.

## Phase 4: client bootstrap and trust confirmation

A client should accept the linking artifact and then:

- store the resolved endpoint candidates,
- call health/bootstrap endpoints,
- confirm it reached the intended authority node,
- hydrate cached bootstrap state,
- show connection freshness and sync transport,
- surface precise failure reasons if bootstrap fails.

Clients should not require testers to know the difference between `vel_base_url`, `vel_tailscale_url`, and `vel_lan_base_url` unless the automatic linking flow fails and advanced override is needed.

## Phase 5: ready-state verification

Vel should expose one tester-facing readiness view showing:

- daemon reachable,
- schema/data paths initialized,
- authority identity,
- active endpoint metadata,
- discovered/configured integrations,
- last successful bootstrap sync,
- client-link availability,
- last evaluate/context generation time,
- actionable warnings.

# Required Deliverables

## 1. A first-run setup contract

Add one canonical setup flow for non-contributor testers.

Minimum capabilities:

- create missing config/state,
- validate effective paths and endpoint metadata,
- print or persist a machine-readable readiness report,
- point to the next exact step.

## 2. A structured discovery registry

Vel should have a daemon-side registry for discoverable setup items.

Each item should report:

- source key,
- platform applicability,
- discovery path or mechanism,
- status,
- permission requirements,
- last checked time,
- operator-facing message.

This can start as a read-only diagnostics surface before evolving into editable setup state.

## 3. A client linking model

Introduce a first-class daemon contract for client linking.

Suggested API surfaces:

- `GET /v1/onboarding/status`
- `POST /v1/onboarding/discover`
- `POST /v1/onboarding/link-artifact`
- `POST /v1/onboarding/link/consume`

Exact route names can change, but the concepts should not remain implicit.

## 4. One source of endpoint truth

The authority node should publish one normalized endpoint set derived from config and runtime checks.

This should replace the current scattered “set these three URLs correctly everywhere” burden.

The published set should include:

- preferred endpoint,
- ordered fallbacks,
- transport labels,
- reachability observations where available.

## 5. Tester-facing docs

User-facing setup docs should be expanded to match the shipped onboarding flow.

This spec depends on the user-documentation architecture in `docs/specs/vel-user-documentation-spec.md`.

## 6. Readiness diagnostics

`vel doctor` and the web/operator surfaces should gain a dedicated tester-readiness mode that answers:

- can I start using Vel now,
- what is missing,
- what is optional,
- what should I do next.

# Proposed Architecture

## Onboarding service

Add a daemon-side onboarding service that owns:

- first-run state detection,
- setup status aggregation,
- local discovery checks,
- link artifact generation,
- readiness scoring.

This keeps route handlers thin and matches the existing service-oriented repo boundaries.

## Setup state model

Add structured setup status objects in the service/domain boundary, mapped to transport DTOs in `vel-api-types`.

Proposed logical objects:

- `OnboardingStatus`
- `SetupCheck`
- `DiscoveryItem`
- `LinkArtifact`
- `LinkedClientRecord`

Storage can start minimally if some state is derived at runtime, but link events and client-link consumption should become durable so support and debugging are possible.

## Linking token model

Client linking should support two modes:

- static link bundle for trusted manual use on the same personal cluster,
- short-lived tokenized bundle for easier remote/device handoff.

The first mode is enough for the initial tester slice. The second mode should be designed in early so the artifact format does not paint the system into a corner.

# Configuration Model

Tester onboarding should classify configuration into three groups.

## Required for first use

- data paths,
- authority bind/base URL,
- one preferred client-reachable endpoint,
- node display name or generated default.

## Optional but common

- Tailscale endpoint,
- LAN fallback endpoint,
- notes path,
- calendar/Todoist/activity/health/git/messaging/transcript sources,
- local model paths.

## Advanced

- cluster-worker settings,
- validation profiles,
- branch-sync capabilities,
- experimental integrations,
- multi-node sync controls.

The onboarding flow should present only the first group by default.

# Auto-Discovery Model

Auto-discovery should be platform-scoped and capability-scoped.

## macOS

Keep and formalize discovery for:

- `~/Library/Application Support/Vel/activity/snapshot.json`
- `~/Library/Application Support/Vel/health/snapshot.json`
- `~/Library/Application Support/Vel/git/snapshot.json`
- `~/Library/Application Support/Vel/messages/snapshot.json`
- `~/Library/Application Support/Vel/notes/`
- `~/Library/Application Support/Vel/transcripts/snapshot.json`

Also report permission-sensitive caveats such as:

- HealthKit authorization missing,
- Full Disk Access likely required for Messages export.

## Repo/dev mode

Preserve current `var/integrations/*` defaults for contributors, but treat them as a dev-profile onboarding path rather than the main tester story.

## Linux/NAS authority mode

Discovery should focus on configured file paths and container-mounted directories rather than pretending Apple-local paths apply.

# Client Linking Model

## Canonical flow

1. Authority node completes setup.
2. Authority generates a link artifact.
3. Tester opens client and chooses `Link to Vel`.
4. Client scans/imports/pastes artifact.
5. Client verifies authority identity and bootstrap reachability.
6. Client stores resolved endpoint candidates.
7. Client hydrates bootstrap state and shows connected status.

## Failure handling

If linking fails, the client should classify the failure as one of:

- malformed artifact,
- endpoint unreachable,
- wrong node reached,
- bootstrap API version mismatch,
- authorization/token expired,
- daemon healthy but setup incomplete.

This is a better support model than a generic “cannot connect” failure.

# Rollout Phases

## Phase 1: codify current bootstrap path

Goal:

- expose current setup/discovery/bootstrap state through one onboarding status surface,
- keep manual config as fallback,
- do not require QR/token flows yet.

Deliverables:

- onboarding service,
- readiness DTOs and endpoint,
- `vel setup` or equivalent guided CLI,
- web/operator onboarding page,
- updated user docs.

## Phase 2: durable link artifacts

Goal:

- remove manual multi-URL entry as the default client path.

Deliverables:

- generated link bundle,
- Apple/web client consume flow,
- linked-client records,
- clearer trust/identity confirmation.

## Phase 3: richer discovery and repair

Goal:

- make setup failures self-diagnosing.

Deliverables:

- discovery registry,
- permission-blocked classifications,
- repair hints,
- re-run discovery from CLI/web.

## Phase 4: tester packaging

Goal:

- give testers a recommended non-repo install path.

Deliverables:

- packaged binary or installer path for authority node,
- versioned distribution notes,
- upgrade/migration notes,
- minimal release checklist.

# Acceptance Criteria

This spec is satisfied when:

1. a tester can set up an authority node without editing multiple config files by hand,
2. the recommended setup path clearly distinguishes required versus optional configuration,
3. the authority node can report setup/discovery/readiness state through one canonical surface,
4. a tester can link a client without manually typing several endpoint variants,
5. clients can explain linking/bootstrap failures in specific terms,
6. macOS local-source discovery is explicit and inspectable,
7. user docs describe the tester flow in present tense and mark bootstrap limitations clearly,
8. the old contributor/dev path still works without becoming the only documented path.

# Open Questions

- Should the initial link artifact be unauthenticated but local-trust scoped, or tokenized from day one?
- Should the first guided setup live primarily in CLI, web, or both?
- Should linked-client records be purely local operational metadata, or do they need lifecycle actions such as revoke/rename?
- How much of Tailscale detection should be automatic versus explicit config entry?
- Which packaged distribution path should be the first-class tester recommendation: Docker/NAS, macOS app bundle plus daemon binary, or a signed standalone daemon package?

# Recommended Next Slice

The minimum viable tester-readiness slice should be:

1. add daemon-side onboarding status aggregation,
2. add `vel setup` with readiness output,
3. add a web onboarding/readiness page,
4. formalize endpoint publication from the authority node,
5. add an exported link bundle format,
6. wire Apple and web clients to consume that bundle,
7. update user docs around the new setup path.

That sequence gives Vel a coherent tester story without requiring full productization or a broader distributed rewrite.
