FROM rust:slim-buster

# Set rust to compile to native
ENV RUSTFLAGS="-C target-cpu=native"

# Install application into container
COPY . .

# Build release
RUN ["cargo", "build", "--release"]

# Run the executable
ENTRYPOINT ["./target/release/collatz"]