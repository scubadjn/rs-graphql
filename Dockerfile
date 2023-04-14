# Using official rust base image
FROM rust

# Set the application directory
WORKDIR /app

# Install cargo-watch
RUN cargo install cargo-watch
