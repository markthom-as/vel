## Phase 64-03 Summary

Phase `64-03` proved Google recurrence, availability, and delete handling through the native calendar core instead of provider-side shadow semantics.

### Landed

- added Google recurrence translation in `crates/vel-adapters-google-calendar/src/recurrence_sync.rs`
- added native availability bridge inputs and projection metadata in `crates/vel-adapters-google-calendar/src/availability_bridge.rs`
- added Google event tombstone transitions in `crates/vel-adapters-google-calendar/src/tombstones.rs`
- exported the new adapter helpers from `crates/vel-adapters-google-calendar/src/lib.rs`
- added black-box recurrence, availability, and tombstone proof in `crates/veld/tests/phase64_recurrence_and_availability.rs`

### Contract outcomes

- Google recurrence now maps into canonical `Series`, `Occurrence`, and `Exception` contracts with raw RRULE retention and explicit unsupported `this_and_following` posture.
- Availability remains derived and rebuildable through the native availability projection contract; the adapter only bridges canonical calendar/event inputs and projection metadata.
- Upstream deletes become hidden tombstones with pending-reconcile state by default and preserve restore/audit lineage when the object reappears.
- Provider facets retain Google-specific recurrence and tombstone metadata without redefining canonical event or availability truth.

### Verification

- `cargo test -p vel-adapters-google-calendar --lib`
- `cargo test -p veld --test phase64_recurrence_and_availability`
- `cargo check -p vel-adapters-google-calendar && cargo check -p veld`
