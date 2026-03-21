## 50-02 Summary

Tightened macOS parity in [`ContentView.swift`](/home/jove/code/vel/clients/apple/Apps/VelMac/ContentView.swift) by making the `Now` lane consume cached shared `Now` transport directly instead of relying on placeholder summary copy. VelMac now renders the canonical title, bucket summary, status row, context line, compact mesh posture, top nudge, and active task from cached `Now` data, and it refreshes `/v1/now` during load so the offline cache stays current.

The `Threads` section now also reflects compact continuation posture rather than generic placeholder text, while still avoiding any Apple-local policy invention.

Verification:

- `rg -n "summary-first|Threads|Inbox|Settings|authority|continuation|Open target|cachedNow|status_row|mesh_summary|task_lane" clients/apple/Apps/VelMac/ContentView.swift clients/apple/VelAPI/Sources/VelAPI/Models.swift`
