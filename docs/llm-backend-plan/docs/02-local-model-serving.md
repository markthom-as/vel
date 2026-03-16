# 02 — Local model serving on 3090 / 9950X3D / 64 GB RAM

## Recommended default stack

### Primary
- Model: `Qwen3-Coder-30B-A3B-Instruct`
- Runtime: `llama.cpp` / `llama-server`
- Format: GGUF quant
- Default role: primary local coding / planning / tool-use model

### Secondary
- Model: `Qwen2.5-Coder-14B-Instruct`
- Runtime: `llama.cpp` / `llama-server`
- Format: GGUF quant
- Default role: fast utility model for summaries, classification, tiny patch generation, commit messages

## Why this stack

On a single RTX 3090, the real question is not “what is the biggest thing that technically boots” but “what maintains useful latency without turning every edit into a weather system.”

A MoE coding model in quantized GGUF form gives a better quality / throughput tradeoff than trying to brute-force a massive dense model into a consumer card.

## Runtime choice

Prefer `llama.cpp` over vLLM for this exact local setup because:

- `llama.cpp` is a good fit for single-GPU GGUF inference
- Qwen documents local support through `llama.cpp`
- vLLM’s GGUF support is explicitly experimental and under-optimized

## Initial operating profile

### `qwen3-coder`
- context: `16384`
- parallel: `2`
- flash attention: enabled
- all GPU layers offloaded
- KV cache q8 where stable

### `qwen25-fast`
- context: `8192`
- parallel: `4`
- flash attention: enabled
- all GPU layers offloaded

These are sane defaults, not commandments engraved on a CUDA tablet.

## Serving topology

Bind each model to a separate localhost port:

- `127.0.0.1:8012` -> primary coder
- `127.0.0.1:8013` -> fast coder
- `127.0.0.1:8014` -> `openai-oauth` proxy

Vel then sees a uniform set of OpenAI-ish endpoints, with provider-specific handling hidden behind config.

## Process management

Use **systemd user services** or a dev-process runner under the monorepo.

Do not:

- shell out from agent code to start servers on demand
- hide auth discovery inside random scripts in home directories
- mix model startup with application startup unless you enjoy debugging deadlocks before coffee
