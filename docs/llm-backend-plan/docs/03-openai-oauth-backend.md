# 03 — Adding `openai-oauth` cleanly

## What it is

`EvanZhouDev/openai-oauth` provides a localhost **OpenAI-compatible** proxy that forwards requests to `chatgpt.com/backend-api/codex/responses` using locally cached ChatGPT / Codex OAuth credentials. It exposes `/v1/responses`, `/v1/chat/completions`, and `/v1/models`, supports streaming, tool calls, and reasoning traces, and defaults to `127.0.0.1:10531`. The project also states that login is not bundled and expects users to create the local auth file with `npx @openai/codex login`. It further warns that the auth file is password-equivalent, should only be used for personal local experimentation on trusted machines, and that the project is unofficial and AGPL-3.0 licensed. The repo currently shows two open issues, including “Built-In Compaction” and “Look Into Stateful Responses API.” citeturn199187view0turn199187view1

## Integration stance

Integrate it as a **local external backend**, not as vendored code and not as Vel’s default provider.

That means:

- **do not copy source** into the monorepo
- **do not depend on its internal packages** from Vel runtime code
- **do not use it for shared or hosted multi-user deployments**
- **do use it** as a local proxy process Vel can talk to via an OpenAI-compatible adapter

This cleanly avoids most licensing and operational ugliness.

## Desired architecture

```text
Vel agent
  -> vel-llm RouterProvider
    -> OpenAiOauthProvider
      -> http://127.0.0.1:8014/v1
        -> openai-oauth local proxy
          -> chatgpt.com/backend-api/codex/responses
```

## Why a dedicated provider wrapper still matters

You could point a generic OpenAI-compatible client at the proxy and call it a day. Resist that temptation.

A dedicated wrapper lets you:

- enforce `localhost` only
- require an explicit `enable_openai_oauth = true`
- surface clearer telemetry and policy labels
- fail closed when auth files are missing or stale
- quarantine backend-specific quirks from the rest of the system

## Provider profile

```toml
id = "oauth-openai"
provider = "openai_oauth"
base_url = "http://127.0.0.1:8014/v1"
model = "gpt-5.4"
context_window = 128000
max_output_tokens = 4096
supports_tools = true
supports_json = true
supports_reasoning = true
latency_class = "deep"
locality = "remote_via_local_proxy"
data_policy = "redact_first"
enabled = false
```

## Required safeguards

### 1. Localhost-only binding
Bind the proxy to `127.0.0.1`, not `0.0.0.0`.

### 2. Explicit feature flag
Require both:

- config flag: `enable_openai_oauth = true`
- health preflight: auth file exists and proxy responds

### 3. Secret hygiene
Never log:

- auth file paths with secrets embedded
- refresh tokens
- raw headers
- full upstream response payloads when traces may contain sensitive content

### 4. Redaction policy
Vel should default to **redacting or summarizing private context** before sending tasks to this backend.

### 5. Usage labeling
Tag every request with provider metadata so evals and telemetry can distinguish:

- local model success
- local model fallback
- OAuth backend usage rate

## Startup script

Use a repo-managed script like:

```bash
#!/usr/bin/env bash
set -euo pipefail

HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-8014}"
MODELS="${MODELS:-gpt-5.4,gpt-5.3-codex}"

exec npx openai-oauth \
  --host "$HOST" \
  --port "$PORT" \
  --models "$MODELS"
```

The auth bootstrap stays separate:

```bash
npx @openai/codex login
```

This separation is good. Authentication is a human consent boundary, not something Vel should silently drive.

## Healthcheck

Vel should check:

- `GET /v1/models`
- verify at least one configured model is present
- optionally mark backend degraded if the proxy is up but target model is absent

## When to route to this backend

Use it for:

- comparative evals against local models
- difficult coding/planning tasks where the user explicitly opts in
- temporary overflow when a local model is unhealthy

Do not use it for:

- default background autonomy
- sensitive local knowledge by default
- server-hosted or multi-user environments
