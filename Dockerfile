FROM messense/rust-musl-cross:x86_64-musl as builder
# Copy source code
WORKDIR /bet365
COPY . .
# Build the application
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /bet365/target/x86_64-unknown-linux-musl/release/bet365 /bet365
ENTRYPOINT [ "/bet365" ]