FROM rust:1.94-bookworm AS build

ARG APP_NAME=dioxus-app

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY Cargo.toml Cargo.lock Dioxus.toml ./
COPY assets ./assets
COPY src ./src
COPY tailwind.css ./tailwind.css

# Match the CLI to the resolved crate version in Cargo.lock to avoid
# dx/dioxus asset-format mismatches during fullstack builds.
RUN DIOXUS_VERSION="$(awk '/name = "dioxus"/ { getline; sub(/^version = "/, "", $0); sub(/"$/, "", $0); print $0; exit }' Cargo.lock)" \
    && cargo install dioxus-cli --version "${DIOXUS_VERSION}" --locked

RUN dx build --release --platform web --fullstack --locked \
    @server --platform server --features server \
    @client --platform web --no-default-features --features web

FROM debian:bookworm-slim AS runtime

ARG APP_NAME=dioxus-app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /srv/app
COPY --from=build /app/target/dx/${APP_NAME}/release/web/ ./

ENV IP=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./server"]
