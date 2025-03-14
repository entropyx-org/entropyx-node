# Build stage
FROM rust:1.82-slim-bullseye as builder

WORKDIR /app
COPY . .

# Install build dependencies including protobuf compiler
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    build-essential \
    pkg-config \
    libclang-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Build the release binaries
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binaries from builder
COPY --from=builder /app/target/release/entropyx /usr/local/bin/

# Set the entrypoint script
COPY --from=builder /app/target/release/entropyx /usr/local/bin/entropyx

# Default command (can be overridden)
ENTRYPOINT ["entropyx"]
CMD ["--help"]
