# Use Rust official image
FROM rust:latest

# Set working directory
WORKDIR /app/orchestrator

# Copy Rust project files
COPY . .

# Build the Rust binary
RUN cargo build --release

# Set the default command to run the compiled binary
CMD ["./target/release/rust_orchestrator"]