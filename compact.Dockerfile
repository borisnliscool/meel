FROM rust:1.88-slim-bookworm AS builder

RUN apt update && apt install -y \
    libpq-dev libssl-dev pkg-config

WORKDIR /usr/src/meel

COPY backend/src src/
COPY backend/Cargo.toml Cargo.toml
COPY backend/Cargo.lock Cargo.lock
COPY LICENSE LICENSE

RUN cargo build --release

FROM rust:1.88-slim-bookworm AS diesel-builder
RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config
RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 libssl3 curl xz-utils postgresql postgresql-contrib \
  && rm -rf /var/lib/apt/lists/*

ENV PATH="/usr/lib/postgresql/15/bin:/usr/local/bin:${PATH}"

COPY --from=diesel-builder /usr/local/cargo/bin/diesel /usr/local/bin/
COPY --from=builder /usr/src/meel/target/release/meel /usr/local/bin/meel
COPY backend/migrations migrations/

RUN mkdir -p /var/lib/postgresql/data && chown -R postgres:postgres /var/lib/postgresql

ENV POSTGRES_USER=meel
ENV POSTGRES_PASSWORD=password
ENV DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost/meel

LABEL org.opencontainers.image.source=https://github.com/borisnliscool/meel

USER postgres

CMD ["/bin/bash", "-c", "\
  if [ ! -s /var/lib/postgresql/data/PG_VERSION ]; then \
    initdb -D /var/lib/postgresql/data; \
  fi && \
  pg_ctl -D /var/lib/postgresql/data -o \"-c listen_addresses=localhost\" -w start && \
  psql -v ON_ERROR_STOP=1 --username=postgres --dbname=postgres -c \"CREATE USER ${POSTGRES_USER} WITH PASSWORD '${POSTGRES_PASSWORD}' CREATEDB;\" -c \"CREATE DATABASE meel OWNER ${POSTGRES_USER};\" || true && \
  diesel migration run && \
  /usr/local/bin/meel \
"]
