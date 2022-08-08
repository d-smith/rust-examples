# Notes from [The Rust Programming Language](https://doc.rust-lang.org/book/)


## Conventions

* Use underscores in filenames as needed, e.g. hello_world.rs
* Use 4 spaces for indent
* Constants are ALL_UPPER_CASE_WITH_UNDERSCORES


## Tools

Compile: rustc main.rs

Cargo

* Cargo is the rust package manager
* Bootstrap program via `cargo new hello_cargo`
* cargo build, cargo run
* cargo check - compile check, but does not create executable
* cargo build --release when building for release

## Crates

* A create is a collection of Rust source code
* Built projects are binary crates
* Library creates - crates other programs can use
    * Declare as dependencies in the toml file
    * Downloaded fron crates.io


