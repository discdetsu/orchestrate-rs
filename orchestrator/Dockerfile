# Stage 1: Build the Rust application
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire project into the container
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust_orchestrator .

# Specify the entry point for the container
CMD ["./rust_orchestrator"]