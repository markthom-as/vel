# Model profiles and routing

- **Profile tomls** — One file per model/backend (e.g. `local-qwen3-coder.toml`, `local-qwen25-fast.toml`). Required: `id`, `provider`, `base_url`, `model`. Optional: `context_window`, `max_output_tokens`, `temperature`, `supports_tools`, `supports_json`, `enabled`.
- **routing.toml** — `[default]` section: task class → profile id (e.g. `chat = "local-qwen3-coder"`, `summarize = "local-qwen25-fast"`).

Load via `vel_config::load_model_profiles("configs/models")` and `vel_config::load_routing("configs/models/routing.toml")`. Invalid or duplicate configs fail on startup. See `crates/vel-config/src/models.rs` and docs/llm-backend-plan/.

## Local dev (make dev)

Chat assistant replies use the profile in `routing.toml` under `chat` (e.g. `local-qwen3-coder`, port 8012). To run the LLM backend as part of `make dev`:

1. **Download the default chat model (once):**
   ```bash
   ./scripts/download-chat-model.sh
   ```
   This fetches Qwen2.5-1.5B-Instruct (q4_k_m, ~1.1 GB) into `configs/models/weights/`. `make dev` will use it automatically if **VEL_LLM_MODEL** is not set.

2. Install [llama.cpp](https://github.com/ggerganov/llama.cpp) and ensure `llama-server` is on your PATH.

3. Run `make dev`. The script starts the LLM server (when the model file and `llama-server` exist), then veld, then the web app. Ctrl+C stops all.

To use a different model, set **VEL_LLM_MODEL** to your .gguf path before `make dev`.
