# Dockerfile for zero2prod

FROM rust:1.60.0

# Create a working directory to build the app into
# If the folder doesn't already exist, Docker will create the app directory 
# for us
WORKDIR /app

# Install required deps
RUN apt update && apt install lld clang -y

# Copy all files from the env to the Docker image
COPY . .

# commands to build the binary
RUN cargo build --release

# run the following command at the CLI
ENTRYPOINT ["./target/release/zero2prod"]
