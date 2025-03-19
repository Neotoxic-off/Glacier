FROM rust:1.85 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

FROM debian:12.10-slim

WORKDIR /app

COPY --from=builder /app/target/release/agent .
RUN chmod +x agent

ENTRYPOINT ["./agent"]
