# Widget + Live Activity Scaffold

Status: implemented scaffold target `VelWidgetExtension` in `clients/apple/Vel.xcodeproj`.

Current scope:
- `VelStatusWidget` timeline scaffold
- accessory family support for complication-ready surfaces (`accessoryInline`, `accessoryCircular`, `accessoryRectangular`)
- `VelLiveActivityWidget` placeholder using `ActivityKit`

Follow-up scope:
- wire widget data to durable `Now` snapshot state
- wire live activity to standup/focus lifecycle events
- add shared rendering tokens via `VelUIShared`
