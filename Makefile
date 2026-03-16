# Vel — top-level build and dev
# veld binds 127.0.0.1:4130 by default; web client uses VITE_API_URL (default http://localhost:4130).

.PHONY: build build-api build-web dev dev-api dev-web seed clean

build: build-api build-web

build-api:
	cargo build -p veld

build-web:
	cd clients/web && npm ci && npm run build

# Run API only (migrations run on startup). Use in one terminal.
dev-api:
	cargo run -p veld

# Run web dev server only. Set VITE_API_URL if veld is not on localhost:4130. Use in another terminal.
dev-web:
	cd clients/web && npm run dev

# Run both: veld in background, then web dev server. Ctrl+C stops both. Requires bash.
dev:
	@bash scripts/dev.sh

# Seed sample chat data. Requires veld running (default http://localhost:4130).
seed:
	cd clients/web && npm run seed

clean:
	cargo clean
	rm -rf clients/web/node_modules clients/web/dist
