# 02. Core Object Model

## 2.1 Purpose

The core object model defines Vel’s canonical, typed, composable semantic substrate.

Every important object should have:

- stable identity
- type
- schema
- lifecycle
- relations
- provenance
- permissions envelope
- audit trail
- optional source mappings

## 2.2 Base object envelope

Every object in Vel should share a common envelope.

Suggested base structure:

```yaml
id: task_01HV...
objectType: Task
schemaVersion: vel/task/v1
createdAt: 2026-03-22T10:00:00Z
updatedAt: 2026-03-22T10:30:00Z
createdBy:
  actorType: user|workflow|skill|module|sync
  actorId: ...
updatedBy:
  actorType: ...
  actorId: ...
status: active
workspaceId: ws_default
visibility: private
traits:
  - Completable
  - Taggable
  - Syncable
  - Auditable
labels:
  tags: []
  warnings: []
  facts: []
ownership: {}
provenance: {}
relations: {}
fields: {}
meta: {}
```

## 2.3 Canonical first-class object types

### Person
Represents a human or possibly an identity-capable participant.

Potential fields:

- displayName
- legalName
- handles
- emailAddresses
- phoneNumbers
- roles
- relationshipType
- availabilityHints
- sourceProfiles
- timezone
- pronouns
- contactMethods

### Project
Represents a scoped area of work, life, or concern.

Potential fields:

- name
- description
- status
- priority
- category
- ownerIds
- taskIds
- threadIds
- goal
- horizon
- deadlines
- tags
- sourceMappings

### Task
Represents a completable or actionable commitment unit.

Potential fields grouped below.

#### Task core fields
- title
- description
- status
- dueAt
- scheduledAt
- completedAt
- priority
- estimatedDuration
- recurrence
- checklist

#### Task planning fields
- urgency
- importance
- effort
- energy
- flexibility
- stretchGoal
- hardDeadline
- softDeadline
- frictionNotes

#### Task relationship fields
- projectId
- parentTaskId
- childTaskIds
- threadId
- eventId
- personIds
- nudgeIds
- tagIds
- artifactIds

#### Task governance fields
- source
- sourceId
- syncState
- fieldLocks
- confidence
- warnings
- facts

### Nudge
Represents a small guided intervention, reminder, prompt, or suggestion.

Potential fields:

- type
- title
- description
- severity
- urgency
- relevanceWindow
- rationale
- projectId
- taskId
- threadId
- sourceSignals
- dismissalRules
- displayStyle

### Event
Represents a time-bound scheduled item.

Potential fields:

- title
- description
- startAt
- endAt
- timezone
- location
- attendees
- eventType
- recurrence
- visibility
- status
- linkedTasks
- linkedThread
- sourceMapping

### Message
Represents a communication unit.

Potential fields:

- sender
- recipients
- sentAt
- receivedAt
- body
- attachments
- messageType
- channel
- source
- threadId
- taskLinks
- eventLinks

### Thread
Represents a multi-message or multi-activity conversational/logical thread.

Potential fields:

- title
- status
- participants
- messageIds
- relatedTaskIds
- relatedProjectIds
- relatedArtifacts
- summary
- threadType
- lastActivityAt

### Tag
Represents a reusable label or semantic classifier.

Potential fields:

- name
- color
- namespace
- description
- usagePolicy
- aliases
- hierarchy

### Template
Represents a reusable composable blueprint targeting one or more object types.

Potential fields:

- targetTypes
- includedTraits
- enabledFieldGroups
- defaults
- validationExtensions
- uiHints
- workflowHooks
- sourceMappingHints

### Tool
Represents a primitive action surface.

Potential fields:

- actionName
- description
- inputSchema
- outputSchema
- requiredCapabilities
- confirmationDefaults
- implementationRef

### Skill
Represents reusable executable behavior.

Potential fields:

- manifest
- entrypoints
- permissions
- routingHints
- templates
- examples
- runtimeLimits

### Workflow
Represents orchestrated executable logic across time, triggers, and context.

Potential fields:

- trigger
- scope
- steps
- bindings
- policies
- retries
- outputs

### Config
Represents settings, preferences, and policy bundles.

Potential fields:

- scope
- values
- schemaRef
- precedence
- source
- overrideRules

### Artifact
Represents generated or attached material.

Potential fields:

- type
- title
- uri
- storageRef
- source
- mimeType
- linkedObjects
- provenance

### Source
Represents an external or local system.

Potential fields:

- sourceType
- displayName
- capabilities
- connectionStatus
- authState
- syncProfiles
- warningLabels

## 2.4 Composable traits / aspects

Instead of deep inheritance, use traits/aspects.

Recommended initial trait catalog:

- `Taggable`
- `Schedulable`
- `Completable`
- `Relational`
- `Threaded`
- `Syncable`
- `Ownable`
- `Assignable`
- `Templated`
- `Auditable`
- `Locatable`
- `Messagelike`
- `Prioritizable`
- `Archivable`
- `Versioned`
- `Queryable`
- `Renderable`

### Trait example: Schedulable
Provides fields:

- scheduledAt
- dueAt
- startAt
- endAt
- timezone
- recurrence
- timeWindow

Provides behavior:

- validate temporal ordering
- expose calendar-like query indexes
- enable workflow triggers based on time

### Trait example: Syncable
Provides fields:

- sourceMappings
- syncState
- fieldOwnership
- lastSyncedAt
- conflictState

Provides behavior:

- adapter eligibility
- sync conflict inspection
- provenance access

## 2.5 Relation model

Vel should support explicit typed relations.

Recommended relation types:

- `belongs_to`
- `contains`
- `blocks`
- `blocked_by`
- `depends_on`
- `references`
- `generated_from`
- `linked_to`
- `scheduled_by`
- `about`
- `assigned_to`
- `mentioned_in`
- `derived_from`
- `mirrors`
- `owns`

Example:

```yaml
relations:
  - relationType: belongs_to
    targetType: Project
    targetId: project_home
  - relationType: linked_to
    targetType: Thread
    targetId: thread_haircut
```

## 2.6 Superset schema philosophy

Vel’s canonical schemas should be **broader than any one source system**.

This allows Vel-native fields such as:

- effort
- energy
- urgency
- flexibility
- rationale
- thread linkage
- nudge history
- source confidence
- semantic tags
- policy labels

Those may not round-trip to external systems, and that is fine.

The rule is:

- **Vel stores more than external systems can store**
- **Adapters declare what can round-trip**
- **Field ownership decides who may mutate which parts**

## 2.7 Field groups and toggles

A type can have field groups that are activated by:

- workspace profile
- template
- feature flag
- module
- source adapter
- user settings

Example `Task` field groups:

- `core`
- `planning`
- `time`
- `context`
- `governance`
- `sync`
- `messaging`
- `location`

This lets the schema remain powerful without forcing every UI or integration to carry all its weight all the time.

## 2.8 Object lifecycle

Each object type should define legal states and transitions.

Example `Task`:

- draft
- active
- blocked
- snoozed
- completed
- canceled
- archived

Transitions should be validated in core.

## 2.9 Explainability

Every object should support an `explain` or `inspect` view that can show:

- current fields
- active traits
- field owners
- source mappings
- policy labels
- relation graph
- recent mutations
- warnings/facts

This will matter enormously for trust and debugging.

## 2.10 Summary recommendation

Vel’s object model should be:

- canonical
- first-class
- typed
- composable
- relation-aware
- provenance-aware
- policy-aware
- integration-friendly without being integration-owned
