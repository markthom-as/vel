# Hosted V1: Signalling, Backup, And Web Dashboard

Status: future planning spec, not shipped behavior
Last updated: 2026-03-18

## Purpose

This document defines a pragmatic first hosted Vel offering that adds three cloud-assisted capabilities without rewriting Vel into a shared multi-tenant cognition backend:

- authenticated client signalling
- paid cloud backup and restore
- a hosted web dashboard

The intent is to preserve Vel's current local-first authority model while adding enough hosted infrastructure to make cross-device continuity, recovery, and remote visibility materially better.

## Non-Authority Note

This file is planning material only.

Current shipped behavior still follows:

- [MASTER_PLAN.md](../MASTER_PLAN.md)
- [README.md](../../README.md)
- [docs/user/privacy.md](../user/privacy.md)
- [docs/user/deployment.md](../user/deployment.md)

Those documents describe the present runtime as local-first, single-operator, and backed by a local daemon with local persistence.

## Product Shape

Hosted V1 is not "Vel as a fully shared SaaS backend."

Hosted V1 is:

- a user-scoped control plane
- a user-scoped signalling service
- encrypted cloud backup for local Vel state
- a hosted web dashboard for status, continuity, and recovery

Hosted V1 is not:

- a shared multi-tenant cognition database
- collaborative multi-user workspaces
- central execution authority for all user cognition state
- a requirement that core daily operation always run in the cloud

## Design Goal

The first hosted offering should reduce fear of loss and reduce cross-device friction before it attempts to centralize more of the product.

That means the cloud should initially help with:

- identity
- device registration
- device rendezvous and signalling
- backup and restore
- visibility through a web surface

The cloud should not initially replace the local authority node as the source of truth for all cognition state.

## Why This Shape Fits The Current Codebase

The current repository still assumes:

- a daemon-centric runtime (`veld`)
- local SQLite persistence
- local artifacts on disk
- local-first privacy and inspectability
- coarse operator and worker token auth rather than full user/session auth

That makes a "cloud assist" product materially easier than a deep move to a shared hosted runtime.

There is already partial groundwork for:

- device linking and scoped pairing
- cluster and sync bootstrap data
- websocket event fanout
- backup manifest and restore plan schemas

Those surfaces can support a hosted assist layer without immediately forcing a full data-model rewrite.

## Core V1 Capabilities

### 1. Authenticated Client Signalling

Users should be able to sign in to a hosted control plane and see their registered devices and active sessions.

The signalling layer should support:

- user identity
- device identity
- session registration
- presence and liveness
- rendezvous between clients and the user's Vel runtime
- user-scoped websocket or equivalent realtime channels

The signalling layer should not grant ambient access to all runtime capabilities. It should carry only enough authority to route, notify, and establish explicitly scoped client-to-runtime sessions.

### 2. Paid Cloud Backup

Users should be able to opt into encrypted backups of:

- the primary database
- artifact storage
- selected configuration and manifest material needed for recovery

The backup feature should include:

- backup target configuration
- scheduled or operator-triggered backup runs
- integrity verification
- retention policy
- downloadable export
- guided restore path

Restore must be treated as a first-class operator flow, not an afterthought behind backup marketing.

### 3. Hosted Web Dashboard

The hosted dashboard should initially focus on:

- sign-in
- device list and trust state
- runtime reachability and health
- backup status and restore/export entrypoints
- basic visibility into recent runs, linking state, and sync health

The web dashboard should not require that all Vel state live in a central cloud database in order to be useful.

## Recommended System Shape

Hosted V1 should use a split model:

- control plane
- user runtime

### Control Plane

The control plane owns:

- user auth
- billing and entitlement state
- device registration metadata
- signalling/session coordination
- backup orchestration metadata
- dashboard APIs

The control plane may use Supabase or a similar system for:

- authentication
- control-plane Postgres metadata
- session records
- lightweight admin workflows

The control plane should not become the first storage location for all durable Vel cognition state in V1.

### User Runtime

The user runtime remains the authority over the user's actual Vel state.

Depending on deployment mode, that runtime may be:

- local on the user's machine
- a user-dedicated hosted container
- a user-dedicated minivm in a future hardened tier

For V1, the simplest acceptable shape is that the runtime stays local and the hosted system adds authenticated signalling, dashboard visibility, and backup workflows.

## Deployment Modes

### Mode A: Local Runtime + Hosted Assist

This is the preferred first version.

Characteristics:

- cognition authority stays local
- cloud stores control-plane metadata and encrypted backups
- web dashboard connects through a user-scoped control surface
- signalling helps clients discover and reach the correct runtime

Benefits:

- lowest architecture rewrite cost
- preserves local-first trust posture
- easiest transition from current codebase

Limits:

- remote access depends on runtime reachability strategy
- offline local machine means limited live dashboard capability

### Mode B: User-Dedicated Hosted Runtime

This should be a later extension, not the initial assumption.

Characteristics:

- each user gets an isolated runtime with persistent storage
- control plane routes traffic to the user runtime
- backup and dashboard become simpler because reachability is guaranteed

Benefits:

- stronger always-on behavior
- simpler remote web access

Limits:

- materially more platform work
- higher operating cost
- more privacy and trust burden

## Security And Trust Requirements

Hosted V1 must preserve Vel's existing security direction:

- explicit capability boundaries
- no ambient privilege expansion
- fail closed on unknown requests
- secret use only at the narrowest boundary

Additional hosted requirements:

- per-user session auth, not only shared operator tokens
- device registration and revocation
- encrypted backups with scoped key handling
- audit trail for backup, restore, login, device link, and session events
- strict separation between control-plane metadata and user cognition payloads where possible

## Data Ownership Rules

Hosted V1 should preserve these operator expectations:

- the user can still understand where their primary state lives
- backup contents are inspectable and exportable
- hosted metadata is narrower than full cognition state when possible
- restore paths are documented and testable

If hosted dashboard features require mirrored data, that mirroring must be explicit, documented, and bounded.

## Out Of Scope For V1

The following should stay out of scope:

- shared team workspaces
- fine-grained RBAC
- organization tenancy
- broad collaborative editing
- central shared storage for all user cognition state
- cloud-only requirement for core capture and recall
- full execution-agent hosting platform

## Primary Workstreams

### Workstream 1: Identity And Device Model

Add:

- hosted user accounts
- device registration records
- session lifecycle records
- trust and revocation flows

Do not yet add:

- complex org or team membership models

### Workstream 2: Signalling Service

Add:

- authenticated websocket or equivalent realtime signalling
- presence and reachability records
- runtime rendezvous metadata
- user-scoped channel authorization

Reuse where possible:

- existing websocket event concepts
- existing linking and cluster bootstrap concepts

### Workstream 3: Backup And Restore

Add:

- execution path behind existing backup foundation schema
- storage adapters for cloud object storage
- integrity verification and restore planning
- dashboard and CLI entrypoints

Reuse where possible:

- existing `storage_targets`, `backup_manifests`, `artifact_copies`, and `restore_plans` schema

### Workstream 4: Hosted Dashboard

Add:

- authenticated dashboard shell
- device and session visibility
- backup status and restore flow
- health, linking, and sync status views

Prefer:

- read-mostly dashboards first
- minimal mutation surfaces

### Workstream 5: Billing And Entitlements

Add:

- subscription state
- entitlement checks for backup volume, retention, or hosted assist features
- operator-visible plan status

Prefer:

- simple plan gating over deep usage metering in the first release

## Suggested Implementation Sequence

1. Define hosted identity, device, and session contracts.
2. Build the signalling control plane and authenticated channel model.
3. Turn the existing backup schema foundation into a real backup and restore service.
4. Ship the hosted dashboard around those two capabilities.
5. Add billing and entitlement enforcement around the hosted features.

This sequencing keeps the first release focused on continuity and trust instead of prematurely centralizing execution.

## Open Decisions

The following decisions should be made before implementation starts:

- whether Hosted V1 supports only local runtimes or also user-dedicated hosted runtimes
- whether Supabase is used only for control-plane auth/data or also for some dashboard-facing cache state
- what exact data is mirrored into hosted control-plane metadata versus fetched live from user runtime
- whether backup encryption keys are user-managed, server-managed, or hybrid
- what restore experience is required for "good enough trust" at launch

## Acceptance Bar For A Real V1

Hosted V1 should not be considered complete unless all of the following are true:

- a user can authenticate and see their registered devices
- clients can establish authenticated signalling without broad ambient authority
- a user can create and verify an encrypted backup
- a user can restore from backup through a documented and tested path
- the hosted dashboard exposes enough health, link, and backup state to be operationally useful
- the system still preserves a truthful local-first explanation of where authority and durable state live
