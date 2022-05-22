# Dockerfile for zero2prod
FROM lukemathwalker/cargo-chef:latest-rust-1.60.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Create a "lock-like" file for the project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build the dependencies, not the project
RUN cargo chef cook --release --recipe-path recipe.json
# Until this point, if dependencies are unchanged
# all layers should be cached

# Copy all files from the env to the Docker image
COPY . .

# Add offline sqlx capabilities
ENV SQLX_OFFLINE true

# commands to build the binary
RUN cargo build --release --bin zero2prod


# RUNTIME STAGE
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# Install OpenSSL, dynamically linked to some of our dependencies
# Install ca-certs, needed to verify TLS 
# when using HTTPS
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*


# Copy the compiled binary from the builder env
# into the runtime env
COPY --from=builder /app/target/release/zero2prod zero2prod

# We will need the configuration file
COPY configuration configuration

# get the APP_ENVIRONMENT
ENV APP_ENVIRONMENT production

# run the following command at the CLI
ENTRYPOINT ["./zero2prod"]
