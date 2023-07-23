# Functions

Rust code uses `snake case` as the conventional style for function and variable names, in which all letters are **lowercase and underscores** separate words. Here’s a program that contains an example function definition:

```rust
// Default function for all actions
fn main() {
    println!("Hello, world!");

    another_function();
}

//Snake case sub method
fn another_function() {
    println!("Another function.");
}
```

**Note:** we defined another_function ***after*** the `main` function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

## Parameters

We can define functions to have *parameters*, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with **concrete values for those parameters**. 

Technically, the concrete values are called `arguments`, but in casual conversation, people tend to use the words `parameter` and `argument` interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

The declaration of `another_function` has one parameter named `x`. The type of `x` is specified as `i32`. When we pass `5` in to `another_function`, the `println!` macro puts `5` where the pair of curly brackets containing x was in the format string.

In function signatures, you *must* declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

// Define two parameters two be used/cast
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression. 

So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Rust is an expression-based language, this is an important distinction to understand. 

Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

- **Statements**: are `instructions` that perform some action and do not return a value.
- **Expressions**: evaluate to a `resultant` value. Let’s look at some examples.

We’ve actually already used statements and expressions. Creating a variable and assigning a value to it with the let keyword is a `statement`. In Listing 3-1, let y = 6; is a `statement`.

```rust
fn main() {
    let y = 6;
}
```

`Expressions` evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11. `Expressions` can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an `expression` that evaluates to the value 6. Calling a function is an `expression`. Calling a macro is an `expression`. A new scope block created with curly brackets is an `expression`, for example:

```rust
fn main() {
    // x + 1 is the expression, this does not use a semi colon
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
is a block that, in this case, evaluates to `4`. That value gets bound to `y` as part of the `let` statement. Note that the `x + 1` line **doesn’t have a semicolon at the end**, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. 

**If** you add a *semicolon* to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

## Functions with Return Values
Functions can return values to the code that calls them. 

**We don’t name return values, but we must declare their type after an arrow (`->`).**

In Rust, the return value of the function is `synonymous` with the value of the `final expression` (i32 in this case) in the block of the body of a function. 

You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

```rust
// Function getter that returns the value 5 as an 32 bit integer
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```