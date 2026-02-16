FROM rust:1.85-slim AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY benches/ benches/

RUN cargo build --release --features cli && \
    strip target/release/mkdlint

FROM debian:bookworm-slim

COPY --from=builder /build/target/release/mkdlint /usr/local/bin/mkdlint

WORKDIR /work

ENTRYPOINT ["mkdlint"]
CMD ["."]
