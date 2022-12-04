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

The owner of a memory must clean up the memory and the owner is determined by the compiler using move or borrow.

```rust
// Moving memory
enum Light {
    Bright,
    Dull,
}
fn do_something(light: Light) {
    // Owner of the "light"/"dull" variable is now fn do_something()
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

To pass in a borrowed variable, use the `&` syntax.

```rust
fn do_something(light: &Light) {
    // Owner of the "light"/"dull" variable is still the main()
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    };
    // Deletes the light variable from main() function
}
do_something(&dull); // moves the variable in, and deletes after fn
do_something(&dull); // Legal
```

## Strings

There are 2 types of strings used

- `String` are owned strings.
- `&str` are borrowed `String` slices.

To store in a `struct`, the `String` must be owned.

And `&str` must be used to pass to a function.

## Option

The option type represents some type with data that might not exist yet.

```rust
// Definition
enum Option<T> {
    Some(T),
    None
}
```

## Documentation

Using 3 slashes to comment will generate a documentation for the program along with these comments.

```rust
/// Example
fn hello(){
}
```

To generate documentation, use the following cargo command

```cargo
cargo doc
```
