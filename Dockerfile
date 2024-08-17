FROM rust:1.79-slim-buster

RUN apt-get update
RUN apt-get install libpq-dev libssl-dev pkg-config -y

WORKDIR /usr/src/meel

COPY src/ src/
COPY migrations/ migrations/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

EXPOSE 3000
CMD ["./target/release/meel"]