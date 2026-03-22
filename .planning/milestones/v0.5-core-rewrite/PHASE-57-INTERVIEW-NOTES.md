# Phase 57 Interview Notes

Captured on `2026-03-22` from operator feedback while deepening milestone `0.5` Phase 57 planning.

## Purpose

Record the operator's architecture calls for Phase 57 before converting them into plan documents and contract specs.

## Call Set

The general principle: keep the canonical core small, typed, and durable; keep runtime, sync, and audit adjacent but not smeared into the object substrate; keep provider semantics mappable rather than sovereign.

Otherwise `0.5` turns into a goo pit with better nouns.

## Chunk 1: Object Kernel

### 1. Minimum first-class object set for `0.5`

Minimum durable canonical set:

- `Task`
- `Project`
- `Event`
- `Calendar`
- `Thread`
- `Message`
- `Person`
- `Tag`
- `IntegrationAccount`
- `Workflow`
- `Skill`
- `Tool`
- `Module`
- `Config`
- `SyncLink`

Do **not** make `Availability` first-class yet.

Do **not** make run/audit/grant records first-class canonical objects in the same substrate.

### 2. Calendar and Availability

- `Calendar`: yes, first-class.
- `Availability`: no, not first-class in `0.5`.

Availability should be a governed read model over:

- `Calendar`
- `Event`
- policy/config
- optional cache/materialization tables

Availability is not an authored durable object in `0.5`; it is a computed interpretation of time occupancy, policy, calendars included/excluded, transparency, travel buffers, and later possible work-hours logic.

### 3. `IntegrationAccount`

`IntegrationAccount` should be a first-class canonical object.

Reasons:

- ownership and policy need a stable principal/boundary
- audit needs account-scoped explanation
- multi-account per provider is already in the proving flows
- sync and grants become clearer when the account exists in the registry

Secrets/tokens must stay out of the canonical payload.

Recommended split:

- `IntegrationAccount` = canonical metadata/policy/identity envelope
- credentials = external secure store referenced by the account object

### 4. `SyncLink` / source mapping

Use a dedicated `SyncLink` record or equivalent canonical linkage entity.

It should minimally capture:

- canonical object id
- provider
- provider object type
- provider account id
- remote id
- sync state
- ownership map / field map snapshot ref
- timestamps / etag / version hints
- tombstone / deletion reconciliation status

Do not bury this in per-object JSON blobs.

### 5. Universal object envelope

Use a narrow universal durable object envelope only for durable canonical objects.

Keep runtime/audit/grants/runs outside it.

Recommended split:

Durable canonical envelope:

- `id`
- `object_type`
- `version`
- `created_at`
- `updated_at`
- `archived_at?`
- `workspace_id?`
- `source_summary?`
- `extension_pack_refs?`
- `governance_ref?`

Separate runtime/control records for:

- run records
- audit entries
- grants
- approvals
- sync jobs
- projections/cache materializations

### 6. ID shape

Use globally prefixed IDs by object family:

- `task_*`
- `event_*`
- `calendar_*`
- `thread_*`
- etc.

Keep type separately too, but typed IDs are preferred because debugging, storage, logs, and cross-substrate references become more legible.

### 7. Relations

Relations should be strongly typed and directional from day one.

Examples:

- `task scheduled_by event`
- `event belongs_to calendar`
- `task tagged_with tag`
- `message in_thread thread`
- `message authored_by person`
- `task backed_by sync_link`
- `event synced_via integration_account`
- `workflow invokes skill`
- `module exposes tool`

Minimum relation metadata:

- relation type
- direction
- source of assertion
- confidence / provenance
- active/inactive lifecycle

### 8. Schema evolution

Keep schemas tighter in `0.5`, with limited extension packs.

Recommendation:

- stable core typed schemas for canonical objects
- one controlled extension mechanism:
  - namespaced field packs
  - provider facets
  - module facets
- no arbitrary extension free-for-all

## Chunk 2: Task and Calendar Semantics

### 9. Canonical `Task.status`

Use a small Vel status set plus provider mapping.

Suggested canonical statuses:

- `active`
- `completed`
- `cancelled`
- `archived`

`snoozed` should only become status if it has cross-provider meaning; otherwise keep it as a scheduling facet.

### 10. Todoist sections

Do not make Todoist sections first-class canonical objects in `0.5`.

Treat them as:

- project-scoped structured fields
- optional relation target later
- provider facet preserved on import

### 11. Todoist labels

Map Todoist labels to canonical `Tag` objects, while preserving a provider-label facet for exact remote identity/color/source metadata.

### 12. Todoist comments

Import Todoist comments as task-attached comment records in `0.5`, not as first-class `Message` objects by default.

Keep a migration path so they could later be projected into `Message` if needed.

### 13. Google Calendar attendees

Use `Person` links plus participation metadata, not plain attendee blobs only.

At minimum:

- link attendee email/name to `Person` when resolvable
- attach participation payload:
  - response status
  - organizer flag
  - optional/self/resource
  - source provenance

### 14. Recurring events

Use:

- recurring series canonical object
- derived/materialized occurrences as read model
- exception/override occurrences as explicit records when needed

Do not import every occurrence as a fully durable first-class object by default.

### 15. Locations

Use a simple canonical location payload on `Event`.

Include:

- display label
- address text
- coordinates optional
- provider metadata optional

Defer a first-class `Place` object.

### 16. Availability

Availability should be computed on demand and optionally cached/materialized.

It is not the primary stored truth.

## Chunk 3: Action Membrane and Policy

### 17. Action namespace

Ground the membrane in object-generic verbs:

- `object.get`
- `object.query`
- `object.create`
- `object.update`
- `object.delete`
- `object.link`
- `object.explain`

Then provide domain-specialized aliases:

- `task.complete`
- `event.schedule`
- `calendar.sync`
- `workflow.run`

### 18. Field ownership

Use a hybrid model:

- static default declarations in schemas/manifests
- dynamic overlays for provider/account configuration, per-object sync state, and special local override policies

### 19. Policy bundles

Use one canonical policy model plus named config presets/profiles.

Do not build a large compositional policy-pack ecosystem in `0.5`.

Allow override layers by:

- workspace
- module
- account
- object
- action

### 20. Confirmation modes

Lock these now:

- `auto`
- `ask`
- `ask_if_destructive`
- `ask_if_cross_source`
- `ask_if_external_write`
- `deny`

### 21. Read-only mode granularity

Support all three:

- integration account
- module
- workspace

Precedence should be restrictive.

### 22. Audit capture

Use before/after capture whenever possible for non-sensitive fields.

Use changed summaries plus references for sensitive payloads.

Audit should capture:

- actor
- intent
- policy used
- object ids
- action name
- before/after diff where safe
- redacted/sensitive ref when not safe
- downstream provider operation refs
- success/failure

### 23. `policy.explain` and `object.explain`

Both should be Phase 57 contract requirements.

Minimum viable explanations:

- why action is allowed/denied/ask
- which policy/profile/rule applied
- which fields are source-owned / Vel-owned / shared
- why a relation or value is present

## Chunk 4: Modules, Skills, and Workflows

### 24. Canonical objects vs registry assets

Preferred split:

- `Module` = canonical object
- `Workflow` = canonical object
- `Skill` = registry asset / manifest-backed descriptor
- `Tool` = registry asset / manifest-backed descriptor

Reasoning:

- workflows are often user-authored or user-governed durable entities
- modules define governance/capabilities and deserve first-class status
- skills/tools are executable affordances rather than durable business objects in `0.5`

### 25. Core module organization

Organize core modules by concern, not as one god-module.

Suggested layout:

- `core/objects`
- `core/tasks`
- `core/calendar`
- `core/threads`
- `core/workflows`
- `core/policy`
- `integration/todoist`
- `integration/gcal`

### 26. Module loading

Support both behind one registry contract, but use crate-compiled internal modules as the primary `0.5` implementation.

Contract should allow:

- internal compiled modules
- filesystem/package modules later

### 27. Workflow step taxonomy

Keep `0.5` step taxonomy limited to:

- `action`
- `skill`
- `approval`
- `sync`

Optional additional step:

- `condition`

If staying stricter in the first cut, mention future `condition`, `loop`, `hook`, `transform`, and `wait` explicitly as deferred.

### 28. Workflow capability grants

Workflows should have their own explicit grant envelope derived from:

- caller
- workflow definition
- module policy
- workspace/account policy

### 29. Skills calling tools

Skills should not call raw tools directly.

They must go through the action membrane or explicit mediated runtime calls that remain auditable and policy-checked.

### 30. Hooks

Mention hooks and define placeholder contracts, but defer executable hooks in `0.5`.

### 31. Run records

Keep run records in separate runtime/audit storage outside the canonical object substrate.

They may reference canonical objects, but should not be canonical objects themselves.

## Chunk 5: Adapter Boundary Contracts

### 32. Todoist field ownership

Recommended split:

Source-owned:

- remote id
- provider project/section membership ids
- provider-created timestamps if exposed
- provider completion state/timestamp
- provider comment ids/comments raw metadata
- provider order/index metadata
- provider assignee/collaborator fields if used
- recurring rule/provider recurrence specifics
- provider labels exact identity

Shared / mapped:

- title/content
- description
- due date/time
- priority
- completion intent/status mapping
- project relation
- tags derived from labels
- parent/child structure if supported

Vel-only:

- canonical relations to events/threads/workflows
- local tags not mapped upstream
- ownership/conflict annotations
- derived scheduling metadata
- policy/config metadata
- local explanation/cache/provenance summaries
- local enriched semantics like must-do, stretch, inferred effort, confidence

If a shared field is source-authoritative for a given account mode, it should be made explicit in ownership metadata.

### 33. Google Calendar field ownership

Recommended split:

Source-owned:

- remote event id / calendar id
- provider recurrence rule details
- provider organizer metadata
- provider conference details raw metadata
- provider attendee response states as last synced truth
- provider original start/end for recurrence exceptions
- provider updated/etag/version fields

Shared:

- title
- description
- start/end
- location
- transparency/free-busy
- reminders
- attendees list, where write-enabled
- recurrence edits, where policy allows

Vel-only:

- local links to tasks/threads/projects/workflows
- local interpretation tags
- availability projections/caches
- ownership/explain/conflict metadata
- enriched semantic fields
- local suppression or relevance flags for UI/runtime planning

### 34. Initial sync window

- Todoist: full active backlog import, plus relevant completed window if supported/configured.
- Google Calendar: bounded default window, such as past X months / future Y months, with explicit expansion.

### 35. Upstream deletes

Create local tombstones immediately on sync, with reconciliation state.

Suggested states:

- `detected_deleted_upstream`
- `pending_reconcile`
- `reconciled`
- `restored`

### 36. Cross-source conflicts

Avoid direct competition where possible by separating owned fields.

Guiding split:

- Todoist owns task/task-provider data
- Google Calendar owns event/event-provider data
- Vel owns cross-object relations between them

### 37. Recurring Google edits

The contract should support:

- this occurrence
- entire series

Defer `this and following` unless the runtime/adapter can support it correctly and explainably.

### 38. Account-scoped policy presets

Include at least:

- `read_only`
- `conservative_sync`
- `manual_confirm`
- `local_enrichment_only` (optional but recommended)

These should be profiles over the core policy model, not a separate subsystem.

## Chunk 6: Cutover and Verification

### 39. Compatibility DTO layer

Preserve a thin compatibility DTO layer during the milestone.

Not forever, but yes during cutover to reduce blast radius and allow internals to change materially while routes/callers are repaired.

Treat it as temporary scaffolding with explicit removal criteria.

### 40. Migration artifact format

Phase 57 should define the migration artifact format.

Define at least:

- artifact structure
- versioning
- source snapshot refs
- transformation result schema
- validation/error reporting
- replay/idempotence expectations

### 41. Must-pass proving flows

Required proving flows:

- import full Todoist backlog from one account
- import bounded Google Calendar history/future window from one account
- link multiple accounts per provider
- explain ownership/conflict on one synced task
- explain ownership/conflict on one recurring event
- compute and explain availability from imported calendar data
- dry-run one write per provider
- enable write mode and perform one conservative outward mutation per provider
- show audit trail with actor, policy path, diff summary, and upstream operation refs
- prove `policy.explain` for allow, ask, and deny cases
- prove tombstone behavior for one upstream deletion
- prove cross-object linkage: task linked to event, thread linked to task/event
- prove read-only enforcement at account, module, and workspace levels
- prove recurring event edit scope behavior: this occurrence vs entire series
- prove migration artifact import on representative old data

## Condensed Call Sheet

- Make `Calendar` first-class, `Availability` a governed read model.
- Make `IntegrationAccount` and `SyncLink` first-class.
- Use a typed durable object envelope only for canonical objects.
- Keep runtime/audit/grants/runs outside the canonical substrate.
- Use prefixed typed IDs.
- Make relations strongly typed and directional now.
- Keep schemas tight with narrow extension packs.
- Use a small Vel status model plus provider mappings.
- Keep Todoist sections non-first-class for now.
- Map Todoist labels to `Tag` plus provider facet.
- Keep Todoist comments task-attached records, not canonical `Message` by default.
- Model Google Calendar attendees as `Person` links with participation metadata.
- Model recurrence as series plus derived occurrences.
- Use object-generic membrane actions first, typed aliases second.
- Use hybrid ownership: static defaults plus dynamic overlays.
- Require `policy.explain` and `object.explain` in contract.
- Canonicalize `Module` and `Workflow`; keep `Skill` / `Tool` registry-backed for now.
- Organize core modules by concern.
- Let workflows have their own grant envelope.
- Do not let skills call raw tools without mediation.
- Mention hooks, defer execution.
- Use bounded Google Calendar sync and broader Todoist import.
- Create tombstones immediately for upstream deletes.
- Keep cross-source conflicts mostly out of shared scalar fields by making Vel own the relations.
- Preserve a temporary compatibility DTO layer.
- Define a migration artifact format now.
- Add proving flows for availability, tombstones, recurring scope, read-only enforcement, and migration.

## Follow-up Clarifications

Captured on `2026-03-22` after review of the initial Phase 57 interview notes.

### `Skill` and `Tool` canonical status

The contradiction resolves as follows:

- `Skill` and `Tool` should exist in the canonical registry model in `0.5`.
- They should not behave like user-authored durable content objects by default.
- They should be treated as canonical system/registry objects with:
  - canonical identity
  - canonical contract presence
  - manifest-backed definitions and defaults
  - optional persisted overlays only where needed

Recommended winning rule for `0.5`:

- `Skill` and `Tool` are canonical registry objects.
- Their implementation and defaults are manifest-backed.
- Durable persistence is only required for mutable overlays, version pinning, policy attachment, enablement, provenance, or installation state.

Why this seam is preferred:

- skills/tools participate in governance
- workflows need stable references to invoked capabilities
- policy attachment and explainability become cleaner
- versioning, deprecation, replacement, and module ownership become explicit

Suggested `Skill` / `Tool` contract fields:

- canonical id
- type
- namespace
- owning module
- manifest source ref
- version
- capability declaration
- input/output contract ref
- risk class
- policy defaults
- enablement state
- deprecation / replacement refs
- human-readable label / description

### Canonical object classes for `0.5`

#### Canonical content objects

- `Task`
- `Project`
- `Event`
- `Calendar`
- `Thread`
- `Message`
- `Person`
- `Tag`
- `IntegrationAccount`
- `SyncLink`
- `Workflow`
- `Config`

#### Canonical registry objects

- `Module`
- `Skill`
- `Tool`

#### Computed / read-model objects

- `Availability`

#### Runtime / control records

- `RunRecord`
- `AuditEntry`
- `Grant`
- `Approval`
- `SyncJob`
- `WriteIntent`
- `ProjectionCache`

### `Message` and Todoist comments

- `Message` remains a first-class canonical object in `0.5` because `Thread` needs it.
- Todoist comments should not become `Message` by default.

Recommended boundary:

- `Message` = canonical threaded communication object
- `AttachedComment` / provider comment record = provider-native annotation attached to another object

Later projection from attached comment to `Message` can remain possible, but it is not the default `0.5` contract.

### Workflow provenance and mutability

Both built-in and user-authored workflows should be canonical objects.

Recommended differentiators:

- `workflow_origin = system | seeded | user | imported`
- `mutability = immutable | forkable | editable`
- `owner_scope = core_module | integration_module | workspace | user`

Built-in workflows should be seeded from modules into the same canonical workflow substrate rather than modeled as a separate species.

### Google Calendar bounded sync default

Lock the default contract to:

- past 90 days
- future 365 days

Also make it:

- account-configurable
- policy-visible
- explainable in sync metadata

Support explicit expansion by:

- date range
- calendar
- event / series hydration on demand

### Person resolution rule

Initial attendee/person identity resolution ladder:

1. normalized email exact match
2. known linked provider identity mapping
3. existing provider-scoped participant stub match
4. create new provider-scoped stub

Important constraint:

- do not merge by display name alone in `0.5`

Suggested maturity states:

- resolved person
- stub person

### Shared-field local edits and outward writes

Even when write mode is enabled, local edits to shared provider-backed fields should become pending write intents first, then mediated outward.

Recommended flow:

1. local canonical mutation recorded
2. ownership/policy evaluated
3. `WriteIntent` runtime record created
4. policy decides whether to auto-execute, ask, dry-run, or deny
5. outward provider operation is executed or blocked through the explicit membrane

### Tombstone visibility

- tombstones should be hidden from default `object.query`
- tombstones should remain addressable by ID
- tombstones should remain visible in audit and reconciliation views

Suggested default query posture:

- `include_deleted = false`
- `include_archived = false`

### `SyncLink` cardinality

Allow:

- many historical `SyncLink` records over time
- multiple links from one canonical object across providers/accounts
- multiple historical links even for the same provider/account

Enforce invariants:

- at most one active `SyncLink` per `(provider, account, remote object)`
- at most one active authoritative mapping from a canonical object to a given remote target

Suggested link states:

- `active`
- `superseded`
- `broken`
- `deleted_upstream`
- `reconciled`
- `conflicted`

### Compatibility DTO removal criteria

Phase 57 should define explicit removal criteria now.

Suggested criteria:

- all active routes are backed by canonical-core queries/actions
- all write paths go through the action membrane
- all provider adapters emit/consume canonical mappings rather than legacy direct DTOs
- audit and `policy.explain` cover all milestone proving flows
- migration artifacts can produce canonical objects without legacy DTO dependency
- no UI/runtime path depends on legacy shape except explicitly deprecated endpoints

### Canonical module namespace

Use:

- `integration/google-calendar` as canonical namespace

Allow:

- `integration/gcal` as shorthand or alias if useful internally

Canonical names should optimize for clarity and durability; aliases can optimize for ergonomics.

## Final Clarifications

Captured on `2026-03-22` after final follow-up review of Phase 57 planning seams.

### Workflow storage model

All workflows should live in the same canonical object storage.

Recommended rule:

- built-in workflows are seeded into canonical storage on bootstrap/import
- user-authored workflows are created there directly
- forked/edited built-ins remain workflows in the same substrate, with provenance fields

Do **not** use a split model where some workflows remain manifest-only until edited.

Recommended workflow fields:

- `workflow_id`
- `origin = system | seeded | user | imported`
- `source_module_id?`
- `manifest_ref?`
- `mutability = immutable | forkable | editable`
- `forked_from_workflow_id?`
- `version`
- `definition`
- `policy_ref?`
- `status = active | deprecated | disabled`

Recommended behavior:

1. module ships workflow manifest
2. bootstrap loader materializes canonical workflow object
3. object becomes the stable referent everywhere
4. module updates reconcile by version/provenance rules
5. user fork creates new canonical workflow object with `forked_from_workflow_id`

### Registry IDs vs content-object IDs

Registry objects should use stable human-readable IDs.

Content objects should use typed opaque IDs.

Recommended split:

Canonical content objects:

- `task_*`
- `event_*`
- `workflow_*`
- `thread_*`
- etc.

Canonical registry objects:

- `module.core.calendar`
- `module.integration.todoist`
- `module.integration.google-calendar`
- `skill.core.daily-brief`
- `tool.object.get`
- `tool.todoist.sync`
- `tool.google-calendar.sync`

Do not rely on parsing IDs as the sole metadata source. Registry objects should also carry explicit fields such as:

- `namespace`
- `slug`
- `display_name`

### `SyncLink` resolution vs object convenience fields

Canonical linkage truth should resolve outward through:

- `SyncLink`
- typed relations
- linkage queries

Objects may carry a lightweight ergonomic projection such as `source_summary` or `sync_summary`.

Recommended summary contents:

- linked providers
- count of active links
- primary source indicator
- last sync timestamp
- tombstone/conflict flags

The summary is a convenience projection, not the source of truth.

### Todoist comments contract name

Define the term `AttachedCommentRecord` now in the Phase 57 contract.

Recommended meaning:

- attached, not standalone
- record, not full object
- structurally defined, but lower ontological rank than canonical content objects

Use it for Todoist in `0.5`, while keeping it non-first-class and attached.

### `WriteIntent` runtime record

Use one provider-agnostic `WriteIntent` runtime record type.

Do not split into multiple runtime families in `0.5`.

Recommended fields:

- `write_intent_id`
- `action_name`
- `target_object_refs`
- `provider?`
- `integration_account_id?`
- `requested_change`
- `ownership_evaluation`
- `policy_evaluation`
- `confirmation_mode`
- `status = proposed | approved | executing | succeeded | failed | cancelled`
- `downstream_operation_refs`
- `created_by_actor`
- `created_at`
- `executed_at?`
- `failure_reason?`

Differentiate concerns by metadata or subkind fields, not by inventing separate runtime ontologies up front.

### Canonical namespace rule for Todoist and Google Calendar

Define the namespace rule now for both providers.

Preferred canonical registry IDs:

- `module.integration.todoist`
- `module.integration.google-calendar`

Optional friendly/path aliases:

- `integration/todoist`
- `integration/google-calendar`

Registry IDs should optimize for semantic clarity and stability, not filesystem-path resemblance.

### Entity classification field

Add an explicit classification field or enum in the spec for every entity type:

- `content`
- `registry`
- `read_model`
- `runtime`

This should be locked in the Phase 57 contract to avoid later drift where “canonical” collapses into one undifferentiated bucket again.

## Final Lock Recommendations

Captured on `2026-03-22` as the last follow-up before drafting Phase 57 plan docs.

### Seeded workflow mutability

Default rule:

- seeded workflows require fork-before-modify

Recommended mutability model:

- `immutable`
- `forkable`
- `editable`

Recommended companion provenance/governance fields:

- `origin = seeded`
- `owner_scope = core_module | integration_module | workspace`
- `local_override_policy = none | editable_in_place | fork_required`

Practical contract:

- most seeded built-ins should be `immutable` or `forkable`
- only a narrow subset should be `editable` in place
- editable seeded workflows should be clearly local/operator-owned rather than core product logic

If editable seeded workflows are allowed, they should also carry explicit reconciliation/version-drift metadata such as:

- `seed_version`
- `local_modified_at`
- `local_modified_by`
- `upstream_update_available`
- `reconciliation_state`

### `source_summary`

Use `source_summary` as an optional shared field on the universal durable envelope.

Recommended meaning:

- derived convenience summary for explain/debug/query ergonomics
- available cross-type where integration/provenance context exists
- not the source of truth for linkage state

Canonical linkage truth remains external through:

- `SyncLink`
- typed relations
- provider/account linkage state

Recommended `source_summary` contents stay compact:

- linked provider names
- active link count
- primary source/provider if relevant
- last sync timestamp
- overall sync health/status
- tombstone/conflict flags
- optional ownership-mode summary

Do not turn `source_summary` into a denormalized copy of full linkage maps or provider payloads.

## Backend-Safe Additions

Captured on `2026-03-22` to make the `0.5` contract safe for a multiplatform Rust backend rather than only ontologically clean.

### Core backend gap statement

The ontology is solid, but ontology alone does not compile.

To make Phase 57 backend-safe, the contract also needs:

- crate/module boundary mapping
- trait-based storage/runtime abstractions
- serialization/versioning rules
- typed ID newtypes
- feature gating
- error taxonomy
- optimistic concurrency/version semantics
- bootstrap/seeding reconciliation rules
- secret-provider abstraction
- scheduler/job interfaces
- projection/read-model separation
- target capability matrix
- testability contracts

### Rust/backend implementation constraints to lock

#### 1. Strict crate or module boundary map

The spec should explicitly map conceptual classes to backend layers such as:

- `vel_core_types`
- `vel_core_registry`
- `vel_core_objects`
- `vel_core_relations`
- `vel_core_policy`
- `vel_core_runtime`
- `vel_core_sync`
- `vel_adapters_todoist`
- `vel_adapters_google_calendar`
- `vel_storage`
- `vel_platform`

If this boundary is not defined now, the substrate-vs-runtime split will be violated by convenience imports and god-crates.

#### 2. Platform-neutral storage trait layer

All persistence should be defined through storage traits rather than implicit backend assumptions.

Required storage roles:

- `ObjectStore`
- `RegistryStore`
- `RelationStore`
- `SyncLinkStore`
- `RuntimeStore`
- `AuditStore`
- `ProjectionStore`
- `TransactionManager`

Requirements:

- local embedded backends first
- works on desktop/server/mobile-capable Rust targets
- SQLite/Postgres/etc only behind traits
- no canonical domain logic may depend on SQL-dialect specifics

#### 3. Canonical serde and wire-format contract

Lock:

- deterministic serialization for canonical objects
- versioned wire forms for runtime/control records
- stable string forms for typed IDs and registry IDs
- consistent timestamp/enum/optional-field encoding rules

Recommended implementation posture:

- `serde` is canonical implementation
- JSON is baseline interchange/debug format
- binary formats may come later, but are not contract truth

Also define:

- unknown-field handling
- forward/backward compatibility rules
- null vs omitted semantics
- enum tagging strategy
- schema-version field location

#### 4. `Send + Sync + 'static` discipline where needed

Lock that service traits used by runtime orchestration should be `Send + Sync` where applicable, and public core contracts should use owned data rather than platform-tied references/handles.

#### 5. Target capability matrix

Define required capabilities across all targets, optional capabilities by target, and capabilities that core must never assume.

Required across all targets:

- stable clock access
- UUID/random generation
- canonical serialization/deserialization
- local persistence
- async task execution or equivalent

Optional by target:

- background sync
- network reachability monitoring
- secure OS keychain
- push notifications
- filesystem watch
- multi-threaded executor

Unsupported in core assumptions:

- direct platform GUI handles
- platform-specific account APIs in the domain layer
- platform-specific secret APIs in core types

#### 6. Feature-gating strategy

Add high-level feature segmentation such as:

- `todoist`
- `google-calendar`
- `sqlite`
- `postgres`
- `wasm`
- `native-secrets`
- `background-sync`
- `full-audit`
- `devtools`

Rules:

- core object model compiles without provider adapters
- registry model compiles without networked integrations
- read models degrade gracefully when features are disabled
- provider-specific code must not leak into core enums in ways that break stripped builds

#### 7. Domain error model

Define a canonical error taxonomy such as:

- validation error
- policy denied
- confirmation required
- conflict detected
- not found
- stale version
- sync failure
- provider auth failure
- provider rate limited
- storage failure
- unsupported capability
- migration failure

Also define which layers may emit which categories.

#### 8. Optimistic concurrency / version semantics

Lock:

- canonical objects carry revision/version
- updates use compare-and-swap or equivalent stale-write detection
- SyncLink and WriteIntent behavior under stale state
- shared-field conflict detection rules
- projections are rebuildable and non-authoritative

#### 9. Migration framework contract

Add backend abstraction for:

- storage schema migration
- canonical object schema migration
- registry/bootstrap seeding migration behavior
- idempotent bootstrap rules
- old-to-new artifact import API shape
- migration dry-run / validation mode

#### 10. Deterministic bootstrap and seeding rules

Lock that bootstrap is deterministic and idempotent.

Also lock:

- registry seeding precedes workflow seeding
- built-in workflow updates reconcile by stable identity + version
- local forks are never overwritten
- editable seeded objects require drift markers instead of silent overwrite

#### 11. Manifest loading interface

Define conceptual roles such as:

- `ManifestSource`
- `RegistryLoader`
- `RegistryReconciler`

These must cover:

- loading manifests from compiled assets or filesystem/package sources
- validating manifests into canonical registry entities
- reconciling manifest-defined objects with persisted registry state
- version pinning and deprecation handling

#### 12. Storage-neutral query abstraction

Define query as a domain abstraction, not “whatever SQL got written first.”

Include:

- filter AST or query-struct model
- pagination contract
- sort semantics
- relation traversal semantics
- tombstone/archive visibility flags
- projection/read-model query rules

#### 13. Background job / scheduler abstraction

Define platform-neutral runtime roles such as:

- `JobScheduler`
- `RuntimeExecutor`
- `IntentProcessor`

These must cover:

- enqueueing sync jobs
- processing write intents
- workflow run orchestration
- retry/backoff
- cancellation
- idempotency keys
- crash recovery / resume semantics

#### 14. Secret/account boundary contract

Lock that:

- `IntegrationAccount` never stores raw secrets
- credentials come through a `SecretStore` / `CredentialProvider`
- core domain logic sees auth/token availability state, not storage mechanism details

#### 15. `no_std` / wasm stance

Even if provisional, the spec should state whether core targets:

- native-only
- native plus wasm-compatible domain core
- native plus mobile embeddings

Recommended posture:

- core types and policy logic avoid unnecessary OS coupling
- storage/network adapters are target-specific
- runtime executor varies by target behind traits

#### 16. Testability contracts

Require:

- in-memory storage-trait implementations
- deterministic fixture seeding
- fake clock interface
- fake provider adapters
- replayable migration tests
- policy-explanation golden tests
- sync reconciliation scenario tests

#### 17. Identity newtypes

Strongly recommend typed Rust newtypes for IDs rather than raw strings everywhere:

- `TaskId`
- `WorkflowId`
- `ModuleId`
- `SkillId`
- `ToolId`
- `IntegrationAccountId`
- `SyncLinkId`
- `WriteIntentId`

Even if serialized as strings, the Rust domain model should use typed wrappers.

#### 18. Canonical model vs projection model separation

Lock that:

- canonical object crates define truth
- projection/read-model crates define derivations
- cached UI/debug summaries are non-authoritative
- rebuild procedures exist

### What to add to the Phase 57 plan packet

Add a Rust-backend-specific contract layer covering:

- core domain is storage-agnostic
- canonical types are serde-versioned
- IDs are strong newtypes
- registry/content/runtime/read-model classes map to distinct crates/modules
- provider adapters are feature-gated
- secrets are externalized behind traits
- bootstrap is deterministic and idempotent

Also add explicit appendices or companion docs for:

- required backend traits
- cross-platform assumptions
- concurrency and versioning

This is the difference between a clean model and a clean system.
