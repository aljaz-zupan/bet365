FROM rust:1.72-alpine
# Copy source code
WORKDIR /app
COPY . .
# Build the application
RUN cargo build --release
CMD [ "./target/release/bet365" ]