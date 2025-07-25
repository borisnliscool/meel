FROM rust:1.88-slim-bullseye

RUN apt update && apt install libpq-dev libssl-dev pkg-config -y

WORKDIR /usr/src/meel

COPY backend/src src/
COPY backend/migrations migrations/
COPY backend/Cargo.toml Cargo.toml
COPY backend/Cargo.lock Cargo.lock
COPY LICENSE LICENSE

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

LABEL org.opencontainers.image.source=https://github.com/borisnliscool/meel

CMD ["/bin/sh", "-c", "diesel migration run && target/release/meel"]