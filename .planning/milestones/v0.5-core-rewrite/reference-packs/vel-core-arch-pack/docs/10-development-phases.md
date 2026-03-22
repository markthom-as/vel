# 10. Development Phases

## 10.1 Strategy

Build from semantic spine outward.
Do not start with broad integration chaos and hope a model prompt will eventually parent the room.

The recommended order is:

1. canonical objects
2. core tools/API
3. policy/permissions/audit
4. templates
5. workflow/skill runtime
6. module system
7. first adapter (Todoist)
8. richer cross-object intelligence

## 10.2 MVP definition

The MVP is **not** “all top-level object types fully complete.”
The MVP is:

- enough core object infrastructure to stabilize semantics
- enough tools/API to make interaction lawful
- enough policy to make integration safe
- one real adapter proving the abstraction
- basic workflows/skills operating against typed objects

## 10.3 Phase 0 — Canonical object kernel MVP

### Goals
Stand up the semantic substrate.

### Deliverables
- base object envelope
- object registry
- initial types:
  - `Task`
  - `Project`
  - `Event`
  - `Tag`
  - `Template`
  - `Tool`
  - `Workflow`
  - `Config`
  - minimal `Thread`
- trait system MVP:
  - `Taggable`
  - `Schedulable`
  - `Completable`
  - `Relational`
  - `Syncable`
  - `Auditable`
- relation model
- schema validation
- object inspect/explain basics

### Success criteria
- can define and store typed objects
- can link objects by typed relation
- can validate field groups and traits
- can inspect an object and understand its state

## 10.4 Phase 1 — Core Tools API MVP

### Goals
Create the lawful membrane.

### Deliverables
- typed action registry
- initial actions:
  - `object.get`
  - `object.query`
  - `object.explain`
  - `task.create`
  - `task.update`
  - `task.complete`
  - `project.create`
  - `project.update`
  - `event.create`
  - `template.apply`
  - `workflow.run`
  - `source.sync` stub
- CLI parity for initial actions
- input/output schemas
- dry-run support for mutating tools

### Success criteria
- all state changes happen through named actions
- CLI and internal callers use same action semantics

## 10.5 Phase 2 — Policy, ownership, and audit MVP

### Goals
Prevent a plugin bloodbath.

### Deliverables
- capability registry
- grant model
- policy evaluation engine
- field ownership metadata
- confirmation policy support
- audit log for mutations
- facts/warnings label model
- `policy.explain`

### Success criteria
- module/workflow/tool actions can be allowed or denied by rule
- field ownership prevents invalid writes
- actions are auditable

## 10.6 Phase 3 — Template system MVP

### Goals
Make types composable and practical.

### Deliverables
- template manifest
- template composition and resolution
- field-group enable/disable support
- default values and validation extensions
- UI hints support
- `template.apply` implementation
- example templates:
  - base timed task
  - haircut task
  - Todoist synced overlay

### Success criteria
- users can create semantically enriched objects through templates
- templates can compose without breaking validation

## 10.7 Phase 4 — Workflow and skill runtime MVP

### Goals
Enable governed execution on top of typed objects.

### Deliverables
- workflow manifest and runtime
- skill manifest and runtime
- context binding to typed objects
- manual + scheduled trigger support
- tool-call steps
- skill-call steps
- artifact outputs
- workflow audit logging

### Success criteria
- a workflow can run over current tasks/events/projects
- a skill can use core tools through the permission model

## 10.8 Phase 5 — Module system MVP

### Goals
Enable packaged extensions.

### Deliverables
- module manifest
- install/enable/disable/inspect lifecycle
- module asset registration
- requested capability declaration
- module validation and sandbox expectations
- module diagnostics

### Success criteria
- a module can register templates/workflows/tools/adapters safely
- core remains the mediator

## 10.9 Phase 6 — Todoist adapter MVP

### Goals
Prove the architecture with one real external source.

### Deliverables
- Todoist source definition
- project/task mapping config
- read-only or conservative sync profile
- source mappings on objects
- field ownership rules
- sync workflow
- conflict labeling
- inspection UI/CLI

### Success criteria
- Todoist projects/tasks appear as Vel objects
- sync state is inspectable
- source-specific warnings/facts are visible
- no direct DB hacks in the integration

## 10.10 Phase 7 — Cross-object intelligence

### Goals
Start using the semantic richness.

### Deliverables
- nudges generated from object state
- task/event/project/thread linkage workflows
- richer explainability
- priority/effort/energy-aware planning views
- source reconciliation helpers

### Success criteria
- Vel demonstrates behavior that only makes sense because the core object model exists

## 10.11 Phase 8 — Hardening and ecosystem growth

### Deliverables
- versioned schema migration system
- module compatibility checks
- import/export formats for skills/workflows/modules
- richer query system
- better admin/debug tools
- additional adapters

## 10.12 Suggested implementation order inside codebase

Likely best order:

1. schema/registry crate/module
2. object storage + relation layer
3. action registry + tool dispatcher
4. policy evaluator + grants + audit
5. template resolver
6. workflow/skill runtime
7. module loader
8. Todoist adapter
9. UI/CLI inspection surfaces

## 10.13 Summary recommendation

Build the boring law first. The fun magic gets better when it has a spine.
