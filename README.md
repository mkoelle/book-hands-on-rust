# Hands on Rust Workshop

- [Hands on Rust source code](https://github.com/thebracket/HandsOnRust)

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

## Chapter 2: first steps in Rust

- Variables on a line without a semicolon are returned from a function.
  - The return keyword is optional in Rust.
- Passing variables to functions moves ownership by default.
  - Use references (&) to borrow variables without taking ownership.

String formatting with `format!` macro:

- `{foo:?}` for debug formatting
- `{foo:#?}` for pretty debug formatting
- `{foo}` for display formatting

Rust has two kinds of strings:

- `String` (heap-allocated, growable)
- `str` immutable string slice (fixed size, usually borrowed)

Numerical lists do not include the last number in the list, unless specified with `=`:

- `0..5` includes 0, 1, 2, 3, 4
- `0..=5` includes 0, 1, 2, 3, 4, 5

Closures are anonymous functions that can capture variables from their surrounding scope.

- Syntax: `|parameters| { body }`
- Example: `let add_one = |x: i32| x + 1;`

Find returns an Option type:

- `Some(value)` if found
- `None` if not found

when declaring a struct, you can add `#[derive(Debug)]` above it to enable debug printing.

Enums can have data and even functions associated with each variant.

Rust uses traits to define shared behavior. this is different from inheritance by allowing types to implement shared functionality without a strict class hierarchy.

## Chapter 3: First game with rust

The main loop is able to use the `?` operator to propagate errors up the call stack, as long as the main function returns a Result type.
