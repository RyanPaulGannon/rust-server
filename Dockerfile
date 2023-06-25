FROM rust:latest

WORKDIR /app

COPY . .

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

RUN cargo build --release

CMD ./target/release/rust-server
