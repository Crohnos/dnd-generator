# Build stage
FROM rust:1.74-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Create app directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this is cached if Cargo files don't change)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Create non-root user
RUN adduser -D -u 1001 appuser

# Copy binary from builder
COPY --from=builder /app/target/release/dnd-campaign-generator /usr/local/bin/dnd-campaign-generator

# Change ownership
RUN chown appuser:appuser /usr/local/bin/dnd-campaign-generator

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3001

# Run the binary
CMD ["dnd-campaign-generator"]