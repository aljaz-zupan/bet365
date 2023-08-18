FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bet365-notificator /bet365-notificator
ENTRYPOINT [ "/bet365-notificator" ]
EXPOSE 3003