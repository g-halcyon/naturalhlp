FROM rust:1.76-slim as builder

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bullseye-slim

WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/dshpc /usr/local/bin/dshpc

# Set the entrypoint
ENTRYPOINT ["dshpc"]

# Default command
CMD ["--help"] 