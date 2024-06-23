FROM rust:1.79-slim-buster

RUN apt-get update
RUN apt-get install libpq-dev -y

COPY ./ ./
RUN cargo build --release

EXPOSE 3000
CMD ["./target/release/meel"]