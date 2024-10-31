FROM rust:1.79-slim-buster

RUN apt-get update
RUN apt-get install libpq-dev libssl-dev pkg-config -y

WORKDIR /usr/src/meel

COPY backend/src src/
COPY backend/migrations migrations/
COPY backend/Cargo.toml Cargo.toml
COPY backend/Cargo.lock Cargo.lock
COPY LICENSE LICENSE

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

CMD ["/bin/sh", "-c", "diesel migration run && target/release/meel"]
