# rust-learning project
## Description
Personal project for learning the fundamentals of the low end language Rust

## Learning Resource
https://doc.rust-lang.org/book

## Useful Introduction Notes
* Rust style is to indent with four spaces, not a tab.
* `println!` calls a Rust macro. If it had called a function instead, it would be entered as println (without the `!`).
    * `!` means that you’re calling a **macro** instead of a normal function and that macros **don’t always** follow the same rules as functions.
* Expreossions are ended with semi colons (I will be happy with this coming from PHP background)
* Cargo is a package dependency manager for Rust. These packages are called **Crates**.

## Cargo
* Package management file is named `cargo.toml`

## Debugging
* VS code has rust analyzer extension available which checks code on the fly prior to being compiled to avoid wasting time. https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Fun Facts
* Rust code can be run on any computer without even having rust installed
* Cargo commands are consistent across all OS

## Questions wanting answered during learning
### What is a rust *macro*?

### Using Cargo
#### Create a new project
```
cargo new
```
##### Or to set a VCS repo as the root dir
```
cargo new --vcs=git
```

#### Building/ Compiling code
```
cargo build
```
##### Building for release (This will include optimizations but takes longer to compile)
```
cargo build --release
```

#### Running code (If the rust program is already installed) 
```
cargo run
```

#### Check code compiles without generating file
```
cargo check
```

### What is a *Toml* file?
#### Definition: Tom’s Obvious, Minimal Language
URL: https://toml.io/en/
* Config file made simplistic for humans
* It prioritizes humans over robots

#### Example:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
#### Explanation
* `[package]`, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.
* The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use. 
* `[dependencies]`, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. 

### How can *crates* (Packages) be installed?
