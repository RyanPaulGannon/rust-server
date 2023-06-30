# Build Stage
FROM rust:latest as builder

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

EXPOSE 8000

# Start the app
CMD ["./rust-server"]
