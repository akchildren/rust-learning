# Conditionals

## If expressions (statement)
An `if` expression allows you to branch your code depending on conditions.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

All `if expressions` start with the keyword `if`, followed by a `condition`.

Optionally, we can also include an `else` expression, which we chose to do here, to give the program an alternative block of code to execute should the condition evaluate to false. If you don’t provide an else expression and the condition is false, the program will just skip the if block and move on to the next bit of code.

*Conditions to a if statement **MUST** be a boolean.*

### else if

You can use multiple conditions by combining `if` and `else` in an `else if` expression. For example:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. **Rust branching construct called `match` for these cases.**

### If statement assigned to Let varriable
Because `if` is an `expression`, we can use it on the right side of a let statement to assign the outcome to a variable, as in Listing 3-2.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```