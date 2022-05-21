# Dockerfile for zero2prod

# BUILDER STAGE
FROM rust:1.60.0 AS builder

# Create a working directory to build the app into
# If the folder doesn't already exist, Docker will create the app directory 
# for us
WORKDIR /app

# Install required deps
RUN apt update && apt install lld clang -y

# Copy all files from the env to the Docker image
COPY . .

# Add offline sqlx capabilities
ENV SQLX_OFFLINE true

# commands to build the binary
RUN cargo build --release


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
ENTRYPOINT ["/zero2prod"]
