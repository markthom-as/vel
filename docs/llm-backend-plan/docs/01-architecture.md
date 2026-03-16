# 01 — Vel LLM architecture

## Goal

Vel needs a model layer that is:

- provider-agnostic
- local-first
- testable
- cheap to swap
- explicit about routing, tool support, JSON mode, and context limits

The mistake to avoid is letting planner / actor / tool code couple itself directly to one serving stack. That is how monorepos become theological disputes with build steps.

## Repo layout

```text
vel/
  apps/
    vel-cli/
    vel-daemon/
  crates/
    vel-core/
    vel-config/
    vel-llm/
    vel-agents/
    vel-tools/
    vel-evals/
  services/
    llm-gateway/
      scripts/
      systemd/
      docker/
  configs/
    models/
      local-qwen3-coder.toml
      local-qwen25-fast.toml
      oauth-openai.toml
      routing.toml
  docs/
    architecture/
```

## Hard rule

`vel-agents` must never talk directly to:

- `llama-server`
- `openai-oauth`
- OpenAI remote APIs
- model-specific prompt templates

Instead:

- `vel-agents` -> `vel-llm`
- `vel-llm` -> provider adapters
- provider adapters -> concrete transport endpoints

## Rust module split

```text
crates/vel-llm/
  src/
    lib.rs
    types.rs
    traits.rs
    router.rs
    errors.rs
    providers/
      mod.rs
      llama_cpp.rs
      openai_compat.rs
      openai_oauth.rs
    prompting/
      tool_schema.rs
      json_mode.rs
    telemetry/
      spans.rs
      usage.rs
```

## Core trait

```rust
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, req: LlmRequest) -> anyhow::Result<LlmResponse>;
    async fn health(&self) -> anyhow::Result<ProviderHealth>;
    async fn models(&self) -> anyhow::Result<Vec<ModelInfo>>;
}
```

## Request type

```rust
pub struct LlmRequest {
    pub system: String,
    pub messages: Vec<Message>,
    pub tools: Vec<ToolSpec>,
    pub response_format: ResponseFormat,
    pub temperature: f32,
    pub max_output_tokens: u32,
    pub model_profile: String,
    pub metadata: serde_json::Value,
}
```

## Provider types

### 1. `LlamaCppProvider`
For local `llama-server` instances serving GGUF models.

### 2. `OpenAiCompatProvider`
Generic provider for any OpenAI-compatible `/v1` endpoint.

### 3. `OpenAiOauthProvider`
A thin specialization over `OpenAiCompatProvider` that:

- points to the local `openai-oauth` proxy
- marks the backend as `developer_managed = true`
- disables use unless the proxy is healthy and an auth file exists

This should be composition, not inheritance cosplay.

## Routing policy

Use profile-based routing instead of hardcoded model IDs.

Routing should be task-driven:

- `codegen` -> `local-qwen3-coder`
- `tool_use` -> `local-qwen3-coder`
- `patch_small` -> `local-qwen25-fast`
- `summarize` -> `local-qwen25-fast`
- `fallback_remote` -> `oauth-openai`

Remote OAuth use should be opt-in or policy-gated.

## Safety / cost / privacy flags

Each model profile should expose:

- `locality = local | remote_via_local_proxy`
- `data_policy = private_ok | redact_first | ephemeral_only`
- `supports_tools`
- `supports_json`
- `supports_reasoning`
- `max_context`
- `max_output_tokens`
- `latency_class = fast | balanced | deep`

This lets Vel make first-class routing decisions instead of relying on vibes and prayer.
