# ── Stage 1: Build ───────────────────────────────────────────────
FROM rust:1.94-bookworm AS build

RUN cargo install dioxus-cli --locked \
    && rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Fullstack build: produces a server binary that serves the WASM
# client and handles any server functions.
RUN dx build --release

# ── Stage 2: Runtime ─────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/dx/dioxus-app/release/web /srv/app

WORKDIR /srv/app

ENV IP=0.0.0.0
ENV PORT=8080
EXPOSE 8080

CMD ["./server"]
