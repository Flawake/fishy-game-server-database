# Use the official Rust image as the build environment
FROM rust:latest AS builder

# Create a new directory for the app
WORKDIR /usr/src/server

# turn sqlx verify off
ENV SQLX_OFFLINE=true

# Copy the source code
COPY . .

# Build your Rust project in release mode
RUN cargo build --release


# Use a minimal image for the final container
FROM debian:bookworm-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/server/target/release/backend /usr/local/bin/backend

# Set the startup command to run your binary
CMD ["backend"]

