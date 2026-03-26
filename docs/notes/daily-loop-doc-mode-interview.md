# Daily Loop Doc-Mode Interview Log

Working note only. This file is an interview log, not a canonical contract or shipped-behavior doc.

- **Started:** 2026-03-25
- **Owner:** User-guided v0 planning (morning/today/standup/overdue flow)
- **Mode:** Structured questions in chunks of 3

## Interview Chunk 1

1) For `v0`, are we targeting `docs/user/daily-use.md` as the behavioral source of truth, or another canonical doc path?
- Answer: _pending_

2) Do we treat `morning` + `standup` + `overdue` as a single mandatory session, or should each step be optional by policy?
- Answer: _pending_

3) Is `mood/pain/sleep` check-in required for every morning run, or optional when users want to skip?
- Answer: _pending_

## Interview Chunk 1 (answered)

1) For `v0`, are we targeting `docs/user/daily-use.md` as the behavioral source of truth, or another canonical doc path?
- Answer: daily use

2) Do we treat `morning` + `standup` + `overdue` as a single mandatory session, or should each step be optional by policy?
- Answer: morning is the wakeup/comms/mood etc, standup is the standard product planning and scheduling flow, and overdue is a component of that -- morning is a dep for standup, overdue is a dep for standup -- morning and standup should be separately configurable

3) Is `mood/pain/sleep` check-in required for every morning run, or optional when users want to skip?
- Answer: user should be able to skip anything (wholesale or bit by bit)

## Interview Chunk 2

4) Should `mood/pain/sleep` be freeform text, constrained scales, or both?
- Answer: _pending_

5) If constrained, which scales should we use (e.g., 1–10 mood, 1–10 pain, sleep quality, hours slept)?
- Answer: _pending_

6) What are the exact required fields you want persisted for check-in in the schema?
- Answer: _pending_

## Interview Chunk 2 (answered)

4) Should `mood/pain/sleep` be freeform text, constrained scales, or both?
- Answer: free text, if not answered specifically then follow up with specific questions. assumes backend scales too

5) If constrained, which scales should we use (e.g., 1–10 mood, 1–10 pain, sleep quality, hours slept)?
- Answer: mood: can be freeform "how are you feeling", specific anxiety/depression levels; pain: how body is feeling in general; could be freeform, but most things backed by numbers too

6) What are the exact required fields you want persisted for check-in in the schema?
- Answer: 
  - mood: freetext, -10 -- 10 scale, extracted keywords, confidence level
  - body: same
  - sleep: same
  - dream: same
  - if anything else suggests, please add

## Interview Chunk 3

7) Do you want to persist `mood/pain/sleep/dream` as one combined check-in object per session, or one record per item (one for each modality)?
- Answer: _pending_

8) Should numeric scale be mandatory for each item when provided, optional when user only gives free text?
- Answer: _pending_

9) Do we need a separate “answered_at”/local timestamp for each item (for trend graphs), or one session timestamp is enough?
- Answer: _pending_

## Interview Chunk 3 (answered)

7) Do you want to persist `mood/pain/sleep/dream` as one combined check-in object per session, or one record per item (one for each modality)?
- Answer: one record per item/type; check-in as own event by type

8) Should numeric scale be mandatory for each item when provided, optional when user only gives free text?
- Answer: yes required, but skippable (skips should be tracked in schema too)

9) Do we need a separate “answered_at”/local timestamp for each item (for trend graphs), or one session timestamp is enough?
- Answer: separate

8) Additional clarifying note:
- pain should be able to be updated throughout the day via events; if signals indicate pain, vel should infer and ask for confirmation/prompt for data

## Interview Chunk 4

10) Should each check-in item track `skipped` state explicitly (with skip reason), and should that be required for auditability?
- Answer: _pending_

11) Do we want to persist check-in events as durable, append-only rows (with follow-ups/update events), or overwrite the latest value per day?
- Answer: _pending_

12) For automatic prompts (e.g., pain detected during day), do they have lower priority than standup flow, or can they interrupt/branch the active session?
- Answer: _pending_

## Interview Chunk 4 (answered)

10) Should each check-in item track `skipped` state explicitly (with skip reason), and should that be required for auditability?
- Answer: yes; skip should support a reason. likely vel should surface options for this with optional text entry

11) Do we want to persist check-in events as durable, append-only rows (with follow-ups/update events), or overwrite the latest value per day?
- Answer: durable; there may be multiple per day

12) For automatic prompts (e.g., pain detected during day), do they have lower priority than standup flow, or can they interrupt/branch the active session?
- Answer: lower priority; but vel should have a way to launch check-in nudges

## Interview Chunk 5

13) Should check-in nudge prompts be user-invoked only, scheduled, or both (auto + user-triggered)?
- Answer: _pending_

14) For check-in nudges, should they have expiry/freshness limits (e.g., re-prompt after X hours) to avoid nagging?
- Answer: _pending_

15) If a check-in event is updated during the day, should this create a new event row, a linked correction event, or an edit-in-place versioned by `updated_at`?
- Answer: _pending_

## Interview Chunk 5 (answered)

13) Should check-in nudges be user-invoked only, scheduled, or both (auto + user-triggered)?
- Answer: both, with a vel-invoked option

14) For check-in nudges, should they have expiry/freshness limits (e.g., re-prompt after X hours) to avoid nagging?
- Answer: yes, but configurable

15) If a check-in event is updated during the day, should this create a new event row, a linked correction event, or an edit-in-place versioned by `updated_at`?
- Answer: always a new event

## Interview Chunk 6

16) For standup commitments, is there a required minimum (e.g., 1) or can users complete with zero?
- Answer: _pending_

17) If a user has no active commitments, should standup still generate a “no-commitments” summary and continue to overdue handling?
- Answer: _pending_

18) Overdue handling: can overdue actions be partially applied (mixed accept/reject in one action), or must each item be a separate decision?
- Answer: _pending_

## Interview Chunk 6 (answered)

16) For standup commitments, is there a required minimum (e.g., 1) or can users complete with zero?
- Answer: can complete with 0 or skip meeting

17) If a user has no active commitments, should standup still generate a “no-commitments” summary and continue to overdue handling?
- Answer: yeah

18) Overdue handling: can overdue actions be partially applied (mixed accept/reject in one action), or must each item be a separate decision?
- Answer: can be mixed

## Interview Chunk 7

19) For mixed overdue decisions, do you want one consolidated confirmation summary, or per-item confirmation for critical actions only?
- Answer: _pending_

20) What should `undo` rollback scope be: undo only the last applied batch, or all unapplied pending proposals from the same session?
- Answer: _pending_

21) Should overdue actions require an explicit audit reason even for auto-applied low-priority defaults?
- Answer: _pending_

## Interview Chunk 7 (answered)

19) For mixed overdue decisions, do you want one consolidated confirmation summary, or per-item confirmation for critical actions only?
- Answer: one time

20) What should `undo` rollback scope be: undo only the last applied batch, or all unapplied pending proposals from the same session?
- Answer: default to last applied batch

21) Should overdue actions require an explicit audit reason even for auto-applied low-priority defaults?
- Answer: no

## Interview Chunk 8

22) Should `confirm` be required before any overdue action applies, or can some actions auto-apply with only a final summary check?
- Answer: _pending_

23) For mixed overdue actions, is partial undo supported later (undo only one item in batch), or only batch-level undo?
- Answer: _pending_

24) Should standup/overdue completion events carry structured user intent metadata (e.g., confidence, rationale, context tags) for future analysis?
- Answer: _pending_

## Interview Chunk 8 (answered)

22) Should `confirm` be required before any overdue action applies, or can some actions auto-apply with only a final summary check?
- Answer: default to confirm, can be configured; vel may infer by confidence score later

23) For mixed overdue actions, is partial undo supported later (undo only one item in batch), or only batch-level undo?
- Answer: if single action single; if group action batch

24) Should standup/overdue completion events carry structured user intent metadata (e.g., confidence, rationale, context tags) for future analysis?
- Answer: yes

## Interview Chunk 9

25) Do you want completion metadata to include both system-inferred and user-affirmed confidence (separate fields)?
- Answer: _pending_

26) Should standup/overdue completion metadata include optional “next action” and “review horizon” fields?
- Answer: _pending_

27) For schema versioning, do you want explicit migration metadata (`schema_version`, `migration_source`) on check-in and workflow records?
- Answer: _pending_

## Interview Chunk 9 (answered)

25) Do you want completion metadata to include both system-inferred and user-affirmed confidence (separate fields)?
- Answer: yes - configurable

26) Should standup/overdue completion metadata include optional “next action” and “review horizon” fields?
- Answer: sure

27) For schema versioning, do you want explicit migration metadata (`schema_version`, `migration_source`) on check-in and workflow records?
- Answer: yes

## Interview Chunk 10

28) For session-level sync reset prevention, is the rule: prefer local session state on reconnect, prefer server authoritative state, or explicit merge with conflict prompts?
- Answer: _pending_

29) During conflict (server/session divergence), should users be prompted once or shown a conflict diff with per-field resolution?
- Answer: _pending_

30) What should happen if a sync conflict affects an in-flight overdue proposal (not yet applied)?
- Answer: _pending_

## Interview Chunk 10 (answered)

28) For session-level sync reset prevention, is the rule: prefer local session state on reconnect, prefer server authoritative state, or explicit merge with conflict prompts?
- Answer: merge on conflict

29) During conflict (server/session divergence), should users be prompted once or shown a conflict diff with per-field resolution?
- Answer: contextual -- thread view should be latter

30) What should happen if a sync conflict affects an in-flight overdue proposal (not yet applied)?
- Answer: wait for overdue complete, merge

## Interview Chunk 11

31) For conflict resolution UI, do you want automatic conflict tagging by field impact (high/medium/low), or manual triage only?
- Answer: _pending_

32) What is the required behavior when a conflict occurs during a long-running sync (e.g., mark stale fields vs retry merge and continue)?
- Answer: _pending_

33) Do you want a dedicated “conflict recovery” endpoint/state or just normal retry with deterministic replay?
- Answer: _pending_

## Interview Chunk 11 (answered)

31) For conflict resolution UI, do you want automatic conflict tagging by field impact (high/medium/low), or manual triage only?
- Answer: both

32) What is the required behavior when a conflict occurs during a long-running sync (e.g., mark stale fields vs retry merge and continue)?
- Answer: retry

33) Do you want a dedicated “conflict recovery” endpoint/state or just normal retry with deterministic replay?
- Answer: whatever you recommend

## Interview Chunk 12

34) For check-in nudge channels, do you want notifications on web, desktop, and mobile notifications, and do they share one schedule config?
- Answer: _pending_

35) Should a check-in nudge include a suggested default action (e.g., open pain capture) and optional snooze duration?
- Answer: _pending_

36) How should skip events be exposed in user-facing timeline/summaries (hidden, compact, or explicit with reason)?
- Answer: _pending_

## Interview Chunk 12 (answered)

34) For check-in nudge channels, do you want notifications on web, desktop, and mobile notifications, and do they share one schedule config?
- Answer: share

35) Should a check-in nudge include a suggested default action (e.g., open pain capture) and optional snooze duration?
- Answer: open thread capture (CLI or main threads). also snooze standard dur (configurable), skip option after 2x snoozes

36) How should skip events be exposed in user-facing timeline/summaries (hidden, compact, or explicit with reason)?
- Answer: compact

## Interview Chunk 13

37) What are default snooze durations (minutes/hours/day), and should they escalate after repeated skips?
- Answer: _pending_

38) If user skips after the final snooze, should the nudge auto-resolve as “skipped” with reason or become a low-priority background check-in?
- Answer: _pending_

39) Do nudges need a hard stop once a session (morning/standup) starts, or can they be queued with “resume later” behavior?
- Answer: _pending_

## Interview Chunk 13 (answered)

37) What are default snooze durations (minutes/hours/day), and should they escalate after repeated skips?
- Answer: 5, 10, 15, 30, 60 -- default to 5, configurable by nudge type, action type, etc. escalation should have sensible default exponential with time

38) If user skips after the final snooze, should the nudge auto-resolve as “skipped” with reason or become a low-priority background check-in?
- Answer: skip

39) Do nudges need a hard stop once a session (morning/standup) starts, or can they be queued with “resume later” behavior?
- Answer: queue

## Interview Chunk 14

40) Should snooze attempts and outcomes be stored in run events (auditable), and visible in timeline?
- Answer: _pending_

41) Do you want max queue depth for nudges (e.g., avoid more than N pending prompts across all types)?
- Answer: _pending_

42) If a queued nudge expires without response, should it auto-cancel or re-enter queue at lower priority?
- Answer: _pending_

## Interview Chunk 14 (answered)

40) Should snooze attempts and outcomes be stored in run events (auditable), and visible in timeline?
- Answer: yes

41) Do you want max queue depth for nudges (e.g., avoid more than N pending prompts across all types)?
- Answer: sure  --- make big for now

42) If a queued nudge expires without response, should it auto-cancel or re-enter queue at lower priority?
- Answer: nudge/action specific, auto expire with reason

## Interview Chunk 15

43) Do we need a first-class API/route for nudge queue inspection and manual dismissal from clients?
- Answer: _pending_

44) Should queued nudges be sorted by severity/urgency score or FIFO across same-priority streams?
- Answer: _pending_

45) Should manual skip on a nudge write a `skip_reason` enum to support later analytics and guardrail learning?
- Answer: _pending_

## Interview Chunk 15 (answered)

43) Do we need a first-class API/route for nudge queue inspection and manual dismissal from clients?
- Answer: yes, probably

44) Should queued nudges be sorted by severity/urgency score or FIFO across same-priority streams?
- Answer: mixed, logic tbd -- come up with a starting point

45) Should manual skip on a nudge write a `skip_reason` enum to support later analytics and guardrail learning?
- Answer: yes, may be empty

## Interview Chunk 16

46) What is the recommended starting point for nudge queue ordering: urgency score first, then scheduled time, then FIFO as tie-breaker?
- Answer: _pending_

47) For initial skip_reason enum, should we include fixed values (busy, not-now, irrelevant, duplicate, other) with optional detail text?
- Answer: _pending_

48) Should `nudge_queue` APIs support batch dismiss and batch snooze operations?
- Answer: _pending_

## Interview Chunk 16 (answered)

46) What is the recommended starting point for nudge queue ordering: urgency score first, then scheduled time, then FIFO as tie-breaker?
- Answer: sure

47) For initial skip_reason enum, should we include fixed values (busy, not-now, irrelevant, duplicate, other) with optional detail text?
- Answer: sure, must fit in expanded nudge, vel should rank top N options and show

48) Should `nudge_queue` APIs support batch dismiss and batch snooze operations?
- Answer: yes -- we will probably also need silent mode, vacation mode, break mode

## Interview Chunk 17

49) How should silent/vacation/break modes interact with nudges (hard suppress, lower-priority queueing, or mode-aware routing)?
- Answer: _pending_

50) Should mode-specific behavior be per nudge type (for example, pain nudges suppressed in break mode, scheduling nudges in vacation mode)?
- Answer: _pending_

51) Should transitions into/out of these modes emit nudge queue events for auditability?
- Answer: _pending_

## Interview Chunk 17 (answered)

49) How should silent/vacation/break modes interact with nudges (hard suppress, lower-priority queueing, or mode-aware routing)?
- Answer: all

50) Should mode-specific behavior be per nudge type (for example, pain nudges suppressed in break mode, scheduling nudges in vacation mode)?
- Answer: yes, probably

51) Should transitions into/out of these modes emit nudge queue events for auditability?
- Answer: the option should at least be there

## Interview Chunk 18

52) Should silent/vacation/break mode toggles be user-explicit only, or can auto-rules infer them (calendar, inactivity, location)?
- Answer: _pending_

53) In vacations, should check-ins be fully deferred, downgraded in priority, or still captured as background-only logs?
- Answer: _pending_

54) Do you want a visible “nudge availability” status in UI (current suppression modes + queue pressure) to help users debug missing prompts?
- Answer: _pending_

## Interview Chunk 18 (answered)

52) Should silent/vacation/break mode toggles be user-explicit only, or can auto-rules infer them (calendar, inactivity, location)?
- Answer: infer and confirm

53) In vacations, should check-ins be fully deferred, downgraded in priority, or still captured as background-only logs?
- Answer: configurable, default to off

54) Do you want a visible “nudge availability” status in UI (current suppression modes + queue pressure) to help users debug missing prompts?
- Answer: something like that, do not expose to front end yet

## Interview Chunk 19

55) Do auto-infer mode toggles require an explicit “confirm this inferences” step or can they auto-apply with later correction from the user?
- Answer: _pending_

56) Should silent/vacation/break be mutually exclusive states or orthogonal flags?
- Answer: _pending_

57) Should inferred mode transitions generate `run` events, and if yes, include rationale and confidence?
- Answer: _pending_

## Interview Chunk 19 (answered)

55) Do auto-infer mode toggles require an explicit “confirm this inferences” step or can they auto-apply with later correction from the user?
- Answer: both modes should exist, default to user confirmation

56) Should silent/vacation/break be mutually exclusive states or orthogonal flags?
- Answer: orthogonal methinks

57) Should inferred mode transitions generate run events, and if yes, include rationale and confidence?
- Answer: in system view / dev view / system thread yes

## Interview Chunk 20

58) Should mode transitions include source labels (calendar, user action, inference rule) in persisted metadata?
- Answer: _pending_

59) Do you want a “cooldown” after entering a mode before nudges can re-fire for that mode?
- Answer: _pending_

60) Should inferred mode toggles block only nudge generation or also active session execution (morning/standup)?
- Answer: _pending_

## Interview Chunk 20 (answered)

58) Should mode transitions include source labels (calendar, user action, inference rule) in persisted metadata?
- Answer: yes

59) Do you want a “cooldown” after entering a mode before nudges can re-fire for that mode?
- Answer: yes -- pick sensible defaults

60) Should inferred mode toggles block only nudge generation or also active session execution (morning/standup)?
- Answer: both, configurable

## Interview Chunk 21

61) Do we need separate cooldown policies for each suppression mode, or one shared global cooldown policy?
- Answer: _pending_

62) When both user and inferred mode flags are active for the same suppression type, should user preference always win?
- Answer: _pending_

63) Should mode transitions be visible in a run-thread only, or also in daily session summary metadata?
- Answer: _pending_

## Interview Chunk 21 (answered)

61) Do we need separate cooldown policies for each suppression mode, or one shared global cooldown policy?
- Answer: shared to start

62) When both user and inferred mode flags are active for the same suppression type, should user preference always win?
- Answer: tie break with confidence score

63) Should mode transitions be visible in a run-thread only, or also in daily session summary metadata?
- Answer: also in session summary in natural language

## Interview Chunk 22

64) Is mode-state persisted as part of daily session, or in a separate policy/state artifact?
- Answer: _pending_

65) Should suppression modes be represented as explicit “state snapshot” records with effective/expiry timestamps?
- Answer: _pending_

66) Should silent/vacation/break mode toggles be user-exportable and searchable in audit/export artifacts?
- Answer: _pending_

## Interview Chunk 22 (answered)

64) Is mode-state persisted as part of daily session, or in a separate policy/state artifact?
- Answer: whatever you recommend for AI/ML workflows

65) Should suppression modes be represented as explicit “state snapshot” records with effective/expiry timestamps?
- Answer: probably, with causes/sources and confidence

66) Should silent/vacation/break mode toggles be user-exportable and searchable in audit/export artifacts?
- Answer: yes

## Interview Chunk 23

67) Do you want a lightweight model for AI/ML readiness (e.g., check-in event schema with feature flags and confidence calibration) in v0 or defer until v1?
- Answer: _pending_

68) Should per-event confidence be persisted as raw model score or calibrated bucket (high/medium/low) too?
- Answer: _pending_

69) Should we gate new features behind explicit config flags until confidence and safety baselines are in place?
- Answer: _pending_

## Interview Chunk 23 (answered)

67) Do you want a lightweight model for AI/ML readiness (e.g., check-in event schema with feature flags and confidence calibration) in v0 or defer until v1?
- Answer: v1

68) Should per-event confidence be persisted as raw model score or calibrated bucket (high/medium/low) too?
- Answer: my hunch is both, whatever you recommend

69) Should we gate new features behind explicit config flags until confidence and safety baselines are in place?
- Answer: sure

## Interview Chunk 24

70) For feature flags, do you prefer static config, dynamic remote toggle, or both (with local override)?
- Answer: _pending_

71) Should feature flags be user-scoped and synced, or device-local with manual propagation?
- Answer: _pending_

72) Do we need a “deprecation policy” for stale flag values and mode configs in v0?
- Answer: _pending_

## Interview Chunk 24 (answered)

70) For feature flags, do you prefer static config, dynamic remote toggle, or both (with local override)?
- Answer: both, should sync across nodes

71) Should feature flags be user-scoped and synced, or device-local with manual propagation?
- Answer: synced

72) Do we need a “deprecation policy” for stale flag values and mode configs in v0?
- Answer: v1

## Interview Chunk 25

73) Should deprecation metadata include sunset date + migration path, or simple tombstoning in v0?
- Answer: _pending_

74) For merged mode state and check-in streams, should we expose a canonical API schema version in each payload?
- Answer: _pending_

75) Do we need backwards-compatible read decoding for old payload versions during v0 rollout?
- Answer: _pending_

## Interview Chunk 25 (answered)

73) Should deprecation metadata include sunset date + migration path, or simple tombstoning in v0?
- Answer: simple for v0

74) For merged mode state and check-in streams, should we expose a canonical API schema version in each payload?
- Answer: sure

75) Do we need backwards-compatible read decoding for old payload versions during v0 rollout?
- Answer: no

## Interview Chunk 26

76) Should we set a strict API payload version required on all write endpoints (`daily_loop`/`nudge`/`overdue`) for v0?
- Answer: _pending_

77) For schema evolution, should new optional fields default silently or be rejected when required by schema version?
- Answer: _pending_

78) Do we want strict JSON Schema validation in Rust layer or permissive with warnings in v0?
- Answer: _pending_

## Interview Chunk 26 (answered)

76) Should we set a strict API payload version required on all write endpoints (`daily_loop`/`nudge`/`overdue`) for v0?
- Answer: no

77) For schema evolution, should new optional fields default silently or be rejected when required by schema version?
- Answer: whatever you recommend

78) Do we want strict JSON Schema validation in Rust layer or permissive with warnings in v0?
- Answer: whatever you recommend

## Interview Chunk 27

79) Do you want one canonical event bus for all daily-loop/mode/nudge/overdue state changes, or domain-specific streams?
- Answer: _pending_

80) Should event ordering be fully linearized per user session, or per-subsystem with eventual merge?
- Answer: _pending_

81) What retention window do you want for diagnostic run events in v0 (days/weeks/months)?
- Answer: _pending_

## Interview Chunk 27 (answered)

79) Do you want one canonical event bus for all daily-loop/mode/nudge/overdue state changes, or domain-specific streams?
- Answer: whatever you suggest -- want vel to be very event driven, so maybe canonical? idk

80) Should event ordering be fully linearized per user session, or per-subsystem with eventual merge?
- Answer: whatever you think

81) What retention window do you want for diagnostic run events in v0 (days/weeks/months)?
- Answer: multiple weeks for now

## Interview Chunk 28

82) Should run event retention be user-configurable with a hard minimum/maximum, or fixed in v0?
- Answer: _pending_

83) For long-tail storage, should run events be archived/hot-cold tiered or fully retained in sqlite/db?
- Answer: _pending_

84) If run events are pruned, should summaries/digests be preserved for audit and trend checks?
- Answer: _pending_

## Interview Chunk 28 (answered)

82) Should run event retention be user-configurable with a hard minimum/maximum, or fixed in v0?
- Answer: dev configurable

83) For long-tail storage, should run events be archived/hot-cold tiered or fully retained in sqlite/db?
- Answer: probably parquet for cold storage on NAS, S3, Google Drive backup targets

84) If run events are pruned, should summaries/digests be preserved for audit and trend checks?
- Answer: yes

## Interview Chunk 29

85) For backup exports (NAS/S3/Drive), should event export include raw events only, or events + resolved projections + checksums?
- Answer: _pending_

86) Should backup export be user-initiated only, scheduled, or both?
- Answer: _pending_

87) Do backup exports need encryption-at-rest/encryption-in-transit guarantees before writing to cold storage?
- Answer: _pending_

## Interview Chunk 29 (answered)

85) For backup exports (NAS/S3/Drive), should event export include raw events only, or events + resolved projections + checksums?
- Answer: all

86) Should backup export be user-initiated only, scheduled, or both?
- Answer: both

87) Do backup exports need encryption-at-rest/encryption-in-transit guarantees before writing to cold storage?
- Answer: should be optional

## Interview Chunk 30

88) Should encryption defaults to off, on, or strict-on-by-default for user data in v0?
- Answer: _pending_

89) For export chunking, should backups be per day, per session, or rolling size windows?
- Answer: _pending_

90) Should cold storage exports be signed/integrity-verified and alert on corruption?
- Answer: _pending_

## Interview Chunk 30 (answered)

88) Should encryption defaults to off, on, or strict-on-by-default for user data in v0?
- Answer: off

89) For export chunking, should backups be per day, per session, or rolling size windows?
- Answer: whatever you rec

90) Should cold storage exports be signed/integrity-verified and alert on corruption?
- Answer: yes, but configurable, if really heavy, default to off

## Interview Chunk 31

91) For integrity verification, do you want defaults of `sha256` hash manifests per export with optional Merkle/ed25519 signing?
- Answer: _pending_

92) For optional encryption, should it be policy-driven with per-backend overrides (e.g., encrypt NAS differently from S3)?
- Answer: _pending_

93) Should cold-storage backup scheduling be event-driven only, fixed cadence, or both (with jitter and backoff)?
- Answer: _pending_

## Interview Chunk 31 (answered)

91) For integrity verification, do you want defaults of `sha256` hash manifests per export with optional Merkle/ed25519 signing?
- Answer: whatever you rec

92) For optional encryption, should it be policy-driven with per-backend overrides (e.g., encrypt NAS differently from S3)?
- Answer: yes

93) Should cold-storage backup scheduling be event-driven only, fixed cadence, or both (with jitter and backoff)?
- Answer: both

## Interview Chunk 32

94) For scheduled backups, what default cadence (hours/days) feels right for v0?
- Answer: _pending_

95) For event-driven backups, what should be the trigger threshold (new events/minute or queue size)?
- Answer: _pending_

96) Should backup jobs run as foreground blocking operations or background jobs with UI progress indicators?
- Answer: _pending_

## Interview Chunk 32 (answered)

94) For scheduled backups, what default cadence (hours/days) feels right for v0?
- Answer: sensible defaults

95) For event-driven backups, what should be the trigger threshold (new events/minute or queue size)?
- Answer: sensible defaults

96) Should backup jobs run as foreground blocking operations or background jobs with UI progress indicators?
- Answer: sensible defaults

## Interview Chunk 33

97) Should v0 include a compact recovery dashboard in CLI for backup/export status and queue health?
- Answer: both (compact CLI surface + optional thread-surface summary)

98) Should failed backup jobs auto-retry with exponential backoff?
- Answer: yes

99) Should unrecoverable backup failures block other system operations or degrade gracefully to local-only mode?
- Answer: both: default to graceful local-only, allow explicit safety-blocking mode

## Interview Chunk 33 (answered)

97) Should v0 include a compact recovery dashboard in CLI for backup/export status and queue health?
- Answer: both (compact CLI surface + optional thread-surface summary)

98) Should failed backup jobs auto-retry with exponential backoff?
- Answer: yes

99) Should unrecoverable backup failures block other system operations or degrade gracefully to local-only mode?
- Answer: both: default to graceful local-only, allow explicit safety-blocking mode

## Interview Chunk 32 (refinement)

94) For scheduled backups, what default cadence (hours/days) feels right for v0?
- Answer: Recommend 24 hours at a local off-peak time, with a [3..168] hour configurable range

95) For event-driven backups, what should be the trigger threshold (new events/minute or queue size)?
- Answer: Recommend immediate flush at >= 500 new events OR >= 5 minutes of accumulated unsynced events, whichever comes first

96) Should backup jobs run as foreground blocking operations or background jobs with UI progress indicators?
- Answer: Recommend background jobs with lightweight status + optional explicit foreground mode for manual user export

## Interview Chunk 33

97) Should v0 include a compact recovery dashboard in CLI for backup/export status and queue health?
- Answer: _pending_

98) Should failed backup jobs auto-retry with exponential backoff?
- Answer: _pending_

99) Should unrecoverable backup failures block other system operations or degrade gracefully to local-only mode?
- Answer: _pending_
