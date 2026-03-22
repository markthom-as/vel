# 09. Todoist Integration Spec

## 9.1 Goal

Implement Todoist as a governed source adapter module that maps Todoist projects and tasks into Vel canonical `Project` and `Task` objects through the Core Tools API.

## 9.2 Design position

Todoist is a source, adapter, and sync partner.
It is not the semantic owner of Task in Vel.

## 9.3 Supported mappings

### Todoist Project -> Vel Project
Potential mappings:

- `id` -> `sourceMappings.todoist.sourceId`
- `name` -> `name`
- `color` -> adapter metadata / tag hint
- `is_favorite` -> Vel metadata hint
- `view_style` -> UI hint metadata
- `parent_id` -> relation mapping

### Todoist Item -> Vel Task
Potential mappings:

- `id` -> `sourceMappings.todoist.sourceId`
- `content` -> `title`
- `description` -> `description`
- `priority` -> `priority`
- `due.datetime` -> `dueAt`
- `due.date` -> `dueOn`
- `deadline.date` -> `hardDeadline`
- `labels` -> `tagRefs` or tag names
- `project_id` -> related project source mapping
- `parent_id` -> parent task relation
- `checked/completed_at` -> `completedAt` / `status`

## 9.4 Vel-native fields that will not round-trip by default

- `effort`
- `energy`
- `importance`
- `urgency` (if richer than Todoist priority)
- `flexibility`
- `threadId`
- `nudgeIds`
- `semanticNotes`
- `rationale`
- `confidence`
- `warningLabels`

These should be stored in Vel and marked as core-owned.

## 9.5 Required core capabilities

Minimum likely needs:

- `project.read`
- `project.upsert`
- `task.read`
- `task.create`
- `task.update`
- `task.complete`
- `tag.apply`
- `source.sync`
- `audit.append`

## 9.6 Facts/warnings label example

```yaml
facts:
  sourceOfTruth: hybrid
  supportsProjects: true
  supportsTasks: true
  supportsSubtasks: true
  supportsNativeThreads: false
  roundTripSupport: partial
warnings:
  - Vel-native planning fields do not round-trip to Todoist.
  - Recurrence semantics may differ between systems.
  - Shared Todoist projects can cause upstream side effects when syncing writes.
constraints:
  - No native Event type in Todoist.
  - No native Nudge object in Todoist.
```

## 9.7 Recommended sync profiles

### Profile A: Read-only mirror
Safest startup mode.
Vel imports Todoist projects/tasks but does not push changes back.

### Profile B: Bidirectional conservative
Vel may update mapped shared fields, but Vel-only fields stay local.
Conflict handling is explicit.

### Profile C: Core-preferred with manual reconcile
Vel allows richer local semantics and asks for confirmation before overwriting source-conflicted fields.

## 9.8 Initial workflow support

Suggested first workflows:

- full sync pull
- incremental sync pull
- push writable field changes upstream
- conflict detection and labeling
- sync summary artifact generation

## 9.9 UI/CLI expectations

Should be able to inspect:

- mapping status
- sync profile
- field ownership
- last sync time
- pending conflicts
- warnings/facts labels

## 9.10 Summary recommendation

Start Todoist in a conservative, explicit, inspectable way. Read-only or limited bidirectional sync first. Avoid pretending the source can represent everything Vel wants to know.
