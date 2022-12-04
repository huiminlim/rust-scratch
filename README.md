# Rust Scratchpad

## Cargo

Cargo is a package manager for Rust. It has some useful commands.

### Setup

To create a new project from scratch, use the following commands.

```cargo
cargo new project-name
cd project-name
cargo run .
```

This command creates a directory with the source code and allows you to compile and execute the binary.

If the directory already exists, use the `init` command instead.

```cargo
cargo init project-name
cd project-name
cargo run .
```

### Running Programs

To compile and run a program, use the `run` command.

This command is to be run in a directory with `Cargo.toml` file.

```cargo
cargo run --bin program
```

## Memory Ownership

Rust uses a memory "ownership" model, where the owner of the memory is in charge of the memory.

The memory can be moved or borrowed.

```rust
    // Moving memory
    enum Light {
        Bright,
        Dull,
    }
    fn do_something(light: Light) {
        match light {
            Light::Bright => println!("Bright"),
            Light::Dull => println!("Dull"),
        };
        // Deletes the light variable from main() function
    }
    let dull = Light::Dull;
    do_something(dull); // moves the variable in, and deletes after fn
    do_something(dull); // WARNING: not allowed! Compiler complaints
```
