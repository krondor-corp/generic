# Build context is the repo root: this crate depends on
# web/rust/crates/core by path, and that crate inherits edition/license
# from web/rust's workspace manifest.
#
# Build stage
FROM rust:1.99-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# generic-rust-core + just enough of its workspace for inheritance to resolve
COPY web/rust/Cargo.toml web/rust/Cargo.toml
COPY web/rust/crates/app/Cargo.toml web/rust/crates/app/Cargo.toml
COPY web/rust/crates/core web/rust/crates/core

# Mobile workspace manifests first for layer caching. Every member must be
# present for cargo to resolve the workspace, but only public-web-api and
# its deps are compiled — tauri crates never build here.
COPY mobile/tauri/Cargo.toml mobile/tauri/Cargo.lock mobile/tauri/
COPY mobile/tauri/std/api mobile/tauri/std/api
COPY mobile/tauri/std/rs/Cargo.toml mobile/tauri/std/rs/
COPY mobile/tauri/public-web-api/app/src-tauri/Cargo.toml mobile/tauri/public-web-api/app/src-tauri/
COPY mobile/tauri/public-web-api/server/Cargo.toml mobile/tauri/public-web-api/server/build.rs mobile/tauri/public-web-api/server/

# Create dummy src for dependency caching
RUN mkdir -p mobile/tauri/std/rs/src \
             mobile/tauri/public-web-api/app/src-tauri/src \
             mobile/tauri/public-web-api/server/src \
             mobile/tauri/public-web-api/server/migrations && \
    echo "" > mobile/tauri/std/rs/src/lib.rs && \
    echo "" > mobile/tauri/public-web-api/app/src-tauri/src/lib.rs && \
    echo "fn main() {}" > mobile/tauri/public-web-api/server/src/main.rs && \
    echo "" > mobile/tauri/public-web-api/server/src/lib.rs

WORKDIR /build/mobile/tauri
RUN cargo build --release -p public-web-api 2>/dev/null || true

# Copy real source
COPY mobile/tauri/public-web-api/server/src public-web-api/server/src
COPY mobile/tauri/public-web-api/server/migrations public-web-api/server/migrations

# Build for real (touch so cargo notices the dummy files were replaced)
RUN touch public-web-api/server/src/main.rs public-web-api/server/src/lib.rs && \
    cargo build --release -p public-web-api

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/mobile/tauri/target/release/public-web-api /usr/local/bin/public-web-api

WORKDIR /data

ENV LISTEN_ADDRESS=0.0.0.0:3000
ENV SQLITE_DATABASE_URL=sqlite:///data/app.db
ENV LOG_LEVEL=info

EXPOSE 3000

CMD ["public-web-api"]
