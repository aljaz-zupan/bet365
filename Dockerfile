FROM messense/rust-musl-cross:x86_64-musl AS builder
# Copy source code
WORKDIR /app
COPY . .
# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with minimal image
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bet365 /bet365
ENTRYPOINT [ "bet365" ]