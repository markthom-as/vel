# Vel — top-level build and dev
# veld binds 127.0.0.1:4130 by default; web client uses VITE_API_URL (default http://localhost:4130).

.PHONY: build build-api build-web clean dev dev-api dev-web download-chat-model install-web lint-web seed smoke test test-api test-web verify verify-repo-truth ci fmt-check clippy-check bootstrap-demo-data

build: build-api build-web

build-api:
	cargo build -p veld

build-web: install-web
	cd clients/web && npm run build

# Run API only (migrations run on startup). Use in one terminal.
dev-api:
	cargo run -p veld

# Run web dev server only. Set VITE_API_URL if veld is not on localhost:4130. Use in another terminal.
dev-web:
	cd clients/web && npm run dev

# Run both: veld in background, then web dev server. Ctrl+C stops both. Requires bash.
dev:
	@bash scripts/dev.sh

# Install frontend dependencies.
install-web:
	cd clients/web && npm ci

# Frontend lint.
lint-web: install-web
	cd clients/web && npm run lint

# Lint and verification.
fmt-check:
	cargo fmt --all -- --check

clippy-check:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

verify-repo-truth:
	node scripts/verify-repo-truth.mjs

verify: verify-repo-truth fmt-check clippy-check lint-web test
	@true

# Download default chat model (Qwen2.5-1.5B-Instruct, ~1.1 GB) to configs/models/weights/. After this, make dev uses it when llama-server is on PATH.
download-chat-model:
	@bash scripts/download-chat-model.sh

# Seed sample chat data. Requires veld running (default http://localhost:4130).
seed:
	cd clients/web && npm run seed

test: test-api test-web

test-api:
	cargo test --workspace --all-features

test-web: install-web
	cd clients/web && npm run test

# Full repo check target for local CI parity.
ci: verify build-web

# Smoke test for daemon/API/CLI.
smoke:
	./scripts/ci-smoke.sh

bootstrap-demo-data:
	./scripts/bootstrap-demo-data.sh

clean:
	cargo clean
	rm -rf clients/web/node_modules clients/web/dist
