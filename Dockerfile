FROM rust:latest as builder

WORKDIR /usr/src/app
COPY Cargo.toml ./
COPY src ./src

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/lucius-v ./lucius-v

# Runtime image
FROM debian:stable-slim

# Run as "lucius" user
RUN useradd -ms /bin/bash lucius
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev
RUN apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

USER lucius
WORKDIR /home/lucius

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/lucius-v /home/lucius/lucius-v

# Run the app

CMD ["./lucius-v"]