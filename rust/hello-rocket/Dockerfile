FROM rust:1.87 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ ./src/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/hello-rocket /usr/local/bin/hello-rocket
USER 1001
EXPOSE 10000
CMD ["hello-rocket"]
