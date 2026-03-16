---
title: Vel Adaptive Configuration and Auditability Spec
status: proposed
owners:
  - nav-runtime
  - settings
  - policy-engine
updated: 2026-03-16
---

# Vel Adaptive Configuration and Auditability Spec

## 1. Purpose

Vel should support both durable user preferences and dynamic runtime adaptation without becoming a mysterious config poltergeist. The system must be able to optimize for latency, token/context usage, privacy, reliability, surface constraints, and autonomy while preserving three invariants:

1. the user’s durable intent remains legible and stable,
2. runtime changes are deterministic and reversible,
3. every effective behavior-changing config mutation is auditable.

This subsystem is not merely a settings page. It is a control plane for how Vel composes cognition, tools, surfaces, and resource budgets at runtime.

## 2. Problem Statement

As Vel grows into a multi-surface, multi-agent, multi-backend system, static settings alone become inadequate:

- watch/mobile/voice need smaller, faster, lower-noise behavior than desktop,
- urgent tasks need different autonomy and retrieval behavior than reflective chat,
- privacy-sensitive contexts need stricter tool and memory policies,
- low-resource conditions need graceful degradation,
- operators need to know why Vel changed behavior and what policy fired.

Without a first-class adaptive settings system, this logic ends up smeared across handlers, UI components, and ad hoc conditionals. That produces hidden precedence, duplicated logic, and behavior that nobody can explain after the fact.

## 3. Design Principles

### 3.1 Deterministic resolution
Given the same base settings, policies, context signals, and system constraints, the resolver must produce the same effective config.

### 3.2 No silent rewriting of durable preferences
Dynamic overrides should not mutate persisted user settings unless the user explicitly saves a temporary state as a new default.

### 3.3 Append-only audit trail
Every material config change should be reconstructable from events.

### 3.4 Reversible runtime adaptation
Overrides require expiration or reversion semantics.

### 3.5 Explainability as a product requirement
The system must answer: “what is active?”, “why?”, “what overrode what?”, and “what would happen under another context?”

### 3.6 Conservative rollout
Prefer rule-based and typed behavior before learned heuristics. Fancy can come later. Haunting comes free.

## 4. Terminology

- **Base setting**: persisted default user preference.
- **Profile**: named bundle of config values, such as `watch`, `urgent`, or `privacy_sensitive`.
- **Context signal**: current observed or inferred fact used by policies, such as `surface=watch` or `battery_state=low`.
- **Policy**: conditional rule that applies one or more overrides or profiles.
- **Override**: a non-durable runtime value change.
- **Effective config**: final merged runtime configuration after precedence resolution.
- **Constraint**: hard bound from safety, platform, compliance, or system state.
- **Decision record**: structured explanation for why a config transition occurred.
- **Replay**: re-running resolution against historic events/signals to explain prior behavior.

## 5. Scope

### In scope

- settings schema and persistence
- scoped configuration (global, surface, workspace, session, task, component)
- dynamic policy-driven overrides
- effective-config resolution
- audit logging and replay
- explainability APIs
- shipped runtime profiles and defaults
- simulation/dry-run endpoints

### Out of scope for v1

- fully learned policy authoring by LLMs
- unconstrained natural-language policy compilation without review
- autonomous mutation of system defaults based solely on inferred preference drift
- cross-user/org policy management beyond a future-compatible model

## 6. Architecture Overview

Vel should introduce an **Adaptive Configuration Engine** composed of the following backend modules:

- `config/schema.rs` — typed config keys, values, scopes, validation
- `config/store.rs` — persisted settings CRUD
- `config/resolver.rs` — deterministic merge engine
- `config/profiles.rs` — built-in profile definitions
- `signals/mod.rs` — context signal ingestion and normalization
- `policy/engine.rs` — policy matching and override generation
- `policy/dsl.rs` — persisted rule format and evaluator
- `audit/config_events.rs` — append-only event writer
- `audit/replay.rs` — snapshot/replay/explanation builder
- `routes/config.rs` — REST endpoints
- `routes/audit.rs` — audit/replay endpoints

High-level flow:

1. base settings are loaded for the relevant scopes,
2. current context signals are gathered,
3. policies are matched,
4. profiles and overrides are emitted,
5. hard constraints are applied,
6. resolver produces effective config,
7. snapshots + events are recorded if materially changed,
8. APIs/UI expose current state and provenance.

## 7. Configuration Domains

Vel should support configuration in at least these domains.

### 7.1 Inference/runtime
- `llm.backend`
- `llm.model`
- `llm.reasoning_depth`
- `llm.temperature`
- `llm.max_output_tokens`
- `llm.parallel_tool_calls`
- `llm.agent_fanout_limit`

### 7.2 Retrieval/context
- `context.max_budget_tokens`
- `context.include_memory`
- `context.include_calendar`
- `context.include_todos`
- `context.retrieval_depth`
- `context.compression_strategy`
- `context.recency_weight`
- `context.salience_weight`

### 7.3 Autonomy/coordination
- `autonomy.level`
- `autonomy.ask_on_low_confidence`
- `autonomy.low_confidence_threshold`
- `autonomy.allow_background_task_start`
- `autonomy.allow_agent_consultation`
- `autonomy.max_unconfirmed_steps`

### 7.4 UX/surface behavior
- `ux.verbosity`
- `ux.hud_density`
- `ux.proactive_nudges`
- `ux.debug_explanations`
- `ux.confidence_visibility`
- `ux.interruption_policy`

### 7.5 Privacy/tooling
- `privacy.memory_write_mode`
- `privacy.external_tool_access`
- `privacy.audit_redaction_level`
- `tools.allowed_sets`
- `tools.network_access_mode`

### 7.6 Resource usage
- `resource.poll_interval_ms`
- `resource.background_refresh_enabled`
- `resource.embedding_refresh_mode`
- `resource.low_power_mode`
- `resource.cache_strategy`

## 8. Scope Model

Each setting can exist across multiple scopes. The resolver must understand scope precedence.

Supported scopes:

- `global`
- `surface:{desktop|mobile|watch|voice|cli}`
- `workspace:{id}`
- `session:{id}`
- `task:{id}`
- `component:{id}`

Example:

- globally prefer local inference,
- on watch, always terse,
- in grant-writing workspace, allow higher context budget,
- for a specific urgent task, enable higher reminder priority,
- for reminder agent component, low creativity and high reliability.

## 9. Resolution Order

The resolver should merge values using explicit precedence:

1. hard safety/compliance/platform constraints
2. explicit current-session command overrides
3. matched policy overrides
4. active profile values
5. persisted scoped user settings
6. system defaults

Where equal-precedence values conflict, tie-break via:

1. narrower scope beats broader scope,
2. higher priority beats lower priority,
3. newer session/task override beats older one if same source class,
4. otherwise deterministic lexical/id fallback with warning logged.

## 10. Data Model

### 10.1 `user_settings`
Persisted durable user preferences.

Suggested columns:

- `id` UUID PK
- `user_id` UUID
- `scope_type` TEXT
- `scope_id` TEXT NULL
- `config_key` TEXT
- `value_json` JSONB
- `locked` BOOLEAN DEFAULT FALSE
- `created_at` TIMESTAMPTZ
- `updated_at` TIMESTAMPTZ
- unique `(user_id, scope_type, scope_id, config_key)`

### 10.2 `config_profiles`
Named bundles of defaults or user-customizable presets.

- `id`
- `user_id` NULLABLE for system profile
- `name`
- `description`
- `values_json`
- `is_system`
- `created_at`
- `updated_at`

### 10.3 `context_signals`
Current or recent normalized signals.

- `id`
- `user_id`
- `signal_key`
- `signal_value_json`
- `source`
- `confidence`
- `observed_at`
- `expires_at`
- `trace_id`

### 10.4 `config_policies`
Stored rules that produce profiles/overrides.

- `id`
- `user_id` NULLABLE for system policy
- `name`
- `description`
- `enabled`
- `priority`
- `scope_filter_json`
- `condition_json`
- `actions_json`
- `cooldown_seconds`
- `created_at`
- `updated_at`

### 10.5 `effective_config_snapshots`
Materialized resolved config for a scope context.

- `id`
- `user_id`
- `subject_kind` (`session`, `task`, `component`, `surface`, `global`)
- `subject_id`
- `config_json`
- `provenance_json`
- `hash`
- `created_at`

### 10.6 `config_events`
Append-only audit stream.

- `id`
- `user_id`
- `event_type`
- `subject_kind`
- `subject_id`
- `config_key` NULLABLE
- `old_value_json`
- `new_value_json`
- `source_kind`
- `source_id`
- `reason_code`
- `evidence_json`
- `trace_id`
- `created_at`

### 10.7 `config_replays`
Optional persisted replay jobs/results if caching is desired.

- `id`
- `user_id`
- `request_json`
- `result_json`
- `created_at`

## 11. Example JSON Shapes

### 11.1 Setting value

```json
{
  "key": "context.retrieval_depth",
  "value": "medium",
  "scope": { "type": "surface", "id": "desktop" },
  "locked": false
}
```

### 11.2 Context signal

```json
{
  "signal_key": "surface",
  "signal_value": "watch",
  "source": "client_session",
  "confidence": 1.0,
  "observed_at": "2026-03-16T11:10:00Z",
  "expires_at": "2026-03-16T13:10:00Z"
}
```

### 11.3 Policy

```json
{
  "name": "watch-low-noise",
  "priority": 80,
  "condition": {
    "all": [
      { "signal": "surface", "op": "eq", "value": "watch" },
      { "signal": "task.urgency", "op": "neq", "value": "high" }
    ]
  },
  "actions": [
    { "type": "apply_profile", "profile": "watch" },
    { "type": "set", "key": "ux.verbosity", "value": "terse" },
    { "type": "set", "key": "context.retrieval_depth", "value": "low" }
  ]
}
```

### 11.4 Provenance entry

```json
{
  "config_key": "context.retrieval_depth",
  "effective_value": "low",
  "base_value": "medium",
  "source": {
    "kind": "policy",
    "id": "watch-low-noise"
  },
  "overrides": [
    {
      "kind": "profile",
      "id": "watch",
      "value": "low"
    }
  ],
  "reason_code": "surface_constraint",
  "evidence": {
    "surface": "watch",
    "urgency": "normal"
  },
  "changed_at": "2026-03-16T11:10:01Z",
  "revert_when": {
    "signal": "surface",
    "op": "neq",
    "value": "watch"
  }
}
```

## 12. API Contract

### 12.1 GET `/v1/config`
Return current persisted settings by scope.

Query params:
- `scope_type`
- `scope_id`
- `include_locked`

Response:

```json
{
  "items": [
    {
      "key": "llm.model",
      "value": "qwen3-32b-local",
      "scope": { "type": "global", "id": null },
      "locked": false,
      "updated_at": "2026-03-16T10:00:00Z"
    }
  ]
}
```

### 12.2 PUT `/v1/config`
Upsert one or more persisted settings.

Request:

```json
{
  "items": [
    {
      "key": "ux.verbosity",
      "value": "detailed",
      "scope": { "type": "global", "id": null }
    }
  ]
}
```

Behavior:
- validate keys and values,
- reject writes to system-only keys,
- emit `setting.updated` or `setting.created` events.

### 12.3 GET `/v1/config/effective`
Resolve effective config for a subject.

Query params:
- `subject_kind`
- `subject_id`
- `surface`
- `workspace_id`
- `include_provenance=true|false`

Response:

```json
{
  "subject": { "kind": "session", "id": "sess_123" },
  "config": {
    "ux.verbosity": "terse",
    "context.retrieval_depth": "low",
    "autonomy.level": "medium"
  },
  "hash": "cfg_abc123",
  "provenance": [
    {
      "config_key": "ux.verbosity",
      "effective_value": "terse",
      "source": { "kind": "policy", "id": "watch-low-noise" },
      "reason_code": "surface_constraint"
    }
  ]
}
```

### 12.4 GET `/v1/config/explain`
Human/operator-friendly explanation for current effective config or one key.

Query params:
- `subject_kind`
- `subject_id`
- `config_key` optional

Response:

```json
{
  "summary": "Switched retrieval depth from medium to low because the active surface is watch and the task urgency is not high.",
  "items": [
    {
      "config_key": "context.retrieval_depth",
      "base_value": "medium",
      "effective_value": "low",
      "source_kind": "policy",
      "source_id": "watch-low-noise",
      "evidence": {
        "surface": "watch",
        "task.urgency": "normal"
      }
    }
  ]
}
```

### 12.5 POST `/v1/config/simulate`
Dry-run the resolver against hypothetical signals or overrides.

Request:

```json
{
  "subject": { "kind": "session", "id": "sess_123" },
  "signals": [
    { "signal_key": "surface", "signal_value": "voice" },
    { "signal_key": "task.urgency", "signal_value": "high" }
  ],
  "session_overrides": {
    "ux.debug_explanations": true
  }
}
```

Response should include resolved config, matched policies, and changed keys relative to current reality.

### 12.6 GET `/v1/config/policies`
List policies visible to the user.

### 12.7 PUT `/v1/config/policies/:id`
Create or update a policy.

### 12.8 GET `/v1/audit/config-events`
List audit events.

Filters:
- `subject_kind`
- `subject_id`
- `event_type`
- `config_key`
- `since`
- `until`
- `trace_id`

### 12.9 POST `/v1/audit/config-replay`
Replay a past decision or time window and explain what effective config would have been.

## 13. Policy DSL

Use a JSON-serializable rule format for v1. Avoid open-ended imperative scripts.

Supported conditions:
- `all`
- `any`
- `not`
- `signal` comparisons (`eq`, `neq`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `exists`)
- `scope` checks
- `time_window`
- optional `confidence_gte` for inferred signals

Supported actions:
- `apply_profile`
- `set`
- `unset`
- `set_if_absent`
- `emit_reason`
- `require_confirmation`
- `start_cooldown`

### Example DSL sketch

```json
{
  "all": [
    { "signal": "surface", "op": "eq", "value": "mobile" },
    { "signal": "battery.level", "op": "lt", "value": 0.2 },
    { "signal": "task.urgency", "op": "neq", "value": "high" }
  ]
}
```

## 14. Built-in Runtime Profiles

Ship conservative system profiles in v1.

### `watch`
- terse output
- low retrieval depth
- no nonessential agent fan-out
- high reminder salience
- compressed explanations

### `mobile`
- medium-low retrieval
- moderate verbosity
- low background refresh

### `voice`
- short response units
- no dense citations by default
- action-first ordering
- high interruption sensitivity

### `urgent`
- higher autonomy if permitted
- higher reminder priority
- lower reflection overhead
- confirmation path if confidence below threshold

### `privacy_sensitive`
- external tools reduced/disabled
- memory writes minimized
- aggressive audit redaction

### `low_resource`
- smaller model/backend if available
- low refresh/polling
- lower retrieval depth
- cache-first behavior

### `deep_work`
- suppress nonessential nudges
- higher context budget
- reduced interruptions

## 15. Context Signals

Context signals should come from normalized sources, not arbitrary strings leaking from every subsystem.

### Signal categories
- surface/device: `surface`, `device_class`, `screen_state`
- temporal: `calendar.busy`, `meeting.imminent`, `local_time_segment`
- task: `task.urgency`, `task.type`, `task.focus_required`
- privacy: `privacy.mode`, `network.trust_level`
- resources: `battery.level`, `battery.state`, `network.quality`, `cpu.pressure`
- cognition/interaction: `user.focus_mode`, `user.interruption_tolerance`, `session.intent`

### Signal ingestion rules
- every signal needs source attribution,
- inferred signals should include confidence,
- expiring signals must have TTL,
- contradictions should be logged,
- normalization should happen before policy evaluation.

## 16. Auditability Requirements

Every config-affecting event must record:
- what changed,
- old and new values,
- subject scope,
- source kind and source id,
- reason code,
- evidence payload,
- correlation/trace id,
- timestamp.

Recommended event types:
- `setting.created`
- `setting.updated`
- `setting.deleted`
- `profile.activated`
- `profile.deactivated`
- `policy.matched`
- `override.applied`
- `override.reverted`
- `constraint.applied`
- `effective_config.changed`
- `simulation.executed`
- `replay.executed`

## 17. Replay and Forensics

Operators should be able to answer:
- Why did Vel behave this way yesterday?
- What config was active for this task?
- Which policy caused this change?
- What would have happened if the user were on desktop instead of watch?

Replay requirements:
- resolve historical signals/events over a time window,
- reconstruct effective snapshots,
- display provenance chain,
- optionally diff actual vs hypothetical outcomes.

## 18. Security and Privacy

- redact sensitive evidence in user-facing audit views,
- keep full evidence behind privileged operator access if such a mode exists,
- treat policy editing as a privileged mutation,
- enforce lockable settings for safety-sensitive keys,
- protect against policy loops and override thrash.

## 19. Failure Modes and Guards

### 19.1 Policy oscillation
Use cooldown windows and change coalescing.

### 19.2 Hidden precedence bugs
Emit warnings when multiple same-priority writes conflict.

### 19.3 Excessive event volume
Deduplicate no-op transitions and hash snapshots.

### 19.4 Policy loops
Disallow policies that depend on values they themselves immediately mutate unless explicitly safe.

### 19.5 Silent degradations
All hard constraints should surface in explain output.

## 20. Backend Implementation Sketch

Suggested files:

- `crates/veld/src/config/schema.rs`
- `crates/veld/src/config/types.rs`
- `crates/veld/src/config/store.rs`
- `crates/veld/src/config/resolver.rs`
- `crates/veld/src/config/provenance.rs`
- `crates/veld/src/signals/mod.rs`
- `crates/veld/src/signals/normalize.rs`
- `crates/veld/src/policy/dsl.rs`
- `crates/veld/src/policy/engine.rs`
- `crates/veld/src/audit/config_events.rs`
- `crates/veld/src/audit/replay.rs`
- `crates/veld/src/routes/config.rs`
- `crates/veld/src/routes/audit.rs`
- `crates/veld/src/app.rs` route mounts
- `migrations/00xx_adaptive_config.sql`

### Rust type sketch

```rust
pub enum ScopeType {
    Global,
    Surface,
    Workspace,
    Session,
    Task,
    Component,
}

pub enum ValueSourceKind {
    Default,
    UserSetting,
    Profile,
    Policy,
    SessionOverride,
    Constraint,
}

pub struct ProvenanceEntry {
    pub key: String,
    pub effective_value: serde_json::Value,
    pub base_value: Option<serde_json::Value>,
    pub source_kind: ValueSourceKind,
    pub source_id: Option<String>,
    pub reason_code: Option<String>,
    pub evidence: serde_json::Value,
}
```

### Resolver pseudocode

```text
load defaults
load persisted settings across scopes
load active profiles
load signals
match policies
collect overrides
apply hard constraints
merge in precedence order
validate final config
compare against prior snapshot hash
if changed: write snapshot + events
return effective config + provenance
```

## 21. Migration Sketch

Illustrative SQL, not final:

```sql
create table user_settings (
  id uuid primary key,
  user_id uuid not null,
  scope_type text not null,
  scope_id text,
  config_key text not null,
  value_json jsonb not null,
  locked boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  unique (user_id, scope_type, scope_id, config_key)
);

create table config_policies (
  id uuid primary key,
  user_id uuid,
  name text not null,
  description text,
  enabled boolean not null default true,
  priority integer not null default 50,
  scope_filter_json jsonb not null default '{}'::jsonb,
  condition_json jsonb not null,
  actions_json jsonb not null,
  cooldown_seconds integer not null default 0,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table config_events (
  id uuid primary key,
  user_id uuid not null,
  event_type text not null,
  subject_kind text not null,
  subject_id text not null,
  config_key text,
  old_value_json jsonb,
  new_value_json jsonb,
  source_kind text not null,
  source_id text,
  reason_code text,
  evidence_json jsonb not null default '{}'::jsonb,
  trace_id text,
  created_at timestamptz not null default now()
);
```

## 22. Testing Strategy

### Unit tests
- schema validation by key
- resolver precedence
- scope precedence
- policy evaluation semantics
- no-op dedupe
- profile composition

### Integration tests
- API CRUD for settings and policies
- effective config resolution across multiple scopes
- replay and explain endpoints
- policy cooldown behavior
- lock/constraint enforcement

### Property tests
- deterministic resolution
- no panics under random valid input sets
- stable hash output for same inputs

### Golden tests
- human-readable explanation strings
- known scenario snapshots (watch, urgent, privacy-sensitive)

## 23. Operator and User UX

The UI should expose:
- current effective profile(s),
- changed keys,
- why they changed,
- whether a change is temporary,
- the base value underneath,
- a one-click “make this my default” action,
- simulation mode,
- replay timeline for debugging.

Recommended UI surfaces:
- settings page with base vs effective tabs,
- debug drawer showing active policies and signals,
- timeline view for config events,
- task/session inspector with “why this behavior?” card.

## 24. Rollout Plan

### Phase 1
- schema + storage
- built-in keys
- manual persisted settings CRUD

### Phase 2
- resolver + effective config endpoint
- built-in profiles
- basic audit events

### Phase 3
- signals ingestion
- rule-based policies
- explain endpoint

### Phase 4
- replay/simulation
- operator UI
- guarded user-authored policies

## 25. Open Questions

- Which settings should be user-lockable vs system-lockable?
- Should profiles be first-class user-visible objects on day one or hidden under policy actions initially?
- What minimum signal set is needed to make dynamic adaptation materially useful without scope creep?
- How much evidence should be visible to end users versus operators?
- Should policy authoring be purely structured JSON in v1 or also have a safe form-based UI?

## 26. Recommended Defaults

Start conservative:
- dynamic adaptation enabled only for a narrow set of keys,
- shipped policies limited to surface/resource/urgency/privacy,
- explainability always on in debug builds,
- replay retained for a bounded window,
- all policy mutations auditable.

That gets you a system that can adapt without turning into an occult bureaucracy nobody can debug.
