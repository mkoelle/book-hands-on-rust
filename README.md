# Hands on Rust Workshop

## Chapter 1: Getting Started with Rust

### Part 1: Rust and your development environment

Projects Created (located in the src/ch1 directory):

- hello
- testrust
- clippy

Commands covered:

```sh
cargo new <project-name>   # Create a new Rust project
cargo build                # Build the project
cargo run                  # Build and run the project
cargo check                # Check the project for errors without building
cargo clean                # Clean the build artifacts
cargo fmt                  # Format the code using rustfmt

# The above commands run in debug mode by default. To run in release mode, use:
cargo build --release
cargo run --release

rustup check             # Check for updates to Rust toolchain
rustup update            # Update Rust toolchain to the latest version
```
