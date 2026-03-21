## Phase 50 Validation

Phase 50 acceptance was validated against the planned Apple parity target:

- iPhone and iPad now use the shared cached `Now` transport blocks as the primary compact frame
- Mac now consumes cached `Now` truth and compact continuity posture instead of summary placeholder copy
- watch is explicitly reduced, not divergent, and now teaches compact handoff to phone/Mac for deeper follow-through
- Apple docs now describe the shipped parity truth instead of a later target

Validation command:

- `rg -n "Now|Inbox|Threads|watch|voice|handoff|offline|sync" clients/apple/Apps/VeliOS/ContentView.swift clients/apple/Apps/VelMac/ContentView.swift clients/apple/Apps/VelWatch/ContentView.swift clients/apple/README.md docs/user/daily-use.md`

Environment note:

- no full Apple app-target build or `swift test` run was possible here; Phase 50 closes on source-level verification and truthful documentation, with the known Xcode/macOS environment limit preserved
