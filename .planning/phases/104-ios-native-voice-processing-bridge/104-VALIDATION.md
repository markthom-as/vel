# Phase 104 Validation

## Boundary

- no privileged Apple session logic is reimplemented in Rust
- FFI boundary is narrow and typed

## Platform

- interruptions recover safely
- route changes recover safely
- permission-denied and permission-granted states are both handled truthfully

## Behavioral

- duplex call mode still honors barge-in and single-active-turn semantics on iOS
