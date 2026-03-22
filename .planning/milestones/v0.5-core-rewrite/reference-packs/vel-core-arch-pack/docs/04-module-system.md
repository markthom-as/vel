# 04. Module System

## 4.1 Purpose

Modules are Vel’s extension units.

A module may contribute groups of:

- templates
- skills
- workflows
- tools
- source adapters
- UI extensions
- field packs
- policy bundles
- config presets
- tags
- example fixtures

Modules should be installable, inspectable, permissioned, and governable.

## 4.2 Architectural role

Modules sit above core and below user-facing behavior.
They may extend Vel, but they should not bypass Vel.

That means modules do **not** get direct arbitrary database access.
They interact through Vel Core tools/API.

## 4.3 What a module may register

Recommended registrable assets:

- `Template`
- `Skill`
- `Workflow`
- `Tool`
- `Adapter`
- `PolicyBundle`
- `FieldPack`
- `UIExtension`
- `ConfigPreset`
- `SeedTagSet`

## 4.4 Module categories

Recommended namespaces:

- `core/*` — first-party foundational modules
- `integrations/*` — Todoist, Google Calendar, Gmail, Spotify, etc.
- `local/*` — machine-specific or personal environment modules
- `workspace/*` — project/workspace-scoped modules
- `community/*` — shareable third-party packages

## 4.5 Module manifest

Suggested module manifest example:

```yaml
apiVersion: vel/v1
kind: Module
metadata:
  name: todoist
  namespace: integrations
  version: 0.1.0
  displayName: Todoist Integration
spec:
  contributes:
    adapters:
      - ./adapters/todoist.yaml
    workflows:
      - ./workflows/todoist-sync.yaml
    templates:
      - ./templates/todoist-task-overlay.yaml
    tools:
      - ./tools/todoist-sync-status.yaml
    configPresets:
      - ./config/default-profile.yaml
  requires:
    capabilities:
      - task.read
      - task.create
      - task.update
      - project.read
      - project.upsert
      - source.sync
    sources:
      - todoist
  runtime:
    sandbox: subprocess
    network: restricted
  policies:
    defaultConfirmation: ask
```

## 4.6 Module lifecycle

Modules should support:

- install
- enable
- disable
- inspect
- update
- remove
- verify
- test
- migrate

## 4.7 Module permissions

Modules should request capabilities explicitly.

Examples:

- `task.read`
- `task.create`
- `task.update`
- `task.complete`
- `project.read`
- `event.read`
- `source.sync`
- `thread.append`
- `artifact.create`

A module should not get “all task powers.”
That is how you accidentally hand the keys of the kingdom to a plugin because it smiled politely.

## 4.8 Module boundaries

Modules may:

- call core tools
- register new assets
- propose policy bundles
- contribute UI hints
- define adapter mapping behavior

Modules may not, by default:

- bypass core validation
- bypass policy checks
- mutate restricted fields directly
- invent raw SQL adventures in the basement
- rewrite core type semantics on the fly

## 4.9 Field packs and schema extensions

A module may contribute field packs for approved types.

Example:

- `todoist-sync-pack` adds source-specific mapping metadata to `Task` and `Project`
- `wellbeing-pack` adds energy/mood/context fields to `Task` and `Nudge`

These should be governed by:

- extension points in the core type definition
- versioned schema fragments
- compatibility checks
- UI activation rules

## 4.10 Module dependency model

Modules may depend on:

- specific core versions
- specific trait packs
- specific other modules
- specific sources being configured

Need dependency validation and graceful degradation.

## 4.11 Module packaging strategy

Internal-first packaging could be plain folders or git-based installs.

Suggested structure:

```text
modules/
  integrations/
    todoist/
      module.yaml
      adapters/
      workflows/
      templates/
      tools/
      config/
      README.md
```

## 4.12 Module governance metadata

Each module should expose:

- requested capabilities
- declared risks
- facts/warnings labels
- source dependencies
- network access needs
- destructive action potential
- expected data sensitivity

## 4.13 Summary recommendation

Modules should be treated as governed extension bundles that enrich Vel through declared assets and capabilities while remaining subordinate to core semantics, policy, and audit rules.
