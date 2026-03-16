# Vel local LLM backend plan

This bundle gives Vel a clean local-first LLM architecture for a single-workstation deployment on an RTX 3090 (24 GB VRAM), Ryzen 9 9950X3D, and 64 GB RAM.

## Recommendation

Use a **two-tier backend strategy**:

1. **Primary local coder:** `Qwen3-Coder-30B-A3B-Instruct` on `llama.cpp` / `llama-server`
2. **Fast local utility model:** `Qwen2.5-Coder-14B-Instruct` on `llama.cpp` / `llama-server`
3. **Optional OAuth-backed remote provider:** `openai-oauth` as a third backend for cases where Vel should use ChatGPT-account-backed models via a local OpenAI-compatible proxy

This keeps Vel architecturally clean:

- local models handle default autonomy and privacy-sensitive work
- the OAuth backend is an explicit, opt-in provider profile
- no agent code should know or care whether the provider is local GGUF, remote OpenAI-compatible, or ChatGPT-account OAuth proxy

## Included docs

- `docs/01-architecture.md` — provider boundaries, routing, repo layout
- `docs/02-local-model-serving.md` — local model choices and serving topology
- `docs/03-openai-oauth-backend.md` — how to add `EvanZhouDev/openai-oauth` cleanly
- `docs/04-implementation-tickets.md` — agent-ready implementation tickets
- `examples/` — sample Rust traits, config, scripts, and systemd units

## Important note on `openai-oauth`

Treat it as a **developer convenience backend**, not a default production transport.

Reasons:

- it is an **unofficial, community-maintained** project
- it uses the local Codex / ChatGPT auth cache and explicitly says the auth file should be treated like **password-equivalent credentials**
- the repo says to use it only for **personal, local experimentation on trusted machines** and not as a hosted service
- it is licensed **AGPL-3.0**, so any code-copying or embedding decisions should be made deliberately

Vel should therefore integrate it via a **process / local proxy boundary**, not by vendoring code into Vel.
