# 04 — Agent-ready implementation tickets

## T1 — Create `vel-llm` provider abstraction

**Status:** todo

### Deliverables
- add `crates/vel-llm`
- define `LlmProvider` trait
- define `LlmRequest`, `LlmResponse`, `ProviderHealth`, `ModelInfo`
- add provider registry and router
- add structured provider errors

### Acceptance criteria
- agent code depends only on `vel-llm`
- no direct HTTP calls to model backends from `vel-agents`
- provider selection is driven by config profile IDs

---

## T2 — Add `LlamaCppProvider`

**Status:** todo

### Deliverables
- implement OpenAI-compatible calls to local `llama-server`
- support chat / responses style requests
- support tools and structured JSON mode where possible
- normalize usage and finish reasons

### Acceptance criteria
- can call primary and fast local profiles
- `health()` checks `/v1/models`
- failures return typed transport / protocol / capability errors

---

## T3 — Add model profiles and router config

**Status:** todo

### Deliverables
- `configs/models/local-qwen3-coder.toml`
- `configs/models/local-qwen25-fast.toml`
- `configs/models/routing.toml`
- parser / validation in `vel-config`

### Acceptance criteria
- task classes route to provider IDs by config
- invalid configs fail on startup, not mid-run

---

## T4 — Add `openai-oauth` as external backend

**Status:** todo

### Deliverables
- `OpenAiOauthProvider` wrapper
- repo-managed startup script
- systemd user unit
- explicit feature flag: `enable_openai_oauth`
- localhost enforcement
- health preflight for auth file and `/v1/models`

### Acceptance criteria
- backend remains disabled by default
- enabling requires explicit config
- if proxy is missing, Vel degrades cleanly without panic

---

## T5 — Add redaction and provider policy gates

**Status:** todo

### Deliverables
- provider-level `data_policy`
- redact-first pass before OAuth backend dispatch
- telemetry labels for `provider_id`, `locality`, `fallback_reason`

### Acceptance criteria
- sensitive context can be blocked or summarized before remote dispatch
- eval logs clearly show when non-local transport was used

---

## T6 — Add dev ergonomics

**Status:** todo

### Deliverables
- `just llm-up`
- `just llm-down`
- `just llm-status`
- `just llm-bench`
- `just oauth-login`

### Acceptance criteria
- one-command local startup for all enabled backends
- startup output names active ports and models

---

## T7 — Add eval harness

**Status:** todo

### Deliverables
- A/B harness for local primary vs local fast vs oauth backend
- benchmark prompts for patching, planning, summarization, tool calling
- latency / token / success logging

### Acceptance criteria
- can compare providers on the same tasks
- output is machine-readable JSON and human-readable markdown summary

---

## T8 — Add docs

**Status:** todo

### Deliverables
- architecture doc
- local inference doc
- oauth backend doc
- operational troubleshooting doc

### Acceptance criteria
- a new contributor can boot the stack without spelunking through shell history like an archaeologist of bad decisions
