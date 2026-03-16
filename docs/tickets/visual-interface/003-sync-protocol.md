---
status: todo
owner: agent
priority: high
---

# 003 — Build `vel-protocol`

## Goal
Create compact phone/watch sync packets.

## Deliverables
- `types.ts`
- serializer/parser
- validation
- tests

## Instructions
1. Sync state, not frames.
2. Include versioned packet type.
3. Validate required numeric ranges.
4. Support optional event cue fields.
5. Add helper to derive packet from affect state.

## Acceptance criteria
- Packets are small and stable.
- Parser rejects malformed payloads.
- No renderer-specific data in packet.
