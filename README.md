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
* To obtain user input we need to `use std::io;` package which can be put at the top of the file
* By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. **This set is called the `prelude`**
    * https://doc.rust-lang.org/std/prelude/index.html
* Constants must be capitalized
* Rust uses `Snake Case` as the conventional style for function naming (lowercase and underscores separate words)
* function parameters ***must*** declare the data type
* Rust ***does not use*** `inheritance`, instead it uses `traits`
* Rust is an **expression-based** language
* `Expressions` do not include ending semicolons (x + 1)
* `Expressions` are the same as an `return` in PHP
* In functions, we don’t name return values, but we must declare their type after an arrow 
* `Loops` can have `loop labels` to specify what the loop is doing if multidementional
* ~~`Foreach`~~ loops do not exist. Instead you should use for loops with using a `range` package.
* Rust Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
* Accessing data in the `heap` is *slower* than accessing data on the `stack` because **you have to follow a pointer to get there**.
* Local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
* When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.
* `integers` that have a known size at compile time are stored entirely on the stack. Therefore can be copied directly into other vars (because there’s no difference between deep and shallow copying in this use case)

## Debugging
* VS code has rust analyzer extension available which checks code on the fly prior to being compiled to avoid wasting time. https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Fun Facts
* Rust code can be run on any computer without even having rust installed
* Cargo commands are consistent across all OS

## Information areas
### What is a rust *macro*?

### Cargo
Package management file is named `cargo.toml`
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

#### Adding prelude package
```
cargo add <prelude package name>
```

#### Linting Code in Rust
```
cargo clippy --fix
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

### Variables
```rust
 let mut guess = String::new();
```

```rust
let // Allows varriable to be created
```

```rust
mut //Set varriable to mutable (can change value)
```

```rust
String:new(); // Init empty string varriable to can be changed (available in the prelude)
```

#### Mutable
Variable can be edited again once initialized

#### Immutable
Varriable can not be edited again once initialized.
 * This is assigned to the varriables by default.

### Constants (Const)

Like `immutable` variables, constants are values that are bound to a name and are not allowed to change.

Constants can be declared in any scope (including global) which is useful for reusability.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

#### Example const expression
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

The constant’s name is `THREE_HOURS_IN_SECONDS` and its value is set to the result of multiplying 60 (the number of seconds in a minute) by 60 (the number of minutes in an hour) by 3 (the number of hours we want to count in this program). Rust’s naming convention for constants is to use all uppercase with underscores between words. 

The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. 

https://doc.rust-lang.org/reference/const_eval.html

Constants are valid for the entire time a program runs, within the scope in which they were declared. This property makes constants useful for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn, or the speed of light.

### Shadowing
Declaring a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

```rust
    let x = 5;

    let x = x + 1; // This shadowed the first var
```

Shadowing is different from marking a variable as `mut` **because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword**. By using `let`, we can perform a few transformations on a value but have the variable be `immutable` after those transformations have been completed.

The other difference between `mut` and `shadowing` is that because we’re effectively **creating a new variable when we use the let keyword again**, *we can change the type of the value but reuse the same name*. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:


### Receiving user input
```rust
    io::stdin()
        .read_line(&mut guess)
```
If we hadn’t imported the io library with use std::io; at the beginning of the program, we could still use the function by writing this function call as `std::io::stdin`. The stdin function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.

Next, the line `.read_line(&mut guess)` calls the `read_line` method on the standard input handle to get input from the user. We’re also passing `&mut guess` as the argument to `read_line` to tell it what **string to store the user input in**. The full job of `read_line` is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. **The string argument needs to be mutable so the method can change the string’s content.**

### Paremeter passed as reference

The `&` indicates that this argument is a **reference**, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 

References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. **Like variables, references are immutable by default**. Hence, you need to write `&mut guess` rather than ~~`&guess`~~ to make it mutable.

### How can *crates* (Packages) be installed?
