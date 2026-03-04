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

when using vectors, `map` is evaluated lazily, so if you want to execute it immediately, you need to call `collect()` at the end, or use `for_each` to iterate over the results.

## Chapter 4: Design a dungeon crawler

Must read Appendix A2 for this chapter.

## Chapter 5: Build a dungeon crawler

How does the prelude work?

- `mod::prelude` is a convention to group together multiple imports, functions, types, and other items; so that they can be easily imported by other modules

How does derive work?
- automatically generates implementations for certain traits on structs or enums
- it is possible to create a custom derive macro, but we haven't covered that yet

What does impl x for y do?

### Working with loops and rendering

#### `render` — Nested Loops (FASTEST ⭐)

Uses nested `for` loops to iterate coordinates and calculate indices with `map_idx()`.

- Cheapest operations: multiplication and addition
- No closures, no branches
- **Best for performance-critical rendering loops**

```rust
pub fn render(&self, ctx: &mut BTerm) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);
            match self.tiles[idx] {
                TileType::Floor => {
                    ctx.set(x, y, GREEN, BLACK, to_cp437('.'));
                }
                TileType::Wall => {
                    ctx.set(x, y, WHITE, BLACK, to_cp437('#'));
                }
            }
        }
    }
}
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
```

#### `render2` — Iterator with Closure (SLOWEST)

Iterates tiles directly and converts flat indices to coordinates with `xy_idx()`.

- Uses `for_each` closure (adds indirection overhead)
- More expensive operations: modulo (`%`) and division (`/`)
- More idiomatic Rust, cleaner syntax
- **Sacrifices ~15-20% performance for code style**

```rust
pub fn render2(&self, ctx: &mut BTerm) {
    self.tiles.iter().enumerate().for_each(|(i, t)| {
        let (x, y) = xy_idx(i);
        match t {
            TileType::Floor => {
                ctx.set(x, y, LIGHT_GREEN, BLACK, to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, WHITE, BLACK, to_cp437('#'));
            }
        }
    });
}
pub fn xy_idx(idx: usize) -> (i32, i32) {
    let x = (idx as i32) % SCREEN_WIDTH;
    let y = (idx as i32) / SCREEN_WIDTH;
    (x, y)
}
```

#### `render2_refactored` — Iterator without Closure (MIDDLE)

Same algorithm as `render2`, but replaces `for_each` with a standard `for` loop.

- Removes closure overhead
- Still uses expensive modulo/division operations
- Better than original `render2`, but still slower than `render`

```rust
pub fn render2_refactored(&self, ctx: &mut BTerm) {
    for (i, t) in self.tiles.iter().enumerate() {
        let (x, y) = xy_idx(i);
        match t {
            TileType::Floor => {
                ctx.set(x, y, LIGHT_GREEN, BLACK, to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, WHITE, BLACK, to_cp437('#'));
            }
        }
    }
}
pub fn xy_idx(idx: usize) -> (i32, i32) {
    let x = (idx as i32) % SCREEN_WIDTH;
    let y = (idx as i32) / SCREEN_WIDTH;
    (x, y)
}
```

#### `render3` — Manual Coordinate Tracking with Closure (SLOW)

Manually tracks `x` and `y` coordinates, wrapping with a conditional check on every tile.

- Uses `for_each` closure (indirection overhead)
- Branch check on every iteration (predictable, but still a cost)
- Attempts to avoid expensive division/modulo operations

```rust
pub fn render3(&self, ctx: &mut BTerm) {
    let (mut x, mut y) = (0, 0);
    self.tiles.iter().enumerate().for_each(|(i, t)| {
        if x >= SCREEN_WIDTH {
            y += 1;
            x = 0;
        }
        match t {
            TileType::Floor => {
                ctx.set(x, y, LIGHT_GREEN, BLACK, to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, WHITE, BLACK, to_cp437('#'));
            }
        }
        x += 1;
    });
}
```

#### `render3_refactored` — Manual Coordinate Tracking without Closure (SECOND FASTEST)

Same approach as `render3`, but replaces `for_each` with a standard `for` loop and uses proper `i32` types.

- Removes closure overhead significantly
- Branch check is predictable (modern CPUs handle well)
- Much closer to `render` performance
- Slightly more verbose than `render`

```rust
pub fn render3_refactored(&self, ctx: &mut BTerm) {
    let (mut x, mut y) = (0i32, 0i32);
    for t in self.tiles.iter() {
        if x >= SCREEN_WIDTH {
            y += 1;
            x = 0;
        }
        match t {
            TileType::Floor => {
                ctx.set(x, y, LIGHT_GREEN, BLACK, to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, WHITE, BLACK, to_cp437('#'));
            }
        }
        x += 1;
    }
}
```

#### Performance Ranking (Best to Worst)

1. **`render`** — Fastest (tight nested loops, cheap arithmetic, no branches)
2. **`render3_refactored`** — Second (minimal overhead, predictable branch)
3. **`render2_refactored`** — Third (no closure, but expensive modulo/division)
4. **`render3`** — Fourth (closure overhead + branch)
5. **`render2`** — Slowest (closure overhead + expensive operations)

#### Key Takeaways

- **For game development**: Use `render` — the performance difference matters when rendering every frame
- **For readability vs performance trade-off**: Use `render3_refactored` — close to `render` performance with cleaner code
- **For pure idiomatic Rust**: Use `render2_refactored` — good performance with iterator-based style
- Avoid `for_each` closures in performance-critical loops; standard `for` loops are nearly always faster

```

```
