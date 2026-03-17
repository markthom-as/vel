# Model profiles and routing

- **Profile tomls** — One file per model/backend (e.g. `local-qwen3-coder.toml`, `local-qwen25-fast.toml`). Required: `id`, `provider`, `base_url`, `model`. Optional: `context_window`, `max_output_tokens`, `temperature`, `supports_tools`, `supports_json`, `enabled`.
- **routing.toml** — `[default]` section: task class → profile id (e.g. `chat = "local-qwen3-coder"`, `summarize = "local-qwen25-fast"`).
- **Template files** — `templates/profile.template.toml` and `templates/routing.template.toml` provide canonical starter shapes.
- **Schema files** — `config/schemas/model-profile.schema.json` and `config/schemas/model-routing.schema.json` define machine-readable contracts.

Load via `vel_config::load_model_profiles("configs/models")` and `vel_config::load_routing("configs/models/routing.toml")`. Invalid or duplicate configs fail on startup. See `crates/vel-config/src/models.rs` and docs/llm-backend-plan/.

## Optional remote provider profile (`openai_oauth`)

You can add an `openai_oauth` profile (example: `oauth-openai`) for optional overflow via a local OpenAI-compatible proxy. It is only registered when `VEL_ENABLE_OPENAI_OAUTH` is truthy (`1`, `true`, `yes`, `on`) and the profile `base_url` points to `localhost`/`127.0.0.1`.

## Local dev (make dev)

Chat assistant replies use the profile in `routing.toml` under `chat` (e.g. `local-qwen3-coder`, port 8012). The fast utility profile uses `local-qwen25-fast` on port 8013. To run the LLM backend as part of `make dev`:

1. **Download the default chat model (once):**
   ```bash
   ./scripts/download-chat-model.sh
   ```
   This fetches Qwen2.5-1.5B-Instruct (q4_k_m, ~1.1 GB) into `configs/models/weights/`. `make dev` will use it automatically as the primary fallback if **VEL_LLM_MODEL** is not set.

2. Install [llama.cpp](https://github.com/ggerganov/llama.cpp) and ensure `llama-server` is on your PATH.

3. For the intended two-tier setup, put the model paths in `vel.toml`:
   - `llm_model_path = "/path/to/qwen3-coder-30b-a3b-instruct-*.gguf"`
   - `llm_fast_model_path = "/path/to/qwen2.5-coder-14b-instruct-*.gguf"`

   Environment variables still override file config:
   - `VEL_LLM_MODEL`
   - `VEL_LLM_FAST_MODEL`

4. Run `make dev`. The script starts the primary server when `VEL_LLM_MODEL` is available and the fast server when `VEL_LLM_FAST_MODEL` is set, then starts veld and the web app. Ctrl+C stops all.

To run the servers manually, use `scripts/llm-server.sh` for the primary model and `scripts/llm-server-fast.sh` for the fast model.

Before starting, use `make check-llm-setup` to confirm model paths, `llama-server`, and GPU visibility from the current shell.
