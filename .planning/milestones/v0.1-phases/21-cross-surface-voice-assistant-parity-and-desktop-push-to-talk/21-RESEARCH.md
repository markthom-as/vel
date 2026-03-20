# Phase 21 Research

## Current Truth

Voice already exists in multiple places, but it is not yet one product seam:

- Web uses local browser speech-to-text in [clients/web/src/hooks/useSpeechRecognition.ts](/home/jove/code/vel/clients/web/src/hooks/useSpeechRecognition.ts) and sends the transcript through the shared assistant-entry route from [clients/web/src/components/MessageComposer.tsx](/home/jove/code/vel/clients/web/src/components/MessageComposer.tsx).
- Apple still uses the dedicated backend endpoint `POST /v1/apple/voice/turn` through [clients/apple/VelAPI/Sources/VelAPI/VelClient.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/VelClient.swift) and the iOS voice shell in [clients/apple/Apps/VeliOS/ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift).
- Backend Apple voice behavior lives in [crates/veld/src/services/apple_voice.rs](/home/jove/code/vel/crates/veld/src/services/apple_voice.rs), which still branches on Apple-specific intent hints before choosing schedule, daily-loop, nudge, commitment, or capture paths.

## Architectural Read

The repo direction from Phases 13-20 is clear:

- backend/Rust owns assistant semantics, continuity, and policy
- shells own microphone permission, push-to-talk presentation, local STT, and offline UX
- typed and voiced input should converge before product logic, not after it

That means this phase should not create a second “voice runtime.” It should narrow Apple-specific behavior and strengthen the shared transcript-driven assistant seam.

## Key Seams

### Shared backend seam

The best migration target is the existing assistant-entry path introduced in Phase 20:

- `POST /api/assistant/entry`
- typed `AssistantEntryRequest` / `AssistantEntryResponse`
- backend-owned route outcomes: `inbox`, `threads`, `inline`

Phase 21 should evolve that into a voice-capable seam rather than leaving voice as an Apple-only route.

### Web/desktop seam

The web already has the right low-level primitive:

- local STT via `SpeechRecognition`
- transcript flows through the same assistant-entry helper

The missing daily-use behavior is push-to-talk polish and explicit failure/fallback semantics, not new product logic.

### Apple seam

Apple already persists transcript provenance and reuses backend daily-loop authority for MorningBriefing, but it still carries too much shell-specific intent routing and fallback branching in Swift and the dedicated Apple backend route.

The likely honest direction is:

- preserve Apple push-to-talk + TTS + offline/cache shell affordances
- reuse a shared transcript-first backend assistant seam where possible
- keep Apple-only route behavior only for genuinely Apple-specific compatibility or offline fallback edges

## Risks

- A big-bang Apple route replacement is risky because the current Apple route also carries offline/cache expectations and typed daily-loop compatibility.
- Web push-to-talk can easily sprawl into UI-only work; keep the scope on shared behavior and transcript provenance.
- Local STT support is browser-dependent; unsupported-state guidance must stay explicit and non-misleading.

## Recommended Slice Order

1. Establish the shared voice assistant contract and backend migration seam.
2. Improve desktop/web push-to-talk over that shared seam.
3. Align Apple voice onto the shared seam where behavior overlaps, while preserving bounded offline/cache behavior.
4. Close docs/tests/parity so the product teaches one voice story instead of two.

## Verification Direction

The strongest focused verification will be:

- backend tests proving transcript-first voice entry uses shared assistant routing and preserves provenance
- web tests proving push-to-talk uses the same assistant entry as typed input and degrades cleanly
- Apple package/client tests proving supported voice paths no longer invent separate policy locally
