FROM rust:1.86-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY migrations ./migrations
COPY config ./config
COPY configs ./configs

RUN cargo build --release -p veld -p vel-cli

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --uid 10001 vel

WORKDIR /app

COPY --from=builder /app/target/release/veld /usr/local/bin/veld
COPY --from=builder /app/target/release/vel /usr/local/bin/vel
COPY --from=builder /app/config ./config
COPY --from=builder /app/configs ./configs
COPY --from=builder /app/migrations ./migrations

RUN mkdir -p /data/db /data/artifacts /data/logs \
    && chown -R vel:vel /app /data

USER vel

ENV VEL_BIND_ADDR=0.0.0.0:4130
ENV VEL_BASE_URL=http://127.0.0.1:4130
ENV VEL_DB_PATH=/data/db/vel.sqlite
ENV VEL_ARTIFACT_ROOT=/data/artifacts
ENV VEL_LOG_LEVEL=info
ENV VEL_AGENT_SPEC_PATH=/app/config/agent-specs.yaml
ENV VEL_MODELS_DIR=/app/configs/models
ENV VEL_POLICIES_PATH=/app/config/policies.yaml

EXPOSE 4130

CMD ["veld"]
