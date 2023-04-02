FROM rust:slim-buster

# Install application into container
COPY . .

# Compile native release build
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run the executable
ENTRYPOINT ["./target/release/collatz"]