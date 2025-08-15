FROM rust:1.88-slim-bookworm AS builder

RUN apt update && apt install libpq-dev libssl-dev pkg-config -y

WORKDIR /usr/src/meel

COPY backend/src src/
COPY backend/Cargo.toml Cargo.toml
COPY backend/Cargo.lock Cargo.lock
COPY LICENSE LICENSE

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update && apt install libpq-dev libssl-dev pkg-config curl xz-utils -y
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

COPY --from=builder /usr/src/meel/target/release/meel /usr/local/bin/meel
COPY backend/migrations migrations/

LABEL org.opencontainers.image.source=https://github.com/borisnliscool/meel

CMD ["/bin/sh", "-c", "/root/.cargo/bin/diesel migration run && /usr/local/bin/meel"]