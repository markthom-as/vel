# vel_apple_and_voice_client_spec.md

Status: Product / interaction and interface-boundary specification  
Audience: coding agent / client app implementer / Vel core implementer  
Purpose: define the first-pass design for iPhone, Apple Watch, and desktop voice as clients of Vel core

---

# 1. Scope

This spec defines:

- Apple Watch role
- iPhone role
- desktop voice role
- interaction model
- action authority
- clarification behavior
- notification behavior
- client/core boundary
- MVP surfaces
- later expansion paths

This is **not** a full pixel-perfect implementation spec.  
It is a product / interaction spec plus interface-boundary spec.

The goal is to make the Apple + voice layer coherent while Vel core continues to mature.

---

# 2. Client Ecology

Vel’s client ecosystem should be treated as a set of specialized surfaces over one shared stateful core.

## 2.1 Apple Watch
Primary role:
- interrupt channel
- haptic nudge acknowledgement
- ultra-brief “what matters now?” surface later

## 2.2 iPhone
Primary role:
- current context review
- active nudges
- data collection / feedback entry
- later explanation, thread review, synthesis review

## 2.3 Desktop Voice
Primary role:
- morning orchestration
- quick command/control
- quick capture
- layered explanation
- later conversational reflective interface

## 2.4 Desktop CLI / TUI
Primary role:
- deep inspection
- planning
- debugging
- explicit control
- dogfooding power surface

These clients must all operate over the same underlying Vel state and command model.

---

# 3. Product Principle

Vel clients should collectively behave like:

> a calm, analytical, stateful assistant / executive-function prosthetic

They should not feel like:
- disconnected apps
- a generic chat wrapper
- a hyperactive nag daemon
- a motivational coach

Tone should be:
- calm
- analytical
- attentive
- slightly human
- non-chirpy
- non-preachy

---

# 4. Shared Core Rule

All clients must defer to the same Vel core for:

- signals
- commitments
- current context
- risk
- nudges
- suggestions
- threads
- artifacts
- synthesis
- self-model

Clients may differ in presentation and available actions, but they must not maintain separate business logic or forked interpretations of the user’s state.

---

# 5. Voice Primary Use Cases

Desktop voice should support these use cases in order of priority.

## 5.1 Morning orchestration (highest priority)
Examples:
- “What do I need to do right now?”
- “What matters this morning?”
- “Why are you warning me?”
- “What should I do next?”

## 5.2 Quick command/control
Examples:
- “Mark meds done.”
- “Snooze that 10 minutes.”
- “What’s my next commitment?”
- “What are my active nudges?”

## 5.3 Quick capture
Examples:
- “Idea: commute defaults should learn from history.”
- “Reminder: reply to Dimitri.”
- “Vel bug: thread relevance feels off.”

## 5.4 Reflective conversation (lower initial priority)
Examples:
- “What patterns are you seeing this week?”
- “Why did this week go sideways?”

This reflective mode is important later, but not required for the first client MVP.

---

# 6. Voice Activation Model

## 6.1 MVP
Use **push-to-talk** first.

This applies to:
- desktop voice
- phone quick action
- watch shortcut later if useful

## 6.2 Later
Wake word may be added later.

Important rule:
- do not build always-listening architecture into the MVP
- do not make wake-word assumptions part of core logic

Voice activation should remain a client concern, not a Vel core concern.

---

# 7. Conversational Modes

Voice and text clients may support multiple interaction modes.

## 7.1 Default mode: command/secretary mode
Most interactions should be:
- short
- action-oriented
- minimal spoken output
- fast to confirm

## 7.2 Explanation mode
When asked “why?” or “explain,” Vel may provide a slightly richer, analytical response.

## 7.3 Reflection mode
Later, Vel may support more conversational reflective dialogue for:
- weekly synthesis
- intent vs behavior
- self-review
- policy adjustment

Important rule:
- do not force every interaction into chat mode
- command-first behavior is the default

---

# 8. Output Style

Use **layered output**.

## 8.1 First response
Keep it short and useful.

Example:
“You should start prep now.”

## 8.2 Follow-up
If user asks “why?” or “what changed?” then elaborate.

Example:
“Your meeting is at 11, prep is unresolved, and your commute buffer is 40 minutes.”

This applies especially to:
- desktop voice
- iPhone chat/explanation
- later smart speaker mode

Watch output should remain ultra-brief.

---

# 9. Ambiguity Handling

When voice or client input is ambiguous, use this policy:

## 9.1 High-risk actions
Ask for clarification.

Examples:
- changing schedule assumptions
- marking important commitment done when reference unclear
- sending messages
- anything that changes timing or state significantly

## 9.2 Medium-risk actions
Make best guess and confirm.

Example:
“You mean meds, right?”

## 9.3 Low-risk actions
If uncertainty is acceptable:
- create a capture
- or ask later
- or treat as note rather than direct action

Example:
“Remind me about Dimitri” → create capture if intent is too ambiguous.

This keeps the system helpful without becoming reckless.

---

# 10. Authority Model for Clients

Clients must not silently perform high-consequence actions.

## 10.1 Allowed without confirmation
Safe actions:
- read current context
- read next commitment
- read active nudges
- create captures
- snooze an unambiguous nudge
- mark a clearly identified nudge done

## 10.2 Require confirmation
Higher-risk actions:
- changing prep/commute defaults
- changing schedule assumptions
- marking important ambiguous commitments done
- creating inferred commitments from fuzzy input
- rescheduling or messaging others

## 10.3 Not allowed automatically
Do not automatically:
- reschedule meetings
- send messages
- rewrite user commitments without confirmation

These may be proposed later as suggestions.

---

# 11. Shared Command Model

Desktop voice, iPhone, Watch, and CLI should share one underlying action grammar.

Examples:

Voice:
- “Mark meds done.”

CLI:
- `vel done meds`

Watch:
- Done button on meds nudge

All three should map to the same underlying intent/action in Vel core.

This is mandatory.  
Do not create separate command semantics per client.

---

# 12. Notification / Acknowledgement Model

## 12.1 Core acknowledgement protocol
All nudge acknowledgements should reduce to:

- Done
- Snooze

This remains the canonical action set across clients.

## 12.2 Watch is primary interrupt channel
Away from computer, Apple Watch is the most important interruption surface.

This should be treated as the primary wrist-level acknowledgement path.

## 12.3 Desktop notification
Desktop notifications are secondary:
- useful while at workstation
- less important when away

## 12.4 iPhone notification
Can exist as a parallel mobile surface, but should not compete with Watch too aggressively.

---

# 13. Apple Watch MVP

## 13.1 Primary role
Respond-first surface.

The first Watch MVP should focus on:
- receiving nudges
- acknowledging Done / Snooze
- optionally viewing one-line context

## 13.2 Required interactions
For MVP:

### Nudge card
Show:
- title
- short message
- severity
- two actions

Example:
“Prep window started.”
[ Done ] [ Snooze ]

### Nudge severities
Map to:
- gentle haptic
- stronger haptic
- danger haptic + optional sound later

## 13.3 Not required for MVP
- rich thread browsing
- weekly synthesis browsing
- open-ended watch chat
- full planning UI
- broad text entry

## 13.4 Later additions
- “What matters now?”
- next commitment glance
- current risk glance
- quick voice query
- smart wake / morning briefing integration later

---

# 14. iPhone MVP

## 14.1 Primary role
Review and context surface.

Most important first surfaces:

### A. Current Context
Show:
- current mode
- next commitment
- meds status
- prep / commute windows
- current risk level

### B. Active Nudges
List active/snoozed nudges with quick actions.

### C. Data collection / feedback
Allow:
- light feedback on usefulness/annoyance
- quick capture
- maybe quick commitment creation later

## 14.2 Nice-to-have early
- explanation pane
- latest synthesis artifact
- thread summaries

## 14.3 Not required for MVP
- full thread graph editor
- full risk inspector
- all synthesis controls
- large-scale historical analytics

---

# 15. Desktop Voice MVP

## 15.1 Primary role
Morning orchestration + command/control + quick capture.

## 15.2 Required interactions
Examples:
- “What matters right now?”
- “What’s my next commitment?”
- “Mark meds done.”
- “Snooze that 10 minutes.”
- “Capture this idea…”

## 15.3 Layered response examples

### Initial answer
“Your first meeting is at 11. Prep should start now.”

### Follow-up
“Why?”
“Prep is unresolved, commute is 40 minutes, and risk is currently high.”

## 15.4 Not required for MVP
- long reflective freeform chat
- complex multi-turn planning wizard
- always-on desktop listening

These can come later.

---

# 16. Contextuality by Input Source

Some behavior may later vary by client/input source, but command semantics must remain shared.

Examples of allowed differences:
- Watch response ultra-brief
- desktop voice can elaborate
- iPhone can show more explanation detail

Examples of forbidden differences:
- Watch `Done` means something different than CLI `done`
- desktop voice uses different underlying commitment resolution semantics

---

# 17. Privacy / Ambient Behavior Constraints

## 17.1 MVP assumptions
- no always-listening voice
- no automatic spoken responses in public by default
- Watch should prefer haptic over sound by default
- danger sound / klaxon-like escalation should require explicit enabling or careful mode gating later

## 17.2 Future ambient behavior
Smart speaker / wake-word behavior can come later, but should be designed as another client surface, not baked into core logic.

---

# 18. MVP “Wow” Moments to Optimize For

These are the most important target experiences.

## 18.1 Watch nudge
Watch buzzes:
“Leave now.”
[ Done ] [ Snooze ]

This is a key MVP moment.

## 18.2 Desktop voice morning command
User sits down and says:
“What do I need to do right now?”

Vel answers correctly and briefly.

This is another key MVP moment.

## 18.3 iPhone coherent context view
Phone shows:
- current context
- active nudges
- what matters next

This is the main iPhone MVP value.

These three should dominate early design decisions.

---

# 19. Client/Core Boundary

## 19.1 Vel core owns
- commitments
- current context
- risk
- nudges
- suggestions
- threads
- artifacts
- synthesis
- explanation data
- self-model

## 19.2 Clients own
- rendering
- local UI state
- notifications display
- voice capture / speech recognition
- text-to-speech playback
- local cache for responsiveness
- Apple-framework integrations for local signals
- action dispatch to Vel core

## 19.3 Important rule
Client apps must not duplicate policy logic or risk logic.

They may cache results, but the authoritative logic remains in Vel core.

---

# 20. API / Intent Boundary Suggestions

Even if the exact API is deferred, design clients assuming operations like:

- get current context
- get active nudges
- acknowledge nudge
- snooze nudge
- create capture
- ask for explanation
- list commitments
- fetch latest synthesis artifact

The voice layer should map speech to these intents, not bypass them.

---

# 21. Voice and Chat Continuity

Desktop voice and future iPhone chat should be treated as part of Vel’s assistant continuity system.

Voice/text sessions may produce:
- captures
- transcript records
- thread links
- feedback signals
- later synthesis inputs

However:

- immediate operational behavior remains deterministic
- conversational logs are valuable for reflective continuity, not immediate policy decisions

---

# 22. Explanation Model Across Clients

## 22.1 Watch
Minimal explanation only if needed, likely not in MVP.

## 22.2 iPhone
Primary mobile explanation surface.

## 22.3 Desktop voice
Can answer:
- why this nudge
- why now
- what changed
- what matters next

The explanation must come from structured Vel core data, not invented chat fluff.

---

# 23. Feedback Loop on Clients

Clients should support lightweight feedback collection.

Good first places:
- iPhone feedback buttons
- desktop voice follow-up
- later Watch mini feedback only if not annoying

Examples:
- “Was that helpful?”
- “Too early / too late / good timing”
- “How annoying was that?”

These feed Vel’s self-model.

---

# 24. Implementation Order for Client Work

Recommended order:

1. Apple/voice interface boundary spec
2. Watch nudge acknowledgement MVP
3. iPhone current context + active nudges MVP
4. desktop voice command/control MVP
5. desktop voice morning briefing
6. iPhone explanation surface
7. feedback capture
8. later richer conversation / reflection
9. later wake word / smart speaker mode

Do not start with full conversational agent design.

---

# 25. Testing Expectations

## 25.1 Shared action semantics
Verify that:
- Watch Done
- desktop voice “done”
- CLI `vel done ...`

all map to the same Vel core action.

## 25.2 Notification action tests
Verify:
- Done resolves correctly
- Snooze sets snoozed_until correctly
- severity maps to correct notification metadata

## 25.3 Voice intent tests
Test:
- command recognition maps to core intents
- ambiguous high-risk input requests clarification
- low-risk ambiguous input falls back to capture or confirm

## 25.4 Context rendering tests
Verify iPhone and desktop clients can render current context without inventing logic locally.

---

# 26. Open Questions for Later Dogfooding

These should remain explicitly open:

- wake word timing and privacy
- smart speaker mode
- richer watch initiation flows
- voice personality tuning
- Apple Health / Watch activity integration
- ambient sound / klaxon thresholds
- richer mobile thread/synthesis browsing

These are important, but not blockers for the first meaningful client layer.

---

# 27. Final Summary

Vel’s client ecology should work like this:

- **Watch**: interrupt + acknowledge
- **iPhone**: review + explain + feedback
- **desktop voice**: command + capture + morning orchestration
- **desktop CLI/TUI**: deep control + debugging

All of them should sit over one shared Vel core.

In short:

> the clients are different instruments, but they all play the same stateful assistant.