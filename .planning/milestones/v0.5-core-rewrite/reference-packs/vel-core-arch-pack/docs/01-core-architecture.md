# 01. Vel Core Architecture

## 1.1 Architectural thesis

Vel should be designed as a **typed, policy-governed semantic core** that exposes a unified object model and action surface to:

- internal logic
- CLI
- UI
- agents
- workflows
- automations
- connectors/integrations
- external APIs

The mistake to avoid is building Vel as a loose stack of:

- chat prompts
- integration-specific logic
- tool wrappers
- ad hoc data sync
- feature toggles with no object-level semantics

That path feels fast early and becomes cursed later.

The stronger architecture is:

```text
Users / UI / CLI / Agents / Automations
                |
         Vel Core Tools API
                |
     Policy / Permissions / Audit
                |
   Canonical Object Model + Relations
                |
 Templates / Skills / Workflows / Tools
                |
   Modules / Adapters / Integrations
                |
 External Systems / Local Systems / Data Sources
```

## 1.2 Core design principles

### Principle 1: Vel Core owns the ontology
Vel defines what a `Task` is, what an `Event` is, what a `Thread` is, what a `Nudge` is, and how they relate.

External systems do not get to define these for Vel.

### Principle 2: Everything important is an object
Important system nouns should be first-class objects with IDs, schemas, relations, audit history, permissions, provenance, and lifecycle.

### Principle 3: All access goes through the tools/API membrane
Neither modules nor integrations should mutate storage directly.
They must call governed core actions.

### Principle 4: Types are composable
Avoid an inheritance swamp.
Use composable traits/aspects/field packs.

### Principle 5: External systems are adapters, not sovereigns
Todoist, calendars, chat systems, media systems, browser history, and sensor feeds are sources/adapters/sync peers.
They map into Vel.
They do not own Vel’s semantic truth.

### Principle 6: The schema is a superset, but activation is selective
Vel may define a rich superset of fields across domains, but workspaces, templates, profiles, and adapters may enable only the needed subsets.

### Principle 7: Automation must be auditable
Every workflow, skill run, sync action, and cross-source mutation should be attributable, inspectable, and reversible where possible.

## 1.3 Architectural layers

### Layer A: Canonical object kernel
Responsible for:

- object type definitions
- schemas
- traits/aspects
- relation graph
- validation
- lifecycle rules
- field packs
- template targeting

### Layer B: Core tools/API surface
Responsible for:

- reads
- queries
- mutations
- bulk operations
- linking relations
- lifecycle transitions
- explain/inspect endpoints
- safe machine-facing action semantics

### Layer C: Policy / permissions / grants / audit
Responsible for:

- capability checking
- per-object/per-field action authorization
- confirmation policies
- execution scopes
- trust labels
- facts/warnings labels
- mutation logs
- attribution

### Layer D: Skills and workflows runtime
Responsible for:

- executable behavior
- multi-step orchestration
- trigger-based automations
- object-context binding
- model/tool invocation
- plan execution
- artifacts and logs

### Layer E: Modules and adapters
Responsible for:

- integrations
- type extensions
- template packs
- UI additions
- source adapters
- sync jobs
- imported skills/workflows/tools

### Layer F: Presentation surfaces
Responsible for:

- web UI
- mobile/native UI
- CLI
- admin/debug UI
- tool-use API
- agent runtime shells

## 1.4 What is “core” in Vel

“Core” should mean:

- fundamental object types
- relations between them
- validation and constraints
- canonical state transitions
- policy and permission enforcement
- execution primitives
- sync semantics
- query and mutation APIs

“Core” should **not** mean every possible vertical feature.

Keep core strict and foundational.
Let modules contribute breadth.

## 1.5 Top-level objects that likely belong in core

Minimum recommended list:

- `Person`
- `Task`
- `Project`
- `Nudge`
- `Event`
- `Message`
- `Thread`
- `Tag`
- `Template`
- `Skill`
- `Tool`
- `Workflow`
- `Config`
- `Artifact`
- `Source`
- `Policy`
- `Fact`
- `Relation`

You could defer some of these in storage implementation, but architecturally they belong in the family.

## 1.6 Why this matters for Vel specifically

Vel is clearly trying to become more than a task app, more than a calendar assistant, and more than a chat shell.

It wants to reason across:

- user context
- time context
- commitments
- communications
- nudges
- threads
- artifacts
- schedules
- integrations
- workflows
- adaptive execution

That requires stable semantic anchors.
Without them, every new feature becomes a local workaround.

## 1.7 Non-goals

Do not attempt in v1 to build:

- universal perfect ontology for all human life
- fully generic graph database semantics exposed raw to users
- unconstrained plugin execution
- arbitrary schema mutation by modules in production
- direct remote execution with ambient full trust
- sync perfection across lossy sources

Aim for a durable, governed, extensible substrate instead.

## 1.8 Naming recommendations

Use consistent terms:

- **Object**: a stored, typed, first-class entity in Vel
- **Trait/Aspect**: composable behavior/field capability set
- **Template**: composable object blueprint/config overlay
- **Tool**: primitive action surface exposed by core or modules
- **Skill**: reusable governed executable behavior
- **Workflow**: orchestrated series of actions/skills/tools across context
- **Module**: package that contributes extensions to Vel
- **Adapter**: mapping/sync layer for an external source/system
- **Source**: an external or local system providing data or action capabilities
- **Policy**: rule system for permissions and behavior
- **Grant**: approved capability for an execution context

## 1.9 Summary recommendation

The platform should be built **object-first, policy-first, API-first**, with workflows and integrations as downstream citizens of a stronger semantic state model.

That is the spine.
Everything else plugs into it.
