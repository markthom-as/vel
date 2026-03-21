## 50-03 Summary

Tightened the watch shell in [`ContentView.swift`](/home/jove/code/vel/clients/apple/Apps/VelWatch/ContentView.swift) so it reads as the reduced canonical `Now` subset rather than a separate watch product. The surface now explicitly frames itself around top status, top nudge, current task, quick voice/capture entry, and reduced thread handoff to iPhone or Mac when work stops being compact.

Updated [`README.md`](/home/jove/code/vel/clients/apple/README.md) so the watch guidance now matches that reduced parity model explicitly.

Verification:

- `rg -n "Now|Inbox actions|thread|handoff|voice|capture|sync|offline" clients/apple/Apps/VelWatch/ContentView.swift clients/apple/README.md`
