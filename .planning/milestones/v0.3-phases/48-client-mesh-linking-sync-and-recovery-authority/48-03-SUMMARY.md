# 48-03 Summary

## Outcome

Phase 48-03 added a typed governed `Now` config lane in Rust and wired the active `Now` service to consume it for header title and bucket count-display policy. The runtime config boundary now carries explicit `now` and `now.watch` sections instead of leaving those behaviors as shell constants or undocumented assumptions.

## What changed

- Added typed `NowConfig`, `NowTitleMode`, `NowCountDisplayMode`, and `NowWatchConfig` to [crates/vel-config/src/lib.rs](/home/jove/code/vel/crates/vel-config/src/lib.rs).
- Extended `AppConfig` loading so `vel.toml`-compatible files can govern:
  - `title_mode`
  - `title_literal`
  - `bucket_count_display`
  - reduced-watch flags under `[now.watch]`
- Updated [crates/veld/src/services/now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs) so the canonical header title and bucket count-display values are Rust-owned config decisions instead of hardcoded strings.
- Updated checked-in config assets:
  - [config/examples/app-config.example.toml](/home/jove/code/vel/config/examples/app-config.example.toml)
  - [config/templates/vel.toml.template](/home/jove/code/vel/config/templates/vel.toml.template)
  - [config/schemas/app-config.schema.json](/home/jove/code/vel/config/schemas/app-config.schema.json)
  - [config/README.md](/home/jove/code/vel/config/README.md)
- Updated [docs/templates/README.md](/home/jove/code/vel/docs/templates/README.md) so future config-bearing work follows the same template/example/schema path.

## Verification

- `cargo test -p vel-config -- --nocapture`
- `cargo test -p veld routes::now::tests::now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `cargo test -p veld app::tests::now_endpoint_returns_consolidated_snapshot -- --nocapture`
- `rg -n "title|count|watch|repair" config/README.md crates/vel-api-types/src/lib.rs crates/veld/src/services/now.rs crates/veld/src/routes/now.rs`

## Notes

- Reduced-watch behavior is now governed in Rust config, but Phase 48-03 does not yet embody those flags in Apple/watch UI code.
- Repair-route targets remain typed through the shared `Now` transport added in 48-02; this slice focused on the governed config lane that feeds the compact `Now` surface.
