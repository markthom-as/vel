# Next Steps

`0.5.8` is closed as a compatibility bridge, not as a full GSD 2 migration.

Completed in this milestone:

1. inventoried the current `get-shit-done` v1 command surface
2. compared that surface against the available `.gsd` migration state
3. selected the honest compatibility-bridge path
4. verified roadmap, health, progress, cleanup inputs, and new-milestone initialization
5. kept duplex voice parked in [hybrid-duplex-voice-runtime-spec.md](/Users/jove/code/vel/docs/future/hybrid-duplex-voice-runtime-spec.md) until a future milestone reopens it

Residual follow-up:

- v1 helper metadata still reports stale `v0.1` milestone labels in `init progress` and `init new-milestone`
- `init cleanup` is not a structured helper; cleanup remains markdown-workflow driven
- `scripts/gsd2.sh` now runs the installed `gsd-pi` command surface under a repo-selected Node `>=22` runtime without requiring ad hoc shell `PATH` edits
- `scripts/gsd2.sh` also repairs the installed bundle's missing internal `@gsd-build/mcp-server` link when `packages/mcp-server` is present, so `npm run gsd2 -- graph build` and `npm run gsd2 -- graph status` now pass
- full GSD 2 migration remains future work until headless behavior is stable beyond the verified `headless query` path and command equivalence is verified

## Guiding Principle

Keep the compatibility bridge honest.
Do not let the planning toolchain become another source of product drift.
