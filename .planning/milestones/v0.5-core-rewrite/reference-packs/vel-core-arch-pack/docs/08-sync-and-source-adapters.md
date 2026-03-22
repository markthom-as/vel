# 08. Sync and Source Adapters

## 8.1 Purpose

External systems must map into Vel through explicit source adapters.

Adapters are responsible for:

- fetching external data
- mapping to/from canonical object types
- declaring ownership and field authority
- handling sync direction
- surfacing lossiness and warnings
- reconciling conflicts

## 8.2 Adapters are not ontology providers

Todoist is not the definition of `Task`.
Google Calendar is not the definition of `Event`.
Gmail is not the definition of `Message`.

They are sources.
Vel is the semantic center.

## 8.3 Adapter responsibilities

Each adapter should define:

- supported external entity types
- target core object types
- field mappings
- field ownership
- sync direction
- merge/conflict rules
- known limitations
- facts/warnings labels
- required capabilities

## 8.4 Sync direction modes

Recommended modes:

- `read_only`
- `write_only`
- `bidirectional`
- `core_preferred`
- `source_preferred`
- `manual_reconcile`

## 8.5 Object and field ownership

Must track at least:

- object owner
- mirrored sources
- field owners
- write authority
- merge strategy

## 8.6 Provenance

Each mapped object should retain provenance, such as:

- source name
- source ID
- fetch timestamp
- sync run ID
- transformed fields
- confidence/reliability notes

## 8.7 Conflict handling

Conflicts should be represented explicitly, not silently swallowed.

Strategies may include:

- last writer wins
- source-preferred
- core-preferred
- field-level merge
- manual reconcile required

Need conflict inspection tools.

## 8.8 Warning/facts labels for adapters

Example:

```yaml
facts:
  sourceOfTruth: external
  roundTripSupport: partial
  recurrenceSupport: lossy
warnings:
  - Parent/subtask completion semantics differ from Vel.
  - Some Vel-native planning fields do not round-trip upstream.
constraints:
  - No native thread objects in source system.
```

## 8.9 Adapter package structure

```text
adapters/
  todoist/
    adapter.yaml
    mappings/
      task.yaml
      project.yaml
    sync/
      pull.ts
      push.ts
      reconcile.ts
```

## 8.10 Summary recommendation

Treat every integration as a formal adapter with explicit mapping and ownership semantics, not a magical blob of sync hope.
