FROM amazonlinux:2023

# Install application into container
COPY . .

# Install Rust
RUN yum install -y cargo

# Compile native release build
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run the executable
ENTRYPOINT ["./target/release/collatz"]