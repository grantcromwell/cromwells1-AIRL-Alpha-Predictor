FROM rust:1.75-alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev libgcc libstdc++

COPY Cargo.toml .
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.19

WORKDIR /app

RUN mkdir /logs && chown -R nobody:nobody /logs

# Copy binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-model rust-model

# Install runtime dependencies (if any)
RUN apk add --no-cache libgcc libstdc++

USER nobody

CMD ["./rust-model"]
