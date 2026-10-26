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
# present for cargo to resolve the workspace, but only example-server and
# its deps are compiled — tauri crates never build here.
COPY mobile/tauri/Cargo.toml mobile/tauri/Cargo.lock mobile/tauri/
COPY mobile/tauri/std/api mobile/tauri/std/api
COPY mobile/tauri/std/rs/Cargo.toml mobile/tauri/std/rs/
COPY mobile/tauri/example/app/src-tauri/Cargo.toml mobile/tauri/example/app/src-tauri/
COPY mobile/tauri/example/server/Cargo.toml mobile/tauri/example/server/build.rs mobile/tauri/example/server/

# Create dummy src for dependency caching
RUN mkdir -p mobile/tauri/std/rs/src \
             mobile/tauri/example/app/src-tauri/src \
             mobile/tauri/example/server/src \
             mobile/tauri/example/server/migrations && \
    echo "" > mobile/tauri/std/rs/src/lib.rs && \
    echo "" > mobile/tauri/example/app/src-tauri/src/lib.rs && \
    echo "fn main() {}" > mobile/tauri/example/server/src/main.rs && \
    echo "" > mobile/tauri/example/server/src/lib.rs

WORKDIR /build/mobile/tauri
RUN cargo build --release -p example-server 2>/dev/null || true

# Copy real source (templates and static are compiled into the binary)
COPY mobile/tauri/example/server/src example/server/src
COPY mobile/tauri/example/server/migrations example/server/migrations
COPY mobile/tauri/example/server/templates example/server/templates
COPY mobile/tauri/example/server/static example/server/static

# Build for real (touch so cargo notices the dummy files were replaced)
RUN touch example/server/src/main.rs example/server/src/lib.rs && \
    cargo build --release -p example-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/mobile/tauri/target/release/example-server /usr/local/bin/example-server

WORKDIR /data

ENV LISTEN_ADDRESS=0.0.0.0:3000
ENV SQLITE_DATABASE_URL=sqlite:///data/app.db
ENV LOG_LEVEL=info

EXPOSE 3000

CMD ["example-server"]
