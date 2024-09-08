# Build stage
FROM rust:1.81.0-bullseye as builder

WORKDIR /app

COPY . . 

# Build the application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

# Install PostgreSQL client libraries (libpq)
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/ACTIX_REST_MVC .

# Expose the port your app listens on
EXPOSE 8080

# Command to run the application
CMD ["./ACTIX_REST_MVC"]
