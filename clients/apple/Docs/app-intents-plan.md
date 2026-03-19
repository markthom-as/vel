# App Intents Scaffold

Status: implemented scaffold target `VelIntentsExtension` in `clients/apple/Vel.xcodeproj`.

Current scope:
- `VelQuickCaptureIntent` placeholder intent
- extension entrypoint via `@main struct VelIntentsExtension: AppIntentsExtension`
- safe placeholder behavior (opens Vel app and returns dialog)

Follow-up scope:
- map intent execution into `VelApplication` use cases
- add auth/session-aware intent routing
- add capability-gated intent availability for iPhone/iPad/Watch/Mac surfaces
