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
