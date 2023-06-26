# Build Stage
FROM rust:1.69-buster as builder

WORKDIR /app

COPY . .

# Accept the build argument
ARG MONGODB_URI
ENV MONGODB_URI=$MONGODB_URI

RUN cargo build --release

# Production Stage
FROM debian:buster-slim

WORKDIR /usr/local/bin 

COPY --from=builder /app/target/release/rust-server .

# Pass the environment variable to the production stage
ENV MONGODB_URI=$MONGODB_URI

# Start the app
CMD ["./rust-server"]
