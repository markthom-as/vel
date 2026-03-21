## 44-02 Summary

Aligned Apple shell hierarchy to the MVP surface model without introducing Apple-local product policy.

### What changed

- Renamed stale iPhone/iPad shell state in `clients/apple/Apps/VeliOS/ContentView.swift` from older internal labels (`today`, `nudges`, `activity`, `planning`, `chat`, `capture`) to MVP-aligned surface names (`now`, `inbox`, `threads`, `quickEntry`).
- Removed the unused `ChatScaffoldTab` placeholder so the Apple shell no longer preserves a dead pre-MVP chat taxonomy.
- Reworked `clients/apple/Apps/VelMac/ContentView.swift` around the same named surfaces as web:
  - `Now` carries status, current context, and quick entry.
  - `Inbox` carries nudges and open commitments.
  - `Threads` carries compact continuity summary only.
  - `Projects` remains secondary.
  - `Settings` is support-only and documentation-focused.
- Updated `clients/apple/README.md` and `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift` to describe Apple affordances as thin wrappers over backend-owned routing and continuity.

### Verification

- `rg -n "planning|activity|chat|Now|Inbox|Threads|Projects|Settings" clients/apple/Apps/VeliOS/ContentView.swift clients/apple/Apps/VelMac/ContentView.swift clients/apple/README.md`
- Attempted `swift test --package-path clients/apple/VelAPI`
  - blocked in this environment: `/etc/profiles/per-user/jove/bin/swift: line 35: exec: swift-test: not found`

### Outcome

Apple shells now teach the same MVP hierarchy as web while preserving native quick-entry, voice, and offline affordances as shell-native wrappers over backend-owned authority.
