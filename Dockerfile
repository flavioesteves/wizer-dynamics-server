# Builde stage

FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 as chef
WORKDIR /app

RUN apt update && apt install lld clang -y

# Copy all files form our working environment to our Docker Image
FROM chef as planner
COPY . .

# Compute a lock-file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up this point, if our dependecy tree stays the same,
# all layers should be cached
COPY . .
RUN cargo build --release --bin server_wizer

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app
# Install OpenSSL - it is dyanmically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS ca-certificates
# when establishing HTPPS Connections

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/server_wizer server_wizer

# WE need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production

# When `docker run` is executed, lauch binary!
ENTRYPOINT ["./server_wizer"]
