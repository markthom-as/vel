# Vel Daily Use

Vel is strongest when used as a repeated local loop, not as a one-shot assistant.

For the architecture proof that this workflow is backend-owned across CLI, web, and Apple, see [cross-surface proof flows](../cognitive-agent-architecture/architecture/cross-surface-proof-flows.md).

## Morning

Start by orienting:

```bash
cargo run -p vel-cli -- morning
cargo run -p vel-cli -- standup
cargo run -p vel-cli -- nudges
```

Use `vel morning` for the bounded morning overview loop and `vel standup` for the one-to-three commitment pass. `vel context` and `GET /v1/context/morning` still help as legacy context briefs, but they are no longer the primary authority for the repeated morning/standup workflow.

If you use the assistant entry from `Now`, `Threads`, or desktop/browser voice, the backend can now start or resume the same typed morning overview and standup sessions directly. Treat that as the same daily-loop authority as `vel morning` / `vel standup`, not as a separate chat-only planning flow.

If a `check_in` turns into a longer clarification or a `reflow` needs editing, Vel should escalate that work into `Threads` with durable backend-owned resolution history. Treat that thread as the continuity record for what was deferred, edited, resolved, or left pending.

If you are using the web shell and need the fastest path to the right help:

- `Now` is the primary surface for current-day orientation. It is action-first: the top overview shows one dominant action when the backend has enough evidence, one visible nudge, a compact timeline, and a `Why + state` disclosure lane.
- that “current day” is now sleep-relative rather than midnight-bound. If you are still awake and inside the same work session after midnight, unfinished commitments, relevant night events, and routine continuity may still belong to the same operator day until the rollover boundary is crossed.
- when there is no trustworthy dominant action, `Now` falls back to a bounded choice state instead of inventing local shell ranking. Expect one to three suggestions and a small explicit action vocabulary.
- the bounded inline commitment vocabulary is `accept / defer / choose / close`. If the work becomes multi-step, needs tool/context work, or needs multiple decisions, continue it in `Threads` instead of stretching the inline loop.
- `Now` should stay commitment-first after orientation: active or chosen commitments remain primary, pullable tasks stay visually secondary, and routine blocks do not pollute the upcoming-event slot.
- `next event` should mean the next future relevant calendar event only. Routine blocks, all-day noise, free/transparent holds, declined events, and cancelled events should not win that slot.
- freshness, sync, trust, review pressure, and debug posture live behind secondary controls instead of dominating the default `Now` view.
- `Now` may still surface inline `check_in` prompts, summary trust warnings, and same-day reflow proposals when the current plan is no longer trustworthy.
- when backend planning has enough signal, `Now` may also show the compact bounded day plan directly: what is scheduled, what was intentionally deferred, what did not fit, and which routine blocks shaped the result
- when durable routine blocks are configured, they now shape that compact day plan before inferred fallback logic runs; if no durable routine blocks exist yet, the shell may explicitly say that it is still using inferred fallback
- when reflow is active, `Now` now shows the compact remaining-day proposal directly: what moved, what no longer fits, and what still needs judgment.
- `Settings` is now the summary-first management surface for the durable routine/planning profile itself: inspect saved routine blocks, add or remove bounded planning constraints, then return to `Now` for the compact plan.
- CLI and Apple now inspect that same backend-owned planning profile too. Treat them as parity readers over one profile, not as separate planning systems.
- if assistant entry or Apple voice stages a routine/planning edit, treat that as a confirmation-first handoff into `Threads`, not as an already-saved profile change.
- thread continuity on `Now` should stay sparse. Queue pressure and follow-through can appear in the compact context lane, but `Now` should not grow a second thread inbox or local thread-ranking surface.
- `Threads` itself should read like a continuity surface, not a chat inbox: resume longer follow-through there, keep triage in `Inbox`, and keep `Now` focused on the current day.
- `Inbox` is the explicit decision queue. Clear or snooze items there before you open longer conversation history.
- the web sidebar should now behave like a thin rail, not a second information column. Use it to move between surfaces; ignore it once you are working.
- `Settings` is the deeper surface for Todoist, linking, writeback trust, Apple/local-source setup guidance, and summary recovery posture.
- the general Settings tab should read as a few clear buckets rather than one long mixed-purpose document: daily-use defaults, planning and recovery, devices and sync, then support and documentation.
- `Threads` is the conversation/history surface with lightweight filtering over persisted conversations, not the setup authority or main triage queue.
- if a shell card points you to setup or troubleshooting work, follow the matching guide in `docs/user/setup.md`, `docs/user/integrations/`, or `docs/user/troubleshooting.md` instead of guessing from stale UI state.
- current closeout limit: help is still guide-routed rather than driven by a dedicated contextual-help surface, and forward-browse schedule pagination remains outside the compact `Now` contract today.

What you are looking for:

- the current mode and morning state,
- what commitments are open,
- whether there are active nudges,
- whether Vel is asking for a check-in or recommending a reflow,
- whether recent source data looks fresh enough to trust.

## During the day

Capture quickly instead of trying to remember everything:

```bash
cargo run -p vel-cli -- capture "remember to send project update"
cargo run -p vel-cli -- capture "book dentist appointment" --type todo --source laptop
echo "snippet from terminal" | cargo run -p vel-cli -- capture -
```

On desktop web, hold the microphone button to use local browser speech-to-text. Vel keeps the transcript local until you send it, then submits it through the same assistant route as typed text with explicit voice provenance. `Now` and `Threads` share that backend-owned entry seam. The backend decides whether what you said belongs inline, in `Inbox`, or in `Threads`; the shell should follow that returned route instead of guessing capture-versus-conversation locally.

When Vel answers from recall, treat it as bounded local recall over persisted Vel data, not as ambient general memory. The backend now assembles a typed assistant-context pack with summary, focus lines, source breakdown, scores, and provenance so the same recall story can surface consistently across chat and assistant entry.

That assistant context can also surface canonical scheduler semantics from open commitments. Today that means bounded facets like `block:*`, duration, `time:*`, `cal:free`, `urgent`, and `defer` can show up consistently in recall or grounding without each shell re-parsing raw provider labels.

If local speech-to-text is unavailable in the current browser, the web shell should say so clearly and fall back to typed input instead of trying to emulate a second voice path.

Search or inspect when needed:

```bash
cargo run -p vel-cli -- search dentist
cargo run -p vel-cli -- recent --today
```

For coding-oriented project work, keep the execution handoff explicit and repo-local:

```bash
cargo run -p vel-cli -- agent inspect
cargo run -p vel-cli -- exec save <project_id> --objective "ship the next safe slice" --constraint "sidecar only"
cargo run -p vel-cli -- exec preview <project_id>
cargo run -p vel-cli -- exec export <project_id>
cargo run -p vel-cli -- exec review --state pending_review
cargo run -p vel-cli -- exec launch-preview <handoff_id>
cargo run -p vel-cli -- exec approve <handoff_id> --reason "scope and output contract look right"
```

The execution export writes only a small sidecar pack under the project's primary repo root, by default at `.planning/vel/`, so supervised GSD-readable handoffs stay inspectable and bounded instead of mutating arbitrary repo files.
The review queue keeps human-to-agent and agent-to-agent coding handoffs explicit: objective, read scope, write scope, routing reasons, and review gate are persisted before any supervised runtime launch can proceed.
Use `vel agent inspect` before supervised runs when you need to confirm what an agent can currently see, what it can review, and why a mutation lane is blocked. The CLI should report the same blocker language the API and Settings page show, including narrow escalation paths such as enabling writeback or approving a pending handoff.

If you use the Apple clients during the day:

- iPhone Voice morning briefing should start or resume the backend daily-loop session, not a client-side morning heuristic.
- Supported Apple voice turns should preserve shared thread continuity after transcript capture, even though the quick-loop reply still comes back through the Apple-specific typed route.
- Apple Watch quick loops should reflect backend `/v1/now` schedule state plus the bounded Apple behavior summary.
- When offline, treat Apple surfaces as cached-render + queued-safe-action shells. Wait for reconnect before trusting new schedule or explainability answers, and do not treat cached morning/standup state as permission to invent a new local standup.
- Apple voice and desktop/browser voice now teach one product rule: shells own permissions, push-to-talk, local STT/TTS, and offline presentation, while the backend owns routing, continuity, and daily-loop authority.
- Apple now also has the first additive iPhone embedded-capable seam for bounded local helper flows, but treat that as responsiveness/offline support only. It does not mean the phone became a second authority runtime.
- the Phase 38 local-first iPhone voice contract now makes the offline baseline explicit too: cached `Now`, queued voice capture, local quick actions, and local thread drafts belong to one bounded recovery lane rather than separate fallback tricks.
- on iPhone, expect that lane to surface as compact continuity rather than a second inbox: `Now` may show draft-or-recovery posture, and `Threads` may show the latest local voice recovery state until canonical follow-through or thread persistence takes over.
- the same rule now applies to bounded planning-profile edits: Apple voice and assistant entry can stage routine/constraint changes, but the backend keeps them explicit, thread-backed, and non-applied until a later approval/apply lane resolves them.
- for endpoint or local-source setup questions, use `docs/user/setup.md` first and then the Apple/local-source integration guides rather than inferring behavior from cached client state.
- current Apple setup limit: local-source path discovery/validation still depends on those setup guides and explicit operator review rather than a fully automatic Apple path-discovery flow.

## Commitments and review

Use commitments as the actionable layer:

```bash
cargo run -p vel-cli -- commitments
cargo run -p vel-cli -- commitment add "finish expense report"
cargo run -p vel-cli -- commitment done <id>
```

Use nudges as prompts to review state, not as unquestioned commands:

```bash
cargo run -p vel-cli -- nudges
cargo run -p vel-cli -- nudge done <id>
cargo run -p vel-cli -- nudge snooze <id> --minutes 10
```

## Sync and refresh

If you know a source changed, sync it and evaluate:

```bash
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- evaluate
```

On macOS, if local source snapshots are already in `~/Library/Application Support/Vel/...`, `veld` may bootstrap them automatically at startup. Manual sync is still useful when you want an immediate refresh.

If the day plan no longer looks trustworthy after a stale sync or missed event, treat that as a candidate `reflow`, not just a generic refresh problem.

Current shipped behavior:

- Vel can now do bounded same-day day shaping before drift, not just repair after drift
- the backend-owned idea of “today” now extends past midnight until the sleep-relative rollover boundary is crossed, so current-day ordering should not fragment late-night work into a fake new day
- the current day-plan output can explicitly show what was scheduled, deferred, did not fit, and still needs judgment
- the proposal also carries the routine blocks the backend used while shaping the day
- those routine blocks now come from durable operator-managed records when configured, with inferred fallback only when no durable blocks exist
- bounded planning constraints can now influence default time-window preference, calendar buffers, and overflow handling
- Vel can now do bounded same-day remaining-day recomputation from persisted commitments and calendar events
- those commitments carry canonical `scheduler_rules` persisted at ingest/update time instead of relying on raw-label parsing at reflow time
- the proposal can explicitly show work that moved, work that did not fit, and work that still needs judgment
- compact summary surfaces now also show same-day schedule proposal continuity: whether one bounded plan/reflow change is still pending review or what was last applied/failed
- the durable routine/planning profile can now be inspected and edited from the web `Settings` surface through typed backend-owned profile mutations rather than hidden generic settings fields
- longer disagreement or manual shaping still belongs in `Threads`
- this is not multi-day autonomous planning

## Backup and trust check

Before risky local changes, confirm backup trust:

```bash
cargo run -p vel-cli -- doctor
cargo run -p vel-cli -- backup --create
cargo run -p vel-cli -- backup --verify <backup_root>
```

Use `cargo run -p vel-cli -- backup --dry-run-restore <backup_root>` if you need to rehearse recovery without touching live state.

## End of day

Use the end-of-day and review flows:

```bash
cargo run -p vel-cli -- end-of-day
cargo run -p vel-cli -- review today
```

Assistant entry can now start that same closeout path inline. If you ask Vel to close out today or review today through `Now`, `Threads`, or voice entry, the backend runs the existing end-of-day context pipeline and returns a typed closeout summary instead of relying on shell-local heuristics.

The same rule applies to thread resolution during closeout and daily use: longer action follow-through belongs in `Threads`, but the backend remains the authority for why the work is there and what state it is in.

That rule also applies when the assistant proposes doing something on your behalf. Vel can stage a bounded proposal from chat or voice, but it still routes the work through the normal operator supervision lanes:

- simple confirmation stays bounded and can point you into a dedicated follow-through thread,
- planning-profile edits now follow the same pattern: assistant or Apple voice can stage a typed routine/constraint edit, but the result stays confirmation-first and thread-backed instead of silently changing your saved planning profile,
- once that proposal is reviewed and applied, `Now`, web `Settings`, CLI, and Apple summary surfaces all report the same backend-owned continuity about what is still pending versus what actually changed,
- repo-local or other supervised write work points at the existing execution review lane,
- SAFE MODE or trust blockers keep the proposal explicitly gated instead of silently mutating anything,
- once you resolve or dismiss the follow-through through the normal intervention/review path, the same proposal thread records whether the result became `applied`, `failed`, or `reversed`.

Current limit:

- assistant proposals still do not bypass review or writeback gates,
- proposal-thread reversal only records Vel-side proposal continuity unless the underlying write lane already has a real undo/reversal story,
- `Threads` is the place to inspect that continuity; it is not a second hidden execution system,
- recall quality is still bounded by the current local hybrid retrieval baseline and the persisted records Vel has actually ingested.

Good uses:

- see what was captured,
- orient in Now before you react to scattered context,
- triage Inbox from the explicit action queue instead of guessing what is urgent,
- respond to inline check-ins when Vel needs missing metadata or context repair,
- check pending writebacks, open conflicts, and people-linked review items from Now or Settings before trusting integration-backed edits,
- inspect what remained unresolved,
- notice response debt or drift,
- prepare a cleaner next start.

## Weekly

Use synthesis for a higher-level pass:

```bash
cargo run -p vel-cli -- review week
cargo run -p vel-cli -- synthesize week
cargo run -p vel-cli -- synthesize project vel
```

Use the weekly pass to review Projects weekly, not just raw capture volume. The JSON review output now includes typed project candidates so you can see which workspaces still have open commitments attached.
The review JSON also reports `pending_writebacks`, `open_conflicts`, and `people_needing_review` so the operator can see whether writeback or contact follow-up is waiting on manual supervision.

## Inspection and trust

When you need to know why Vel said something, inspect the underlying objects:

```bash
cargo run -p vel-cli -- runs --today
cargo run -p vel-cli -- run inspect <id>
cargo run -p vel-cli -- inspect artifact <id>
```

Vel is meant to be inspectable. If a conclusion matters, verify the source freshness and the related run or artifact instead of treating the surface output as magic.
That same rule applies to write lanes: SAFE MODE leaves writeback disabled by default, so the operator should explicitly enable it in Settings before expecting Todoist, GitHub, email, notes, or reminders mutations to apply.
If a proposal is blocked, treat that as expected fail-closed behavior: review the blocker, enable the required capability only if you actually want it, then confirm the staged work through the normal review surface instead of expecting chat to bypass it.
