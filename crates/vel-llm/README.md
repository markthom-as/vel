# vel-llm

Provider-agnostic LLM layer for Vel. Agent and tool code depend only on this crate; no direct HTTP to model backends. Provider selection is driven by config profile IDs.

## T1 (done)

- **Crate** `vel-llm` with `LlmProvider` trait, `LlmRequest` / `LlmResponse`, `ProviderHealth`, `ModelInfo`
- **Structured errors** `LlmError`, `ProviderError` (transport, protocol, capability, auth, rate limit, backend)
- **Provider registry** and **router** — register by profile ID, route by `req.model_profile`

## T2 (done) — LlamaCppProvider

- **`LlamaCppProvider`** — OpenAI-compatible client for local llama-server (llama.cpp)
- **`LlamaCppConfig`** — base_url, model_id, context_window, supports_tools, supports_json
- **Chat/completions** — POST /v1/chat/completions; supports tools and JSON mode when configured
- **health()** — GET /v1/models
- **models()** — GET /v1/models, normalized to `ModelInfo`
- **Errors** — transport, protocol, capability (e.g. tools not supported), backend

## T3 (done) — Model profiles and router config

- **configs/models/** — `local-qwen3-coder.toml`, `local-qwen25-fast.toml`, `routing.toml`
- **vel-config** — `ModelProfile`, `RoutingConfig`, `load_model_profiles(dir)`, `load_routing(path)`; validation (required fields, known provider, no duplicate id) fails on load
- Task classes (chat, codegen, summarize, …) route to profile IDs via `routing.toml` [default] section

## Next (T4–T8)

See `docs/llm-backend-plan/docs/04-implementation-tickets.md`: openai-oauth (T4), redaction/policy (T5), dev ergonomics (T6), eval harness (T7), docs (T8).
