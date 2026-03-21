## 50-01 Summary

Aligned iPhone and iPad `Now` toward the canonical compact contract in [`ContentView.swift`](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift). `TodayTab` now reads from cached shared `Now` transport blocks for the compact header, status row, context line, trust posture, nudge stack, task lane, and docked input shell instead of relying on the older summary-heavy section stack as the primary presentation.

The previous deeper context remains available behind `More context and controls`, which keeps Apple thin over the Rust-owned transport while preserving existing utility during the parity migration. [`README.md`](/home/jove/code/vel/clients/apple/README.md) was updated so Apple guidance now describes `Now` as a compact execution lane rather than a summary-first shell.

Verification:

- `rg -n "cachedNow\\?|status_row|context_line|mesh_summary|nudge_bars|task_lane|docked_input|Now|Needs input|Nudges|Open capture|Open voice" clients/apple/Apps/VeliOS/ContentView.swift clients/apple/README.md`
