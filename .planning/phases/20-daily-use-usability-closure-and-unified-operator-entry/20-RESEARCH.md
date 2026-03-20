# Phase 20: Grounded Assistant Entry And Daily-Use Usability Closure - Research

**Researched:** 2026-03-19
**Domain:** Rust-owned grounded assistant entry, daily-use operator UX, and thread continuity
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
## Purpose

Phase 20 exists because the current product substrate is broad enough to be impressive, but still not usable enough for repeated daily life. The operator explicitly chose to defer more milestone-closeout bookkeeping and prioritize usability work that makes Vel something they can actually live in.

That usability work is now explicitly assistant-centric: the user wants Vel's LLM to be aware of real Vel data, to use Vel as tools, to support morning/daily/closeout and thread-based resolution, and to let voice flow into the same assistant path. The immediate phase should therefore turn the grounded Rust-owned assistant into the default operator entry instead of treating it as an isolated chat feature.

## Product Direction

The core shape is already decided:

- `Now` is the compact urgent-first orientation surface
- `Inbox` is the explicit triage queue
- `Threads` is continuity, archive, and deeper interaction
- `Projects` is secondary context, not the primary work surface
- the backend owns policy, action semantics, `check_in`, `reflow`, trust/readiness, and thread escalation

Phase 20 should improve how that shipped model feels in use.

## Expected Focus

This phase should likely focus on:

1. `Now` usability
   - better contextual urgency handling
   - cleaner compact context strip / top area
   - stronger actionable cards when something really needs attention

2. Grounded assistant entry
   - one clearer path for capture, conversation, and quick operator intent over the grounded assistant seam
   - avoid separate fragmented “chat vs capture vs command” affordances where possible
   - keep the assistant bounded to real Vel data and explicit tool surfaces rather than generic freeform prompting
   - preserve optional localhost `openai_oauth` routing without making remote models a dependency for core use

3. `Inbox` / `Threads` usability
   - clearer triage versus continuity boundary in practice
   - reduced friction getting from summary state into the right deeper interaction
   - thread continuity for assistant work without inventing a separate assistant archive

4. Settings/setup friction reduction
   - make the default experience less settings-heavy
   - push more advanced/runtime complexity out of the operator’s immediate path

### Claude's Discretion
No explicit `## Claude's Discretion` section appears in `20-CONTEXT.md`.

### Deferred Ideas (OUT OF SCOPE)
## Non-Goals

- broad new provider/platform expansion
- another architecture-definition phase
- milestone archival bookkeeping
- re-opening the already-decided product taxonomy unless real usage proves it wrong
- Apple voice parity, desktop push-to-talk parity, and assistant-capable daily loop/closeout closure beyond what is needed to establish the Phase 20 grounded entry seam
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| USABLE-01 | The grounded assistant becomes a practical default operator entry instead of a side chat surface | Use one backend-owned assistant entry seam, make `Now` the primary launch point, and preserve thread continuity through existing conversation records |
| USABLE-02 | Daily-use friction across `Now`, `Inbox`, `Threads`, and setup is reduced enough for repeated operator use | Tighten summary-first routing, reduce settings-first recovery, and keep triage/history/search distinct |
| NOW-UX-01 | `Now` better balances urgent inline actions with subtle links into deeper surfaces | Keep `Now` compact, show only highest-pressure cards inline, and use typed thread/inbox routing hints for the rest |
| INBOX-UX-01 | `Inbox` better supports explicit triage over the shared action model without turning into a second thread archive | Keep Inbox backed by intervention/action items, not conversation history |
| THREADS-UX-01 | `Threads` better support continuity, search, and assistant escalation over real product state | Reuse conversations plus filtered-thread routing, add retrieval/search affordances over existing persisted state |
| ENTRY-01 | Capture, text conversation, and operator intent entry converge on one clearer assistant-first path | Route typed input, capture intent, and assistant requests through the same backend entry contract |
| SETTINGS-UX-01 | Default setup friction drops without expanding advanced/runtime complexity | Keep summary-first settings payloads and route setup from `Now`/Settings summaries instead of exposing more runtime detail |
| ASSIST-01 | One backend-owned grounded assistant seam powers the default text/capture entry path over real Vel data and bounded tools | Extend existing Rust service seam (`messages.rs` + `assistant.rs` + `tools.rs`) rather than adding client-owned assistant logic |
| ASSIST-02 | Configured remote LLM routing, including localhost `openai_oauth`, remains optional, bounded, and compatible with the local-first core | Keep router/profile gating in `crates/veld/src/llm.rs`; fail safe when no provider is configured |
| THREADS-02 | Assistant continuity and escalation preserve thread ownership and product boundaries instead of inventing a separate assistant archive model | Reuse conversation/thread records, existing intervention links, and typed thread-route hints |
</phase_requirements>

## Summary

Phase 20 should be planned as convergence work, not as a new assistant subsystem. The backend already has a real grounded assistant seam: user messages persist through the conversation model, assistant replies are generated in Rust, the prompt is grounded from `agent_grounding`, and tool access is explicitly read-only over `Now`, semantic memory, projects, people, commitments, daily-loop state, and filtered threads. The planning problem is that this seam still behaves like a thread-local chat feature, while the product docs define the assistant as the default operator entry.

The existing product taxonomy is unusually clear. `Now` is the urgent-first summary and routing surface, `Inbox` is the explicit triage queue, `Threads` is continuity/history/search, and `Settings` is advanced setup and trust disclosure. The phase should therefore improve how operators move across those surfaces, not redefine them. The biggest design risk is letting the shell invent routing logic, capture heuristics, or fallback behavior that belongs in Rust services.

Current evidence also shows the main execution seams are already testable. Backend chat-tool tests and the chat grounding integration test are green, and the web has focused tests for `Now`, `Inbox`, `ThreadView`, `MessageComposer`, and `SettingsPage`. One targeted `ThreadView` test is currently failing because the placeholder assertion no longer matches the composer copy, which is a useful warning: this phase will need deliberate UI-contract stabilization as the entry path converges.

**Primary recommendation:** Plan Phase 20 around one Rust-owned assistant entry contract that can be launched from `Now`, land in existing conversations/threads, and route setup/triage/history through the existing surface taxonomy instead of widening shell-owned behavior.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `veld` service seam (`assistant.rs`, `messages.rs`, `tools.rs`) | workspace `0.1.0` | Persist user messages, invoke grounded assistant replies, expose bounded Vel tools | Already owns assistant behavior in Rust and preserves shell-thin boundaries |
| `vel-llm` router | workspace `0.1.0` | Model-profile routing, tool-call support, fallback behavior | Keeps provider logic out of web/Apple shells and already gates `openai_oauth` safely |
| `axum` | `0.7` | HTTP/API boundary for conversations, inbox, settings, and thread reads | Existing backend transport layer for all assistant-adjacent surfaces |
| `sqlx` + `vel-storage` | `0.8` + workspace `0.1.0` | Durable conversations, messages, threads, settings, and intervention state | Maintains canonical persisted continuity and product evidence |
| `react` / `react-dom` | `19.2.4` | Web shell embodiment for `Now`, `Inbox`, `Threads`, and `Settings` | Existing shell stack; phase should reuse rather than replace |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `vite` | `8.0.0` | Web build/dev server | Existing web package baseline |
| `vitest` | `2.1.8` | Focused component/data tests | For UI and query-cache behavior around entry convergence |
| `@testing-library/react` | `16.1.0` | Interaction-level web tests | For composer, surface routing, and summary-first behavior |
| `tailwindcss` | `4.2.1` | Existing web styling system | Keep visual changes inside the established shell language |
| `tokio` | `1.44` | Async backend execution | Supports assistant replies, tool calls, and event broadcast flow |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extending the current Rust assistant seam | New client-owned assistant controller | Faster to prototype, but violates shell-thin boundaries and duplicates routing/policy logic |
| Reusing existing conversations/threads | Separate assistant archive surface | Creates continuity drift and breaks `THREADS-02` |
| Optional local/localhost provider routing | Mandatory remote provider | Conflicts with local-first core and Phase 20 scope |
| Existing intervention/action queue for triage | Thread list as de facto inbox | Collapses `Inbox` and `Threads` back together |

**Installation:**
```bash
cargo test -p veld chat::tools -- --nocapture
npm test -- src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/ThreadView.test.tsx src/components/MessageComposer.test.tsx
```

**Version verification:** Versions above come from checked-in workspace manifests (`Cargo.toml`, `clients/web/package.json`). Registry publish dates were not verified because this phase does not require introducing new third-party packages.

## Architecture Patterns

### Recommended Project Structure
```text
crates/veld/src/routes/chat.rs          # Thin HTTP layer for conversations/messages/inbox/settings
crates/veld/src/services/chat/          # Assistant, message, conversation, read, and settings logic
crates/veld/src/llm.rs                  # Provider/profile bootstrapping and fallback gating
clients/web/src/components/NowView.tsx  # Summary-first launch and routing surface
clients/web/src/components/InboxView.tsx
clients/web/src/components/ThreadView.tsx
clients/web/src/components/MessageComposer.tsx
clients/web/src/data/chat.ts            # Thin transport/query helpers
```

### Pattern 1: Backend-Owned Assistant Entry
**What:** Persist the operator input first, then let Rust decide whether and how to call the grounded assistant.
**When to use:** Any new “assistant-first” entry path for text or capture-like intent.
**Example:**
```rust
// Source: crates/veld/src/services/chat/messages.rs
let user_message = create_user_message(state, conversation_id, payload).await?;
let (assistant_message, assistant_error) = if let (Some(router), Some(profile_id)) =
    (state.llm_router.as_ref(), state.chat_profile_id.as_ref())
{
    match generate_assistant_reply(
        state,
        conversation_id,
        profile_id,
        state.chat_fallback_profile_id.as_deref(),
        router,
    ).await {
        Ok(Some(assistant_message)) => (Some(assistant_message), None),
        Ok(None) => (None, None),
        Err(error) => (None, Some(error.to_string())),
    }
} else {
    (None, Some(chat_model_not_configured_error()))
};
```

### Pattern 2: Read-Only Tool Grounding Over Real Vel State
**What:** Give the assistant a bounded tool surface for current product state instead of raw prompt stuffing.
**When to use:** Questions about `Now`, memory, commitments, people, daily-loop status, or threads.
**Example:**
```rust
// Source: crates/veld/src/services/chat/tools.rs
ToolSpec {
    name: "vel_get_now".to_string(),
    description: "Read Vel's current operator-facing Now summary...".to_string(),
    schema: json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false,
    }),
}
```

### Pattern 3: Typed Surface Routing Instead Of Client Heuristics
**What:** `Now` summarizes and routes; `Inbox` triages; `Threads` carries continuity. Shells should consume typed thread-route hints or conversation IDs instead of inventing navigation rules.
**When to use:** Any cross-surface link from `Now`/`Inbox` into `Threads`.
**Example:**
```rust
// Source: crates/veld/src/services/projects.rs
ActionThreadRoute {
    target: ActionThreadRouteTarget::FilteredThreads,
    label,
    thread_id: None,
    thread_type: Some(thread_type.to_string()),
    project_id: Some(project.id.clone()),
}
```

### Pattern 4: Summary-First Settings, Not Runtime-First Recovery
**What:** Put trust/setup summaries in `Settings` and `Now`, with deeper runtime detail one step away.
**When to use:** Setup-friction reduction and onboarding/trust routing work in this phase.
**Example:**
```rust
// Source: crates/veld/src/services/chat/settings.rs
let runtime_config =
    crate::services::operator_settings::runtime_sync_config(&state.storage, &state.config)
        .await?;
map.insert("node_display_name".to_string(), serde_json::to_value(runtime_config.node_display_name)?);
map.insert("writeback_enabled".to_string(), serde_json::json!(runtime_config.writeback_enabled));
map.insert("backup".to_string(), serde_json::to_value(BackupSettingsData { ... })?);
```

### Anti-Patterns to Avoid
- **Client-owned assistant routing:** Do not let React decide capture-vs-chat-vs-command semantics independently of Rust services.
- **Second assistant archive:** Do not create a parallel assistant history model outside existing conversations/threads.
- **Inbox-as-history:** Do not stuff continuity/search behavior into `Inbox`; it is the action queue.
- **Remote-model dependency creep:** Do not make `openai_oauth` or any remote provider required for core daily use.
- **Settings-first daily loop:** Do not send operators into runtime/integration detail before showing summary-level guidance in `Now` or the general Settings tab.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Assistant grounding | Ad hoc prompt concatenation in the web shell | `build_chat_grounding_prompt` + `chat_tool_specs` | Existing seam already preserves explainability and bounded access |
| Entry routing | Separate web-only capture/chat/command router | One backend-owned assistant entry contract over conversation/message services | Prevents cross-shell drift |
| Thread escalation | Hard-coded client URL logic | Existing conversation IDs and typed `thread_route` hints | Keeps ownership and project scope in backend data |
| Triage state | Local UI queues derived from message history | `/api/inbox` intervention/action items | Existing queue already models explicit triage |
| Setup/trust summaries | New bespoke onboarding blob | Existing settings payload + docs-backed summary-first flows | Avoids another authority surface |
| Search/history | Client-side filtering over loaded conversations only | Existing semantic search + thread listing filters | Persisted retrieval already exists and is explainable |

**Key insight:** Phase 20 is mostly about connecting existing bounded seams into one default flow. Custom shell-side glue will feel faster during implementation and make Phase 21/22 materially harder.

## Common Pitfalls

### Pitfall 1: Treating The Assistant As A Thread-Only Feature
**What goes wrong:** The composer remains buried in `Threads`, so `Now` and capture still feel like separate product paths.
**Why it happens:** The current grounded seam is mounted on conversation messages, and the web already embodies that as the thread composer.
**How to avoid:** Plan a backend entry contract that can be launched from `Now` and resolve into a conversation/thread when needed.
**Warning signs:** Users must navigate to `Threads` before using the assistant for everyday capture or intent entry.

### Pitfall 2: Re-Collapsing `Now`, `Inbox`, And `Threads`
**What goes wrong:** `Now` becomes a second inbox, or `Threads` starts carrying active triage pressure.
**Why it happens:** Surface convergence work often drifts toward “put everything near the composer.”
**How to avoid:** Keep `Now` to compact pressure + routing, `Inbox` to explicit decisions, and `Threads` to continuity/search.
**Warning signs:** New UI adds unresolved queue items to thread lists or large history blocks to `Now`.

### Pitfall 3: Making Optional LLM Routing Feel Mandatory
**What goes wrong:** The default experience degrades when no model is configured, or setup flows imply remote inference is required.
**Why it happens:** Assistant-centric work can accidentally assume a configured model path.
**How to avoid:** Preserve the current safe fallback behavior and plan a useful non-configured state for entry surfaces.
**Warning signs:** Phase tasks require `openai_oauth` or a remote profile to validate basic entry UX.

### Pitfall 4: UI Contract Drift During Entry Convergence
**What goes wrong:** Small copy/layout changes silently break targeted web tests and hide true behavior regressions.
**Why it happens:** This phase touches shared surfaces with existing focused tests and optimistic-update behavior.
**How to avoid:** Stabilize the entry-copy contract early and update tests intentionally.
**Warning signs:** Existing targeted tests fail for placeholder/label mismatches rather than logic regressions.

## Code Examples

Verified patterns from repo authority:

### Bounded Tool Rounds For Assistant Replies
```rust
// Source: crates/veld/src/services/chat/assistant.rs
for round in 0..=MAX_CHAT_TOOL_ROUNDS {
    let req = LlmRequest {
        system: system.to_string(),
        messages: messages.clone(),
        tools: if tools_enabled { tools.to_vec() } else { Vec::new() },
        response_format: vel_llm::ResponseFormat::Text,
        temperature: 0.2,
        max_output_tokens: 2048,
        model_profile: profile_id.to_string(),
        metadata: serde_json::json!({
            "conversation_id": conversation_id,
            "assistant_surface": "chat",
            "tools_enabled": tools_enabled,
            "tool_round": round,
        }),
    };
```

### Local Voice Transcript Reusing The Typed Message Path
```tsx
// Source: clients/web/src/components/MessageComposer.tsx
const res = await apiPost<ApiResponse<CreateMessageResponse>>(
  `/api/conversations/${conversationId}/messages`,
  { role: 'user', kind: 'text', content: { text: trimmed } },
  (value) => decodeApiResponse(value, decodeCreateMessageResponse),
);
```

### `Now` Carrying Summary Counts Plus Deeper Routing Hints
```tsx
// Source: clients/web/src/components/NowView.tsx
const threadAttentionCount = actionItems.filter((item) => item.thread_route !== null).length
  + (data.reflow_status?.thread_id ? 1 : 0);
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Side-chat mental model in `Threads` | Rust-owned grounded assistant over persisted conversations/messages and bounded tools | Phase 11 shipped before 2026-03-20 | The assistant is already real; Phase 20 should promote and unify it |
| Shell-specific surface sprawl | Phase 14-17 taxonomy: `Now` summary, `Inbox` triage, `Threads` continuity, `Settings` advanced | 2026-03-19 to 2026-03-20 planning/implementation state | Planning should preserve these boundaries |
| Provider-specific assistant assumptions | Optional routed profiles with local `llama_cpp` and localhost-only `openai_oauth` | Present in `crates/veld/src/llm.rs` and `docs/user/setup.md` as of 2026-03-19 | Local-first remains intact if the phase does not widen it |
| Voice-specific product logic in Apple/web | Web typed input already shares the chat message path; Apple voice still uses specialized intent handling | Web path documented by 2026-03-19; Apple parity deferred to Phase 21 | Phase 20 should avoid solving full voice parity prematurely |

**Deprecated/outdated:**
- Treating `Threads` as the only place the assistant can live: outdated for product planning because the roadmap now defines assistant-first entry as a core usability requirement.
- Using setup/runtime surfaces as first-contact daily-use explanations: outdated relative to the summary-first taxonomy in Phase 17 docs and current Settings copy.

## Open Questions

1. **What should the first unified entry surface actually create by default?**
   - What we know: the primary entry is supposed to live in `Now`, and resulting artifacts should land in the surface that owns them.
   - What's unclear: whether a new input should always create a conversation immediately or only after the backend classifies it as thread-worthy.
   - Recommendation: plan an explicit backend entry response contract that can return `capture_only`, `reply_inline`, or `open_conversation`.

2. **How much Threads search/filter UX belongs in Phase 20 versus Phase 22?**
   - What we know: `THREADS-UX-01` calls for better continuity/search, and the backend already lists filtered threads.
   - What's unclear: whether this phase should ship rich query UX or just enough search/filter affordances to make continuity usable.
   - Recommendation: keep this phase to persisted filter/search affordances over existing thread state, not a new research-grade search system.

3. **How should the no-model-configured state feel in the default entry path?**
   - What we know: the backend currently returns a clear assistant error string when no chat model is configured.
   - What's unclear: whether the unified entry should still be the default when the operator has not configured any model profile.
   - Recommendation: treat “capture without reply” and “guided setup prompt” as first-class planning cases.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + Web `vitest` |
| Config file | [`/home/jove/code/vel/clients/web/vitest.config.ts`](/home/jove/code/vel/clients/web/vitest.config.ts) |
| Quick run command | `cargo test -p veld chat::tools -- --nocapture && npm test -- src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/ThreadView.test.tsx src/components/MessageComposer.test.tsx` |
| Full suite command | `make verify` plus `npm test` in `clients/web` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ASSIST-01 | Grounded assistant answers from persisted Vel data via bounded tools | integration | `cargo test -p veld --test chat_grounding` | ✅ |
| ASSIST-02 | Optional local/localhost routing remains bounded and safe | unit | `cargo test -p vel-llm openai_oauth -- --nocapture` | ✅ |
| ENTRY-01 | Typed text entry sends through one composer/message path | component | `npm test -- src/components/MessageComposer.test.tsx` | ✅ |
| NOW-UX-01 | `Now` summarizes pressure and routes deeper work | component | `npm test -- src/components/NowView.test.tsx` | ✅ |
| INBOX-UX-01 | Inbox stays triage-first with explicit actions | component | `npm test -- src/components/InboxView.test.tsx` | ✅ |
| THREADS-UX-01 | Threads preserve continuity and realtime updates | component | `npm test -- src/components/ThreadView.test.tsx` | ✅ but baseline currently red |
| THREADS-02 | Assistant escalation preserves existing conversation/thread ownership | integration | `cargo test -p veld --test chat_grounding` | ❌ Wave 0 |
| SETTINGS-UX-01 | Summary-first setup/trust remains intact | component | `npm test -- src/components/SettingsPage.test.tsx` | ✅ |
| USABLE-01 | Assistant becomes practical default entry from daily-use surfaces | smoke/manual | `npm test -- src/components/NowView.test.tsx src/components/ThreadView.test.tsx` | ❌ Wave 0 |
| USABLE-02 | Repeated daily-use loop is materially smoother across surfaces | smoke/manual | `cargo test -p veld --test apple_voice_loop && npm test -- src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/SettingsPage.test.tsx` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p veld chat::tools -- --nocapture` and the smallest affected `npm test -- <file>`
- **Per wave merge:** `cargo test -p veld --test chat_grounding --test apple_voice_loop` plus affected web component suites
- **Phase gate:** `make verify` and `npm test` with targeted manual walkthrough of `Now` → entry → `Inbox`/`Threads`/Settings routing

### Wave 0 Gaps
- [ ] [`/home/jove/code/vel/crates/veld/tests/chat_entry_routing.rs`](/home/jove/code/vel/crates/veld/tests/chat_entry_routing.rs) — covers `ENTRY-01`, `USABLE-01`, `THREADS-02`
- [ ] [`/home/jove/code/vel/clients/web/src/components/UnifiedEntry.test.tsx`](/home/jove/code/vel/clients/web/src/components/UnifiedEntry.test.tsx) — covers converged entry behavior if Phase 20 introduces a new shared entry component
- [ ] [`/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx`](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx) baseline repair — current test assumes `/message/i` placeholder, but composer copy is now “Ask, capture, or talk to Vel…”
- [ ] Manual script/checklist for `no model configured` entry behavior — needed because Phase 20 must keep local-first non-LLM usability credible

## Sources

### Primary (HIGH confidence)
- [`/home/jove/code/vel/.planning/phases/20-daily-use-usability-closure-and-unified-operator-entry/20-CONTEXT.md`](/home/jove/code/vel/.planning/phases/20-daily-use-usability-closure-and-unified-operator-entry/20-CONTEXT.md) - phase purpose, scope, and non-goals
- [`/home/jove/code/vel/.planning/REQUIREMENTS.md`](/home/jove/code/vel/.planning/REQUIREMENTS.md) - Phase 20 requirement contract
- [`/home/jove/code/vel/docs/product/operator-surface-taxonomy.md`](/home/jove/code/vel/docs/product/operator-surface-taxonomy.md) - canonical surface taxonomy
- [`/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md`](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md) - boundary rules for `Now`/`Inbox`/`Threads`
- [`/home/jove/code/vel/docs/api/chat.md`](/home/jove/code/vel/docs/api/chat.md) - mounted assistant/chat/inbox/settings API behavior
- [`/home/jove/code/vel/crates/veld/src/services/chat/assistant.rs`](/home/jove/code/vel/crates/veld/src/services/chat/assistant.rs) - grounded assistant reply flow
- [`/home/jove/code/vel/crates/veld/src/services/chat/tools.rs`](/home/jove/code/vel/crates/veld/src/services/chat/tools.rs) - bounded tool surface and prompt grounding
- [`/home/jove/code/vel/crates/veld/src/services/chat/messages.rs`](/home/jove/code/vel/crates/veld/src/services/chat/messages.rs) - persisted message-first assistant invocation seam
- [`/home/jove/code/vel/crates/veld/src/llm.rs`](/home/jove/code/vel/crates/veld/src/llm.rs) - profile routing and `openai_oauth` gating
- [`/home/jove/code/vel/clients/web/src/components/NowView.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.tsx) - current summary/routing embodiment
- [`/home/jove/code/vel/clients/web/src/components/InboxView.tsx`](/home/jove/code/vel/clients/web/src/components/InboxView.tsx) - current triage embodiment
- [`/home/jove/code/vel/clients/web/src/components/ThreadView.tsx`](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) - current continuity embodiment
- [`/home/jove/code/vel/clients/web/src/components/MessageComposer.tsx`](/home/jove/code/vel/clients/web/src/components/MessageComposer.tsx) - typed/voice entry behavior
- Command evidence: `cargo test -p veld chat::tools -- --nocapture` - backend chat tool tests passed

### Secondary (MEDIUM confidence)
- [`/home/jove/code/vel/docs/user/setup.md`](/home/jove/code/vel/docs/user/setup.md) - operator-facing setup and `openai_oauth` guidance
- [`/home/jove/code/vel/docs/user/daily-use.md`](/home/jove/code/vel/docs/user/daily-use.md) - intended repeated-use workflow
- [`/home/jove/code/vel/crates/veld/tests/chat_grounding.rs`](/home/jove/code/vel/crates/veld/tests/chat_grounding.rs) - backend integration coverage for grounded replies
- [`/home/jove/code/vel/crates/veld/tests/apple_voice_loop.rs`](/home/jove/code/vel/crates/veld/tests/apple_voice_loop.rs) - adjacent voice/daily-loop evidence relevant to future-proofing

### Tertiary (LOW confidence)
- Targeted web run `npm test -- src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/ThreadView.test.tsx src/components/MessageComposer.test.tsx` - useful current-state signal, but one failing test reflects local UI-contract drift rather than a fully audited Phase 20 baseline

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM - versions are repo-pinned and stable in the workspace, but not registry-verified because no new dependency selection is required
- Architecture: HIGH - backed by canonical product docs, current code seams, and active roadmap requirements
- Pitfalls: HIGH - derived from shipped surface boundaries, current code shape, and observed test drift

**Research date:** 2026-03-19
**Valid until:** 2026-03-26
