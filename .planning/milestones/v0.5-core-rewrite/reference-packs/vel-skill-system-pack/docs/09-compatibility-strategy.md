# Compatibility Strategy

## Goal

Vel should be capable of partial compatibility with Claude/Codex-style “skills” or prompt packs, but this should be implemented through adapters rather than by making those external formats the source of truth.

## Principle

Vel’s internal representation should be **more expressive** than imported formats.

That way:

- external packs can be imported into a subset of Vel features
- Vel-native skills can remain richer than imported ones
- export is possible where fields map cleanly

## Import adapter responsibilities

An import adapter should:

- read external package metadata
- normalize names and version fields
- map instructions/prompts into `prompt.md`
- translate capability declarations where possible
- infer missing defaults conservatively
- wrap unsupported behaviors explicitly
- mark unresolved fields instead of silently dropping meaning

## Export adapter responsibilities

An export adapter should:

- emit mapped metadata
- flatten instructions/resources as needed
- warn when Vel-native fields cannot be represented externally
- avoid pretending feature parity where none exists

## Compatibility levels

Recommend explicit labels:

- `full` — round-trips cleanly
- `partial` — usable with dropped or shimmed features
- `import_only` — can ingest but not export faithfully
- `unsupported` — too lossy or unsafe

## What not to do

Do not build the Vel manifest around another vendor’s assumptions about:

- permissions
- runtime hooks
- workflow composition
- context mounting
- CLI execution
- artifact models

That would sabotage Vel’s internal coherence for the sake of shallow familiarity.

## Recommendation

Treat compatibility as a perimeter feature, not a constitutional principle.
