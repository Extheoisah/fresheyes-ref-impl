FROM rust:1.76.0-slim-buster as builder

WORKDIR /usr/src/fresh_eyes_api

COPY . .

# Install the required dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libpq-dev \
    libssl-dev \
    libz-dev

# Install the required target
RUN rustup target add x86_64-unknown-linux-gnu

# Build the application
RUN cargo build --release --bin fresh_eyes_api

# Run the application
ENTRYPOINT ["./target/release/fresh_eyes_api"]

EXPOSE 8080
